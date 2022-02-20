use super::events::NodeEventEmitter;
use futures::future;
use futures::stream::{Stream, StreamExt};
use librespot::connect::spirc::Spirc;
use librespot::core::authentication::Credentials;
use librespot::core::config::{ConnectConfig, SessionConfig};
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::discovery::DeviceType;
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::mixer::{find, MixerConfig};
use librespot::playback::player::{Player, PlayerEvent};
use librespot::protocol::authentication::AuthenticationType;
use neon::context::{Context, TaskContext};
use neon::event::Channel;
use neon::prelude::Root;
use neon::types::{Finalize, JsFunction, JsValue};
use std::boxed::Box;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tokio::runtime::Handle;
use tokio::sync::Mutex as SyncMutex;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Debug, Clone)]
pub struct NotReadyError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for NotReadyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Spotify not ready")
  }
}

pub struct SpotifyPlayerWrapper {
  pub player: SyncMutex<SpotifyPlayer>,
}

impl SpotifyPlayerWrapper {
  pub fn new(callback: Root<JsFunction>, channel: Channel) -> Arc<SpotifyPlayerWrapper> {
    Arc::new(SpotifyPlayerWrapper {
      player: SyncMutex::new(SpotifyPlayer::new(callback, channel)),
    })
  }
  pub fn new_with_emitter(emitter: NodeEventEmitter) -> Arc<SpotifyPlayerWrapper> {
    Arc::new(SpotifyPlayerWrapper {
      player: SyncMutex::new(SpotifyPlayer::new_with_emitter(emitter)),
    })
  }
}

impl Finalize for SpotifyPlayerWrapper {}

pub struct SpotifyPlayer {
  player: Option<Arc<SyncMutex<Player>>>,
  emits: Arc<NodeEventEmitter>,
}

impl SpotifyPlayer {
  pub fn new(callback: Root<JsFunction>, channel: Channel) -> SpotifyPlayer {
    SpotifyPlayer {
      player: None,
      emits: Arc::new(NodeEventEmitter::new(callback, channel)),
    }
  }

  pub fn new_with_emitter(emitter: NodeEventEmitter) -> SpotifyPlayer {
    SpotifyPlayer {
      player: None,
      emits: Arc::new(emitter),
    }
  }

  pub async fn connect_oauth(
    &mut self,
    username: String,
    oauth: String,
  ) -> Result<bool, fmt::Error> {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    let credentials = Credentials {
      username: username,
      auth_type: AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN,
      auth_data: oauth.into_bytes(),
    };

    // let credentials = Credentials::with_password(username, oauth);

    let session = Session::connect(session_config, credentials, None)
      .await
      .unwrap();

    let backend = audio_backend::find(None).unwrap();

    let (mut player, rx) = Player::new(player_config, session, None, move || {
      backend(None, audio_format)
    });

    self.player = Some(Arc::new(SyncMutex::new(player)));
    let handle = Handle::current();
    let emits = Arc::clone(&self.emits);
    let client_rcv = UnboundedReceiverStream::new(rx); // <-- this
    handle.spawn(async move {
      client_rcv
        .for_each(move |res| {
          // debug!("PlayerEvent: {:?}", res);

          // let mut state = local_state.lock().unwrap();
          // println!("EVENT: STOPPED");
          match res {
            PlayerEvent::Started { .. } => {
              // println!("EVENT: STARTED");
              emits.send_event(String::from("track_started"), |cx| {
                return cx.lock().unwrap().undefined().upcast::<JsValue>();
              });
            }
            PlayerEvent::Changed { .. } => {
              // println!("EVENT: CHANGED");
              emits.send_event(String::from("track_changed"), |cx| {
                return cx.lock().unwrap().undefined().upcast::<JsValue>();
              });
            }
            PlayerEvent::EndOfTrack { .. } => {
              // println!("EVENT: STOPPED");
              emits.send_event(String::from("track_finished"), |cx| {
                return cx.lock().unwrap().undefined().upcast::<JsValue>();
              });
            }
            PlayerEvent::Stopped { .. } => {
              emits.send_event(String::from("track_stopped"), |cx| {
                return cx.lock().unwrap().undefined().upcast::<JsValue>();
              });
            }
            _ => println!("Ain't special"),
          }
          future::ready(())
        })
        .await;
    });
    // return self;
    Ok(true)
  }

  pub fn change_event_listener(&mut self, callback: Root<JsFunction>, channel: Channel) {
    self.emits = Arc::new(NodeEventEmitter::new(callback, channel))
  }

  pub async fn connect_credencials(
    &mut self,
    credentials: librespot::discovery::Credentials,
  ) -> Result<bool, fmt::Error> {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    // let credentials = Credentials::with_password(username, oauth);

    let session = Session::connect(session_config, credentials, None)
      .await
      .unwrap();

    let backend = audio_backend::find(None).unwrap();

    let (mut player, rx) = Player::new(player_config, session.clone(), None, move || {
      backend(None, audio_format)
    });

    // let connect_config = ConnectConfig {
    //   name: String::from("me3"),
    //   autoplay: false,
    //   has_volume_ctrl: false,
    //   device_type: DeviceType::Speaker,
    //   initial_volume: Some(100),
    // };
    // let mixer_config = MixerConfig::default();
    // let mixer = find(None).unwrap();
    // let (spirc_, spirc_task_) = Spirc::new(connect_config, session, player, mixer(mixer_config));
    // spirc_task_.await;

    self.player = Some(Arc::new(SyncMutex::new(player)));
    let handle = Handle::current();
    let emits = Arc::clone(&self.emits);
    let client_rcv = UnboundedReceiverStream::new(rx); // <-- this
    handle.spawn(async move {
      client_rcv
        .for_each(move |res| {
          // debug!("PlayerEvent: {:?}", res);

          // let mut state = local_state.lock().unwrap();
          println!("EVENT: STOPPED");
          match res {
            PlayerEvent::Started { .. } => {
              emits.send_event(String::from("track_started"), |cx| {
                return cx.lock().unwrap().undefined().upcast::<JsValue>();
              });
            }
            PlayerEvent::Changed { .. } => {
              emits.send_event(String::from("track_changed"), |cx| {
                return cx.lock().unwrap().undefined().upcast::<JsValue>();
              });
            }
            PlayerEvent::Stopped { .. } | PlayerEvent::EndOfTrack { .. } => {
              println!("EVENT: STOPPED");
              emits.send_event(String::from("track_finished"), |cx| {
                return cx.lock().unwrap().undefined().upcast::<JsValue>();
              });
            }
            _ => println!("Ain't special"),
          }
          future::ready(())
        })
        .await;
    });
    // return self;
    Ok(true)
  }

  pub async fn play(&mut self, track: &String, waitForEnd: bool) -> Result<u64, NotReadyError> {
    let id = SpotifyId::from_uri(track).unwrap();
    // let mut player = self.player.borrow_mut();
    if let Some(s2) = &self.player {
      s2.lock().await.load(id, true, 0);
      // if (waitForEnd) {
      // s2.await_end_of_track().await;
      // }
      // s2.await_end_of_track().await;
      Ok(0)
    } else {
      Err(NotReadyError)
    }
  }

  pub async fn stop(&self) -> Result<u64, NotReadyError> {
    if let Some(s2) = &self.player {
      s2.lock().await.stop();
      // s2.await_end_of_track().await;
      Ok(0)
    } else {
      Err(NotReadyError)
    }
  }

  pub async fn pause(&self) -> Result<u64, NotReadyError> {
    if let Some(s2) = &self.player {
      s2.lock().await.pause();
      // s2.await_end_of_track().await;
      Ok(0)
    } else {
      Err(NotReadyError)
    }
  }
}

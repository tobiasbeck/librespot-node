use super::events::NodeEventEmitter;
use super::player::{NotReadyError, SpotifyPlayerWrapper};
use futures::stream::StreamExt;
use librespot::discovery::{Builder, DeviceType, Discovery};
use neon::context::{Context, TaskContext};
use neon::event::Channel;
use neon::handle::Handle as JsHandle;
use neon::prelude::Root;
use neon::types::{Finalize, JsFunction, JsValue};
use sha1::{Digest, Sha1};
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use tokio::runtime::Handle;
use tokio::sync::Mutex as SyncMutex;

pub struct SpotifyDiscoveryWrapper {
  pub discovery: SyncMutex<SpotifyDiscovery>,
}
impl SpotifyDiscoveryWrapper {
  pub fn new(callback: Root<JsFunction>, channel: Channel) -> Arc<SpotifyDiscoveryWrapper> {
    Arc::new(SpotifyDiscoveryWrapper {
      discovery: SyncMutex::new(SpotifyDiscovery::new(callback, channel)),
    })
  }
}

impl Finalize for SpotifyDiscoveryWrapper {}

pub struct SpotifyDiscovery {
  emits: Arc<NodeEventEmitter>,
  discovery: Option<Arc<SyncMutex<Discovery>>>,
  discoveryClose: Option<Arc<Mutex<std::sync::mpsc::Sender<bool>>>>,
}

impl SpotifyDiscovery {
  pub fn new(callback: Root<JsFunction>, channel: Channel) -> SpotifyDiscovery {
    SpotifyDiscovery {
      discovery: None,
      discoveryClose: None,
      emits: Arc::new(NodeEventEmitter::new(callback, channel)),
    }
  }

  pub async fn enable_discovery(
    &mut self,
    name: String,
    deviceType: String,
  ) -> Result<(), librespot::discovery::Error> {
    let deviceType = DeviceType::from_str(&deviceType).unwrap();
    let builder = Builder::new(hex::encode(Sha1::digest(name.as_bytes())));
    let builder = builder.name(name);
    let builder = builder.device_type(deviceType);
    let discover = builder.launch();
    match discover {
      Ok(v) => {
        self.discovery = Some(Arc::new(SyncMutex::new(v)));
        self.discoveryClose = Some(Arc::new(Mutex::new(self.start_discovery_search().unwrap())));
        Ok(())
      }
      Err(e) => Err(e),
    }
  }

  fn start_discovery_search(&self) -> Result<std::sync::mpsc::Sender<bool>, NotReadyError> {
    let handle = Handle::current();
    if let Some(discovery) = &self.discovery {
      let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
      // let d = discovery;
      let discovery = Arc::clone(discovery);
      let emits = Arc::clone(&self.emits);
      handle.spawn(async move {
        while let Some(x) = discovery.lock().await.next().await {
          match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
              println!("Terminating.");
              break;
            }
            Err(TryRecvError::Empty) => {}
          }
          let emitter = NodeEventEmitter::new_empty();
          let player = SpotifyPlayerWrapper::new_with_emitter(emitter);
          player.player.lock().await.connect_credencials(x).await;
          emits.send_event(String::from("discovered_player"), move |cx| {
            let boxed = cx.lock().unwrap().boxed(player);
            return boxed.upcast::<JsValue>();
          })
        }
      });
      return Ok(tx);
    }
    return Err(NotReadyError);
  }
}

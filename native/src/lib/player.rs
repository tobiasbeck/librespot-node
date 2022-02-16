use std::env;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::player::Player;
use librespot::protocol::authentication::AuthenticationType;
use std::error::Error;
use neon::types::Finalize;
use std::fmt;
use std::cell::RefCell;


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

pub struct SpotifyPlayer {
  player: RefCell<Option<Player>>,
}

impl SpotifyPlayer {
  pub fn new() -> SpotifyPlayer {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    SpotifyPlayer {
      player: RefCell::new(None),
    }
  }

  pub async fn connect(&self, username: String, oauth: String) {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    let credentials = Credentials{
      username: username,
      auth_type:  AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN,
      auth_data: oauth.into_bytes(),
    };

    let session = Session::connect(session_config, credentials, None)
        .await
        .unwrap();

    let backend = audio_backend::find(None).unwrap();

    let (mut player, _) = Player::new(player_config, session, None, move || {
        backend(None, audio_format)
    });

    self.player.replace(Some(player));
    // return self;
  }

  pub async fn play(&self, track: &String) -> Result<u64, NotReadyError> {
    let id = SpotifyId::from_uri(track).unwrap();
    let player = self.player.borrow_mut();
    if let Some(s2) = self.player.borrow_mut().as_mut() {
      s2.load(id, true, 0);
      Ok(0)
    } else {
      Err(NotReadyError)
    }
  }
}

impl Finalize for SpotifyPlayer { }
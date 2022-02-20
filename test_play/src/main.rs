use futures::stream::{Stream, StreamExt};
use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::discovery::{Builder, DeviceType};
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::player::Player;
use librespot::protocol::authentication::AuthenticationType;
use sha1::{Digest, Sha1};
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() {
  // let name = String::from("Me");
  // let deviceType = DeviceType::from_str("speaker").unwrap();
  // let builder = Builder::new(hex::encode(Sha1::digest(name.as_bytes())));
  // let builder = builder.name(name);
  // let builder = builder.device_type(deviceType);
  // let mut discover = builder.launch().unwrap();

  // while let Some(x) = discover.next().await {
  //   println!("Received {:?}", x);
  // }
  let session_config = SessionConfig::default();
  let player_config = PlayerConfig::default();
  let audio_format = AudioFormat::default();

  let args: Vec<_> = env::args().collect();
  if args.len() != 4 {
    eprintln!("Usage: {} USERNAME PASSWORD TRACK", args[0]);
    return;
  }
  let credentials = Credentials::with_password(&args[1], &args[2]);

  let track = SpotifyId::from_uri(&String::from("spotify:track:4eEDECI99JmE2w7H2VLUag")).unwrap();

  let backend = audio_backend::find(None).unwrap();

  println!("Connecting ..");
  let session = Session::connect(session_config, credentials, None)
    .await
    .unwrap();

  let (mut player, _) = Player::new(player_config, session, None, move || {
    backend(None, audio_format)
  });
  //println!("TRACK: {}", track.to_string())
  player.load(track, true, 0);

  println!("Playing...");

  player.stop();

  println!("Done");
}

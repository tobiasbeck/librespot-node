use neon::prelude::*;
extern crate librespot;
extern crate neon;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
// use std::thread;
// extern crate tokio_core;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

#[macro_use]

mod lib {
  pub mod discovery;
  pub mod events;
  pub mod player;
}

use lib::discovery::SpotifyDiscoveryWrapper;
use lib::player::{SpotifyPlayer, SpotifyPlayerWrapper};

fn new_player(mut cx: FunctionContext) -> JsResult<JsBox<Arc<SpotifyPlayerWrapper>>> {
  let eventCallback = cx.argument::<JsFunction>(0)?.root(&mut cx);
  let channel = cx.channel();
  let player = SpotifyPlayerWrapper::new(eventCallback, channel);
  Ok(cx.boxed(player))
}

fn player_replace_eventlistener(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let client = Arc::clone(&&cx.argument::<JsBox<Arc<SpotifyPlayerWrapper>>>(0)?);
  let eventCallback = cx.argument::<JsFunction>(1)?.root(&mut cx);
  let channel = cx.channel();
  RUNTIME.spawn(async move {
    client
      .player
      .lock()
      .await
      .change_event_listener(eventCallback, channel);
  });
  Ok(cx.undefined())
}

fn player_connect_oauth(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  // let channel = cx.channel();
  let client = Arc::clone(&&cx.argument::<JsBox<Arc<SpotifyPlayerWrapper>>>(0)?);
  let username = cx.argument::<JsString>(1)?.value(&mut cx);
  let oauth = cx.argument::<JsString>(2)?.value(&mut cx);
  let cb = cx.argument::<JsFunction>(3)?.root(&mut cx);
  let channel = cx.channel();

  RUNTIME.spawn(async move {
    let result = client
      .player
      .lock()
      .await
      .connect_oauth(username, oauth)
      .await;
    channel.send(move |mut cx| {
      let cb = cb.into_inner(&mut cx);
      let this = cx.undefined();
      match result {
        Ok(v) => {
          let args = vec![
            cx.undefined().upcast::<JsValue>(),
            cx.boolean(true).upcast(),
          ];
          cb.call(&mut cx, this, args);
        }
        Err(v) => {
          let args = vec![
            //cx.error(String::from("Failed connection")).upcast::<JsValue>(),
            cx.string("Failed connection").upcast::<JsValue>(),
            cx.undefined().upcast::<JsValue>(),
          ];
          cb.call(&mut cx, this, args);
        }
      }

      Ok(())
    });
  });

  Ok(cx.undefined())
}

fn player_play(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  // let channel = cx.channel();
  let client = Arc::clone(&&cx.argument::<JsBox<Arc<SpotifyPlayerWrapper>>>(0)?);
  let song = cx.argument::<JsString>(1)?.value(&mut cx);
  let waitforend = cx.argument::<JsBoolean>(2)?.value(&mut cx);
  let cb = cx.argument::<JsFunction>(3)?.root(&mut cx);
  let channel = cx.channel();

  RUNTIME.spawn(async move {
    let result = client.player.lock().await.play(&song, waitforend).await;
    channel.send(move |mut cx| {
      let cb = cb.into_inner(&mut cx);
      let this = cx.undefined();
      match result {
        Ok(v) => {
          let args = vec![
            cx.undefined().upcast::<JsValue>(),
            cx.boolean(true).upcast(),
          ];
          cb.call(&mut cx, this, args);
        }
        Err(v) => {
          let args = vec![
            //cx.error(String::from("Failed connection")).upcast::<JsValue>(),
            cx.string("Failed play").upcast::<JsValue>(),
            cx.undefined().upcast::<JsValue>(),
          ];
          cb.call(&mut cx, this, args);
        }
      }

      Ok(())
    });
  });
  Ok(cx.undefined())
}

fn player_pause(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  // let channel = cx.channel();
  let client = Arc::clone(&&cx.argument::<JsBox<Arc<SpotifyPlayerWrapper>>>(0)?);
  let cb = cx.argument::<JsFunction>(1)?.root(&mut cx);
  let channel = cx.channel();

  RUNTIME.spawn(async move {
    let result = client.player.lock().await.pause().await;
    channel.send(move |mut cx| {
      let cb = cb.into_inner(&mut cx);
      let this = cx.undefined();
      match result {
        Ok(v) => {
          let args = vec![
            cx.undefined().upcast::<JsValue>(),
            cx.boolean(true).upcast(),
          ];
          cb.call(&mut cx, this, args);
        }
        Err(v) => {
          let args = vec![
            //cx.error(String::from("Failed connection")).upcast::<JsValue>(),
            cx.string("Failed play").upcast::<JsValue>(),
            cx.undefined().upcast::<JsValue>(),
          ];
          cb.call(&mut cx, this, args);
        }
      }

      Ok(())
    });
  });
  Ok(cx.undefined())
}

fn player_stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  // let channel = cx.channel();
  let client = Arc::clone(&&cx.argument::<JsBox<Arc<SpotifyPlayerWrapper>>>(0)?);
  let cb = cx.argument::<JsFunction>(1)?.root(&mut cx);
  let channel = cx.channel();
  RUNTIME.spawn(async move {
    let result = client.player.lock().await.stop().await;
    channel.send(move |mut cx| {
      let cb = cb.into_inner(&mut cx);
      let this = cx.undefined();
      match result {
        Ok(v) => {
          let args = vec![
            cx.undefined().upcast::<JsValue>(),
            cx.boolean(true).upcast(),
          ];
          cb.call(&mut cx, this, args);
        }
        Err(v) => {
          let args = vec![
            //cx.error(String::from("Failed connection")).upcast::<JsValue>(),
            cx.string("Failed play").upcast::<JsValue>(),
            cx.undefined().upcast::<JsValue>(),
          ];
          cb.call(&mut cx, this, args);
        }
      }

      Ok(())
    });
  });
  Ok(cx.undefined())
}

// Discovery

fn new_discovery(mut cx: FunctionContext) -> JsResult<JsBox<Arc<SpotifyDiscoveryWrapper>>> {
  let eventCallback = cx.argument::<JsFunction>(0)?.root(&mut cx);
  let channel = cx.channel();
  let discovery = SpotifyDiscoveryWrapper::new(eventCallback, channel);
  Ok(cx.boxed(discovery))
}

fn discovery_enable(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  // let channel = cx.channel();
  let client = Arc::clone(&&cx.argument::<JsBox<Arc<SpotifyDiscoveryWrapper>>>(0)?);
  let name = cx.argument::<JsString>(1)?.value(&mut cx);
  let deviceType = cx.argument::<JsString>(2)?.value(&mut cx);
  let cb = cx.argument::<JsFunction>(3)?.root(&mut cx);
  let channel = cx.channel();
  RUNTIME.spawn(async move {
    let result = client
      .discovery
      .lock()
      .await
      .enable_discovery(name, deviceType)
      .await;
    channel.send(move |mut cx| {
      let cb = cb.into_inner(&mut cx);
      let this = cx.undefined();
      match result {
        Ok(v) => {
          let args = vec![
            cx.undefined().upcast::<JsValue>(),
            cx.boolean(true).upcast(),
          ];
          cb.call(&mut cx, this, args);
        }
        Err(v) => {
          let args = vec![
            //cx.error(String::from("Failed connection")).upcast::<JsValue>(),
            cx.string("Failed enable discovery").upcast::<JsValue>(),
            cx.undefined().upcast::<JsValue>(),
          ];
          cb.call(&mut cx, this, args);
        }
      }

      Ok(())
    });
  });
  Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("newPlayer", new_player);
  cx.export_function("connectOauth", player_connect_oauth);
  cx.export_function("playerReplaceEventListener", player_replace_eventlistener);
  cx.export_function("play", player_play);
  cx.export_function("stop", player_stop);
  cx.export_function("pause", player_pause);
  cx.export_function("newDiscovery", new_discovery);
  cx.export_function("discoveryEnable", discovery_enable);
  Ok(())
}

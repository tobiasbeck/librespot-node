use neon::prelude::*;
extern crate neon;
extern crate librespot;
// extern crate tokio_core;

#[macro_use]

mod lib {
    pub mod player;
}

use lib::player::SpotifyPlayer;


fn newClient(mut cx: FunctionContext) -> JsResult<JsBox<SpotifyPlayer>> {
  let player = SpotifyPlayer::new();
  Ok(cx.boxed(player))
}

fn connect(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  // let channel = cx.channel();
  let client = cx.argument::<JsBox<SpotifyPlayer>>(0)?;
  let username = cx.argument::<JsString>(0)?.value(&mut cx);
  let oauth = cx.argument::<JsString>(1)?.value(&mut cx);
  client.connect(username, oauth);

  // match result {
  //   Ok(v) => Ok(),
  //   Err(e) => Err(e)
  // }
  Ok(cx.undefined())
}

fn play(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  // let channel = cx.channel();
  let client = cx.argument::<JsBox<SpotifyPlayer>>(0)?;
  let song = cx.argument::<JsString>(0)?.value(&mut cx);
  client.play(&song);

  // match result {
  //   Ok(v) => Ok(),
  //   Err(e) => Err(e)
  // }
  Ok(cx.undefined())
}

// impl JsSpotify {
//       init(mut cx) {
//           // env::set_var("RUST_LOG", "debug");

//           let options = cx.argument::<JsObject>(0)?;
          
//           let username = options.get(&mut cx, "username")?.downcast::<JsString>().unwrap();
//           let oauth = options.get(&mut cx, "oauth")?.downcast::<JsString>().unwrap();

//           let player = SpotifyPlayer::new(username.value(), oauth.value());

//           Ok(Spotify {
//               player: player
//           })
//       }

//       method connect(mut cx) {
//         let mut this = ct.this();
//         let options = cx.argument::<JsObject>(0)?;

        
//         let initial_volume = options.get(&mut cx, "initialVolume")?.downcast::<JsNumber>().unwrap().value();
//       }

//       // method enableConnect(mut cx) {
//       //     let mut this = cx.this();

//       //     let options = cx.argument::<JsObject>(0)?;

//       //     let device_name = options.get(&mut cx, "deviceName")?.downcast::<JsString>().unwrap().value();
//       //     let device_type = options.get(&mut cx, "deviceType")?.downcast::<JsString>().unwrap().value();
//       //     let initial_volume = options.get(&mut cx, "initialVolume")?.downcast::<JsNumber>().unwrap().value();
//       //     let volume_ctrl = options.get(&mut cx, "volumeCtrl")?.downcast::<JsString>().unwrap().value();

//       //     {
//       //         let guard = cx.lock();
//       //         let mut spotify = this.borrow_mut(&guard);

//       //         println!("enabling connect");

//       //         spotify.player.enable_connect(device_name, DeviceType::from_str(&device_type).unwrap(), initial_volume as u16);
//       //     }

//       //     Ok(cx.undefined().upcast())
//       // }

//       method play(mut cx) {
//           let mut this = cx.this();
//           let track_id: Handle<JsString> = cx.argument::<JsString>(0)?;

//           {
//               let guard = cx.lock();
//               let mut spotify = this.borrow_mut(&guard);

//               spotify.player.play(track_id.value());
//           }

//           Ok(cx.undefined().upcast())
//       }

//       method stop(mut cx) {
//           let this = cx.this();

//           {
//               let guard = cx.lock();
//               let spotify = this.borrow(&guard);

//               spotify.player.stop();
//           }

//           Ok(cx.undefined().upcast())
//       }

//       method pause(mut cx) {
//           let this = cx.this();

//           {
//               let guard = cx.lock();
//               let spotify = this.borrow(&guard);

//               spotify.player.pause();
//           }

//           Ok(cx.undefined().upcast())
//       }

//       method seek(mut cx) {
//           let this = cx.this();
//           let position_ms: Handle<JsNumber> = cx.argument::<JsNumber>(0)?;

//           {
//               let guard = cx.lock();
//               let spotify = this.borrow(&guard);

//               spotify.player.seek(position_ms.value() as u32);
//           }

//           Ok(cx.undefined().upcast())
//       }

//       method getToken(mut cx) {
//           let this = cx.this();
//           let ctor = JsAccessToken::constructor(&mut cx)?;

//           let client_id: Handle<JsString> = cx.argument::<JsString>(0)?;
//           let scopes: Handle<JsString> = cx.argument::<JsString>(1)?;
//           let cb: Handle<JsFunction> = cx.argument::<JsFunction>(2)?;

//           let mut token: Option<AccessToken> = None;

//           {
//               let guard = cx.lock();
//               let spotify = this.borrow(&guard);

//               spotify.player.get_token(client_id.value(), scopes.value(), |tok| {
//                   match tok {
//                       Some(t) => {
//                           token = Some(AccessToken {
//                               token: t.access_token,
//                               scope: t.scope,
//                               expires_in: t.expires_in
//                           });
//                       },
//                       None => {
//                           token = None;
//                       }
//                   };
//               });
//           }

//           match token {
//               Some(t) => {
//                   let scopes = JsArray::new(&mut cx, t.scope.len() as u32);
//                   for (i, scope) in t.scope.iter().enumerate() {
//                       let val = cx.string(scope);
//                       let _ = scopes.set(&mut cx, i as u32, val);
//                   }

//                   let args: Vec<Handle<JsValue>> = vec![
//                       cx.string(t.token).upcast(),
//                       scopes.upcast(),
//                       cx.number(t.expires_in).upcast()
//                   ];

//                   let access_token_instance = ctor.construct(&mut cx, args);

//                   let cb_args: Vec<Handle<JsValue>> = vec![
//                       access_token_instance.unwrap().upcast(),
//                   ];

//                   let _ = cb.call(&mut cx, JsNull::new(), cb_args);
//               },
//               None => {
//                   let _ = cb.call(&mut cx, JsNull::new(), vec![ JsUndefined::new() ]);
//               }
//           }

//           Ok(cx.undefined().upcast())
//       }

//       // method poll(mut cx) {
//       //     let cb = cx.argument::<JsFunction>(0).expect("callback function");
//       //     let this = cx.this();

//       //     let events = cx.borrow(&this, |spotify| Arc::clone(&spotify.player.emitter.events));
//       //     let emitter = EventEmitterTask(events);

//       //     emitter.schedule(cb);

//       //     Ok(JsUndefined::new().upcast())
//       // }
//     }

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("newClient", newClient);
    cx.export_function("connect", connect);
    cx.export_function("play", play);
    Ok(())
}

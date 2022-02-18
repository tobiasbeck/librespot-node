use async_trait::async_trait;
use neon::context::{Context, TaskContext};
use neon::event::Channel;
use neon::handle::Handle;
use neon::prelude::Root;
use neon::types::{JsFunction, JsValue};
use std::marker::Sized;
use std::sync::{Arc, Mutex};

pub struct NodeEventEmitter {
  callback: Option<Arc<Mutex<Root<JsFunction>>>>,
  channel: Option<Channel>,
}

impl NodeEventEmitter {
  pub fn new(callback: Root<JsFunction>, channel: Channel) -> Self {
    Self {
      callback: Some(Arc::new(Mutex::new(callback))),
      channel: Some(channel),
    }
  }

  pub fn new_empty() -> Self {
    Self {
      callback: None,
      channel: None,
    }
  }

  pub fn send_event<F>(&self, event: String, dataCallback: F)
  where
    for<'a> F:
      FnOnce(Arc<Mutex<&mut TaskContext<'a>>>) -> (Handle<'a, JsValue>) + Send + Sync + 'static,
  {
    if let None = self.callback {
      return;
    }
    if let Some(ref callback) = &self.callback {
      let callback = Arc::clone(callback);
      self.channel.as_ref().unwrap().send(move |mut cx| {
        let cb = callback.lock().unwrap().to_inner(&mut cx);
        let this = cx.undefined();
        let cx2 = Arc::new(Mutex::new(&mut cx));
        let data = dataCallback(cx2);
        let args = vec![
          cx.string(event).upcast::<JsValue>(),
          data,
          // data.upcast::<JsValue>(),
          //cx.error(String::from("Failed connection")).upcast::<JsValue>(),
          // cx.string("Failed connection").upcast::<JsValue>(),
          // cx.undefined().upcast::<JsValue>(),
        ];
        cb.call(&mut cx, this, args);
        Ok(())
      });
    }
  }
}

use serde::Deserialize;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{EventSource, MessageEvent};
use yew::Callback;

pub struct EventSourceService {
    pub es: EventSource,
    _cb: Closure<dyn FnMut(MessageEvent) -> ()>
}

#[derive(Deserialize, Debug)]
pub struct EventSourceData {
    epoch: u64,
}

impl EventSourceService {
    pub fn new(url: String, callback: Callback<u64>) -> Self {
        let event_source = EventSource::new(&url).unwrap();

        let cb = Closure::wrap(
            Box::new(move |event: MessageEvent| {
                let text = event.data().as_string();
                let e = serde_json::from_str::<EventSourceData>(&text.as_ref().unwrap());
                callback.emit(e.unwrap().epoch);
            }) as Box<dyn FnMut(MessageEvent)>
        );

        event_source.set_onmessage(Some(cb.as_ref().unchecked_ref()));

        Self {
            es: event_source,
            _cb: cb,
        }

    }
}
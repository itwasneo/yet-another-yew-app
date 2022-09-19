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
    pub epoch: u64,
    pub pair: String,
    pub close: String,
}
impl Default for EventSourceData {
    fn default() -> Self {
        Self {
            epoch: 0,
            pair: "Unknown".to_string(),
            close: "Unknown".to_string(),
        }
    }
}

impl EventSourceService {
    pub fn new(url: String, callback: Callback<EventSourceData>) -> Self {
        let event_source = EventSource::new(&url).unwrap();

        let cb = Closure::wrap(
            Box::new(move |event: MessageEvent| {
                let text = event.data().as_string();
                let e = serde_json::from_str::<EventSourceData>(&text.as_ref().unwrap());
                callback.emit(e.unwrap());
            }) as Box<dyn FnMut(MessageEvent)>
        );

        event_source.set_onmessage(Some(cb.as_ref().unchecked_ref()));

        Self {
            es: event_source,
            _cb: cb,
        }

    }
}
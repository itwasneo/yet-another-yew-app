use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{EventSource, MessageEvent};
use yew::Callback;

pub struct EventSourceService {
    pub es: EventSource,
    _cb: Closure<dyn FnMut(MessageEvent) -> ()>
}
impl EventSourceService {
    pub fn new(url: String, callback: Callback<String>) -> Self {
        let event_source = EventSource::new(&url).unwrap();

        let cb = Closure::wrap(
            Box::new(move |event: MessageEvent| {
                let text = event.data().as_string();
                callback.emit(text.unwrap());
            }) as Box<dyn FnMut(MessageEvent)>
        );

        event_source.set_onmessage(Some(cb.as_ref().unchecked_ref()));

        Self {
            es: event_source,
            _cb: cb,
        }

    }
}
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{EventSource, MessageEvent};
use yew_agent::Dispatched;

use crate::services::event_bus::{EventBus, Req};

pub struct EvenSourceService {
    pub es: EventSource,
    _cb: Closure<dyn FnMut(MessageEvent) -> ()>
}
impl EvenSourceService {
    pub fn new(url: &str) -> Self {
        let event_source = EventSource::new(url).unwrap();
        let mut event_bus = EventBus::dispatcher();

        let cb = Closure::wrap(
            Box::new(move |event: MessageEvent| {
                let text = event.data().as_string();
                event_bus.send(Req::Msg(text.unwrap()));
            }) as Box<dyn FnMut(MessageEvent)>
        );

        event_source.set_onmessage(Some(cb.as_ref().unchecked_ref()));

        Self {
            es: event_source,
            _cb: cb,
        }

    }
}

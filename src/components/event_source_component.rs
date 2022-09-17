use yew::prelude::*;
use yew_agent::Dispatched;

use crate::services::event_source::{EventSourceService, EventSourceData};
use crate::services::event_bus::{EventBus, BusMessage, BusMessageTopic};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
    pub title: String,
    pub topic: BusMessageTopic,
}

#[function_component(EventSourceComponent)]
pub fn event_source_component(props: &Props) -> Html {

    let epoch = use_state(  || 0);
    
    let cb = {
        let epoch = epoch.clone();
        Callback::from(move |new: EventSourceData| {
            epoch.set(new.epoch);
        })
    };

    let _es = use_ref(|| EventSourceService::new(props.url.clone(), cb));

    let eb = use_mut_ref(|| EventBus::dispatcher());
    {
        let epoch = epoch.clone();
        let msg = match props.topic {
            BusMessageTopic::Main => |msg| BusMessage::Main(msg),
            BusMessageTopic::Replica => |msg| BusMessage::Replica(msg),
        };
        use_effect(move || {
            eb.as_ref().borrow_mut().send(msg(*epoch));
            || {}
        });
    }

    html! {
        <div class="center">
            <h2>
                {props.title.clone()}
            </h2>
            <p>
            { (*epoch).clone() }
            </p>
        </div>
    }

}
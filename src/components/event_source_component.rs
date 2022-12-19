use yew::prelude::*;
use yew_agent::Dispatched;

use crate::services::event_bus::{BusMessage, BusMessageTopic, EventBus};
use crate::services::event_source::{EventSourceData, EventSourceService};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
    pub title: String,
    pub topic: BusMessageTopic,
}

#[function_component(EventSourceComponent)]
pub fn event_source_component(props: &Props) -> Html {
    let data = use_state(|| EventSourceData::default());

    let cb = {
        let data = data.clone();
        Callback::from(move |new: EventSourceData| {
            data.set(new);
        })
    };

    let _es = use_ref(|| EventSourceService::new(props.url.clone(), cb));

    let eb = use_mut_ref(|| EventBus::dispatcher());
    {
        let data = data.clone();
        let msg = match props.topic {
            BusMessageTopic::Main => |msg| BusMessage::Main(msg),
            BusMessageTopic::Replica => |msg| BusMessage::Replica(msg),
        };
        use_effect(move || {
            eb.as_ref().borrow_mut().send(msg(data.epoch));
            || {}
        });
    }

    html! {
        <div class="event-source">
            <h2>
            { props.title.clone() }
            </h2>
            <p>
            { data.pair.clone() }
            </p>
            <p>
            { data.close.trim_end_matches(char::from(48)) }
            </p>
            <p>
            { data.epoch.clone() }
            </p>
        </div>
    }
}

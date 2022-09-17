use yew::prelude::*;
use yew_agent::Dispatched;

use crate::services::event_source::EventSourceService;
use crate::services::event_bus::{EventBus, Request};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
    pub title: String,
}

#[function_component(EventSourceComponent)]
pub fn event_source_component(props: &Props) -> Html {

    let epoch = use_state(|| 0);

    let eb = use_mut_ref(|| EventBus::dispatcher());

    let cb = {
        let epoch = epoch.clone();
        Callback::from(move |new: u64| {
            epoch.set(new);
            eb.as_ref().borrow_mut().send(Request::EventBusMsg(new));
        })
    };

    let _es = use_ref(|| EventSourceService::new(props.url.clone(), cb));

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
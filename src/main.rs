mod services;
mod components;

use yew::{function_component, html};

use crate::components::event_source_component::EventSourceComponent;

#[function_component(Body)]
fn body() -> Html {
    html! {
        <>
            <EventSourceComponent url="http://localhost:7070/sse"/>
        </>
    }
}

fn main() {
    yew::start_app::<Body>();
}

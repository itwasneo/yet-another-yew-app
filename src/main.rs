mod services;
mod components;

use yew::{function_component, html};

use crate::components::event_source_component::EventSourceComponent;
use crate::components::latency_component::LatencyComponent;

#[function_component(Body)]
fn body() -> Html {

    html! {
        <body>
            <div class="container">
                <h1>{"two component listening different event sources"}</h1>
                <div class="inner-container">
                    <EventSourceComponent url="http://localhost:7070/sse" title="main"/>
                    <EventSourceComponent url="http://localhost:7071/sse" title="replica"/>
                </div>
                <LatencyComponent/>
                <footer>
                    <p class="footer">{"🧡\u{00a0}wasm"}</p>
                </footer>
            </div>
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Body>();
}

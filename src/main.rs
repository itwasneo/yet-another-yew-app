mod services;
mod components;

use yew::{function_component, html};

use crate::components::event_source_component::EventSourceComponent;
use crate::components::latency_component::LatencyComponent;
use crate::services::event_bus::BusMessageTopic;

#[function_component(App)]
fn app() -> Html {

    html! {
        <body>
            <div class="container">
                <h1>{"vamos Connect!!!"}</h1>
                <div class="inner-container" id="showcase">
                    <EventSourceComponent url="http://localhost:7070/sse" title="main" topic={BusMessageTopic::Main}/>
                    <i class="arrow left"></i>
                    <LatencyComponent/>
                    <i class="arrow right"></i>
                    <EventSourceComponent url="http://localhost:7071/sse" title="replica" topic={BusMessageTopic::Replica}/>
                </div>
                <footer>
                    <p class="footer">{"yew\u{00a0}🧡\u{00a0}wasm"}</p>
                </footer>
            </div>
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

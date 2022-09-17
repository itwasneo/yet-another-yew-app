use yew::prelude::*;
use yew_agent::{Bridged};

use crate::services::event_bus::EventBus;

#[function_component(LatencyComponent)]
pub fn latency_component() -> Html{

    let main = use_state(|| 0);
    let replica = use_state(|| 0);

    {
        let main = main.clone();
        let replica = replica.clone();
        use_ref(|| EventBus::bridge(Callback::from(move |msg| {
            main.set(msg);
            replica.set(msg);
        })));
    }


    html! {
        <div class="center">
            <h2>
                {"latency"}
            </h2>
            <p>
            { (*main).clone() - (*replica).clone() }
            </p>
        </div>
    }
}
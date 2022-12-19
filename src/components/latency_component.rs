use gloo_utils::document;
use yew::prelude::*;
use yew_agent::Bridged;

use crate::services::event_bus::{BusMessage, EventBus};

enum LatencyAction {
    Wait,
    Update(u64, u64),
}

#[derive(PartialEq)]
struct LatencyState {
    latency: u64,
}

impl Default for LatencyState {
    fn default() -> Self {
        Self { latency: 0 }
    }
}
impl Reducible for LatencyState {
    type Action = LatencyAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let next_val = match action {
            LatencyAction::Wait => self.latency,
            LatencyAction::Update(main, replica) => replica.saturating_sub(main),
        };
        Self { latency: next_val }.into()
    }
}

#[function_component(LatencyComponent)]
pub fn latency_component() -> Html {
    let main = use_mut_ref(|| 0);
    let replica = use_mut_ref(|| 0);
    let latency = use_reducer_eq(LatencyState::default);

    {
        let latency = latency.clone();
        use_effect(move || {
            match document().get_element_by_id("showcase") {
                Some(elem) => {
                    match elem.set_attribute("style", &format!("gap: {}em", latency.latency)) {
                        Ok(_) => {}
                        Err(e) => log::error!("Error setting attribute: {:?}", e),
                    }
                }
                None => {
                    log::error!("#showcase not found");
                }
            }

            || match document().get_element_by_id("showcase") {
                Some(elem) => match elem.set_attribute("gap", "10em") {
                    Ok(_) => {}
                    Err(e) => log::error!("Error setting attribute: {:?}", e),
                },
                None => {
                    log::error!("#showcase not found");
                }
            }
        });
    }

    {
        let latency = latency.clone();
        use_ref(|| {
            EventBus::bridge(Callback::from(move |msg| match msg {
                BusMessage::Main(u) => {
                    *main.borrow_mut() = u;
                    latency.dispatch(LatencyAction::Wait);
                }
                BusMessage::Replica(u) => {
                    *replica.borrow_mut() = u;
                    latency.dispatch(LatencyAction::Update(
                        *main.borrow_mut(),
                        *replica.borrow_mut(),
                    ))
                }
            }))
        });
    }

    html! {
        <div class="latency">
            <h2>
                {"latency"}
            </h2>
            <p>
            { format!("{}ms", latency.latency) }
            </p>
        </div>
    }
}

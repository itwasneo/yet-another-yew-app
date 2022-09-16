mod services;
mod components;

use yew::{function_component, html, use_mut_ref, Html};

use crate::components::event_source_component::EventSourceComponent;
use crate::services::event_source::EvenSourceService;

#[function_component(Body)]
fn body() -> Html {

    let _es =  use_mut_ref(|| EvenSourceService::new("http://localhost:7070/sse"));

    let width: Vec<i32> = (0..10).collect();
    let girth: Vec<i32> = (0..100).collect();


    html! {
        <>
        {
            width.iter().map(|_| {
                html!{
                    <EventSourceComponent>
                        {
                            girth.iter().map(|_| {
                                html!
                                {
                                    <EventSourceComponent/>
                                }
                            }).collect::<Html>()
                        }
                    </EventSourceComponent>
                }
            }).collect::<Html>()
        }
        </>
    }
}

fn main() {
    yew::start_app::<Body>();
}

use yew::prelude::*;

use crate::services::event_source::EventSourceService;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
}

#[function_component(EventSourceComponent)]
pub fn event_source_component(props: &Props) -> Html {

    let color = use_state(|| "bg-yellow".to_string());

    let cb = {
        let color = color.clone();
        Callback::from(move |new: String| {
            color.set(new);
        })
    };

    let _es = use_mut_ref(|| EventSourceService::new(props.url.clone(), cb));

    html! {
        <div class={(*color).clone()}>
        </div>
    }

}
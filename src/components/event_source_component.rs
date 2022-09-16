use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;

pub enum SSE {
    HandleEvent(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

pub struct EventSourceComponent {
    msg: String,
    _producer: Box<dyn Bridge<EventBus>>,
}
impl Component for EventSourceComponent {
    type Message = SSE;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            msg: "".to_string(),
            _producer: EventBus::bridge(ctx.link().callback(SSE::HandleEvent))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SSE::HandleEvent(s) => {
                self.msg = s;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={self.msg.clone()}>
                { for ctx.props().children.iter() }
            </div>
        }
    }

}
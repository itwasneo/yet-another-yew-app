use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_source::EvenSourceService;
use crate::services::event_bus::EventBus;

pub enum SSE {
    HandleEvent(String),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub url: String,
}

pub struct EventSourceComponent {
    msg: String,
    ess: EvenSourceService,
    _producer: Box<dyn Bridge<EventBus>>,
}
impl Component for EventSourceComponent {
    type Message = SSE;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let ess = EvenSourceService::new(&ctx.props().url);
        Self {
            msg: "".to_string(),
            ess,
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>
                    { self.msg.clone() }
                </p>
            </div>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.ess.es.close();
    }

}
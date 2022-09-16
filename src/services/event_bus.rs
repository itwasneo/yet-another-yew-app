use yew_agent::{Agent, AgentLink, Context, HandlerId};
use std::collections::HashSet;

pub enum Req {
    Msg(String),
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>
}
impl Agent for EventBus {
    type Input = Req;
    type Message = ();
    type Output = String;
    type Reach = Context<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link, subscribers: HashSet::new() }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Req::Msg(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        if id.is_respondable() {
            self.subscribers.insert(id);
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}


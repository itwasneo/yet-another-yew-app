use std::collections::HashSet;
use yew_agent:: {Agent, AgentLink, Context, HandlerId};

#[derive(Debug, PartialEq, Clone)]
pub enum BusMessage {
    Main(u64),
    Replica(u64),
}

#[derive(PartialEq, Clone)]
pub enum BusMessageTopic {
    Main,
    Replica,
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}
impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = BusMessage;
    type Output = BusMessage;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message){}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
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
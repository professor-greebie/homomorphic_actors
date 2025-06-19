use log::info;
use sids::actors::actor::Actor;
use sids::actors::messages::ResponseMessage;
use sids::actors::messages::Message;
use super::trust_message::TrustMessage;


pub struct TrustActor;
impl Actor<TrustMessage> for TrustActor {
    async fn receive(&mut self, message: Message<TrustMessage>) where Self:Sized + 'static {
        if let Message {
            payload,
            stop: _,
            responder: Some(courrier), 
            blocking: _,
        } = message {

            match payload.expect("Message received without payload.") {
                TrustMessage::TrustAddKey { key } => {
                    info!("Trust key added: {}", key);
                }
                TrustMessage::TrustRemoveKey { key } => {
                    info!("Trust key removed: {}", key);
                }
            }
            let _ = courrier
                .send(ResponseMessage::Success);
        }
    }
}
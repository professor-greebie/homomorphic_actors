use env_logger::{Builder, Env};
use log::info;

use sids::actors::messages::Message;
use sids::actors::actor::Actor;
use sids::actors::messages::ResponseMessage;
use sids::actors::{get_response_channel, send_message_by_id, spawn_actor, start_actor_system};

fn get_loggings() {
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    Builder::from_env(env).init()
}


#[derive(Debug, Clone)]
enum TrustMessage {
    TrustAddKey {
        key: String,
    },
    TrustRemoveKey {
        key: String,
    },
}

struct TrustActor;
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


#[tokio::main]
 async fn main() {
    get_loggings();
    let trust_actor = TrustActor{};
    let mut actor_system = start_actor_system::<TrustMessage>();
    let (tx, rx) = get_response_channel(&mut actor_system);
    let (rtx,rrx) = get_response_channel(&mut actor_system);
    let message = Message {
        payload: Some(TrustMessage::TrustAddKey {
            key: "example_key".to_string(),
        }),
        stop: false,
        responder: Some(tx),
        blocking: None,
    };
    let rem_message = Message {
        payload: Some(TrustMessage::TrustRemoveKey {
            key: "example_key".to_string(),
        }),
        stop: false,
        responder: Some(rtx),
        blocking: None,
    };
    spawn_actor(&mut actor_system, trust_actor, Some("First Trust Actor".to_string())).await;
    send_message_by_id(&mut actor_system, 1, message).await;
    if let Ok(response) = rx.await {
        info!("Received response: {:?}", response);
    }
    send_message_by_id(&mut actor_system, 1, rem_message).await;
    if let Ok(response) = rrx.await {
        info!("Received response: {:?}", response);
    }
}

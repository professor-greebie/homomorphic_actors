use env_logger::{Builder, Env};
use log::info;

use sids::actors::messages::Message;
use trust::trust_message::TrustMessage;
use uuid::{uuid};

mod trust;

use sids::actors::{get_response_channel, send_message_by_id, spawn_actor, start_actor_system};

use crate::trust::trust_message::TrustResponse;

fn get_loggings() {
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    Builder::from_env(env).init()
}


#[tokio::main]
 async fn main() {
    get_loggings();
    let trust_actor = trust::trust_actor::TrustActor;
    let mut actor_system = start_actor_system::<TrustMessage, TrustResponse>();
    let (tx, rx) = get_response_channel(&mut actor_system);
    let (rtx,rrx) = get_response_channel(&mut actor_system);
    let (rptx, rprx) = get_response_channel(&mut actor_system);
    let message = Message {
        payload: Some(TrustMessage::CreateKeys),
        stop: false,
        responder: Some(tx),
        blocking: None,
    };
    
    spawn_actor(&mut actor_system, trust_actor, Some("First Trust Actor".to_string())).await;
    send_message_by_id(&mut actor_system, 0, message).await;
    let cuid = match rx.await {
        Ok(response) => {
            info!("Received response: {:?}", &response);
            match response {
                TrustResponse::SuccessWithData {id, data: _ } => {
                    // Assuming the data is a serialized UUID
                    id
                }
                _ => panic!("Unexpected response type"),
            }
        }
        Err(e) => panic!("Failed to receive response: {:?}", e),
    };
    let rem_message = Message {
        payload: Some(TrustMessage::RemoveKey {
            key: uuid!("1425a642-4c9d-4f5a-8458-15336f4891f9"),
        }),
        stop: false,
        responder: Some(rtx),
        blocking: None,
    };
    let rep_message = Message {
        payload: Some(TrustMessage::ReplaceKey {
            key: cuid, // Example key data
        }),
        stop: false,
        responder: Some(rptx),
        blocking: None,
    };
    
    send_message_by_id(&mut actor_system, 0, rep_message).await;
    if let Ok(response) = rprx.await {
        info!("Received response: {:?}", response);
    }
    send_message_by_id(&mut actor_system, 0, rem_message).await;
    if let Ok(response) = rrx.await {
        info!("Received response: {:?}", response);
    }
}

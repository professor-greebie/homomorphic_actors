extern crate homomorphic_actors;

use std::io::Error;

use env_logger::{Builder, Env};
use log::info;
use sids::actors::{actor_system::ActorSystem, messages::Message, spawn_actor, start_actor_system};
use homomorphic_actors::trust::{self, trust_message::{TrustMessage, TrustResponse}};
use uuid::Uuid;

fn init_logger() {
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    Builder::from_env(env).init();
}

async fn create_keys(actor_system: &mut ActorSystem<TrustMessage, TrustResponse>) -> Result<(Uuid, Vec<u8>), Error> {
    let trust_actor = trust::trust_actor::TrustActor;
    spawn_actor(actor_system, trust_actor, Some("Trust Actor".to_string())).await;
    let (tx, rx) = sids::actors::get_response_channel(actor_system);
    let message = Message {
        payload: Some(TrustMessage::CreateKeys),
        stop: false,
        responder: Some(tx),
        blocking: None,
    };
    sids::actors::send_message_by_id(actor_system, 0, message).await;
    let cuid = match rx.await {
        Ok(response) => {
            info!("Received response: {:?}", &response);
            match response {
                TrustResponse::SuccessWithData { id, data } => {
                    info!("Created keys with ID: {}", id);
                    (id, data)
                }
                _ => panic!("Unexpected response type"),
            }
        }
        Err(e) => panic!("Failed to receive response: {:?}", e),
    };
    Ok(cuid)
}

async fn replace_key(actor_system: &mut ActorSystem<TrustMessage, TrustResponse>, key_id: Uuid) -> Result<(Uuid, Vec<u8>), Error> {
    let (tx, rx) = sids::actors::get_response_channel(actor_system);
    let message = Message {
        payload: Some(TrustMessage::ReplaceKey { key: key_id }),
        stop: false,
        responder: Some(tx),
        blocking: None,
    };
    sids::actors::send_message_by_id(actor_system, 0, message).await;
    let response = rx.await.unwrap();
    let cuid = match response {
        TrustResponse::SuccessWithData { id, data } => (id, data),
        _ => panic!("Unexpected response type"),
    };
    Ok(cuid)
}

async fn remove_key(actor_system: &mut ActorSystem<TrustMessage, TrustResponse>, key_id: Uuid) -> Result<(), Error> {
    let (tx, rx) = sids::actors::get_response_channel(actor_system);
    let message = Message {
        payload: Some(TrustMessage::RemoveKey { key: key_id }),
        stop: false,
        responder: Some(tx),
        blocking: None,
    };
    sids::actors::send_message_by_id(actor_system, 0, message).await;
    let response = rx.await.unwrap();
    match response {
        TrustResponse::Success => Ok(()),
        _ => Err(Error::new(std::io::ErrorKind::Other, "Failed to remove key")),
    }
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    init_logger();
    let mut actor_system = start_actor_system::<TrustMessage, TrustResponse>();
    let cuid = create_keys(&mut actor_system).await?;
    let rep_key = cuid.0;
    let new_keys = replace_key(&mut actor_system, rep_key).await?;
    let removed = remove_key(&mut actor_system, new_keys.0).await;
    if let Err(e) = removed {
        info!("Failed to remove key: {:?}", e);
    } else {
        info!("Key removed successfully.");
    }
    Ok(())

}
use std::fs::File;
use std::io::Write;

use log::info;
use sids::actors::actor::Actor;
use sids::actors::messages::Message;
use tfhe::safe_serialization::safe_serialize;
use crate::trust::trust_message::TrustResponse;
use crate::trust::DEFAULT_KEY_SIZE;

use super::trust_message::TrustMessage;
use tfhe::{generate_keys, ConfigBuilder};




pub struct TrustActor;

impl TrustActor {
    pub fn generate_id() -> String {
        use uuid::Uuid;
        Uuid::new_v4().to_string()
    }
}



impl Actor<TrustMessage, TrustResponse> for TrustActor {
    async fn receive(&mut self, message: Message<TrustMessage, TrustResponse>) where Self:Sized + 'static {
        if let Message {
            payload,
            stop: _,
            responder: Some(courrier), 
            blocking: _,
        } = message {

            match payload.expect("Message received without payload.") {
                TrustMessage::CreateKeys => {
                    let config = ConfigBuilder::default().build();
                    let (client_key, server_key) = generate_keys(config);
                    info!("Saving server key to disk.");
                    let file_path = format!("{}/server_key_{}", super::DEFAULT_KEY_PATH, TrustActor::generate_id());
                    std::fs::create_dir_all(super::DEFAULT_KEY_PATH).expect("Failed to create key directory.");
                    info!("Server key file path: {}", file_path);
                    let mut buffer = Vec::new();
                    let _ = safe_serialize(&server_key, &mut buffer, DEFAULT_KEY_SIZE);
                    let mut file = File::create(file_path).expect("Failed to create server key file.");
                    file.write_all(&buffer).expect("Failed to write server key to file.");
                    info!("Server key saved successfully.");safe_serialize(&server_key, &mut buffer, 1 << 30).unwrap();
                    let mut client_buffer = Vec::new();
                    let _ = safe_serialize(&client_key, &mut client_buffer, DEFAULT_KEY_SIZE);
                    let _ = courrier
                        .send(TrustResponse::SuccessWithData {
                            data: client_buffer,
                        });
                },
                TrustMessage::RemoveKey { key } => {
                    info!("Trust key removed: {}", key);
                    let _ = courrier
                        .send(TrustResponse::Success);
                },
                TrustMessage::ReplaceKeys { new_key } => {
                    info!("ReplaceKeys message received.");
                    info!("New key data: {:?}", new_key);
                    let _ = courrier
                        .send(TrustResponse::Success);
                    // Add your logic for replacing keys here if needed
                },

            }
        }
    }

}


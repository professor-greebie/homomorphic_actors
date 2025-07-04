use std::fs::File;
use std::io::Write;

use log::info;
use sids::actors::actor::Actor;
use sids::actors::messages::Message;
use tfhe::{ safe_serialization::safe_serialize, ClientKey, ServerKey};
use crate::trust::trust_message::TrustResponse;
use crate::trust::DEFAULT_KEY_SIZE;

use super::trust_message::TrustMessage;
use tfhe::{generate_keys, ConfigBuilder};

pub struct TrustActor;


impl TrustActor {

    pub fn create_keys_path(id: &uuid::Uuid) -> String {
        format!("{}/server_key_{}", super::DEFAULT_KEY_PATH, id)
    }

    pub fn create_new_keys() -> (ClientKey, uuid::Uuid) {
        let config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        let server_key_id = uuid::Uuid::new_v4();
        let file_path = TrustActor::create_keys_path(&server_key_id);
        TrustActor::save_key_to_file(&server_key, &file_path, Some(DEFAULT_KEY_SIZE))
            .expect("Failed to save server key to file.");
        (client_key, server_key_id)
    }

    pub fn remove_key(key: &uuid::Uuid) -> std::io::Result<()> {
        let file_path = TrustActor::create_keys_path(key);
        std::fs::remove_file(&file_path)?;
        Ok(())
    }

    pub fn save_key_to_file(key: &ServerKey, file_path: &str, size: Option<u64>) -> std::io::Result<()> {
        let mut buffer = Vec::new();
        let _ = safe_serialize(key, &mut buffer, size.unwrap_or(DEFAULT_KEY_SIZE));
        let mut file = File::create(file_path)?;
        file.write_all(&buffer)?;
        Ok(())
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
                    let (client_key, key_id) = TrustActor::create_new_keys();
                    let mut client_buffer = Vec::new();
                    let _ = safe_serialize(&client_key, &mut client_buffer, DEFAULT_KEY_SIZE);
                    let _ = courrier
                        .send(TrustResponse::SuccessWithData {
                            id: key_id,
                            data: client_buffer,
                        });
                },
                TrustMessage::RemoveKey { key } => {
                    info!("Trust key removed: {}", &key);
                    let file_path = TrustActor::create_keys_path(&key);
                    if std::fs::remove_file(&file_path).is_err() {
                        info!("Failed to remove key file: {}", &file_path);
                        let _ = courrier
                            .send(TrustResponse::Failure {
                                error: format!("Failed to remove key file: {}", &file_path),
                            });

                    } else {
                        info!("Key file removed successfully: {}", &file_path);
                        let _ = courrier
                        .send(TrustResponse::Success);

                    }
                    
                },
                TrustMessage::ReplaceKey { key  } => {
                    info!("ReplaceKeys message received.");
                    info!("New key data: {:?}", key.to_string());
                    TrustActor::remove_key(&key)
                        .expect("Failed to remove key.");
                    let (client, id) = TrustActor::create_new_keys();
                    let mut buffer = vec![];
                    let _ = safe_serialize(&client, &mut buffer, DEFAULT_KEY_SIZE);
                    
                    let _ = courrier
                        .send(TrustResponse::SuccessWithData { id: id , data: buffer });
                    // Add your logic for replacing keys here if needed
                },

            }
        }
    }

}


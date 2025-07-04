#[derive(Debug, Clone)]
pub enum TrustMessage {
    CreateKeys,
    ReplaceKey {
        key: uuid::Uuid,
    },
    RemoveKey {
        key: uuid::Uuid,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrustResponse {
    Success,
    SuccessWithData {
        id: uuid::Uuid,
        data: Vec<u8>,
    },
    Failure {
        error: String,
    },
}
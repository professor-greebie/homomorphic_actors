#[derive(Debug, Clone)]
pub enum TrustMessage {
    CreateKeys,
    ReplaceKeys {
        new_key: Vec<u8>,
    },
    RemoveKey {
        key: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrustResponse {
    Success,
    SuccessWithData {
        data: Vec<u8>,
    }
}
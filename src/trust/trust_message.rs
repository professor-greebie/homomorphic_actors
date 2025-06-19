#[derive(Debug, Clone)]
pub enum TrustMessage {
    TrustAddKey {
        key: String,
    },
    TrustRemoveKey {
        key: String,
    },
}
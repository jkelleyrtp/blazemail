use google_gmail1::api::Message;

/// A unified type over different email providers that uses pre-extracted data
pub struct IndexedMessage {
    pub message: Message,
    pub index: usize,
    pub name: String,
    pub email: String,
    pub snippet: String,
}

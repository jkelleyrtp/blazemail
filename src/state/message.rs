use google_gmail1::api::Message;

pub struct IndexedMessage {
    pub message: Message,
    pub index: usize,
    pub name: String,
    pub email: String,
    pub snippet: String,
}

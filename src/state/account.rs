pub struct LoggedInAccount {
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub provider: AccountProvider,
}

pub enum AccountProvider {
    Gmail,
    Outlook,
    Yahoo,
    Other,
}

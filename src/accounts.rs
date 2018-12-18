pub struct Account {
    pub user_id: String,
    pub in_wallet: Option<String>,
    pub out_wallet: Option<String>,
    pub balance: i64,
}

impl Account {
    pub fn find_or_new(user_id: &str) -> Account {
        // TODO: Lookup in DB
        Account {
            user_id: user_id.to_string(),
            in_wallet: None,
            out_wallet: None,
            balance: 0,
        }
    }
}

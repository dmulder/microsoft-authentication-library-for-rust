pub struct PublicClientApplication {
    client_id: String,
    authority: String,
}

impl PublicClientApplication {
    pub fn new(client_id: &str, authority: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            authority: authority.to_string(),
        }
    }
}

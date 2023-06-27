use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    id: String,
    url: String,
    webhook_secret: String,
}

impl Project {
    pub fn new(id: String, url: String, webhook_secret: String) -> Self {
        Self {
            id,
            url,
            webhook_secret,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn webhook_secret(&self) -> &String {
        &self.webhook_secret
    }
}

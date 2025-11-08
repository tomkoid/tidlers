#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TidalPage {
    access_token: Option<String>,
}

impl TidalPage {
    pub fn new() -> Self {
        Self { access_token: None }
    }

    pub fn set_access_token(&mut self, access_token: String) {
        self.access_token = Some(access_token);
    }

    pub fn is_access_token_set(&self) -> bool {
        self.access_token.is_some()
    }

    pub fn r_get(&self, path: &str) {
        println!("path: {path}");
    }
}

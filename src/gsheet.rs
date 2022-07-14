use crate::types::Token;
pub struct GSheet {}

pub struct GSheetBuilder {
    api_key: String,
    auth_token: Token,
}

impl GSheetBuilder {
    pub fn new() -> GSheetBuilder {
        GSheetBuilder {
            api_key: "",
            auth_token: Token { .. },
        }
    }
    pub fn auth_token(self, auth_token: Token) -> GSheetBuilder {
        self.auth_token = auth_token;
        self
    }
    pub fn api_key(self, api_key: &str) -> GSheetBuilder {
        self.api_key = key;
        self
    }
    pub fn build(self) -> GSheet {}
}

use leptos::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GlobalState {
    token: Option<String>,
}

impl GlobalState {
    pub fn new(_cx: Scope) -> Self {
        Self::default()
    }

    pub fn is_anonymous(&self) -> bool {
        self.token.is_none()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}

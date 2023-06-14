use leptos::{ServerFnError, *};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// base url is not used in client code

#[allow(unused)]
static BASE_URL: OnceLock<String> = OnceLock::new();

#[allow(unused)]
fn base_url() -> &'static str {
    BASE_URL.get_or_init(|| {
        std::env::var("VOTE_BACKEND_URL").unwrap_or("http://localhost:8000".to_string())
    })
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteOption {
    pub id: String,
    pub label: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Topic {
    pub id: String,
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub options: Vec<VoteOption>,
    pub stage: String,
}

#[server(GetTopics, "/api")]
pub async fn get_topics() -> Result<Vec<Topic>, ServerFnError> {
    let resp = reqwest::get(format!("{}/topic", base_url()))
        .await
        .unwrap()
        .json::<Vec<Topic>>()
        .await
        .unwrap();

    Ok(resp)
}

#[server(GetOneTopic, "/api")]
pub async fn get_one_topic(id: String) -> Result<Topic, ServerFnError> {
    let resp = reqwest::get(format!("{}/topic/{id}", base_url()))
        .await
        .unwrap()
        .json::<Topic>()
        .await
        .unwrap();

    Ok(resp)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateOptionInput {
    pub label: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTopicInput {
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub options: Vec<CreateOptionInput>,
}

#[server(CreateTopic, "/api")]
pub async fn create_topic(input: CreateTopicInput) -> Result<(), ServerFnError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/topic", base_url()))
        .json(&input)
        .send()
        .await
        .unwrap();

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuth2PasswordRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
}

#[server(CreateAccessToken, "/api")]
pub async fn create_access_token(input: OAuth2PasswordRequest) -> Result<Token, ServerFnError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/auth/token", base_url()))
        .form(&input)
        .send()
        .await
        .unwrap();
    if resp.status() != reqwest::StatusCode::OK {
        return Err(ServerFnError::ServerError(format!(
            "login failed: {resp:?}"
        )));
    }

    let token = resp.json::<Token>().await.unwrap();
    Ok(token)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignupInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[server(Signup, "/api")]
pub async fn signup(input: SignupInput) -> Result<(), ServerFnError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/user/signup", base_url()))
        .json(&input)
        .send()
        .await
        .unwrap();

    if resp.status() != reqwest::StatusCode::OK {
        return Err(ServerFnError::ServerError(format!(
            "signup failed: {resp:?}"
        )));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
}

#[server(GetMe, "/api")]
pub async fn get_me(token: String) -> Result<User, ServerFnError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/me", base_url()))
        .bearer_auth(token)
        .send()
        .await
        .unwrap();
    if resp.status() != reqwest::StatusCode::OK {
        return Err(ServerFnError::ServerError(format!("auth failed: {resp:?}")));
    }
    let user = resp.json::<User>().await.unwrap();

    Ok(user)
}

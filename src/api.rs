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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VoteOption {
    pub id: String,
    pub label: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
pub struct CreateVoteInput {
    pub topic_id: String,
    pub option_id: String,
}

#[server(CreateVote, "/api")]
pub async fn create_vote(token: String, input: CreateVoteInput) -> Result<(), ServerFnError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/vote", base_url()))
        .bearer_auth(token)
        .json(&input)
        .send()
        .await
        .unwrap();

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vote {
    pub id: String,
    pub username: String,
    pub topic_id: String,
    pub option_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetMyVoteInput {
    pub topic_id: String,
}

#[server(GetMyVote, "/api")]
pub async fn get_my_vote(token: String, input: GetMyVoteInput) -> Result<Vote, ServerFnError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/topic/{}/my-vote", base_url(), input.topic_id))
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    if resp.status() != reqwest::StatusCode::OK {
        return Err(ServerFnError::ServerError(format!(
            "get my vote failed: {resp:?}"
        )));
    }

    let vote = resp
        .json::<Vote>()
        .await
        .map_err(|e| ServerFnError::Deserialization(e.to_string()))?;

    Ok(vote)
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetCommentsInput {
    pub topic_id: String,
}

#[server(GetComments, "/api")]
pub async fn get_comments(input: GetCommentsInput) -> Result<Vec<Comment>, ServerFnError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/comment", base_url()))
        .query(&input)
        .send()
        .await
        .unwrap();
    if resp.status() != reqwest::StatusCode::OK {
        return Err(ServerFnError::ServerError(format!(
            "get comments failed: {resp:?}"
        )));
    }
    let comments = resp
        .json::<Vec<Comment>>()
        .await
        .map_err(|e| ServerFnError::Deserialization(e.to_string()))?;

    Ok(comments)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCommentInput {
    pub topic_id: String,
    pub content: String,
}

#[server(CreateComment, "/api")]
pub async fn create_comment(token: String, input: CreateCommentInput) -> Result<(), ServerFnError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/comment", base_url()))
        .bearer_auth(token)
        .json(&input)
        .send()
        .await
        .unwrap();

    if resp.status() != reqwest::StatusCode::OK {
        return Err(ServerFnError::ServerError(format!(
            "create comment failed: {resp:?}"
        )));
    }

    Ok(())
}

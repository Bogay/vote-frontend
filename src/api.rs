use leptos::{ServerFnError, *};
use serde::{Deserialize, Serialize};

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

const BASE_URL: &'static str = "http://localhost:8000";

#[server(GetTopics, "/api")]
pub async fn get_topics() -> Result<Vec<Topic>, ServerFnError> {
    let resp = reqwest::get(format!("{}/topic", BASE_URL))
        .await
        .unwrap()
        .json::<Vec<Topic>>()
        .await
        .unwrap();

    Ok(resp)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        .post(format!("{BASE_URL}/topic"))
        .json(&input)
        .send()
        .await
        .unwrap();

    Ok(())
}

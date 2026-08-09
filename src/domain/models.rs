use serde::{
    Deserialize, 
    Serialize
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub _email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}
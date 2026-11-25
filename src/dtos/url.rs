use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUrlReq {
    pub full_url: String,
}

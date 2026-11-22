use serde::Serialize;

#[derive(Serialize, Clone, Default)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

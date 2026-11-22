use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use crate::models::Task;
use crate::{seed_task};

pub(crate) async fn get_tasks(Query(params): Query<HashMap<String, String>>) -> Json<Vec<Task>> {
    println!(
        "provided params: \n{{\n\t{}\n}}",
        params
            .iter()
            .map(|entry| {
                format!(
                    "{}: {\
            }",
                    entry.0, entry.1
                )
            })
            .collect::<Vec<_>>()
            .join(",\n\t")
    );

    Json(seed_task())
}
pub(crate) async fn get_task_by_id(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    seed_task()
        .into_iter()
        .find(|task| task.id == id)
        .map(|t| {Json(t)})
        .ok_or(StatusCode::NOT_FOUND)
}
// async fn get_task_by_id(payload: Result<Path<u32>, PathRejection>) -> String {
//     match payload {
//         Ok(Path(id)) => format!("you asked for task {id}"),
//         Err(err) => {
//             println!("error: {err}");
//             "error".into()
//         }
//     }
// }


#[derive(Deserialize)]
pub struct TaskDTO {
    pub title: String,
}

pub(crate) async fn create_task(Json(dto): Json<TaskDTO>) {}
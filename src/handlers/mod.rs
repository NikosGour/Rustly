pub mod tasks;

pub(crate) async fn root() -> &'static str {
    "hello world"
}

pub(crate) async fn health() -> &'static str {
    "ok"
}


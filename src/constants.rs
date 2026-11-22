pub(crate) enum Env {
    PORT,
}

//TODO: maybe write an attribute or macro to automate this
impl Env {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Env::PORT => "PORT",
        }
    }
}

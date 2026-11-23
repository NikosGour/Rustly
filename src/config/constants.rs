pub(crate) enum Env {
    PORT,
    PostgresMaxConnections,
    PostgresUrl,
    PostgresDb,
    PostgresRootUsername,
    PostgresRootPassword,
    DatabaseUrl,
}

//TODO: maybe write an attribute or macro to automate this
impl Env {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Env::PORT => "PORT",
            Env::PostgresMaxConnections => "POSTGRES_MAX_CONNECTIONS",
            Env::PostgresUrl => "POSTGRES_URL",
            Env::PostgresDb => "POSTGRES_DB",
            Env::PostgresRootUsername => "POSTGRES_ROOT_USERNAME",
            Env::PostgresRootPassword => "POSTGRES_ROOT_PASSWORD",
            Env::DatabaseUrl => "DATABASE_URL"
        }
    }

    pub(crate) fn not_defined(self) -> String {
        self.as_str().to_owned() + " is not defined"
    }
}

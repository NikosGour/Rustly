pub(crate) enum Env {
    PORT,
    PostgresMaxConnections,
    _PostgresUrl,
    _PostgresDb,
    _PostgresRootUsername,
    _PostgresRootPassword,
    DatabaseUrl,
}

//TODO: maybe write an attribute or macro to automate this
impl Env {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Env::PORT => "PORT",
            Env::PostgresMaxConnections => "POSTGRES_MAX_CONNECTIONS",
            Env::_PostgresUrl => "POSTGRES_URL",
            Env::_PostgresDb => "POSTGRES_DB",
            Env::_PostgresRootUsername => "POSTGRES_ROOT_USERNAME",
            Env::_PostgresRootPassword => "POSTGRES_ROOT_PASSWORD",
            Env::DatabaseUrl => "DATABASE_URL"
        }
    }

    pub(crate) fn not_defined(self) -> String {
        self.as_str().to_owned() + " is not defined"
    }
}

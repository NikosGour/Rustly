use crate::controllers::UrlController;
use crate::repositories::UrlRepository;
use crate::services::UrlService;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState<'db_pool> {
    repositories: Repositories<'db_pool>,
    services: Services<'db_pool>,
    pub controllers: Controllers<'db_pool>,
}

struct Repositories<'db_pool> {
    url_repository: Arc<UrlRepository<'db_pool>>,
}

struct Services<'db_pool> {
    url_service: Arc<UrlService<'db_pool>>,
}

pub struct Controllers<'db_pool> {
    pub url_controller: UrlController<'db_pool>,
}

impl<'db_pool> AppState<'db_pool> {
    pub fn new(pool: &'db_pool PgPool) -> AppState<'db_pool> {
        let repositories = Self::create_repositories(pool);
        let services = Self::create_services(&repositories);
        let controllers = Self::create_controllers(&services);

        AppState {
            repositories,
            services,
            controllers,
        }
    }

    fn create_repositories(pool: &PgPool) -> Repositories<'_> {
        let url_repository = Arc::new(UrlRepository { pool });
        Repositories { url_repository }
    }

    fn create_services(repositories: &Repositories<'db_pool>) -> Services<'db_pool> {
        let url_service = Arc::new(UrlService {
            url_repository: Arc::clone(&repositories.url_repository),
        });

        Services { url_service }
    }

    fn create_controllers(services: &Services<'db_pool>) -> Controllers<'db_pool> {
        let url_controller = UrlController {
            url_service: Arc::clone(&services.url_service),
        };

        Controllers { url_controller }
    }
}

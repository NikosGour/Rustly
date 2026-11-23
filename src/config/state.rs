use crate::controllers::UrlController;
use crate::repositories::UrlRepository;
use crate::services::UrlService;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    _repositories: Repositories,
    _services: Services,
    pub controllers: Controllers,
}

#[derive(Clone)]
struct Repositories {
    url_repository: Arc<UrlRepository>,
}

#[derive(Clone)]
struct Services {
    url_service: Arc<UrlService>,
}

#[derive(Clone)]
pub struct Controllers {
    pub url_controller: UrlController,
}

impl AppState {
    pub fn new(pool: PgPool) -> AppState {
        let pool = Arc::new(pool);
        let repositories = Self::create_repositories(Arc::clone(&pool));
        let services = Self::create_services(&repositories);
        let controllers = Self::create_controllers(&services);

        AppState {
            _repositories: repositories,
            _services: services,
            controllers,
        }
    }

    fn create_repositories(pool: Arc<PgPool>) -> Repositories {
        let url_repository = Arc::new(UrlRepository { pool });
        Repositories { url_repository }
    }

    fn create_services(repositories: &Repositories) -> Services {
        let url_service = Arc::new(UrlService {
            url_repository: Arc::clone(&repositories.url_repository),
        });

        Services { url_service }
    }

    fn create_controllers(services: &Services) -> Controllers {
        let url_controller = UrlController {
            url_service: Arc::clone(&services.url_service),
        };

        Controllers { url_controller }
    }
}

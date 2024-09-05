mod graphql;
mod rest;

use warp::Filter;
use crate::storage::Storage;
use crate::config::Config;

pub struct ApiServer {
    storage: Storage,
    config: Config,
}

impl ApiServer {
    pub fn new(storage: Storage, config: &Config) -> Self {
        Self {
            storage,
            config: config.clone(),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let graphql_route = graphql::schema(self.storage.clone())
            .and_then(graphql::graphql_handler);

        let rest_routes = rest::routes(self.storage.clone());

        let routes = graphql_route.or(rest_routes);

        warp::serve(routes)
            .run(([127, 0, 0, 1], 8080))
            .await;

        Ok(())
    }
}
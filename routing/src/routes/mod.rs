mod hello_world;

use axum::Router;
use axum::routing::patch;
use hello_world::hello_world;

pub fn create_routes() -> Router<> {
    Router::new().route("/", patch(hello_world))
}
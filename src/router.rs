use salvo::Router;

use crate::api::token::create_token;

pub fn build() -> Router {
    Router::with_path("api").push(Router::with_path("token").get(create_token))
}

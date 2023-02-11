use actix_web::web;

use crate::user::infrastructure::controller::add_user_handler::add_user_handler;

pub fn add_routes(config: &mut web::ServiceConfig) {
    config
        .route("/api/v1/users", web::get().to(add_user_handler));
}


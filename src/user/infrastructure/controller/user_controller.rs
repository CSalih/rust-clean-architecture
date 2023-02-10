use actix_web::{HttpResponse, Responder, web};

pub fn add_routes(config: &mut web::ServiceConfig) {
    config
        .route("/api/v1/users", web::get().to(add_user_handler));
}

async fn add_user_handler() -> impl Responder  {
    // TODO: wrong http status but funny
    HttpResponse::ExpectationFailed().body("To be implemented!")
}
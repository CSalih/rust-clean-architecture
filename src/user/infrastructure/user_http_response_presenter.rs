use actix_web::HttpResponse;

use crate::common::presenter::Presenter;
use crate::user::domain::model::User;

pub struct UserHttpResponsePresenter {
    response: Option<HttpResponse>,
}

impl UserHttpResponsePresenter {
    pub fn new() -> Self {
        UserHttpResponsePresenter {
            response: None
        }
    }
    pub fn response(self) -> HttpResponse {
        self.response.expect("Response is None! Please call success() or error() before use")
    }
}

impl Presenter<User> for UserHttpResponsePresenter {
    fn success(&mut self, data: User) {
        self.response = Some(HttpResponse::Created().json(&data));
    }
    fn error(&mut self, error: String) {
        self.response = Some(HttpResponse::InternalServerError().body(error));
    }
}
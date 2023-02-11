use actix_web::{HttpResponse, Responder, web};

use crate::common::presenter::Presenter;
use crate::user::core::use_case::add_user::{AddUserCommand, AddUserUseCase};
use crate::user::core::use_case::add_user_interceptor::AddUserInterceptor;
use crate::user::domain::user::User;
use crate::user::infrastructure::in_memory_user_repository::InMemoryUserRepository;

pub fn add_routes(config: &mut web::ServiceConfig) {
    config
        .route("/api/v1/users", web::get().to(add_user_handler));
}

async fn add_user_handler() -> impl Responder {
    let user_exists_repository = InMemoryUserRepository::new();
    let mut add_user_repository = InMemoryUserRepository::new();
    let mut add_user_use_case = AddUserInterceptor::new(
        &mut add_user_repository,
        &user_exists_repository,
    );
    let command = AddUserCommand::new(
        String::from("csalih"),
        String::from("new"),
    );
    let mut presenter = PrintLinePresenter {
        response: None
    };
    add_user_use_case.execute(&command, &mut presenter);

    presenter.response.unwrap()
}


struct PrintLinePresenter {
    response: Option<HttpResponse>,
}

impl Presenter<User> for PrintLinePresenter {
    fn success(&mut self, data: User) {
        self.response = Some(
            HttpResponse::Created().body(String::from("Username: ") + &*String::from(&data.username))
        );
    }
    fn error(&mut self, error: String) {
        self.response = Some(
            HttpResponse::InternalServerError().body(error)
        );
    }
}
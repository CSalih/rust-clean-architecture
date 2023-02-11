use actix_web::Responder;

use crate::user::core::use_case::add_user::{AddUserCommand, AddUserUseCase};
use crate::user::core::use_case::add_user_interceptor::AddUserInterceptor;
use crate::user::infrastructure::in_memory_user_repository::InMemoryUserRepository;
use crate::user::infrastructure::user_http_response_presenter::UserHttpResponsePresenter;

pub async fn add_user_handler() -> impl Responder {
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
    let mut presenter = UserHttpResponsePresenter::new();
    add_user_use_case.execute(&command, &mut presenter);

    presenter.response()
}

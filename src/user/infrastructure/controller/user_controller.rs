use std::collections::HashMap;

use actix_web::{HttpResponse, Responder, web};

use crate::common::presenter::Presenter;
use crate::common::problem::Problem;
use crate::user::core::use_case::add_user::{AddUserCommand, AddUserGateway, AddUserUseCase, UserExistsGateway, UserExistsQuery};
use crate::user::core::use_case::add_user_interceptor::AddUserInterceptor;
use crate::user::domain::user::User;

pub fn add_routes(config: &mut web::ServiceConfig) {
    config
        .route("/api/v1/users", web::get().to(add_user_handler));
}

async fn add_user_handler() -> impl Responder {
    let repository = InMemoryUserRepository::new();
    let add_user_use_case = AddUserInterceptor::new(&repository, &repository);
    let command = AddUserCommand::new(
        String::from("csalih"),
        String::from("new"),
    );
    let mut presenter = PrintLinePresenter {
        response: None
    };
    add_user_use_case.execute(command, &mut presenter);

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

struct InMemoryUserRepository {
    data: HashMap<String, User>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        InMemoryUserRepository {
            data: HashMap::new()
        }
    }
}

impl AddUserGateway for InMemoryUserRepository {
    fn add_user(&self, command: AddUserCommand) -> Result<User, Problem> {
        // let user = User {
        //     username: String::from(&command.username),
        //     status: String::from(&command.status),
        // };
        // self.data.insert(String::from(&command.username), user);

        return Ok(User {
            username: String::from(&command.username),
            status: String::from(&command.status),
        });
    }
}

impl UserExistsGateway for InMemoryUserRepository {
    fn exists(&self, _: UserExistsQuery) -> bool {
        return false;
    }
}
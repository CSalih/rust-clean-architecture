use crate::common::presenter::Presenter;
use crate::common::problem::Problem;
use crate::user::domain::user::User;

pub trait AddUserUseCase {
    fn execute(&mut self, command: &AddUserCommand, presenter: &mut impl Presenter<User>);
}

// Add gateway
pub trait AddUserGateway {
    fn add_user(&mut self, command: &AddUserCommand) -> Result<User, Problem>;
}

pub struct AddUserCommand {
    pub username: String,
    pub status: String,
}

impl AddUserCommand {
    pub fn new(username: String, status: String) -> Self {
        AddUserCommand { username, status }
    }
}

// Exits gateway
pub trait UserExistsGateway {
    fn exists(&self, query: &UserExistsQuery) -> bool;
}

pub struct UserExistsQuery {
    pub username: String,
}

impl UserExistsQuery {
    pub fn new(username: String) -> Self {
        UserExistsQuery { username }
    }
}
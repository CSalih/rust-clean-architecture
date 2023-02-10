use std::error::Error;

use crate::common::presenter::Presenter;
use crate::user::domain::model::User;

pub trait AddUserUseCase {
    fn execute(&mut self, command: &AddUserCommand, presenter: &mut impl Presenter<User>);
}

// Add gateway
pub trait AddUserGateway {
    fn add_user(&mut self, command: &AddUserCommand) -> Result<User, Box<dyn Error>>;
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
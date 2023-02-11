use std::collections::HashMap;

use crate::common::problem::Problem;
use crate::user::core::use_case::add_user::{AddUserCommand, AddUserGateway, UserExistsGateway, UserExistsQuery};
use crate::user::domain::model::User;

pub struct InMemoryUserRepository {
    data: HashMap<String, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            data: HashMap::new()
        }
    }
}

impl AddUserGateway for InMemoryUserRepository {
    fn add_user(&mut self, command: &AddUserCommand) -> Result<User, Problem> {
        let user = User {
            username: String::from(&command.username),
            status: String::from(&command.status),
        };
        self.data.insert(String::from(&command.username), user);

        return Ok(User {
            username: String::from(&command.username),
            status: String::from(&command.status),
        });
    }
}

impl UserExistsGateway for InMemoryUserRepository {
    fn exists(&self, query: &UserExistsQuery) -> bool {
        self.data.contains_key(&query.username)
    }
}
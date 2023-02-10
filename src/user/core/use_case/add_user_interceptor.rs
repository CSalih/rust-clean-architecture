use crate::common::presenter::Presenter;
use crate::user::core::use_case::add_user::{AddUserCommand, AddUserGateway, AddUserUseCase, UserExistsGateway, UserExistsQuery};
use crate::user::domain::model::User;

pub struct AddUserInterceptor<'a> {
    user_exists_gateway: &'a dyn UserExistsGateway,
    add_user_gateway: &'a mut dyn AddUserGateway,
}

impl<'a> AddUserInterceptor<'a> {
    pub fn new(add_user_gateway: &'a mut dyn AddUserGateway, user_exists_gateway: &'a dyn UserExistsGateway) -> Self {
        AddUserInterceptor { add_user_gateway, user_exists_gateway }
    }
}

impl<'a> AddUserUseCase for AddUserInterceptor<'a> {
    fn execute(&mut self, command: &AddUserCommand, presenter: &mut impl Presenter<User>) {
        let user_exists = self.user_exists_gateway.exists(&UserExistsQuery::new(
            String::from(&command.username)
        ));
        if user_exists {
            presenter.error(String::from("User exists! ...."));
            return;
        }

        let user_result = self.add_user_gateway.add_user(&command);
        match user_result {
            Ok(user) => presenter.success(user),
            Err(_) => presenter.error(String::from("err")),
        };
    }
}
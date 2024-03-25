use super::models::user_model::{NewUser, User};
use super::repositories::user_repository::UserRepository;
use diesel::result::Error as DieselError;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            user_repository: UserRepository::new(),
        }
    }

    pub fn create_user(&self, new_user: NewUser) -> Result<User, DieselError> {
        self.user_repository.create_user(new_user)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, DieselError> {
        self.user_repository.get_user_by_id(user_id)
    }

    // pub fn update_user(
    //     &self,
    //     user_id: i32,
    //     username: String,
    //     email: String,
    // ) -> Result<User, DieselError> {
    //     self.user_repository.update_user(user_id, username, email)
    // }

    // pub fn delete_user(&self, user_id: i32) -> Result<usize, DieselError> {
    //     self.user_repository.delete_user(user_id)
    // }
}

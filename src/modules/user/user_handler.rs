use super::user_service::UserService;
use crate::modules::user::models::user_model::{NewUser, UpdateUser, User};

use rocket::{delete, get, http::Status, post, response::status};

// use diesel::sql_types::Json;
use rocket::serde::json::Json;

pub struct UserHandler {
    user_service: UserService,
}

impl UserHandler {
    pub fn new() -> Self {
        UserHandler {
            user_service: UserService::new(),
        }
    }

    pub async fn create_user(&self, new_user: Json<NewUser>) -> status::Custom<&'static str> {
        match self.user_service.create_user(new_user.into_inner()) {
            Ok(_) => status::Custom(Status::Created, "User created successfully"),
            Err(_) => status::Custom(Status::InternalServerError, "Failed to create user"),
        }
    }

    // #[get("/user/<user_id>", format = "json")]
    pub async fn get_user(&self, user_id: i32) -> status::Custom<String> {
        match self.user_service.get_user_by_id(user_id) {
            Ok(Some(user)) => status::Custom(Status::Ok, format!("User: {:?}", user)),
            Ok(None) => status::Custom(Status::NotFound, "User not found".to_string()),
            Err(_) => status::Custom(
                Status::InternalServerError,
                "Failed to get user".to_string(),
            ),
        }
    }

    // #[put("/user/<user_id>", format = "json", data = "<update_user>")]
    // pub async fn update_user(user_id: i32, update_user: Json<UpdateUser>, user_handler: rocket::State<Self>) -> status::Custom<&'static str> {
    //     match user_handler.user_service.update_user(user_id, update_user.into_inner()) {
    //         Ok(_) => status::Custom(Status::Ok, "User updated successfully"),
    //         Err(_) => status::Custom(Status::InternalServerError, "Failed to update user"),
    //     }
    // }

    // #[delete("/user/<user_id>", format = "json")]
    pub async fn delete_user(&self, user_id: i32) -> status::Custom<&'static str> {
        match self.user_service.delete_user(user_id) {
            Ok(_) => status::Custom(Status::NoContent, "User deleted successfully"),
            Err(_) => status::Custom(Status::InternalServerError, "Failed to delete user"),
        }
    }
}

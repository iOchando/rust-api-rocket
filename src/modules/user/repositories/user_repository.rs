use super::super::models::user_model::{NewUser, UpdateUser, User};
use super::super::schemas::user_schema::users;
use crate::config::database::DatabaseRepository;
// use chrono;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub struct UserRepository {
    connection: Result<PgConnection, ConnectionError>,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {
            connection: DatabaseRepository::new().connection,
        }
    }

    pub fn create_user(&self, new_user: NewUser) -> Result<User, DieselError> {
        let connection = DatabaseRepository::new();
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(connection.connection.as_mut())
    }

    pub fn get_user_by_id(&mut self, user_id: i32) -> Result<Option<User>, DieselError> {
        users::table
            .find(user_id)
            .first::<User>(&mut self.connection)
            .optional()
    }

    // pub fn update_user(
    //     &mut self,
    //     user_id: i32,
    //     username: String,
    //     email: String,
    // ) -> Result<User, DieselError> {
    //     // let updated_at = chrono::Utc::now().naive_utc();
    //     diesel::update(users::table.find(user_id))
    //         .set((users::username.eq(username), users::email.eq(email)))
    //         .get_result(&mut self.connection)
    // }

    // pub fn delete_user(&mut self, user_id: i32) -> Result<usize, DieselError> {
    //     diesel::delete(users::table.find(user_id)).execute(&mut self.connection)
    // }
}

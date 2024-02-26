use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::Utc;
use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{models::user_model::{Login, Register,User, LoginModel}, utils::{api_error::ApiError, jwt::encode_jwt}};

pub async fn register(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<Register>
) -> Result<(), ApiError>{
    

    let check_exists = entity::user::Entity::find()
    .filter(entity::user::Column::Email.eq(data.email.clone()))
    .one(&db).await
    .map_err(|err| ApiError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?;

    if check_exists != None {
        return Err(ApiError{message: "User exists".to_owned(), status_code:StatusCode::CONFLICT, error_code: Some(40)});
    }
    let user_model = user::ActiveModel{
        name: Set(data.name.to_owned()),
        email: Set(data.email.to_owned()),
        password: Set(data.password.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
         ..Default::default()
    };
    user_model.insert(&db).await
    .map_err(|err| ApiError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?;
    Ok(())
}

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<Login>
) -> Result<Json<LoginModel>, ApiError>{
   let user_data = entity::user::Entity::find()
       .filter(
           Condition::all()
               .add(entity::user::Column::Email.eq(data.email))
               .add(entity::user::Column::Password.eq(data.password))
       ).one(&db).await
       .map_err(|err| ApiError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?
       .ok_or(ApiError {message: "Email or Password invalid!!!".to_owned(), status_code:StatusCode::NOT_FOUND, error_code:Some(44)})?;

    let token = encode_jwt(user_data.email)
    .map_err(|_| ApiError { message: "Failed to login".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41) })?;
    
    Ok(Json(LoginModel{token}))
}
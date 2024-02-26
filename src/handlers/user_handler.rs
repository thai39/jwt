use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{models::user_model::{Update, User}, utils::{api_error::ApiError}};

pub async fn update(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(data): Json<Update>,
) -> Result<(), ApiError> {

    let mut user_data: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .map_err(|err| ApiError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?
        .ok_or(ApiError {message: "NOT FOUND".to_owned(), status_code:StatusCode::NOT_FOUND, error_code:Some(44)})?
        .into();

    user_data.name = Set(data.name);
    user_data.password = Set(data.password);

    user_data.update(&db)
    .await
    .map_err(|err| ApiError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?;

    Ok(())
}

pub async fn del(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>
) -> Result<(), ApiError> {

    let user_data = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db).await
        .map_err(|err| ApiError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?
        .ok_or(ApiError {message: "NOT FOUND".to_owned(), status_code:StatusCode::NOT_FOUND, error_code:Some(44)})?;

    entity::user::Entity::delete_by_id(user_data.id)
        .exec(&db)
        .await
        .map_err(|err| ApiError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?;

    Ok(())
}

pub async fn get_all_users(
    Extension(db): Extension<DatabaseConnection>
) -> Result<Json<Vec<User>>, ApiError> {

    let user_data: Vec<User> = entity::user::Entity::find()
        .all(&db)
        .await
        .map_err(|err| ApiError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?
        .into_iter()
        .map(|item| User {
            name: item.name,
            email: item.email,
            password: item.password,
            uuid: item.uuid,
            created_at: item.created_at,
        })
        .collect();

        Ok(Json(user_data))
}

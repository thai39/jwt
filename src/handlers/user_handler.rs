use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::models::user_model::{Update, User};

pub async fn update(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(data): Json<Update>,
) -> impl IntoResponse {

    let mut user_data: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    user_data.name = Set(data.name);
    user_data.password = Set(data.password);

    user_data.update(&db).await.unwrap();

    (StatusCode::ACCEPTED, "Updated")
}

pub async fn del(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>
) -> impl IntoResponse {

    let user_data = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    entity::user::Entity::delete_by_id(user_data.id)
        .exec(&db)
        .await
        .unwrap();

    (StatusCode::ACCEPTED, "Deleted successfully")
}

pub async fn get_all_users(
    Extension(db): Extension<DatabaseConnection>
) -> impl IntoResponse {

    let user_data: Vec<User> = entity::user::Entity::find()
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|item| User {
            name: item.name,
            email: item.email,
            password: item.password,
            uuid: item.uuid,
            created_at: item.created_at,
        })
        .collect();

    (StatusCode::OK, Json(user_data))
}

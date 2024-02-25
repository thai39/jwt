use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::user_model::{Login, Register,User};

pub async fn register(
    Json(data): Json<Register>
) -> impl IntoResponse{
    let db: DatabaseConnection = Database::connect("postgres://postgres:Admin123@localhost:5432/jwt_axum").await.unwrap();

    let user_model = user::ActiveModel{
        name: Set(data.name.to_owned()),
        email: Set(data.email.to_owned()),
        password: Set(data.password.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
         ..Default::default()
    };
    user_model.insert(&db).await.unwrap();

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, "Successfully")
}

pub async fn login(
    Json(data): Json<Login>
) -> impl IntoResponse{
    let db: DatabaseConnection = Database::connect("postgres://postgres:Admin123@localhost:5432/jwt_axum").await.unwrap();

   let user_data = entity::user::Entity::find()
       .filter(
           Condition::all()
               .add(entity::user::Column::Email.eq(data.email))
               .add(entity::user::Column::Password.eq(data.password))
       ).one(&db).await.unwrap().unwrap();

    let info_response = User{
        name: user_data.name.to_string(),
        email: user_data.email.to_string(),
        password: user_data.password.to_string(),
        uuid: user_data.uuid,
        created_at: user_data.created_at,
    };
    
    db.close().await.unwrap();
    (StatusCode::ACCEPTED, Json(info_response))
}
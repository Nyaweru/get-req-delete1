use serde::{Deserialize, Serialize};

use actix_web::{web::{Data, Path, Json}, App, HttpResponse, HttpServer, Responder,
    post, get, delete, put};
use sqlx::{self, FromRow};

use crate:: AppState;
#[derive(Serialize, FromRow, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub address: String,
}


#[get("/users")]
pub async fn get_users(State:Data <AppState>,) -> impl Responder{
    match sqlx::query_as::<_, User>("select * from people")
        .fetch_all(&State.db)
        .await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(_) => HttpResponse::NotFound().json("Error Occured"),
        }
}

#[post("/users")]
pub async fn add_user(State:Data <AppState>, user: Json<User>) -> impl Responder{
    match sqlx::query("insert into people (id, name, address) values ($1,$2, $3) returning id, name, address")
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.address)
        .execute(&State.db)
        .await {
            Ok(_) => HttpResponse::Ok().json("User Added"),
            Err(_) => HttpResponse::NotFound().json("Error Occured"),
        }
}
#[delete("/users/{id}")]
pub async fn delete_user(State:Data <AppState>, id: Path<i32>) -> impl Responder{
    match sqlx::query("delete from people where id = $1")
        .bind(id.into_inner())
        .execute(&State.db)
        .await {
            Ok(_) => HttpResponse::Ok().json("User Deleted"),
            Err(_) => HttpResponse::NotFound().json("Error Occured"),
        }
}

#[put("/users/{id}")]

pub async fn edit_user(State:Data <AppState>, id: Path<i32>, user:Json<User>) -> impl Responder{
    match sqlx::query("update people set name = $1, address = $2 where id = $3")
        .bind(&user.name)
        .bind(&user.address)
        .bind(id.into_inner())
        .execute(&State.db)
        .await {
            Ok(_) => HttpResponse::Ok().json("User Updated"),
            Err(_) => HttpResponse::NotFound().json("Error Occured"),
        }
}

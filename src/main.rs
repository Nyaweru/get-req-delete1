use actix_web::{web::Data, HttpServer, App};
use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenv::dotenv;
mod services;
use services::{get_users, add_user, delete_user, edit_user};

pub struct  AppState {
    pub db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_pool.clone(),
            }))
            .service(get_users)
            .service(add_user)
            .service(delete_user)
            .service(edit_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
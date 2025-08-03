use actix_web::{delete, get, middleware::from_fn, post, put, web, App, HttpResponse, HttpServer, Responder};
use crate::{db::connection::connect, routes::{posts::{add_post, delete_post, get_posts, update_post, view_post}, users::{login, register_user}}};
use crate::middleware::middleware::auth_middleware;

pub mod db;
pub mod models;
pub mod routes;
pub mod entities;
pub mod error;
pub mod jwt;
pub mod middleware;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = connect().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))

            .service(register_user)
            .service(login)

            .service(
                web::scope("/api")  
                    .wrap(from_fn(auth_middleware))
                    .service(add_post)
                    .service(update_post)
                    .service(get_posts)
                    .service(view_post)
                    .service(delete_post)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use crate::{db::connection::connect, routes::posts::{add_post, delete_post, get_posts, update_post, view_post}};

pub mod db;
pub mod models;
pub mod routes;
pub mod entities;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = connect().await.unwrap();

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(db.clone()))
            .service(add_post)
            .service(update_post)
            .service(get_posts)
            .service(view_post)
            .service(delete_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// use std::io::stdin;


// fn takeinput()-> String {
//     let mut input = String::new();
//     stdin().read_line(&mut input ).expect("Cannot read input");
//     input.trim().to_string()
// }

// fn main(){

//     let list =["mobile", "mouse", "moneypot", "monitor", "mousepad"];

//     loop{

//             let input = takeinput();

//             let search : Vec<&'static str >= list.into_iter().filter(|t| t.starts_with(&input)).collect();

//             println!("Search = {:?}", search);
//     }

// }
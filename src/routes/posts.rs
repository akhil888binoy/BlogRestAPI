use std::default;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use serde::Deserialize;
use crate::models::db_models::DbConnection;
use actix_web::{ Result};

use crate::entities::{prelude::*, *};


#[derive(Deserialize, Clone)]
struct AddPost{ 
    title: String,
    text: String
}



#[derive(Deserialize, Clone)]
struct UpdatePost{ 
    title: String,
    text: String
}

#[post("/posts")]
pub async fn add_post(db : web::Data<DbConnection> , posts: web::Json<AddPost>) -> impl Responder {

    let post = post::ActiveModel{
        title: Set(posts.title.clone()),
        text: Set(posts.text.clone()),
        ..Default::default()
    };

    post.insert(&db.db).await.expect("Cannot insert to DB");

    HttpResponse::Ok().body("Post added")
}


#[get("/posts/{id}")]
pub async fn view_post(db : web::Data<DbConnection>, path : web::Path<u32>) -> Result<impl Responder>  {
    let id= path.into_inner();
    let post = post::Entity::find()
                                .filter(post::Column::Id.eq(id))
                                .one(&db.db)
                                .await.expect("Cannot get post");
    
    Ok(web::Json(post))
}


#[get("/posts")]
pub async fn get_posts( db : web::Data<DbConnection>) ->Result<impl Responder> {

    let post = post::Entity::find()
                                .all(&db.db)
                                .await.expect("Cannot get post");
    
    Ok(web::Json(post))
}


#[put("/posts/{id}")]
pub async fn update_post(posts: web::Json<UpdatePost> , db : web::Data<DbConnection> , path : web::Path<u32>) -> impl Responder {
    let id= path.into_inner();
    let post = post::Entity::find()
                                .filter(post::Column::Id.eq(id))
                                .one(&db.db)
                                .await.expect("Cannot get post");
    let mut  active : post::ActiveModel = post.unwrap().into();
    active.text = Set(posts.text.clone());
    active.title = Set(posts.title.clone());
    active.update(&db.db).await.expect("Cannot update");

    HttpResponse::Ok().body("Post updated")
}


#[delete("/posts/{id}")]
pub async fn delete_post(req_body: String, db : web::Data<DbConnection>, path : web::Path<u32>) -> impl Responder {
    let id= path.into_inner();
    let post = post::Entity::find()
                                .filter(post::Column::Id.eq(id))
                                .one(&db.db)
                                .await.expect("Cannot get post");
    post.unwrap().delete(&db.db).await.expect("Cannot delete");
    
    HttpResponse::Ok().body("Post Deleted")
}
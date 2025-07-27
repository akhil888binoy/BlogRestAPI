use std::{default, ops::Add, time::{self, Duration, SystemTime, UNIX_EPOCH}};

use actix_web::{  delete, get, http::Error, post, put, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use serde::Deserialize;
use crate::{error::error::MyError, jwt::model::Claims, models::db_models::DbConnection};
use actix_web::{ Result};
use argon2::{self, Config};
use serde_json::json;
use crate::entities::{prelude::*, *};

#[derive(Deserialize, Clone)]
struct RegisterUser{
    email: String ,
    password : String
}

#[post("/register")]
pub async fn register_user(db : web::Data<DbConnection> , user: web::Json<RegisterUser> )-> Result<impl Responder, MyError >{

    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded( user.password.as_bytes(), salt, &config).unwrap();

    let existing_user = user::Entity::find()
                                                .filter(user::Column::Email.eq(user.email.clone()))
                                                .one(&db.db)
                                                .await
                                                .map_err(|_| MyError::BadClientData);
    
    match existing_user{
        Ok(data)=>{
            return Err(MyError::NotFound)
        },
        Err(_)=>{
                let user = user::ActiveModel{
                    email: Set(user.email.clone()),
                    password: Set(hash),
                    ..Default::default() 
                };
            user.insert(&db.db).await.expect("Cannot insert in DB");
        }
    }


    Ok( HttpResponse::Ok().body("Register added"))
}


#[derive(Deserialize, Clone)]
struct LoginUser{
    email: String ,
    password : String
}

#[post("/login")]
pub async fn login(db: web::Data<DbConnection> , userbody: web::Json<LoginUser>)-> Result<impl Responder, MyError >{

    let user = user::Entity::find()
                                .filter(user::Column::Email.eq(userbody.email.clone()))
                                .one(&db.db)
                                .await
                                .map_err(|_| MyError::InternalError)?;
    
    match user {
        Some(user)=> {

            let verified = argon2::verify_encoded( &user.password ,  userbody.password.as_bytes()).unwrap();
            let sys_time = SystemTime::now();
            let one_sec = Duration::from_secs(900);

                let now_plus_60 = std::time::SystemTime::now()
                    .checked_add(time::Duration::from_secs(60))
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

            if (verified){

                let claims = Claims{
                    sub : user.id,
                    email: user.email,
                    exp: now_plus_60
                };

                let header = jsonwebtoken::Header::default();
                let secret = jsonwebtoken::EncodingKey::from_secret("secret".as_bytes());
                let res = jsonwebtoken::encode(&header, &claims, &secret).unwrap();

                Ok( HttpResponse::Ok().json(json!({
                    "result": "success",
                    "token": res
                })))

            }else{
                return Err(MyError::IncorrectPassword);
            }

        },
        None=>{
            return  Err(MyError::NotFound);
        }
    }
}


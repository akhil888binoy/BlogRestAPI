use std::time::SystemTime;

use actix_web::cookie::time::UtcDateTime;
use serde::Serialize;


#[derive(Serialize)]
pub struct Claims{
    pub sub: i32 ,
    pub email: String ,
    pub exp: u64
}
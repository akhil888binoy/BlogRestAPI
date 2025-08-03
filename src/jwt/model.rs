use std::time::SystemTime;

use actix_web::cookie::time::UtcDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Claims{
    pub sub: i32 ,
    pub email: String ,
    pub exp: u64
}
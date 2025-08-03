use actix_web::http::header::HeaderValue;
use jsonwebtoken::{Algorithm, TokenData, Validation};

use crate::{error::error::MyError, jwt::model::Claims};



pub fn validate_jwt(token : &str)->Result<TokenData<Claims>, MyError> {

    let secret = jsonwebtoken::DecodingKey::from_secret("secret".as_bytes());           
    let res = jsonwebtoken::decode::<Claims>( token, &secret, &Validation::new(Algorithm::HS256)).unwrap();
    return Ok(res) ;
    
}
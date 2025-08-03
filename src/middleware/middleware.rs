
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{
    body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, web, Error, HttpMessage, HttpRequest
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{db::connection::connect, entities::user, error::error::MyError, jwt::jwt::validate_jwt, models::db_models::DbConnection};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {

        let authorization_header = req.headers().get("Authorization").expect("Cannot get header value");
        
        let bearer_token: Vec<&str> = authorization_header
                .to_str()
                .unwrap()
                .split_whitespace()
                .collect();

        if bearer_token[0] != "Bearer" {
            return Err(MyError::NotFound.into());
        }

        let token = bearer_token[1];

        let is_valid = validate_jwt(token).map_err(|_| MyError::Unauthorized)?;

        req.extensions_mut().insert(is_valid.claims.clone());
        req.request().extensions_mut().insert(is_valid.claims);

    next.call(req).await

}

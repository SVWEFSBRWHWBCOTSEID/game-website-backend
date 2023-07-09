use actix_web::{web, HttpRequest, HttpResponse, get};

use crate::common::CustomError;
use crate::helpers::general::get_user_by_username;
use crate::prisma::PrismaClient;


// route for getting user info
#[get("/api/user/{username}")]
pub async fn get_user(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, CustomError> {

    let username: String = req.match_info().get("username").unwrap().parse().unwrap();

    Ok(HttpResponse::Ok().json(
        match get_user_by_username(&client, &username).await {
            Some(u) => Some(u.to_create_user_res()),
            None => None,
        }
    ))
}

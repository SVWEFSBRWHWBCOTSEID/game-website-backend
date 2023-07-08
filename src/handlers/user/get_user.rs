use actix_web::{web, HttpRequest, HttpResponse, get};

use crate::common::CustomError;
use crate::prisma::{PrismaClient, user};


// route for getting user info
#[get("/api/user/{username}")]
pub async fn get_user(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, CustomError> {

    let username: String = req.match_info().get("username").unwrap().parse().unwrap();

    Ok(HttpResponse::Ok().json(
        match client
            .user()
            .find_unique(user::username::equals(username))
            .exec()
            .await
            .unwrap()
        {
            Some(u) => Some(u.to_create_user_res()),
            None => None,
        }
    ))
}

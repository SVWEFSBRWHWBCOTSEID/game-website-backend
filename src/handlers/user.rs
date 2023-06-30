use actix_web::{web, HttpRequest, HttpResponse, get, post};

use crate::CustomError;
use crate::prisma::{PrismaClient, user};
use crate::models::req::CreateUserReq;


// route for creating a new user
#[post("/api/user/new")]
pub async fn create_user(
    client: web::Data<PrismaClient>,
    data: web::Json<CreateUserReq>
) -> Result<HttpResponse, CustomError> {

    let create_user_req: CreateUserReq = data.into_inner();
    if !create_user_req.validate(&client).await {
        return Err(CustomError::BadRequest);
    }

    let user = create_user_req.create_user(&client).await;
    Ok(HttpResponse::Ok().json(user.to_user_res()))
}

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
            .find_unique(user::name::equals(username))
            .exec()
            .await
            .unwrap()
        {
            Some(u) => Some(u.to_user_res()),
            None => None,
        }
    ))
}

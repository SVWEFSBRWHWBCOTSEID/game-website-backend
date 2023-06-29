use actix_web::{web, HttpRequest, HttpResponse, post};
use rand::Rng;

use crate::CustomError;
use crate::prisma::{PrismaClient, Side};
use crate::models::req::CreateUserReq;


// route for creating a new user
#[post("/api/user/new")]
pub async fn create_user(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    data: web::Json<CreateUserReq>
) -> Result<HttpResponse, CustomError> {

    let create_user_req: CreateUserReq = data.into_inner();

    Ok(HttpResponse::Ok().json(create_user_req.to_game_res()))
}

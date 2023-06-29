use actix_web::{web, HttpResponse, post};

use crate::CustomError;
use crate::prisma::PrismaClient;
use crate::models::req::CreateUserReq;


// route for creating a new user
#[post("/api/user/new")]
pub async fn create_user(
    client: web::Data<PrismaClient>,
    data: web::Json<CreateUserReq>
) -> Result<HttpResponse, CustomError> {

    let create_user_req: CreateUserReq = data.into_inner();

    let user = create_user_req.create_user(client).await;

    Ok(HttpResponse::Ok().json(user.to_user_res()))
}

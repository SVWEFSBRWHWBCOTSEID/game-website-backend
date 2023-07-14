use actix_session::Session;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpResponse, post};

use crate::common::CustomError;
use crate::prisma::PrismaClient;
use crate::models::req::CreateUserReq;


// route for creating a new user
#[post("/api/user/new")]
pub async fn create_user(
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<CreateUserReq>
) -> Result<HttpResponse, CustomError> {

    let create_user_req: CreateUserReq = data.into_inner();
    if !create_user_req.validate(&client).await {
        return Err(CustomError::BadRequest);
    }
    let user = create_user_req.create_user(&client).await;

    session.insert("username", &user.username).map_err(|_| CustomError::InternalError)?;

    let mut cookie = Cookie::new("username", &user.username);
    cookie.set_same_site(SameSite::None);
    cookie.set_path("/");

    let mut res = HttpResponse::Ok().json(user.to_create_user_res());
    res.add_cookie(&cookie).map_err(|_| CustomError::InternalError)?;
    Ok(res)
}

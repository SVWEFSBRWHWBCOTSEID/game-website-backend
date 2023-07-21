use actix_session::Session;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpResponse, post};

use crate::common::WebErr;
use crate::prisma::PrismaClient;
use crate::models::req::CreateUserReq;


// route for creating a new user
#[post("/api/user/new")]
pub async fn create_user(
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<CreateUserReq>
) -> Result<HttpResponse, WebErr> {

    let create_user_req: CreateUserReq = data.into_inner();
    if !create_user_req.validate(&client).await? {
        return Err(WebErr::Forbidden(format!("invalid create user request")));
    }
    let user = create_user_req.create_user(&client).await?;

    session.insert("username", &user.username).or(Err(WebErr::Internal(format!("error inserting username to user session"))))?;

    let mut cookie = Cookie::new("username", &user.username);
    cookie.set_same_site(SameSite::None);
    cookie.set_path("/");

    let mut res = HttpResponse::Ok().json(user.to_user_res()?);
    res.add_cookie(&cookie).or(Err(WebErr::Internal(format!("error adding cookie for username"))))?;
    Ok(res)
}

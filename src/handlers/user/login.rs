use actix_session::Session;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpResponse, post};

use crate::common::CustomError;
use crate::helpers::general::get_user_by_username;
use crate::prisma::PrismaClient;
use crate::models::req::LoginReq;


// route for logging in user
#[post("api/login")]
pub async fn login(
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<LoginReq>,
) -> Result<HttpResponse, CustomError> {

    let login_req: LoginReq = data.into_inner();
    let user = get_user_by_username(&client, &login_req.username).await?;

    if !bcrypt::verify(&login_req.password, &user.password).unwrap() {
        return Err(CustomError::Unauthorized);
    }

    session.renew();
    session.insert("username", &user.username).map_err(|_| CustomError::InternalError)?;

    let mut cookie = Cookie::new("username", &user.username);
    cookie.set_same_site(SameSite::None);
    cookie.set_path("/");

    let mut res = HttpResponse::Ok().json(user.to_create_user_res()?);
    res.add_cookie(&cookie).map_err(|_| CustomError::InternalError)?;
    Ok(res)
}

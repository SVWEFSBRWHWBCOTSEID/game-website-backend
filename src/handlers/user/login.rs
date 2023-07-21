use actix_session::Session;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpResponse, post};

use crate::common::WebErr;
use crate::helpers::general::get_user_with_relations;
use crate::prisma::PrismaClient;
use crate::models::req::LoginReq;


// route for logging in user
#[post("api/login")]
pub async fn login(
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<LoginReq>,
) -> Result<HttpResponse, WebErr> {

    let login_req: LoginReq = data.into_inner();
    let user = get_user_with_relations(&client, &login_req.username).await?;

    if !bcrypt::verify(&login_req.password, &user.password).unwrap() {
        return Err(WebErr::Unauth(format!("incorrect username/password")));
    }

    session.renew();
    session.insert("username", &user.username).or(Err(WebErr::Internal(format!("error inserting username to user session"))))?;

    let mut cookie = Cookie::new("username", &user.username);
    cookie.set_same_site(SameSite::None);
    cookie.set_path("/");

    let mut res = HttpResponse::Ok().json(user.to_user_res()?);
    res.add_cookie(&cookie).or(Err(WebErr::Internal(format!("error adding cookie for username"))))?;
    Ok(res)
}

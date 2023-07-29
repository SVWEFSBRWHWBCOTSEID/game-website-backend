use std::env;

use actix_session::Session;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpResponse, post};

use crate::common::WebErr;
use crate::helpers::general::create_guest_user;
use crate::models::req::CreateGuestReq;
use crate::prisma::PrismaClient;


// route for creating a guest user
#[post("/api/guest/new")]
pub async fn create_guest(
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<CreateGuestReq>,
) -> Result<HttpResponse, WebErr> {

    let create_guest_req: CreateGuestReq = data.into_inner();
    let guest = create_guest_user(&client, &create_guest_req.username).await?;
    session.insert("username", &guest.username)
        .or(Err(WebErr::Internal(format!("error inserting username to user session"))))?;

    let mut cookie = Cookie::new("username", guest.username.split(' ').collect::<Vec<&str>>()[0]);
    cookie.set_same_site(SameSite::None);
    cookie.set_path("/");
    if let Ok(x) = env::var("DOMAIN") {
        if x != "http://localhost:3000" {
            cookie.set_domain(x);
        }
    }

    let mut res = HttpResponse::Ok().json(guest.to_user_res()?);
    res.add_cookie(&cookie).or(Err(WebErr::Internal(format!("error adding cookie for username"))))?;
    Ok(res)
}

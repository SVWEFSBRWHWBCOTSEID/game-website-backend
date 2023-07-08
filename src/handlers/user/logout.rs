use actix_session::Session;
use actix_web::cookie::SameSite;
use actix_web::{HttpRequest, HttpResponse, post};

use crate::common::CustomError;


// route for logging in user
#[post("api/logout")]
pub async fn logout(req: HttpRequest, session: Session) -> Result<HttpResponse, CustomError> {
    session.purge();
    let mut res = HttpResponse::Ok().finish();

    match req.cookie("username") {
        Some(mut cookie) => {
            cookie.make_removal();
            cookie.set_same_site(SameSite::None);
            cookie.set_path("/");
            res.add_cookie(&cookie).expect("failed to remove username cookie");
        },
        None => {},
    }

    Ok(res)
}

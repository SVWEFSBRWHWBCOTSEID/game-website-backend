use actix_session::Session;
use actix_web::{web, HttpResponse, post};

use crate::common::WebErr;
use crate::models::res::OK_RES;
use crate::prisma::PrismaClient;


// route for creating a guest user
#[post("/api/guest/new")]
pub async fn create_guest(
    _client: web::Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, WebErr> {

    session.insert("username", "guest").or(Err(WebErr::Internal(format!("error inserting username to user session"))))?;

    Ok(HttpResponse::Ok().json(OK_RES))
}

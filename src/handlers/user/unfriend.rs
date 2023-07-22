use actix_session::Session;
use actix_web::{web, HttpResponse, HttpRequest, post};

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, friend};


// route for creating a new user
#[post("/api/unfriend/{username}")]
pub async fn unfriend(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let other_name: String = req.match_info().get("username").unwrap().parse().unwrap();

    if client
        .friend()
        .delete(friend::username_friend_name(username.clone(), other_name.clone()))
        .exec()
        .await
        .is_err()
    {
        client
            .friend()
            .delete(friend::username_friend_name(other_name.clone(), username.clone()))
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error deleting friend relation from {} to {}", username, other_name))))?;
    }

    Ok(HttpResponse::Ok().json(OK_RES))
}

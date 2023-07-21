use actix_session::Session;
use actix_web::{web, HttpResponse, HttpRequest, post};

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::models::general::FriendRequest;
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, user};


// route for creating a new user
#[post("/api/follow/{username}")]
pub async fn friend_request(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let other_name: String = req.match_info().get("username").unwrap().parse().unwrap();

    client
        .friend()
        .create(
            FriendRequest::Out.to_string(),
            user::username::equals(username.clone()),
            user::username::equals(other_name.clone()),
            vec![],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error creating friend request from {} to {}", username, other_name))))?;

    Ok(HttpResponse::Ok().json(OK_RES))
}

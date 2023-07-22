use actix_session::Session;
use actix_web::{web, HttpResponse, HttpRequest, post};

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::models::general::FriendRequest;
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, user, friend};


// route for creating a new user
#[post("/api/friend/{username}")]
pub async fn friend_request(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let other_name: String = req.match_info().get("username").unwrap().parse().unwrap();

    let user = client
        .user()
        .find_unique(user::username::equals(username.clone()))
        .with(user::friends::fetch(vec![]))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error finding user {}", username))))?
        .ok_or(WebErr::NotFound(format!("could not find user {}", username)))?;

    if user
        .friends()
        .or(Err(WebErr::Internal(format!("friends not fetched"))))?
        .iter()
        .find(|f|
            f.friend_name == other_name &&
            FriendRequest::from_str(&f.r#type).unwrap() == FriendRequest::Pending
        )
        .is_some()
    {
        client
            .friend()
            .update(
                friend::username_friend_name(username.clone(), other_name.clone()),
                vec![
                    friend::r#type::set(FriendRequest::Accepted.to_string()),
                ],
            )
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error creating friend request from {} to {}", username, other_name))))?;
    } else {
        client
            .friend()
            .create(
                FriendRequest::Pending.to_string(),
                user::username::equals(username.clone()),
                user::username::equals(other_name.clone()),
                vec![],
            )
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error creating friend request from {} to {}", username, other_name))))?;
    };

    Ok(HttpResponse::Ok().json(OK_RES))
}

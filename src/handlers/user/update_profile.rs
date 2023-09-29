use actix_session::Session;
use actix_web::{post, HttpResponse};
use actix_web::web::{Json, Data};

use crate::common::WebErr;
use crate::helpers::general::{get_username, get_user_with_relations};
use crate::helpers::user::get_user_res;
use crate::models::general::Profile;
use crate::prisma::{PrismaClient, user};


// route for updating a user's profile
#[post("/api/profile/update")]
pub async fn update_profile(
    client: Data<PrismaClient>,
    session: Session,
    data: Json<Profile>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let new_profile: Profile = data.into_inner();
    client
        .user()
        .update(
            user::username::equals(username.clone()),
            vec![
                user::country::set(new_profile.country.to_string()),
                user::location::set(new_profile.location),
                user::bio::set(new_profile.bio),
                user::first_name::set(new_profile.first_name),
                user::last_name::set(new_profile.last_name),
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Forbidden(format!("could not find user with username {}", username))))?;

    let new_user = get_user_with_relations(&client, &username).await?;
    Ok(HttpResponse::Ok().json(get_user_res(&client, new_user).await?))
}

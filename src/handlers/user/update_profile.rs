use actix_session::Session;
use actix_web::{web, HttpResponse, post};

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::models::general::Profile;
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, user};


// route for creating a new user
#[post("/api/profile/update")]
pub async fn update_profile(
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<Profile>,
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

    Ok(HttpResponse::Ok().json(OK_RES))
}

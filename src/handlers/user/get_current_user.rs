use actix_session::Session;
use actix_web::{web, HttpResponse, get};

use crate::common::WebErr;
use crate::helpers::general::{get_user_with_relations, get_username};
use crate::helpers::user::get_user_res;
use crate::prisma::PrismaClient;


// route for getting current user's info
#[get("/api/user")]
pub async fn get_current_user(
    client: web::Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let mut user = get_user_with_relations(&client, &username).await?;
    user.update_perfs(&client).await?;

    Ok(HttpResponse::Ok().json(get_user_res(&client, user).await?))
}

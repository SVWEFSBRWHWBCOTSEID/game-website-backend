use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

use crate::common::WebErr;
use crate::helpers::general::{get_username, get_user_conversations};
use crate::prisma::PrismaClient;


// route for getting signed in user's conversations
#[get("/api/conversations")]
pub async fn get_conversations(
    client: Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;

    Ok(HttpResponse::Ok().json(get_user_conversations(&client, &username).await?))
}

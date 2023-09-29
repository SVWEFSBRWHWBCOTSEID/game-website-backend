use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use prisma_client_rust::or;

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::models::res::ConversationResponse;
use crate::prisma::{PrismaClient, conversation};


// route for getting signed in user's conversations
#[get("/api/conversations")]
pub async fn get_conversations(
    client: Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;

    let mut conversations_res: Vec<ConversationResponse> = vec![];

    client
        .conversation()
        .find_many(vec![
            or![
                conversation::username::equals(username.clone()),
                conversation::other_name::equals(username.clone()),
            ],
        ])
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error getting conversations for user {}", username))))?
        .iter()
        .for_each(|c| conversations_res.push(c.to_conversation_res(&username)));

    Ok(HttpResponse::Ok().json(conversations_res))
}

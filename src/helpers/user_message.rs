use crate::{models::general::UserMessage, prisma::user_message};


impl user_message::Data {
    pub fn to_user_message(&self) -> UserMessage {
        UserMessage {
            username: self.username.clone(),
            text: self.text.clone(),
            created_at: self.created_at.to_string(),
        }
    }
}

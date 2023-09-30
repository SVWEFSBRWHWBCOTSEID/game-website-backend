use crate::{models::general::Conversation, prisma::conversation, common::WebErr};


impl conversation::Data {
    pub fn to_conversation(&self, username: &str) -> Result<Conversation, WebErr> {
        let messages = self.messages()
            .or(Err(WebErr::Internal(format!("messages not fetched in {}'s conversation", username))))?
            .iter()
            .map(|m| m.to_user_message())
            .collect();

        Ok(Conversation {
            other_name: if self.username == username {
                self.other_name.clone()
            } else {
                self.username.clone()
            },
            messages,
        })
    }
}

use crate::{models::res::ConversationResponse, prisma::conversation};


impl conversation::Data {
    pub fn to_conversation_res(&self, username: &str) -> ConversationResponse {
        ConversationResponse {
            other_name: if self.username == username {
                self.other_name.clone()
            } else {
                self.username.clone()
            }
        }
    }
}

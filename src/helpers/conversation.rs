use crate::{models::general::Conversation, prisma::conversation};


impl conversation::Data {
    pub fn to_conversation(&self, username: &str) -> Conversation {
        Conversation {
            other_name: if self.username == username {
                self.other_name.clone()
            } else {
                self.username.clone()
            }
        }
    }
}

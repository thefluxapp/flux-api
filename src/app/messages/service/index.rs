use super::MessagesService;

impl MessagesService {
    pub async fn index() -> String {
        String::from("MESSAGES")
    }
}

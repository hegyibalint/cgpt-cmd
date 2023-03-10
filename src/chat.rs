use serde::{Serialize, Deserialize};
use core::fmt::Display;
use std::io;

static OPENAI_CHAT_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug)]
pub enum ChatSessionError {
    RequestError(ureq::Error),
    ParseError(io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: Role,
    pub(crate) content: String,
}

#[derive(Serialize, Debug)]
pub struct Session {
    #[serde(skip_serializing)]
    api_key: String,
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Response {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    index: u64,
    message: Message
}

impl Session {
    pub fn new(api_key: String, model: String) -> Session {
        return Session {
            api_key,
            model,
            messages: vec![],
        };
    }

    pub fn say(&mut self, role: Role, message: String) -> Result<&Message, ChatSessionError> {
        self.messages.push(Message {
            role,
            content: message,
        });

        let bearer_token = format!("Bearer {}", self.api_key);
        // let payload = serde_json::to_string(self);
        let response = ureq::post(OPENAI_CHAT_ENDPOINT)
            .set("Authorization", &bearer_token)
            .send_json(&self);

        let data: Response = match response {
            Ok(response) => match response.into_json() {
                Ok(response) => response,
                Err(err) => return Err(ChatSessionError::ParseError(err))
            }
            Err(err) => return Err(ChatSessionError::RequestError(err))
        };

        self.messages.push(Message {
            role: Role::Assistant,
            content: data.choices[0].message.content.clone()
        });

        return Ok(self.messages.last().expect("Should always have messages"));
    }
}
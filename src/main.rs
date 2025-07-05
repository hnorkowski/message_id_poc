use macros::MessageId;

#[derive(MessageId)]
#[message_id = 0x12]
struct User {
    name: String,
}

impl User {
    pub fn encode_content(&self) -> Vec<u8> {
        self.name.clone().into_bytes()
    }
}

fn main() {
    let user = User {
        name: "John Doe".to_string(),
    };

    dbg!(User::MESSAGE_ID);
    dbg!(user.encode());
}

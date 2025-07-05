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

#[derive(MessageId)]
#[message_id = 0x13]
#[allow(dead_code)]
enum UnsignedInteger {
    U32(u32),
    U64(u64),
    U128(u128),
}

impl UnsignedInteger {
    /// Encodes an unsigned integer into big endian bytes and prepends the length of the number in bytes as single u8
    fn encode_content(&self) -> Vec<u8> {
        let mut encoded = match self {
            Self::U32(number) => number.to_be_bytes().to_vec(),
            Self::U64(number) => number.to_be_bytes().to_vec(),
            Self::U128(number) => number.to_be_bytes().to_vec(),
        };

        let size =
            u8::try_from(encoded.len()).expect("Maximum of 16 bytes => should always fit in an u8");
        encoded.insert(0, size);

        encoded
    }
}

fn main() {
    let user = User {
        name: "John Doe".to_string(),
    };

    dbg!(User::MESSAGE_ID);
    dbg!(user.encode());

    let tlv = UnsignedInteger::U64(0x1337);

    dbg!(UnsignedInteger::MESSAGE_ID);
    dbg!(tlv.encode());
}

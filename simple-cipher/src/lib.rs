enum Method {
    Encode,
    Decode,
}

fn thing(key: &str, s: &str, method: Method) -> Option<String> {
    dbg!(&key);
    dbg!(&s);
    let mut result = String::new();
    for (k, c) in key.bytes().zip(s.bytes()) {
        dbg!(&k);
        dbg!(&c);
        let distance = dbg!(k as i8 - c as i8);
        let new_c = match method {
            Method::Encode => {
                let mut new_c = dbg!((c as i8 + distance) as u8);
                if new_c > b'z' {
                    new_c -= b'a';
                }
                dbg!(new_c)
            }
            Method::Decode => (b'a' as i8 + distance) as u8,
        };

        result.push(char::from_u32(new_c as u32)?);
    }

    dbg!(&result);
    Some(result)
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    // todo!("Use {key} to encode {s} using shift cipher");
    thing(key, s, Method::Encode)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    // todo!("Use {key} to decode {s} using shift cipher")
    thing(key, s, Method::Decode)
}

pub fn encode_random(s: &str) -> (String, String) {
    todo!("Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)")
}

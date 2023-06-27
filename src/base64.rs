fn u8_to_u6(src: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut arr = src.iter();
    let mut part: u8;
    
    loop {
        match arr.next() {
            Option::Some(&i) => {
                result.push(i >> 2);
                part = (i & 0b00000011) << 4;
            },
            Option::None => {break;}
        }
        match arr.next() {
            Option::Some(&i) => {
                result.push(part | (i >> 4));
                part = (i & 0b00001111) << 2;
            },
            Option::None => {
                result.push(part);
                result.push(64);
                result.push(64); // '='
                break;
            }
        }
        match arr.next() {
            Option::Some(&i) => {
                result.push(part | (i >> 6));
                result.push(i & 0b00111111);
            },
            Option::None => {
                result.push(part);
                result.push(64); // '='
                break;
            }
        }
    }
    result
}

//make sure u6.len() % 4 is 0
fn u6_to_u8(src: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut arr = src.iter();
    let mut part: u8;

    loop {
        match arr.next() {
            Option::Some(&i) => {part = i << 2;},
            Option::None => {break;}
        }
        match arr.next() {
            Option::Some(&i) => {
                result.push(part | (i >> 4));
                part = i << 4;
            },
            Option::None => {todo!();}
        }
        match arr.next() {
            Option::Some(&i) => {
                if i == 64 {break;}
                result.push(part | ((i & 0b00111100) >> 2));
                part = i << 6;
            },
            Option::None => {todo!();}
        }
        match arr.next() {
            Option::Some(&i) => {
                if i == 64 {break;}
                result.push(part | (i & 0b00111111));
            },
            Option::None => {todo!();}
        }
    }
    result
}

fn base64_to_string(src: Vec<u8>) -> String {
    String::from_utf8(
        src.iter()
            .map(|&x| match x {
                0..=25 => x + 65, // A~Z
                26..=51 => x + 71, // a~z
                52..=61 => x - 4, // 0~9
                62 => 43, // '+'
                63 => 47, // '/'
                64 => 61, // '='
                _ => unreachable!()
            })
            .collect()
    ).unwrap()
}

fn string_to_base64(src: &String) -> Vec<u8> {
    src.as_bytes()
        .iter()
        .map(|x| match x {
            65..=90 => x - 65, // A~Z
            97..=122 => x - 71, // a~z
            48..=57 => x + 4, // 0~9
            43 => 62, // '+'
            47 => 63, // '/'
            61 => 64, // '='
            _ => panic!("invalid base64 code")
        })
        .collect()
}

pub fn base64_encode(src: &String) -> String {
    base64_to_string(
        u8_to_u6(src.as_bytes())
    )
}

pub fn base64_decode(src: &String) -> String {
    String::from_utf8(
        u6_to_u8(
            string_to_base64(src)
        )
    ).unwrap()
}
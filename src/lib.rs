// The Vigenere Cypher
// https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher#Algebraic_description

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// dammit theres gotta be a better way to do this
fn alpha_to_num(a: char) -> i32 {
    match a {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        _   => 26
    }
}
// dammit theres gotta be a better way to do this
fn num_to_alpha(n: i32) -> char {
    match n {
        0  => 'A',
        1  => 'B',
        2  => 'C',
        3  => 'D',
        4  => 'E',
        5  => 'F',
        6  => 'G',
        7  => 'H',
        8  => 'I',
        9  => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        _  => '0'
    }
}


// `c` is the character to encrypt, `k` is the key character
fn encrypt_char(c: char, k: char) -> i32 { alpha_to_num(c) + alpha_to_num(k) % 26 }
fn decrypt_char(c: char, k: char) -> i32 { alpha_to_num(c) - alpha_to_num(k) % 26 }


// Create an object to store a key, and then use this object to encrypt/decrypt
// messages like: `let v = Vigenere { key: my_key }; v.encrypt(my_message)`
struct Vigenere {
    key: &'static str
}

impl Vigenere {
    fn encrypt(&self, msg: &mut String) -> String {
        let msg_size = msg.len();
        let key_len  = self.key.len();
        let key_reps = msg_size / key_len;

        let mut ret = String::new();

        for (i, c) in msg.chars().enumerate() {
            let l = self.key.len();
            let k = self.key.chars().nth(i % l).unwrap();  // `i%l` should effectively loop `i` over the key index

            ret.push(num_to_alpha(encrypt_char(c, k)));
        }

        ret
    }


    fn decrypt(&self, msg: &mut String) -> String {
        let mut ret = String::new();

        for (i, c) in msg.chars().enumerate() {
            let l = self.key.len();
            let k = self.key.chars().nth(i % l).unwrap();

            ret.push(num_to_alpha(encrypt_char(c, k)));
        }

        ret

    }
}


#[test]
fn t_crypto() {
    let k = "LEMON";
    let m = "ATTACKATDAWN";
    let c = "LXFOPVEFRNHR";

    let v = Vigenere {key: k};
    let mut msg = String::from(m);

    let mut e = v.encrypt(&mut msg);
    assert_eq!(e, c);

    let d = v.decrypt(&mut e);
    assert_eq!(d, m)
}

#[test]
fn t_wikipedia_example() {
    let v = Vigenere { key: "L" };
    let mut m = String::from("A");
    assert_eq!(v.encrypt(&mut m), num_to_alpha(11).to_string() )
}

#[test]
fn t_alpha_conv() {
    assert_eq!(alpha_to_num('A'), 0);
    assert_eq!(num_to_alpha(0), 'A')
}

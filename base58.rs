use {std::io, std::io::prelude::*};

macro_rules! flip_alpha {
    ( $lookup:expr ) => {{
        let mut out: [u8; 123] = [123; 123];
        let mut i = 0;
        loop {
            if i < 58 {
                out[$lookup[i] as usize] = i as u8;
            } else if i == 123 {
                break out;
            }
            i += 1;
        }
    }};
}

const ALPHABET: [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const ALPHABET_MAP: [u8; 123] = flip_alpha!(ALPHABET);

fn main() {
    match &std::env::args().last().unwrap() as &str {
        "-d" => decode(),
        _ => encode(),
    }
}

fn decode() {
    let mut out: Vec<u8> = vec![];
    let input = io::stdin()
        .bytes()
        .enumerate()
        .map(|(i, byte)| {
            match ALPHABET_MAP[*byte.as_ref().unwrap() as usize] {
                123 => panic!("Unmapped incoming byte {:?} at index {:?}.", byte, i),
                any => any as i32,
            }
        })
        .collect::<Vec<i32>>();
    input.clone().iter_mut()
        .for_each(|b58_index| {
            out.iter_mut().for_each(|digit| {
                *b58_index += (*digit as i32) * 58; // move over one sig digit
                *digit = (*b58_index & 0xff) as u8;
                *b58_index >>= 8;
            });

            if *b58_index > 0 {
                out.push((*b58_index & 0xff) as u8);
            }
        });

    input.iter() // highest sig digits will get filtered out by > 0 check, add back
        .take_while(|c| **c == 0)
        .for_each(|_| out.push(0));

    out.reverse();
    io::stdout().write(&out).unwrap();
}

fn encode() {
    let mut out: Vec<u8> = vec![];
    let input = io::stdin()
        .bytes()
        .map(|c| c.unwrap() as i32)
        .collect::<Vec<i32>>();
    input.clone().iter_mut()
        .for_each(|carry| {
            out.iter_mut().for_each(|byte| {
                *carry += (*byte as i32) << 8;
                *byte = (*carry % 58) as u8;
                *carry /= 58;
            });

            while *carry > 0 {
                out.push((*carry % 58) as u8);
                *carry /= 58;
            }
        });

    input.iter() // highest sig digits will get filtered out by > 0 check, add back
        .take_while(|c| **c == 0)
        .for_each(|_| out.push(0));

    out = out.iter().map(|c| ALPHABET[*c as usize]).rev().collect();
    io::stdout().write(&out).unwrap();
}

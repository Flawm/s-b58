use {std::io, std::io::prelude::*};

macro_rules! flip_alphabet {
    ( $lookup:expr ) => {{
        let mut out: [u8; 123] = [123; 123];
        let mut i = 0;
        while i < 58 {
            out[$lookup[i] as usize] = i as u8;
            i += 1;
        }
        out
    }};
}

const ALPHABET: [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const ALPHABET_MAP: [u8; 123] = flip_alphabet!(ALPHABET);

fn main() {
    let input: Vec<i32> = io::stdin().bytes().map(|b| b.unwrap() as i32).collect();
    let mut out: Vec<u8> = vec![];

    match &std::env::args().last().unwrap() as &str {
        "-d" => decode(&mut out, &input),
        _ => encode(&mut out, &input),
    };

    io::stdout().write(&out).unwrap();
}

fn decode(out: &mut Vec<u8>, input: &Vec<i32>) {
    let input = input
        .iter()
        .enumerate()
        .map(|(i, byte)| match ALPHABET_MAP[*byte as usize] {
            123 => panic!("Unmapped incoming byte {:?} at index {:?}.", byte, i),
            any => any as i32,
        })
        .collect::<Vec<i32>>();

    input.clone().iter_mut().for_each(|b58_index| {
        out.iter_mut().for_each(|digit| {
            *b58_index += (*digit as i32) * 58; // move over one sig digit
            *digit = (*b58_index & 0xff) as u8;
            *b58_index >>= 8;
        });

        if *b58_index > 0 {
            out.push((*b58_index & 0xff) as u8);
        }
    });

    input
        .iter() // highest sig digits will get filtered out by > 0 check, add back
        .take_while(|c| **c == 0)
        .for_each(|_| out.push(0));

    out.reverse();
}

fn encode(out: &mut Vec<u8>, input: &Vec<i32>) {
    input.clone().iter_mut().for_each(|in_byte| {
        out.iter_mut().for_each(|out_byte| {
            *in_byte += (*out_byte as i32) << 8;  // move over one sig digit
            *out_byte = (*in_byte % 58) as u8;
            *in_byte /= 58;
        });

        while *in_byte > 0 {
            out.push((*in_byte % 58) as u8);
            *in_byte /= 58;
        }
    });

    input
        .iter() // highest sig digits will get filtered out by > 0 check, add back
        .take_while(|c| **c == 0)
        .for_each(|_| out.push(0));

    out.iter_mut().for_each(|c| *c = ALPHABET[*c as usize]);
}

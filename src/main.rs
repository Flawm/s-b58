use {clap::Parser, std::io, std::io::prelude::*};

macro_rules! flip_alpha {
    ( $lookup:expr ) => {{
        let mut a: [u8; 128] = [255; 128];
        let mut i = 0;
        loop {
            if i < 58 {
                a[$lookup[i] as usize] = i as u8;
            } else if i == 128 {
                break a;
            }
            i += 1;
        }
    }};
}

const ALPHABET: [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const ALPHABET_MAP: [u8; 128] = flip_alpha!(ALPHABET);

fn main() {
    match Args::parse().d {
        true => decode(),
        _ => encode(),
    }
}

fn decode() {
    let mut input = io::stdin()
        .bytes()
        .enumerate()
        .map(|(i, byte)| {
            let byte = byte.unwrap();
            match ALPHABET_MAP[byte as usize] as usize {
                255 => panic!("Unmapped incoming byte {:?} at index {:?}.", byte, i),
                any => any,
            }
        })
        .collect::<Vec<usize>>();

    let out: &mut [u8] = &mut vec![0; input.len()][..];

    input.iter_mut().enumerate().for_each(|(index, b58_index)| {
        out[..index].iter_mut().for_each(|digit| {
            *b58_index += (*digit as usize) * 58; // move up one sig digit
            *digit = (*b58_index & 0xff) as u8;
            *b58_index >>= 8;
        });

        out[index] = (*b58_index & 0xff) as u8;
    });

    let parsed = out.len() * 6 / 8; // how many encoding blocks did we get
    let out = &mut out[..parsed];
    out.reverse();

    io::stdout().write(&out).unwrap();
}

fn encode() {
    panic!("Fuck");
}

////

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// To decode std::in
    #[clap(short, long, action)]
    d: bool,
}

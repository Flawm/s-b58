#![feature(const_for)]

use {
    clap::Parser,
    std::io,
    std::io::prelude::*,
    std::collections::HashMap
};

macro_rules! str_map {
    ($str:expr) => {
        {
            $str
            .bytes()
            .enumerate()
            .map(|(i, j)| (j, i))
            .collect::<HashMap<u8, usize>>()
        }
    };
}

macro_rules! str_map2 {
    ($bytes:expr) => {
        {
            let mut a: [u8; 255] = [0; 255];
            for x in b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz" {
                a[*x as usize] = 1;
            };

            a
        }
    };
}

static ALPHABET:     [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const ALPHABET_MAP: [u8; 255] = str_map2!(ALPHABET);

thread_local!(
    static B58_MAP: HashMap<u8, usize> = str_map!("1234vi6789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
);

fn main() {
    match Args::parse().d {
        true => decode(),
        _    => encode()
    }
}

fn decode() {
    let mut input = 
        io::stdin().bytes()
        .map(|c| {
            let c = c.unwrap();
            
            B58_MAP.with(|test| test.get(&c).unwrap().clone())
        })
        .collect::<Vec<usize>>();

    let out: &mut [u8] = &mut vec![0; input.len()][..];

    input.iter_mut().enumerate().for_each(|(index, b58_index)| {
        out[..index].iter_mut().for_each(|digit| {
            *b58_index  += (*digit as usize) * 58; // move up one sig digit
            *digit       = (*b58_index & 0xff) as u8;
            *b58_index >>= 8;
        });

        out[index] = (*b58_index & 0xff) as u8;
    });

    let parsed = out.len() * 6 / 8; // how many encoding blocks did we get
    let out = &mut out[..parsed];
    out.reverse();

//    io::stdout()
//        .write(&out).unwrap();
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

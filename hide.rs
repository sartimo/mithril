//Adapted from lightaidra
use std::env;

const ENCODES: [char; 64] = [
    '%', 'q', '*', 'K', 'C', ')', '&', 'F', '9', '8', 'f', 's', 'r', '2', 't', 'o', '4', 'b', '3', 'y',
    'i', '_', ':', 'w', 'B', '>', 'z', '=', ';', '!', 'k', '?', '"', 'E', 'A', 'Z', '7', '.', 'D', '-',
    'm', 'd', '<', 'e', 'x', '5', 'U', '~', 'h', ',', 'j', '|', '$', 'v', '6', 'c', '1', 'g', 'a', '+',
    'p', '@', 'u', 'n',
];
const DECODES: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '.', ' ',
];

fn encode(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            DECODES
                .iter()
                .position(|&x| x == c)
                .map(|index| ENCODES[index])
                .unwrap_or(c)
        })
        .collect()
}

fn decode(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            ENCODES
                .iter()
                .position(|&x| x == c)
                .map(|index| DECODES[index])
                .unwrap_or(c)
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: ./lighthide [-encode|-decode] [string]");
        return;
    }

    match &args[1][..] {
        "-encode" => {
            let encoded = encode(&args[2]);
            let decoded = decode(&encoded);
            println!("encoded[{}]:\n{}", decoded, encoded);
        }
        "-decode" => {
            let decoded = decode(&args[2]);
            let encoded = encode(&decoded);
            println!("decoded[{}]:\n{}", &args[2], decoded);
        }
        _ => println!("Usage: ./lighthide [-encode|-decode] [string]"),
    }
}

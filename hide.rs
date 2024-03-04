//Adapted from lightaidra

let encodes: [char; 66] = [
  '%', 'q', '*', 'K', 'C', ')', '&', 'F', '9', '8', 'f', 's', 'r', '2', 't', 'o', '4', 'b', '3',
  'y', 'i', '_', ':', 'w', 'B', '>', 'z', '=', ';', '!', 'k', '?', '"', 'E', 'A', 'Z', '7', '.',
  'D', '-', 'm', 'd', '<', 'e', 'x', '5', 'U', '~', 'h', ',', 'j', '|', '$', 'v', '6', 'c', '1',
  'g', 'a', '+', 'p', '@', 'u', 'n'
];

let decodes: [char; 66] = [
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
  'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
  'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '.', ' '
]

let encoded[char; 512];
let decoded[char; 512];

fn encode() {
  let x = 0;
  let i = 0;
  let c = 0;
  memset(encoded, 0, sizeof(encoded))
}

fn decode() {

}

fn main() {

}

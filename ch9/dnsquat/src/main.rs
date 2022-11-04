use std::env;

fn bitflip(charac: u8, pos: u8) -> u8 {
    let shiftval = 1 << pos;
    charac ^ shiftval
}

fn is_valid(charc: char) -> bool {
    charc.is_ascii_alphanumeric() || charc == '-'
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage: dnsquat domain .com");
        return;
    }

    let name = args[1].to_lowercase();
    let tld = args[2].to_lowercase();

    for i in 0..name.len() {
        let charc = name.as_bytes()[i];
        for bit in 0..8 {
            let bitflipped = bitflip(charc.into(), bit);
            if is_valid(bitflipped as char)
                && bitflipped.to_ascii_lowercase() != charc.to_ascii_lowercase()
            {
                let mut bitsquatting_candiat = name.as_bytes()[..i].to_vec();
                bitsquatting_candiat.push(bitflipped);
                bitsquatting_candiat.append(&mut name.as_bytes()[i + 1..].to_vec());
                println!(
                    "{}{}",
                    String::from_utf8(bitsquatting_candiat).unwrap(),
                    tld
                );
            }
        }
    }
}

use md5;
use message::{MD5HashCashInput,MD5HashCashOutput};
use rand::distributions::{Alphanumeric, DistString};



pub fn md5challenge(input: MD5HashCashInput) -> MD5HashCashOutput {

    let mut output: MD5HashCashOutput = MD5HashCashOutput {
        hashcode: "".to_string(),
        seed: 0,
    };

    let mut finished = false;
    let mut seed = 0;
    let mut hashcode: String = "".to_string();

    while finished == false {
        let seed_in_hexa = convert_decimal_to_hexa(seed);
        let concatenated_seed = concat_message(seed_in_hexa.to_string(), input.message.to_string());
        let digest = md5::compute(concatenated_seed);
        hashcode = convert_digest_to_hexa(digest);
        let mut binary_hashcode: String = convert_to_binary(&hashcode);
        finished = respect_complexity(binary_hashcode, input.complexity);
        seed += 1;
    }
     output.seed = seed.to_string();
     output.hashcode = hashcode;

     return output;

}


fn convert_decimal_to_hexa(value: i32) -> String {
    format!("{:016X}", value);
}

fn convert_digest_to_hexa(digest: Digest) -> String {
    format!("{:032X}", digest)
}

fn concat_message(seed: String, message: String) -> String {
    format!("{}{}\n", seed, message)
}

fn convert_to_binary(hashCode: &String) -> String {
    hashCode.chars().map(binary).collect();
}

fn respect_complexity(binary: String, complexity: u32) -> bool{

    let mut i = 0;
    for b in binary.chars() {
        if b == '1' && i < complexity {
            return false;
        } else if i >= complexity {
            return true;
        }
        i += 1;
    }
    return false;

}

fn binary(c: char) -> String {
    let value = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };
    return String::from(value);
}


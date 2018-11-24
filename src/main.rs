extern crate rand;
extern crate promptly;

use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use promptly::prompt;

static CHAR_LIST: [&str; 10] = ["M", "Q", "S", "Y", "L", "J", "Z", "C", "H", "G"];

#[derive(PartialEq)]
enum Edition {
    //Single License 1 PC
    SG,
    //Personal License 3 PCs
    PS,
    //Home License 10 PCs
    HM,
    //Team License (> 10 PCs)
    TM,
    //Enterprise License (>100 PCs)
    EP,
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Edition::SG => write!(f, "SG"),
            Edition::PS => write!(f, "PS"),
            Edition::HM => write!(f, "HM"),
            Edition::TM => write!(f, "TM"),
            Edition::EP => write!(f, "EP"),
        }
    }
}


fn num_to_string(num: u32) -> String {
    let mut n = num;
    let mut digits = Vec::new();
    let mut num_vec = Vec::new();

    if num < 1000000 && num >= 11 {
        if num < 100 {
            num_vec.push("MMMM")
        } else if num < 1000 {
            num_vec.push("MMM")
        } else if num < 10000 {
            num_vec.push("MM")
        }else if num <100000 {
            num_vec.push("M")
        }

        while n > 0 {
            digits.push(CHAR_LIST[(n % 10) as usize]);
            n /= 10;
        }
        digits.reverse();
        num_vec.append(digits.as_mut())
    } else {
        num_vec.push("MMMMQQ")
    }

    num_vec.join("").to_string()
}


fn get_rand(list: &[char]) -> char {
    let mut rng = thread_rng();
    let choice = *list.choose(&mut rng).unwrap();
    choice
}


fn keygen(edition: Edition, no_of_pcs: u32) -> String {
    let mut key = String::new();
    let mut chr: char;
    key.push_str("NRSP-");
    key.push(get_rand(&['M', 'Q', 'S', 'Y', 'L', 'J']));
    key.push(get_rand(&['C', 'G', 'H', 'J', 'L', 'M', 'Q', 'S', 'Y', 'Z']));
    chr = get_rand(&['M', 'Q']);
    key.push(chr);
    match chr {
        'M' => key.push(get_rand(&['C', 'G', 'H', 'J', 'L', 'Q', 'S', 'Y', 'Z'])),
        'Q' => key.push(get_rand(&['M', 'Q'])),
        _ => println!("error!")
    }
    key.push('-');
    chr = get_rand(&['M', 'Q', 'S', 'Y']);
    key.push(chr);
    match chr {
        'M' => key.push(get_rand(&['C', 'G', 'H', 'J', 'L', 'Q', 'S', 'Y', 'Z'])),
        'Q' | 'S' => key.push(get_rand(&['C', 'G', 'H', 'J', 'L', 'M', 'Q', 'S', 'Y', 'Z'])),
        'Y' => key.push(get_rand(&['M', 'Q'])),
        _ => println!("error!")
    }
    key.push_str(&edition.to_string());
    key.push('-');
    chr = get_rand(&['M', 'Q', 'S', ]);
    key.push(chr);
    match chr {
        'M' | 'Q' => key.push(get_rand(&['C', 'G', 'H', 'J', 'L', 'M', 'Q', 'S', 'Y', 'Z'])),
        'S' => key.push(get_rand(&['M', 'Q', 'S', 'Y'])),
        _ => println!("error!")
    }
    key.push(get_rand(&['M', 'Q', 'S', 'Y', 'L', 'J']));
    key.push(get_rand(&['C', 'G', 'H', 'J', 'L', 'M', 'Q', 'S', 'Y', 'Z']));

    match edition {
        Edition::TM | Edition::EP => {
            key.push('-');
            key.push_str(&num_to_string(no_of_pcs))
        }
        _ => ()
    }
    key
}

fn main() {
    let i: u32;
    println!("License Generator for Advanced Date and Time Calculator");
    i = prompt("License for Number of PCs (1-999999)");

    if i <= 1 {
        println!("License: {}", keygen(Edition::SG, i));
    } else if i < 4 {
        println!("License: {}", keygen(Edition::PS, i));
    } else if i < 11 {
        println!("License: {}", keygen(Edition::HM, i));
    } else if i < 101 {
        println!("License: {}", keygen(Edition::TM, i));
    } else {
        println!("License: {}", keygen(Edition::EP, i));
    }
}
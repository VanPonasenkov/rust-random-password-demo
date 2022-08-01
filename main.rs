use math::round;
use rand::*;

type RandomAppender = fn(&mut String, u8) -> ();

fn option_vec_length<T>(vector: &Vec<Option<T>>) -> u8 {
    let mut counter: u8 = 0;
    for element in vector {
        match element {
            Some(_) => counter += 1,
            _ => (),
        }
    }
    return counter;
}

fn append_numbers(password: &mut String, upper_limit: u8) -> () {
    let mut rng = rand::thread_rng();

    for _ in 0..upper_limit {
        let num: u32 = rng.gen_range(0..8);
        password.push_str(&num.to_string());
    }
}

fn generate_password(is_numbers: bool, pass_len: u8) -> String {
    let pipeline: Vec<Option<RandomAppender>> = vec![if is_numbers {
        Some(append_numbers)
    } else {
        None
    }];
    let mut vec_len = option_vec_length(&pipeline);
    let medium = round::ceil((pass_len / vec_len).into(), 0);
    let mediums: Vec<u8> = vec![medium as u8; vec_len.into()];
    let mut i = 0;
    let mut password = String::new();

    for pipe in pipeline {
        match pipe {
            Some(f) => f(&mut password, mediums[i]),
            None => (),
        }
        i += 1
    }
    return password;
}

fn main() {
    println!("{}", generate_password(true, 16));
}

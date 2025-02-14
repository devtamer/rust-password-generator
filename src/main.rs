use std::env;
use rand::{distr::Alphanumeric, Rng};

fn password_generator(length: Option<u16>){
    let mut password: String = Default::default();
    let length = length.unwrap_or(12);
    // longer method
    for _ in 0..=length {
        let randchar: char = rand::rng().sample(Alphanumeric)  as char;
        password.push(randchar);
    }
    println!("Your new password is {password}");
    // one liner method
    //let rand_string: String = rand::rng().sample_iter(&Alphanumeric).take(length as usize).map(char::from).collect();
    //println!("Your new password is {rand_string}");
}
fn main() {
   let args: Vec<String> = env::args().collect();
   match args.len() {
    1 => password_generator(None),
    2 => {
        let num: u16 = args[1]
        .parse::<u16>()
        .expect("Please enter a valid number between 1 and 65535");
        password_generator(Some(num));
    },
    _ => ()
   }
}

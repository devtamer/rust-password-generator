use std::env;
use rand::{distr::Alphanumeric, Rng};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn password_generator(length: Option<u16>, symbols: Option<bool>){
    let mut password: String = Default::default();
    let length = length.unwrap_or(12);
    let symbols = symbols.unwrap_or(true);
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789$%#@*&^!)(";
    // longer method
    //for _ in 0..=length {
    //    let randchar: char = rand::rng().sample(Alphanumeric)  as char;
    //    password.push(randchar);
    //}
    // one liner method
    if symbols {
        password = (0..length).map(|_| {
            let idx = rand::random_range(0..charset.len());
            charset[idx] as char
        }).collect();
    } else {
        password = rand::rng().sample_iter(&Alphanumeric).take(length as usize).map(char::from).collect();
    }
    let mut ctx = cli_clipboard::ClipboardContext::new().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), password);
    println!("Your new password\n--------------------\n{password}\n--------------------\nhas been copied to your clipboard!");
    
}
fn main() {
   let args: Vec<String> = env::args().collect();
   match args.len() {
    1 => password_generator(None, None),
    2 => {
        let arg = args[1].clone();
        let mut symbols = true;
        let mut num : Option<u16> = None;
        if arg.chars().all(|c| c.is_ascii_digit()) {
            num = Some(args[1]
            .parse::<u16>()
            .expect("Please enter a valid number between 1 and 65535"));
        } else if arg.contains("--no-symbols") {
            symbols = false;
        }
        password_generator(num, Some(symbols));

    },
    3 => {
        let mut symbols = true;
        let mut length: Option<u16> = None;
        for arg in args {
            let arg = arg.to_lowercase();
            println!("arg is {arg}");
            if arg.chars().all(|c| c.is_ascii_digit()) && length == None {
                length = Some(arg.parse::<u16>().expect("Please enter a valid number between 1 and 65535"));
            } else if arg.contains("--no-symbols") {
                symbols = false;
            }
        }
        password_generator(length, Some(symbols));
    },
    _ => ()
   }
}

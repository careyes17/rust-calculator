use std::io::{stdin, stdout, Write};

pub fn read_input() -> String {
    let mut input = String::new();
    stdout().flush().expect("failed to flush");
    stdin().read_line(&mut input).expect("failed to read");
    input
}

pub fn get_number(prompt: &str) -> f32 {
    let num: f32;
    loop {
        println!("{}", prompt);
        num = match read_input().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR: invalid number submitted.");
                continue;
            },
        };
        break
    }
    num
}

pub fn get_operator(prompt: &str) -> String {
    let mut operator: String;
    loop {
        println!("{}", prompt);
        operator = read_input().trim().to_string();
        match &operator[..] {
            "+" => break,
            "-" => break,
            "*" => break,
            "/" => break,
            _ => {
                println!("invalid operator submitted.");
                continue;
            }
        }
    }
    operator
}

pub fn get_menu_option(prompt: &str) -> i32 {
    let mut item: i32;
    loop {
        println!("{}", prompt);
        item = match read_input().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR: invalid item selected.\n");
                continue;
            },
        };
        match item {
            1 => break,
            2 => break,
            _ => {
                println!("ERROR: invalid item selected.\n");
                continue;
            }
        }
    }
    item
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
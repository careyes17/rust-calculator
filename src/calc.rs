mod input;

#[allow(dead_code)]
enum Menu {
    CalculatorMode = 1,
    Exit = 2
}

fn main() {
    input::clear_terminal();
    println!("Welcome to the Rust Calculator");
    loop {
        println!("------------------------------\n");
        let selection: i32 = input::get_menu_option(
            "Select one of the following options:\n  1) Calculator mode\n  2) Exit\n"
        );
        if selection == Menu::Exit as i32 { break };
        input::clear_terminal();
        let num1: f32 = input::get_number("Please input the first number.");
        let num2: f32 = input::get_number("Please input the second number.");
        let operator: String = input::get_operator("Please input the desired operator: +, -, *, /");
        let product: f32 = match perform_calculation(num1, num2, &operator) {
            Ok(product) => product,
            Err(error) => {
                println!("\nERROR: {}", error);
                continue;
            }
        };
        println!("\nResult: {} {} {} = {}", num1, operator, num2, product)
    }
    println!("Shutting down...")
}

fn perform_calculation(num1: f32, num2: f32, operator: &String) -> Result<f32, &'static str> {
    return match &operator[..] {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("ERROR: You cannot divide a number by 0.")
            } else {
                Ok(num1 / num2)
            }
        }
        _ => panic!("ERROR: Something has gone wrong internally."),
    };
}

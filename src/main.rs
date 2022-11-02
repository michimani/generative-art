mod images;
mod methods;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let method = &args[1];
    let mut sub_command = "";
    if args.len() > 2 {
        sub_command = &args[2];
    }

    render(&method, &sub_command);
}

fn render(method: &str, sub_command: &str) {
    match method {
        "gcd" => methods::gcd::render(sub_command),

        _ => println!("method {} is not found.", method),
    }
}

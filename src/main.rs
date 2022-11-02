mod images;
mod methods;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let method = &args[1];
    render(&method);
}

fn render(method: &str) {
    match method {
        "gcd" => methods::gcd::render(),

        _ => println!("method {} is not found.", method),
    }
}

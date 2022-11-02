use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let method = &args[1];
    render(&method);
}

fn render(method: &str) {
    match method {
        _ => println!("method {} is not found.", method),
    }
}

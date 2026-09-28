fn greet(name: &str) -> Result<String, &'static str> {
    let cleaned = name.trim();
    if cleaned.is_empty() {
        return Err("name must not be empty");
    }
    Ok(format!("Hello, {cleaned}!"))
}

fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "world".to_string());

    match greet(&name) {
        Ok(message) => println!("{message}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

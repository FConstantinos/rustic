pub fn error(message: &String) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

pub fn warn(message: &String) {
    eprintln!("Warning: {}", message);
}

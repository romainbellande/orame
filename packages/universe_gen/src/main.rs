fn main() {
    if let Err(e) = universe_gen::run() {
        eprintln!("Error: {}", e);

        std::process::exit(1);
    }
}

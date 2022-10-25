fn main() {
    if let Err(err) = reqr::run() {
        eprintln!("{}", err);
        std::process::exit(1)
    }
}

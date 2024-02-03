fn main() {
    if let Err(e) = my_echo::get_args().and_then(my_echo::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

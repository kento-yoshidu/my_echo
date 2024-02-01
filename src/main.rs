use clap::{App, Arg};

fn main() {
    /*
    if let Err(e) = my_echo::get_args().and_then(my_echo::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    */

    let matches = App::new("my_echo")
        .version("0.1.0")
        .author("Kento Yoshizu <toriwatari@0225.gmail.com")
        .about("Toy echo command by Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)  // 環境変数のみ出力も可能にするから、後で消す
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("env")
                .short("e")
                .long("env")
                .help("Show environment variable")
        )
        .get_matches();

    println!("{:#?}", matches);
}

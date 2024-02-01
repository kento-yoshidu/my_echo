use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    text: Option<Vec<String>>,
    omit_newline: bool,
    env: Option<String>
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("my_echo")
        .version("0.1.0")
        .author("Kento Yoshizu <toriwatari@0225.gmail.com")
        .about("Toy echo command by Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
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
                .takes_value(true)
                .conflicts_with("text")
        )
        .get_matches();

    let text = matches.values_of_lossy("text");
    let omit_newline = matches.is_present("omit_newline");
    let env = matches.value_of("env").map(|c| c.to_string());

    Ok(Config {
        text,
        omit_newline,
        env,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut result = Vec::new();

    if let Some(vec) = config.text {
        for str in vec {
            result.push(str);
        }
    }

    if let Some(env) = config.env {
        match std::env::var(env.as_str()) {
            Ok(res) => {
                let vec: Vec<&str> = res.split(";").collect();

                for path in vec {
                    result.push(format!("{}\n", path.to_string()));
                }
            },
            Err(e) => {
                eprintln!("err! {}", e);
                std::process::exit(1);
            }
        }
    }

    for str in result {
        print!("{} ", str);
    }

    if !config.omit_newline {
        println!();
    }

    Ok(())
}

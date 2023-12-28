use clap::Parser;

#[derive(Parser)]
struct Cli {
    first_name: String,
    last_name: Option<String>,
}

fn main() {
    let args = Cli::parse();
    println!(
        "Hello {} {}, world!",
        args.first_name,
        match args.last_name {
            Some(name) => name,
            None => String::from(""),
        }
    );
}

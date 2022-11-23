use clap::Parser;

#[derive(Parser)]
struct Cli {
    method: String,
    url: String,
}

fn main() { 
    let args = Cli::parse();
    println!("{}", &args.method);
    println!("{}", &args.url);
}


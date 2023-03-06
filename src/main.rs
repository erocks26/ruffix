extern crate clap;
use clap::Parser;
use std::io::Write;
mod suffix;
use suffix::suffixer;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// word to prepend <SUFFIX> onto
    #[clap(short,long)]
    word: String,
    /// what to append to <WORD>
    #[clap(short,long)]
    suffix: String,
}

fn main() {
    let cli = Cli::parse();
    let word = cli.word;
    #[allow(unused_assignments)]
    let suffix: String = cli.suffix;

    let result: String = match suffixer(&word.as_str(), &suffix.as_str()) {
        Ok(o) => o,
        Err(e) => {
            println!("Error: {:?}", e);
            std::process::exit(2);
        }
    };

    let mut stdout = std::io::stdout();

    match writeln!(stdout, "{}", &result) { // writeln because we can properly handle errors
        Err(e) => { // match errors
            if e.kind() != std::io::ErrorKind::BrokenPipe { // if it isnt a broken pipe (the reason this is necessary)
                eprintln!("{}", e); // print the error
                std::process::exit(1); // and exit
            }
        }
        Ok(o) => o, // basaically just unwrap but errors are handled above
    };
}

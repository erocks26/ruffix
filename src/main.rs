extern crate clap;
use clap::Parser;
mod suffix;
use suffix::suffixer;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// word to prepend <SUFFIX> onto
    #[clap(short,long,default_value = "sus")]
    word: String,
    /// what to append to <WORD>
    #[clap(short,long,default_value = "ussy")]
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
    println!("{}", result);
}

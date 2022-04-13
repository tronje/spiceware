mod short_words;
mod words;

use clap::Parser;

/// Generate diceware-like passphrases
#[derive(Parser)]
#[clap(version = "1.0.0")]
struct Arguments {
    /// The number of words a passphrase shall be made up of
    #[clap(
        short = 'w',
        long = "words",
        value_name = "n",
        required = false,
        default_value = "4"
    )]
    num_words: usize,

    /// The number of passphrases to generate
    #[clap(
        short = 'n',
        long = "passphrases",
        value_name = "n",
        required = false,
        default_value = "1"
    )]
    num_passwords: usize,

    #[clap(short = 'd', long = "delimiter", default_value = " ")]
    delimiter: String,

    /// Print nothing but the passphrase (implied when -n is used)
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,

    /// Use the list of short words
    #[clap(short = 's', long = "short")]
    short: bool,
}

/// Generate a passphrase made up of `n` words
fn gen_passphrase(n: usize, delimiter: &str, short: bool) -> String {
    (0..n)
        .map(|_| {
            if short {
                short_words::get_word()
            } else {
                words::get_word()
            }
        })
        .collect::<Vec<&str>>()
        .join(delimiter)
}

fn main() {
    let args = Arguments::parse();

    if args.num_passwords > 1 {
        for _ in 0..args.num_passwords {
            println!(
                "{}",
                gen_passphrase(args.num_words, &args.delimiter, args.short)
            );
        }

        return;
    }

    let password = gen_passphrase(args.num_words, &args.delimiter, args.short);

    if args.quiet {
        println!("{}", password);
        return;
    }

    let possible_combinations = if args.short {
        short_words::possible_combinations(args.num_words as f64)
    } else {
        words::possible_combinations(args.num_words as f64)
    };

    let power_of_ten = possible_combinations.log10().floor() as u64;

    println!("Your password is:");
    println!();
    println!("\t{}", password);
    println!();
    println!(
        "This password is one of about 10^{} possible combinations.",
        power_of_ten
    );
}

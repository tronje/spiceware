mod short_words;
mod words;

use clap::Parser;
use rand::Rng;
use short_words::SHORT_WORDS;
use words::WORDS;

/// Generate diceware-like passphrases
#[derive(Parser)]
#[clap(version = "1.0.0")]
struct Spiceware {
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

impl Spiceware {
    fn main(self) {
        if self.num_passwords > 1 || self.quiet {
            self.batch_mode();
        } else {
            self.verbose_mode();
        }
    }

    fn batch_mode(self) {
        for _ in 0..self.num_passwords {
            let passphrase = self.gen_passphrase();
            println!("{}", passphrase);
        }
    }

    fn verbose_mode(self) {
        let possible_combinations = self.possible_combinations();
        let power_of_ten = possible_combinations.log10().floor() as u64;

        let passphrase = self.gen_passphrase();
        println!("Your password is:");
        println!();
        println!("\t{}", passphrase);
        println!();
        println!(
            "This password is one of about 10^{} possible combinations.",
            power_of_ten
        );
    }

    fn wordlist(&self) -> &[&str] {
        if self.short {
            &SHORT_WORDS
        } else {
            &WORDS
        }
    }

    fn get_word(&self) -> &str {
        let mut rng = rand::thread_rng();
        let wordlist = self.wordlist();
        let index = rng.gen_range(0..wordlist.len());
        wordlist[index]
    }

    fn possible_combinations(&self) -> f64 {
        let wordlist = self.wordlist();
        let num_words = self.num_words as f64;
        (wordlist.len() as f64).powf(num_words)
    }

    fn gen_passphrase(&self) -> String {
        (0..self.num_words)
            .map(|_| self.get_word())
            .collect::<Vec<&str>>()
            .join(&self.delimiter)
    }
}

fn main() {
    let args = Spiceware::parse();
    args.main();
}

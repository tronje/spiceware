mod short_words;
mod words;

use clap::Parser;
use rand::Rng;
use short_words::SHORT_WORDS;
use words::WORDS;

/// Generate diceware-like passphrases
#[derive(Parser)]
#[command(version)]
struct Spiceware {
    /// The number of words a passphrase shall be made up of
    #[clap(
        short = 'w',
        long = "words",
        value_name = "n",
        required = false,
        default_value = "4"
    )]
    num_words: u32,

    /// The number of passphrases to generate
    #[clap(
        short = 'n',
        long = "passphrases",
        value_name = "n",
        required = false,
        default_value = "1"
    )]
    num_passwords: u32,

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
        let (power_of_ten, overflowed) = match self.possible_combinations() {
            Some(combs) => (combs.ilog10(), false),
            None => (usize::MAX.ilog10(), true),
        };

        let passphrase = self.gen_passphrase();

        let qualifier = if overflowed { "over" } else { "about" };

        println!("Your password is:\n");
        println!("\t{}\n", passphrase);
        println!("This password is one of {qualifier} 10^{power_of_ten} possible combinations.");
    }

    fn wordlist(&self) -> &[&str] {
        if self.short {
            &SHORT_WORDS
        } else {
            &WORDS
        }
    }

    fn worst_case_passphrase_size(&self) -> usize {
        let word_size = if self.short {
            short_words::MAX_SIZE
        } else {
            words::MAX_SIZE
        };

        let delimiter_size = self.delimiter.len() * (self.num_words as usize - 1);
        self.num_words as usize * word_size + delimiter_size
    }

    fn possible_combinations(&self) -> Option<usize> {
        self.wordlist().len().checked_pow(self.num_words)
    }

    fn get_word(&self) -> &str {
        let mut rng = rand::thread_rng();
        let wordlist = self.wordlist();
        let index = rng.gen_range(0..wordlist.len());
        wordlist[index]
    }

    fn gen_passphrase(&self) -> String {
        let mut passphrase = String::with_capacity(self.worst_case_passphrase_size());
        for _ in 0..self.num_words - 1 {
            passphrase.push_str(self.get_word());
            passphrase.push_str(&self.delimiter);
        }

        passphrase.push_str(self.get_word());

        passphrase
    }
}

fn main() {
    let args = Spiceware::parse();
    args.main();
}

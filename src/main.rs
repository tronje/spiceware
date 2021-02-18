#[macro_use]
extern crate clap;
extern crate rand;

mod words;

use words::WORDS;

/// Generate diceware-like passphrases
#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Arguments {
    /// The number of words a passphrase shall be made up of
    #[clap(
        short = "w",
        long = "words",
        value_name = "n",
        required = false,
        default_value = "4"
    )]
    num_words: usize,

    /// The number of passphrases to generate
    #[clap(
        short = "n",
        long = "passphrases",
        value_name = "n",
        required = false,
        default_value = "1"
    )]
    num_passwords: usize,

    /// Print nothing but the passphrase (implied when -n is used)
    #[clap(short = "q", long = "quiet")]
    quiet: bool,
}

/// A primitive time type that has hour-resolution
struct Time {
    years: f64,
    days: f64,
    hours: f64,
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.years < 0.1 {
            if self.days < 0.1 {
                write!(f, "{} hours", self.hours)
            } else {
                write!(f, "{} days, {} hours", self.days, self.hours)
            }
        } else if self.years <= 10f64.powf(10.0) {
            write!(f, "{} years, {} days", self.years, self.days)
        } else {
            write!(f, "10^{} years", self.years.log10().floor())
        }
    }
}

/// Roll `n` dice and collect the results into a vector.
fn roll_dice(n: usize) -> Vec<u8> {
    use rand::distributions::Standard;
    use rand::Rng;

    rand::thread_rng()
        .sample_iter(Standard)
        .take(n)
        .map(|i: u8| i % 6 + 1)
        .collect()
}

/// Translate the diceware key to an index
///
/// A "diceware key" in this case is a set of 5 die-throw-results. To translate this to an index,
/// we subtract one from each throw, and then treat these as a base-6 number. That gives us the
/// index into the word list, eliminating the need for a map of some kind.
///
/// # Examples
///
/// If our throws are `[6, 5, 4, 3, 2]`, we subtract one from each, and then combine them into a
/// single number, 54321. When treated as base-6, this number is equal to 7465 in base-10, which is
/// our index into the word-list.
fn to_index(key: &Vec<u8>) -> usize {
    key.iter()
        .rev()
        .enumerate()
        .map(|(pow, &n)| (n as usize - 1) * 6_usize.pow(pow as u32))
        .sum()
}

/// Get a random word from the word list. The index is generated by throwing 5 dice.
///
/// Of course, a word could just be picked by generating a single random number in the range [0,
/// list length), but then it wouldn't be diceware.
fn get_word<'a>() -> &'a str {
    let key = roll_dice(5);
    let index = to_index(&key);
    WORDS[index]
}

/// Generate a passphrase made up of `n` words
fn gen_passphrase(n: usize) -> String {
    (0..n).map(|_| get_word()).collect::<Vec<&str>>().join(" ")
}

/// Compute the rounded-down time to guess one of `possible_combinations` when performing
/// `guesses_per_s` guesses per second.
///
/// The assumption here is that half of all combinations need to be generated on average to arrive
/// at the correct combination.
fn time_to_guess(guesses_per_s: f64, possible_combinations: f64) -> Time {
    // divide by two to get average guess time instead of the time to visit all possible
    // combinations
    let seconds_to_guess = possible_combinations / guesses_per_s / 2.0;
    let minutes = seconds_to_guess / 60.0;
    let hours = minutes / 60.0;
    let days = hours / 24.0;

    let years = days / 365.2425;

    let remaining_days = days % 365.2425;
    let remaining_hours = hours % 24.0;

    Time {
        years: years.floor(),
        days: remaining_days.floor(),
        hours: remaining_hours.floor(),
    }
}

fn main() {
    let args = Arguments::parse();

    if args.num_passwords > 1 {
        for _ in 0..args.num_passwords {
            println!("{}", gen_passphrase(args.num_words));
        }

        return;
    }

    let password = gen_passphrase(args.num_words);

    if args.quiet {
        println!("{}", password);
        return;
    }

    let possible_combinations = (WORDS.len() as f64).powf(args.num_words as f64);
    let power_of_ten = possible_combinations.log10().floor() as u64;

    println!("Your password is:");
    println!();
    println!("\t{}", password);
    println!();
    println!(
        "This password is one of 10^{} possible combinations.",
        power_of_ten
    );
    println!();

    println!("Assuming 1,000,000 guesses per second, the average time it takes to guess your password is:");
    let time = time_to_guess(1_000_000f64, possible_combinations);
    println!(
        "\t{} if the attacker knows the scheme used to generate your password",
        time
    );

    // the magic number `62` is derived from the assumption that there are 62 possible characters
    // in any given password: 26 lower-case letters + 26 upper-case letters + 10 digits
    // this is quite generous, of course, because it doesn't include special characters like
    // punctuation, etc.
    let possible_combinations = 62f64.powf(password.len() as f64);
    let time = time_to_guess(1_000_000f64, possible_combinations);
    println!("\t{} if the attacker does not know the scheme", time);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_to_index() {
        let key = vec![1, 1, 1, 1, 1];
        assert_eq!(to_index(&key), 0);

        let key = vec![1, 2, 3, 4, 5];
        assert_eq!(to_index(&key), 310);

        let key = vec![6, 6, 6, 6, 6];
        assert_eq!(to_index(&key), 7775);
    }

    #[test]
    fn test_get_word() {
        use std::collections::HashMap;

        let iterations = 1_000_000;
        let expected_mean = iterations / WORDS.len();

        let mut word_occurrences = HashMap::new();
        for word in WORDS.iter() {
            word_occurrences.insert(*word, 0);
        }

        for _ in 0..iterations {
            let word = get_word();
            let count = word_occurrences.entry(word).or_insert(0);
            *count += 1;
        }

        for (_, count) in word_occurrences {
            assert!(count < (expected_mean as f64 * 1.5) as usize);
            assert!(count > (expected_mean as f64 * 0.5) as usize);
        }
    }

    #[test]
    fn test_gen_passphrase() {
        for n in 1..50 {
            let pw = gen_passphrase(n);
            assert!(pw.split(' ').collect::<Vec<_>>().len() == n);
        }
    }

    #[test]
    fn test_time_to_guess() {
        let guesses_per_s = 1_000.0;

        let possible_combinations = 2.0 * 60.0 * 60.0 * guesses_per_s;
        let t = time_to_guess(guesses_per_s, possible_combinations);
        assert_eq!(t.hours as u32, 1);

        let possible_combinations = 24.0 * possible_combinations;
        let t = time_to_guess(guesses_per_s, possible_combinations);
        assert_eq!(t.days as u32, 1);

        let possible_combinations = 365.2425 * possible_combinations;
        let t = time_to_guess(guesses_per_s, possible_combinations);
        assert_eq!(t.years as u32, 1);
    }
}

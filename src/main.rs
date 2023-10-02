use rand::seq::SliceRandom;
use std::io;

const NUM_DIGITS: u8 = 3;
const MAX_TURNS: u32 = 10;

struct SecretNumber {
    secret_number: String,
}

impl SecretNumber {
    fn new(number: u8) -> Self {
        let mut str = String::new();

        let mut numbers: Vec<u32> = (0..=9).collect();
        numbers.shuffle(&mut rand::thread_rng());

        for i in 0..number {
            let index: usize = i as usize;
            str += format!("{}", numbers[index]).as_str();
        }

        Self { secret_number: str }
    }

    fn get(&self) -> &String {
        &self.secret_number
    }

    fn to_vec(&self) -> Vec<char> {
        self.secret_number.chars().collect()
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

fn check_guess (secret: &SecretNumber, player_guess: &String) -> bool {
    &secret.get().trim() == &player_guess.trim()
}

fn get_hints<'a>(secret: &SecretNumber, player_guess: &String) -> Result<bool, Vec<&'a str>> {
    let secret_vec = secret.to_vec();
    let player_guess_vec: Vec<char> = player_guess.chars().collect();
    println!("{}", secret_vec.len());

    let mut hints = vec![];
    for _ in 0..secret_vec.len() {
        hints.push("bagel");
    }

    for index in 0..secret_vec.len() {
        // convert i into usize
        let index: usize = index.into();

        if secret.get().contains(player_guess_vec[index]) {
            hints[index] = "pico";
        }

        if player_guess_vec[index] == secret_vec[index] {
            hints[index] = "fermi";
        }
    }
    Err(hints)
}

fn main() {
    println!("\n          _|_|_|      _|_|      _|_|_|  _|_|_|_|  _|");
    println!("          _|    _|  _|    _|  _|        _|        _|");
    println!("          _|_|_|    _|_|_|_|  _|  _|_|  _|_|_|    _|");
    println!("          _|    _|  _|    _|  _|    _|  _|        _|");
    println!("          _|_|_|    _|    _|    _|_|_|  _|_|_|_|  _|_|_|_|");
    println!("A deductive logic game where you must guess a number based on clues");

    loop {
        let mut win: bool = false;

        println!("How many digits would you like in the secret number?");
        let num_digits: u8 = match get_input().expect("something went wrong").trim().parse() {
            Ok(v) => {
                match v {
                    1 => {
                        println!("must be at least 3. Using 3 digits...");
                        3
                    }
                    2 => {
                        println!("must be at least 3. Using 3 digits...");
                        3
                    },
                    _ => v,
                }
            }
            _ => 3,
        };

        println!("\n                          Instructions                             ");
        println!("I'm thinking of a {num_digits} digit number. Try to guess what it is.\n");
        println!("|-----------------------------------------------------------------|");
        println!("| When I say  | That means                                        |");
        println!("|-----------------------------------------------------------------|");
        println!("| Pico        | One digit is correct, but it's in the wrong place |");
        println!("| Fermi       | One digit is correct and it's in the right place  |");
        println!("| Bagel       | No digit is correct                               |");
        println!("|-----------------------------------------------------------------|");

        let secret_num = SecretNumber::new(num_digits);
        for i in 1..=MAX_TURNS {
            println!("Turn {}", i);
            let input = match get_input() {
                Some(value) => value,
                None => continue,
            };

            if check_guess(&secret_num, &input) {
                win = true;
                break;
            }

            match get_hints(&secret_num, &input) {
                Ok(v) => {
                    println!("{}", v);
                    println!("Press any key to continue...");
                    match get_input() {
                        _ => break,
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
        if win {
            println!("You Win!")
        } else {
            println!("Sorry, you lose");
        }
        println!("Would you like to play again? (y/n)");

        if get_input().expect("invalid input").to_lowercase().trim() != "y" {
            break;
        }
    }
}

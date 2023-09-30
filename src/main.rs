use std::io;
use rand::seq::SliceRandom;

const NUM_DIGITS: u8 = 3;
const  MAX_TURNS: u32 = 10;

struct SecretNumber {
    secret_number: String,
}

impl SecretNumber {
    fn new() -> Self {
	let mut str = String::new();

	let mut numbers: Vec<u32> = (0..=9).collect();
	numbers.shuffle(&mut rand::thread_rng());

	for i in 0..NUM_DIGITS {
	    let index: usize = i as usize;
	    str += format!("{}", numbers[index]).as_str();
	}

	Self {
	    secret_number: str,
  }
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

fn get_hints<'a>(secret:  &SecretNumber, player_guess: &String) -> Result<String, Vec<&'a str>> {
    let secret_vec = secret.to_vec();
    let player_guess_vec: Vec<char> = player_guess.chars().collect();

    let mut hints = vec!["bagels","bagels","bagels"];

    for index in 0..NUM_DIGITS{
	// convert i into usize
	let index: usize = index.into();

	if &secret.get().trim() ==  &player_guess.trim() {
	    return Ok("You Win!!!".to_owned());
	}

	if secret.get().contains(player_guess_vec[index])  {
	    hints[index] = "pico";
	}

	if player_guess_vec[index] == secret_vec[index] {
	    hints[index] = "fermi";
	}

    }
    Err(hints)
}

fn main() {
    let secret_num = SecretNumber::new();

    println!("I have selected a three digit number that contains no repeated digits. you have {MAX_TURNS} guesses to guess what it is");

    for i in 1..=MAX_TURNS {
	println!("Turn {}", i);
	let input = match get_input() {
	    Some(value) => value,
	    None => continue,
	};
	match get_hints(&secret_num, &input) {
	    Ok(v) => {
		      println!("{}", v);
          println!("Press any key to continue...");
          match get_input() {
              _ => break,
          }
	    },
	    Err(e) => println!("{:?}", e),
	}
    }
}

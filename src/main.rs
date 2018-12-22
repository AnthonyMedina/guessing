use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 10);
    let mut count = 0;

    loop {
        if count == 0 {
            println!("guess a number, 1 to 10.");
        }
        else {
            println!("guess again.");
        }

        count += 1;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                count = 0;
                continue;
            }
        };

        println!("you guessed: {}.", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too smoll."),
            Ordering::Greater => println!("too beeg."),
            Ordering::Equal => {
                println!("spot on. it took you {} goes.", count);
                break;
            }
        }
    }
}

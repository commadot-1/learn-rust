use rand::Rng;
use std::cmp::Ordering;
use std::io;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_check_guess(move |guess: i32| {
        let ui = ui_handle..unwrap();
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let result = match guess.cmp(&secret_number) {
            Ordering::Less => ui.set_message("Too small!"),
            Ordering::Greater => ui.set_message("Too big!"),
            Ordering::Equal => ui.set_message("You win!"),
        };

    
    });

    ui.run()

   /*  println!("Guess the number!");
    //let ui = AppWindow::new()?;
    let secret_number = rand::thread_rng().gen_range(1..=100);
   
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
      
    }
   */
 
}

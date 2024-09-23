fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 0
    }else if guess > secret{
        return 1
    }else{
        return -1
    }
}
fn main() {
    let mut secret = 46;
    let mut guess = 45;
    let mut number_of_guesses = 0;
    loop{
        number_of_guesses += 1;
        if check_guess(guess, secret) == 0{
            println!("{}, That guess is correct!", guess);
            break;
        }else if check_guess(guess, secret) == 1{
            println!("{}, That guess was too high.", guess);
            guess -= 3;
        }else if check_guess(guess, secret) == -1{
            println!("{}, That guess was too low.", guess);
            guess += 2;
        }
    }
    println!("That took {} guesses.", number_of_guesses);
}

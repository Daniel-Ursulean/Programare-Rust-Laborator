mod app;
mod problems;

use app::*;
use problems::*;

fn main() {
    //P1
    let mut x: u16 = 64450;
    let mut l: bool = true;
    while l {
        match next_prime(x) {
            Some(y) => {
                println!("Next prime of {}: {}", x, y);
                x = y;
            }
            None => {
                println!("No next prime number for {x}");
                l = false;
            }
        }
    }

    //P2
    println!(
        "\nchecked_addition 1824 + 4562 = {}",
        checked_addition(1824, 4562)
    );
    //println!("checked_addition 4294967295 + 4562 = {}", checked_addition(4294967295, 1));
    println!(
        "checked_multiplication 1824 * 4562 = {}\n",
        checked_multiplication(1824, 4562)
    );
    //println!("checked_multiplication 2294967295 * 2 = {}", checked_multiplication(2294967295, 2));

    //P3
    function_tester();

    //P4
    println!();
    match to_uppercase('b') {
        Ok(c) => println!("Success: {}", c),
        Err(e) => print_error(e),
    }
    match to_uppercase('5') {
        Ok(c) => println!("Success: {}", c),
        Err(e) => print_error(e),
    }

    println!();
    match to_lowercase('G') {
        Ok(c) => println!("Success: {}", c),
        Err(e) => print_error(e),
    }
    match to_lowercase('@') {
        Ok(c) => println!("Success: {}", c),
        Err(e) => print_error(e),
    }

    println!();
    match print_char('A') {
        Ok(_) => (),
        Err(e) => print_error(e),
    }
    match print_char('\n') {
        Ok(_) => (),
        Err(e) => print_error(e),
    }

    println!();
    match char_to_number('7') {
        Ok(n) => println!("Success: number is {}", n),
        Err(e) => print_error(e),
    }
    match char_to_number('X') {
        Ok(n) => println!("Success: number is {}", n),
        Err(e) => print_error(e),
    }
    match char_to_number('ñ') {
        Ok(n) => println!("Success: number is {}", n),
        Err(e) => print_error(e),
    }

    println!();
    match char_to_number_hex('f') {
        Ok(n) => println!("Success: hex number is {}", n),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('g') {
        Ok(n) => println!("Success: hex number is {}", n),
        Err(e) => print_error(e),
    }

    //P5 Semi working Sign in/Log in app

    let mut username: String = String::new();
    let mut pass: String = String::new();

    println!("\n       ---Choose how to connect---       ");
    println!("Sign in (Enter 's')    Log in (Enter 'l)'");
    let mut choice = None;
    while choice.is_none() {
        println!("Please enter 'l' or 's':");
        choice = connection_options();
        if choice.is_none() {
            println!("Invalid input, please try again!\n");
        }
    }

    match choice {
        Some('l') => {
            let result = log_in(username.clone(), pass.clone());
            match result {
                Ok(_) => println!("Succsessfully logged in!"),
                Err(er) => app_error_management(er, &mut username, &mut pass),
            }
        }
        Some('s') => {
            let result = sign_in(&mut username, &mut pass);
            match result {
                Ok(_) => {
                    println!("Succsessfully signed in!");
                    let resultlog = log_in(username.clone(), pass.clone());
                    match resultlog {
                        Ok(_) => println!("Succsessfully logged in!"),
                        Err(er) => app_error_management(er, &mut username, &mut pass),
                    }
                }
                Err(er) => app_error_management(er, &mut username, &mut pass),
            }
        }
        _ => println!("Something went wrong"),
    }
}

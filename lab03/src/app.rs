use AppError::*;
use std::io;

// P5 application error
pub enum AppError {
    NotSigned,
    AlreadySigned,
    WrongData,
    TerminalError,
    CriticalError,
}

// P5 App functions
pub fn app_error_management(
    er: AppError,
    username: &mut String,
    pass: &mut String,
    attempts: &mut u8,
) {
    match er {
        AppError::NotSigned => {
            println!("You are not signed in. Please sign in first!");
            sign_in(username, pass, attempts);
        }
        AppError::AlreadySigned => {
            println!("You are already signed it. Please log in!");
            log_in(username, pass, attempts);
        }
        WrongData => {
            println!("Username or password is wrong");
            if *attempts > 0 {
                println!("Please try again. {attempts} attempts left.");
                *attempts -= 1;
                log_in(username, pass, attempts);
            } else {
                println!("No attempts left. You are not connected :(")
            }
        }
        TerminalError => println!("Terminal Error"),
        CriticalError => panic!(),
    }
}

pub fn connection_options() -> Option<char> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "l" | "L" => Some('l'),
            "s" | "S" => Some('s'),
            _ => None,
        },
        Err(error) => {
            println!("Terminal error: {}", error);
            None
        }
    }
}

pub fn log_in(username: &mut String, pass: &mut String, attempts: &mut u8) {
    let result = log_in_result(username.clone(), pass.clone());
    match result {
        Ok(_) => println!("Succsessfully logged in!"),
        Err(er) => app_error_management(er, username, pass, attempts),
    }
}

pub fn log_in_result(username: String, pass: String) -> Result<(), AppError> {
    if username.is_empty() || pass.is_empty() {
        Err(NotSigned)
    } else {
        println!("---Log in--- ");
        let mut input_username: String = String::new();
        let mut input_pass: String = String::new();
        println!("Enter username: ");
        match io::stdin().read_line(&mut input_username) {
            Ok(_) => {
                println!("Enter password: ");
                match io::stdin().read_line(&mut input_pass) {
                    Ok(_) => {
                        if username == input_username && pass == input_pass {
                            Ok(())
                        } else {
                            Err(WrongData)
                        }
                    }
                    Err(_) => Err(TerminalError),
                }
            }
            Err(_) => Err(TerminalError),
        }
    }
}

pub fn sign_in(username: &mut String, pass: &mut String, attempts: &mut u8) {
    let result = sign_in_result(username, pass);
    match result {
        Ok(_) => {
            println!("Succsessfully signed in!");
            log_in(username, pass, attempts);
        }
        Err(er) => app_error_management(er, username, pass, attempts),
    }
}

pub fn sign_in_result(username: &mut String, pass: &mut String) -> Result<(), AppError> {
    if !username.is_empty() && !pass.is_empty() {
        Err(AlreadySigned)
    } else {
        println!("---Sign in--- ");
        let mut input_username: String = String::new();
        println!("Enter username: ");
        match io::stdin().read_line(&mut input_username) {
            Ok(_) => {
                let mut option = None;
                while option.is_none() {
                    option = sign_in_password();
                    if option.is_none() {
                        println!("Passwords dont match, try again!");
                    }
                }
                match option {
                    Some(input_pass) => {
                        *username = input_username;
                        *pass = input_pass;
                        Ok(())
                    }
                    None => Err(CriticalError),
                }
            }
            Err(_) => Err(TerminalError),
        }
    }
}

pub fn sign_in_password() -> Option<String> {
    let mut input_pass1: String = String::new();
    let mut input_pass2: String = String::new();
    println!("Enter password: ");
    match io::stdin().read_line(&mut input_pass1) {
        Ok(_) => {
            println!("Confirm password: ");
            match io::stdin().read_line(&mut input_pass2) {
                Ok(_) => {
                    if input_pass1 == input_pass2 {
                        Some(input_pass1)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

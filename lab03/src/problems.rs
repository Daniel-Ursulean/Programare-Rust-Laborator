// P3 Error type
pub enum MathError {
    AddOverflow(u32, u32),
    MullOverflow(u32, u32),
}

pub fn math_error_text(er: MathError) {
    match er {
        MathError::AddOverflow(x, y) => {
            println!("Addition overflow: {} + {}", x, y);
        }
        MathError::MullOverflow(x, y) => {
            println!("Multiplication overflow: {} * {}", x, y);
        }
    }
}

// P4 Error type
pub enum CharError {
    NotAscii(char),
    NotDigit(char),
    NotBase16Digit(char),
    NotLetter(char),
    NotPrintable(),
}

// P1 functions
pub fn is_prime(x: u16) -> bool {
    if x == 0 || x == 1 {
        return false;
    }
    let mut d: u16 = 2;
    while d <= x.isqrt() {
        if x.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

pub fn next_prime(x: u16) -> Option<u16> {
    let mut y = x;
    while y != u16::MAX {
        y += 1;
        if is_prime(y) {
            return Some(y);
        }
    }
    None
}

// P2 functions
pub fn checked_addition(x: u32, y: u32) -> u32 {
    if x > u32::MAX - y {
        panic!("Adition overflow: {x} + {y} > {}", u32::MAX);
    }
    x + y
}

pub fn checked_multiplication(x: u32, y: u32) -> u32 {
    if x == 0 || y == 0 {
        return 0;
    }
    if x > u32::MAX / y {
        panic!("Multiplication overflow: {x} * {y} > {}", u32::MAX);
    }
    x * y
}

// P3 functions
pub fn better_checked_addition(x: u32, y: u32) -> Result<u32, MathError> {
    if x > u32::MAX - y {
        Err(MathError::AddOverflow(x, y))
    } else {
        Ok(x + y)
    }
}

pub fn better_checked_multiplication(x: u32, y: u32) -> Result<u32, MathError> {
    if x == 0 || y == 0 {
        return Ok(0);
    }
    if x > u32::MAX / y {
        Err(MathError::MullOverflow(x, y))
    } else {
        Ok(x * y)
    }
}

pub fn function_tester() {
    let mut x: u32 = 1824;
    let y: u32 = 4526;

    match better_checked_addition(x, y) {
        Ok(sum) => println!("checked_addition 1824 + 4526 = {}", sum),
        Err(er) => math_error_text(er),
    }

    match better_checked_multiplication(x, y) {
        Ok(sum) => println!("checked_multiplication 1824 + 4526 = {}", sum),
        Err(er) => math_error_text(er),
    }

    x = 4294967295;

    match better_checked_addition(x, y) {
        Ok(sum) => println!("checked_addition 1824 + 4526 = {}", sum),
        Err(er) => math_error_text(er),
    }

    match better_checked_multiplication(x, y) {
        Ok(sum) => println!("checked_multiplication 1824 + 4526 = {}", sum),
        Err(er) => math_error_text(er),
    }
}

// P4 functions
pub fn print_error(err: CharError) {
    match err {
        CharError::NotAscii(c) => println!("Error: '{}' is not an ASCII character.", c),
        CharError::NotDigit(c) => println!("Error: '{}' is not a digit.", c),
        CharError::NotBase16Digit(c) => println!("Error: '{}' is not a base16 digit.", c),
        CharError::NotLetter(c) => println!("Error: '{}' is not a letter.", c),
        CharError::NotPrintable() => println!("Error: character is not printable."),
    }
}

pub fn to_uppercase(c: char) -> Result<char, CharError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(CharError::NotLetter(c))
    }
}

pub fn to_lowercase(c: char) -> Result<char, CharError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(CharError::NotLetter(c))
    }
}

pub fn print_char(c: char) -> Result<(), CharError> {
    if (' '..='~').contains(&c) {
        println!("{}", c);
        Ok(())
    } else {
        Err(CharError::NotPrintable())
    }
}

pub fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii(c));
    }

    match c.to_digit(10) {
        Some(number) => Ok(number),
        None => Err(CharError::NotDigit(c)),
    }
}

pub fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii(c));
    }
    match c.to_digit(16) {
        Some(number) => Ok(number),
        None => Err(CharError::NotBase16Digit(c)),
    }
}

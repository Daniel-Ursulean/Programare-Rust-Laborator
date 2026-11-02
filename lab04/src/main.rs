use std::{
    fs::{self},
    io,
};

use crate::MyErrors::{NotAscii, ReadError};

fn read_from_file(path: &'static str) -> Result<String, io::Error> {
    let s = fs::read_to_string(path)?;
    Ok(s)
}

enum MyErrors {
    NotAscii,
    ReadError,
}

fn error_handler(er: MyErrors) {
    match er {
        NotAscii => println!("Text contains not-ascii characters"),
        ReadError => println!("Error at reading the file"),
    }
}

fn p1(path: &'static str) {
    let r = read_from_file(path);
    if let Ok(s) = r {
        let mut max_bytes = 0;
        let mut max_chars = 0;
        for l in s.lines() {
            let bytes = l.len();
            let chars = l.chars().count();
            if max_bytes < bytes {
                max_bytes = bytes;
            }
            if max_chars < chars {
                max_chars = chars;
            }
        }
        println!("Longest lines cosidering the number of bytes ({max_bytes} bytes):");
        for l in s.lines() {
            if l.len() == max_bytes {
                println!("{l}");
            }
        }
        println!("Longest lines cosidering the number of bytes ({max_chars} characters):");
        for l in s.lines() {
            if l.chars().count() == max_chars {
                println!("{l}");
            }
        }
    } else {
        error_handler(ReadError);
    }
}

fn rot13(c: char) -> Result<char, MyErrors> {
    if c.is_ascii() {
        match c {
            'a'..='z' => Ok((((c as u8 - b'a' + 13) % 26) + b'a') as char),
            'A'..='Z' => Ok((((c as u8 - b'A' + 13) % 26) + b'A') as char),
            _ => Ok(c),
        }
    } else {
        Err(NotAscii)
    }
}

fn p2(path: &'static str) {
    let r = read_from_file(path);
    let mut rot13_string: String = String::new();
    if let Ok(s) = r {
        for c in s.chars() {
            match rot13(c) {
                Ok(ch) => {
                    rot13_string.push(ch);
                }
                Err(er) => {
                    error_handler(er);
                    break;
                }
            }
        }
    } else {
        error_handler(ReadError);
    }
    println!("{rot13_string}");
}

fn p3(path: &'static str) {
    let r = read_from_file(path);
    let mut final_string: String = String::new();

    if let Ok(s) = r {
        for word in s.split(' ') {
            match word {
                "pt" | "ptr" => final_string.push_str("pentru"),
                "dl" => final_string.push_str("domnul"),
                "dna" => final_string.push_str("doamna"),
                _ => final_string.push_str(word),
            }
            final_string.push(' ');
        }
    } else {
        error_handler(ReadError);
    }
    println!("{final_string}");
}
fn main() {
    println!("Problema 1");
    p1("f1_in.txt");
    println!("\n Problema 2");
    p2("f2_in.txt");
    println!("\nProblema 3");
    p3("f3_in.txt");
}

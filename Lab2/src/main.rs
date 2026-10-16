//P1
fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    let mut k: i32 = 0;
    while k < n {
        s.push(c);
        k += 1;
    }
    s
}

//P2
fn add_chars_n_nd(s: &mut String, c: char, n: i32) {
    let mut k: i32 = 0;
    while k < n {
        s.push(c);
        k += 1;
    }
}

//P3
//*
fn add_space(s: &mut String, n: i32) {
    let mut k: i32 = 0;
    while k < n {
        s.push(' ');
        k += 1;
    }
}

fn add_str(s: &mut String, to_add: &str) {
    s.push_str(to_add);
}

fn add_integer(s: &mut String, x: i32) {
    if x < 0 {
        s.push('-');
    }
    let mut k: i32 = 0;
    let mut aux: i32 = x;
    let mut ogl: i32 = 0;

    while aux != 0 {
        ogl = ogl * 10 + aux % 10;
        aux /= 10;
        k += 1;
    }
    k -= 1;
    s.push((b'0' + (ogl % 10) as u8) as char);
    ogl /= 10;
    while k > 0 {
        if k % 3 == 0 {
            s.push('_');
        }
        s.push((b'0' + (ogl % 10) as u8) as char);
        ogl /= 10;
        k -= 1;
    }
}
//*/
fn add_float(s: &mut String, x: f32) {
    let mut xi = (x * 1000.0) as i32;
    let mut ogl: i32 = 0;
    let mut k: i32 = 0;

    while xi != 0 {
        ogl = ogl * 10 + xi % 10;
        xi /= 10;
        k += 1;
    }

    while k > 0 {
        if k == 3 {
            s.push('.');
        }
        s.push((b'0' + (ogl % 10) as u8) as char);
        ogl /= 10;
        k -= 1;
    }
}

fn main() {
    // P1
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }
    println!("Problema 1: {}", s);

    //P2
    i = 0;
    let mut s2: String = String::from("");
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_nd(&mut s2, c, 26 - i);
        i += 1;
    }
    println!("Problema 2: {}", s2);

    //P3
    let mut s = String::new();
    //Line 1
    add_space(&mut s, 41);
    add_str(&mut s, "I 💚\n");
    //Line 2
    add_space(&mut s, 41);
    add_str(&mut s, "RUST.\n\n");
    //Line 3
    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 11);
    add_str(&mut s, "crate");
    add_space(&mut s, 7);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");
    //Line 4
    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 9);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 3);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".\n");
    // Line 5
    add_space(&mut s, 20);
    add_str(&mut s, "\n");

    println!("{s}");
}

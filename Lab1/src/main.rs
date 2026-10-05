fn prim(x: u32) -> bool {
    if (x == 1) || (x == 0) {
        return false;
    }
    let mut i = 2;
    let mut l = true;
    while (i <= x / 2) && l {
        if x.is_multiple_of(i) {
            l = false;
        }
        i += 1;
    }
    l
}

fn coprime(mut x: u32, mut y: u32) -> bool {
    while x != y {
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }
    x == 1
}

fn beer_love(mut n: u32) {
    if n < 1 {
        println!("Have more drinks");
        return;
    }
    while n > 1 {
        println!("{} bottles of beer on the wall,", n);
        println!("{} bottles of beer.", n);
        println!("Take one down, pass it around,");
        n -= 1;
        println!("{} bottles of beer on the wall.", n);
        println!();
    }
    print!(
        "1 bottle of beer on the wall,
1 bottle of beer.
Take one down, pass it around,
No bottles of beer on the wall."
    );
}
fn main() {
    //Ex 1
    let mut i = 0;
    while i < 100 {
        if prim(i) {
            println!("Numarul {} este prim", i);
        }
        i += 1;
    }
    //Ex 2
    i = 1;
    while i < 100 {
        let mut j = 1;
        while j < 100 {
            if coprime(i, j) {
                println!("Numerele {} si {} sunt coprime", i, j);
            }
            j += 1;
        }
        i += 1;
    }
    //Ex 3
    beer_love(99);
}

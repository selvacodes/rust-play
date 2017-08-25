use std::fmt;

enum FB {
    Val(u64),
    Fizz(u64),
    Buzz(u64),
    FizzBuzz(u64),
}


impl fmt::Display for FB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            FB::Val(x) => x.to_string(),
            FB::Fizz(_) => String::from("Fizz"),
            FB::Buzz(_) => String::from("Buzz"),
            FB::FizzBuzz(_) => String::from("FizzBuzz"),
        };
        write!(f, "{}", printable)
    }
}

fn main() {
    for x in (1..20).map(fz_bz) {
        println!("{}", x);
    }
}

fn fz_bz(n: u64) -> FB {
    match n {
        n if n % 3 == 0 && n % 5 == 0 => FB::FizzBuzz(n),
        n if n % 3 == 0 => FB::Fizz(n),
        n if n % 5 == 0 => FB::Buzz(n),
        n => FB::Val(n),
    }
}

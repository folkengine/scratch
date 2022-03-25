macro_rules! add_as {
    ( $($a:expr), * ) => {
        {
            0
            $(+$a)*
        }
    }
}

macro_rules! add {
    ($a: expr) => {
        $a
    };
    // ($a: expr, $b: expr) => {
    //     $a + $b
    // };
    // TT Muncher
    ($a: expr, $($b:tt)*) => {
        $a+add!($($b)*)
    }
}

fn main() {
    let r = add!(1, 2);
    println!("r = {}", r);
    println!("r = {}", add!(45));
    println!("r = {}", add_as!(45, 44, 43));
    println!("{}", add_as!(1, 2, 3, 4));
    println!("{}", add!(1, 2, 3, 4));
}

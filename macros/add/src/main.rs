macro_rules! add {
    ($a: expr, $b: expr) => {{
        $a + $b
    }};
    ($a: expr) => {{
        $a
    }};
}

macro_rules! add_as {
    ( $($a:expr), * ) => {
        {
            0
            $(+$a)*
        }
    }
}

fn main() {
    let r = add!(1, 2);
    println!("r = {}", r);
    println!("r = {}", add!(45));
    println!("r = {}", add_as!(45, 44, 43));
}

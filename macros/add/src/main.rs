macro_rules! add {
    ($a: expr, $b: expr) => {
        {
            $a + $b
        }
    };
    ($a: expr) => {
        {
            $a
        }
    }
}

fn main() {
    let r = add!(1, 2);
    println!("r = {}", r);
    println!("r = {}", add!(45));
}

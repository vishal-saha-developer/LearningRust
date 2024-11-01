fn main() {
    let mut x = 5;
    println!("Value of {x}");
    x = 6;
    println!("Value of {x}");

    const VAL: i32 = 5;
    println!("Value of {VAL}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

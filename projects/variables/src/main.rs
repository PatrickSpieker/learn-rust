fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 6;
    let y = y + 1; // bro what 

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    let _guess: u32 = "42".parse().expect("Not a number!");

    let sum: i8 = 5 + 10;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;
    let tmp = tup.1;
    println!("The value of y is: {y}");
    println!("The value of tup.1 is: {tmp}");
    let a = [1,2,3,4,5];
}


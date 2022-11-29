fn main() {
    println!("Hello, world!");

    let y = another_function(10);
    println!("{y}");

    if y < 5 {
        // println!("{y}");
    } else if y < 20 { 
        // println!("10!");
    }
    
    let _num = if y < 20 { 5 } else { 7 };
    // println!("{num}");

    loop {
        println!("yum!");
        break
    }

    let mut number = 5;

    while number != 10 {
        println!("{number}");
        number += 1; 
    }

    let a = [10, 20, 30, 50];

    for element in a {
        println!("{element}");
    }

    {
        let _s = "hello";
    }

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);


    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);

    // having to do this is kind a pain in the butt
    let (s2, len) = string_func(s2);
    println!("{} {}", s2, len);

    let s3 = String::from("hello");
    let len = calc_len(&s3); 
    println!("s3 {} {}", s3, len);

    let mut s4 = String::from("hello");

    let modded_str = change_string(&mut s4);

    // let r1 = &mut s4;
    // can't do
    // let r2 = & s4;
    // or 
    // let r2 = &mut s4
    // println!("{} {}", r1, r2);
    //
    struct User {
        // fields
        active: bool, 
        username: String,
        sign_in_count: u64,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("The area of the rectangle is {} square pixels", rect1.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn another_function(x: i32) -> i32 {
    println!("Another function was called!");
    x + 1
}

fn string_func(some_str: String) -> (String, usize) {
    println!("function that took the string was called!");
    let len = some_str.len();
    (some_str, len)
}

fn calc_len(st: &String) -> usize {
    // st doesn't have ownership over the string it points to
    st.len()
}

fn change_string(st: &mut String) {
    st.push_str(" world!!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


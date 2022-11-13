fn main() {
    // let mut s = String::from("hello");
    // s.push_str(" wordl!");
    // println!("{s}");
    // let mut s1 = String::from("sad");
    // // let r1 = &s1;
    // let r2 = &mut s1;
    // // let r3 = &s1;
    // let r4 = &mut s1;

    let mut s = String::from("hello");

    let r2 = &s;
    println!("{}", r2);
    let r1 = &mut s;
    println!("{}", r1);

    // take_and_return_ownership(&mut s1);
    // println!("{s1}")
}

// fn take_and_return_ownership(s1: &mut String) {
//     s1.push_str("fd");
// }

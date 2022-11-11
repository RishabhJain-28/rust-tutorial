struct User {
    email: String,
    pass: String,
}
struct Color(i32, i32, i32);

fn create_user(email: String, pass: String) -> User {
    let u = User { email, pass };
    return u;
}
fn main() {
    let s1 = String::from("email1");
    let s2 = s1.clone();
    let u1 = create_user(s1, String::from("pass1"));
    let u2 = User { email: s2, ..u1 };
    println!("{}", u2.pass);
    let c = Color(0, 0, 0);
}

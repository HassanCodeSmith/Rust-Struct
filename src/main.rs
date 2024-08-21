#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        sign_in_count: 1,

        // Struct Field Init Shorthand
        username,
        email,
    }
}
fn main() {
    let user1: User = build_user(
        String::from("Hassan Murtaza"),
        String::from("hassan.murtaza@smartfunstudios.com"),
    );
    let user2: User = User {
        email: String::from("hassanmurtaza.dev@gmail.com"),

        // Struct Update Syntax
        ..user1
    };

    println!("Personal Credentials: {:#?}", user2);
}

/*
----------------------------------------------------------------------------
Defining and Instantiating Structs
Using the Field Init Shorthand
Creating Instances from Other Instances with Struct Update Syntax
----------------------------------------------------------------------------
*/

/*
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
*/

/*
----------------------------------------------------------------------------
    Using Tuple Structs Without Named Fields to Create Different Types
----------------------------------------------------------------------------
*/
/*
#[allow(dead_code)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    #[allow(unused_variables)]
    let black: Color = Color(34, 345, 23);
    let origin: Point = Point(23, 45, 54);
    let Point(twenty_three, fourty_five, fifty_four) = origin;

    println!("Sturct Tuple (Destructure):");
    println!("Twenty Three: {twenty_three}");
    println!("Fourty Five: {fourty_five}");
    println!("Fifty Four: {fifty_four}");

    println!("\nSturct Tuple (Dot Notation):");
    println!("Black index 1: {:?}", black.0);
    println!("Black index 2: {:?}", black.1);
    println!("Black index 3: {:?}", black.2);

    let tup = ("Hassan", 25, 43000);
    let (name, age, salary) = tup;

    println!("\nSimple Tuple (Destructuring):");
    println!("Name: {name}");
    println!("Age: {age}");
    println!("Salary: {salary}");

    println!("\nSimple Tuple (Dot Notation):");
    println!("Name: {:?}", tup.0);
    println!("Age: {:?}", tup.1);
    println!("Salary: {:?}", tup.2);
}

*/

/*
----------------------------------------------------------------------------
    Unit-Like Structs Without Any Fields
----------------------------------------------------------------------------
*/
struct AlwaysEqual;

fn main() {
    #[allow(unused_variables)]
    let subject = AlwaysEqual;
}

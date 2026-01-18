
use std::collections::HashMap;

// ================= FUNCTIONS =================

fn hello_world() {
    println!("Hello, world!");
}

fn tell_height(height: u32) {
    println!("My height is {} cm.", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}. I am {} years old, and my height is {} cm.",
        name, age, height
    );
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_length_one_owner(s: String) -> usize {
    s.len()
}

// ================= STRUCTS =================

struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct BankAccount {
    owner: String,
    balance: f64,
}

// ================= IMPLEMENTATION =================

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has balance of {}",
            self.owner, self.balance
        );
    }
}

// ================= USER BUILDER =================

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// ================= ENUM =================

#[derive(Debug)]
enum IpAddressKnd {
    V4,
    V6,
}

fn route(_ip_kind: &IpAddressKnd) {}



// Exception Hnadling

fn divideOption (numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divideResult (numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

// Collection Types:
// Vectors - UTF8 - Hashmaps

// ================= MAIN =================

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer x: {}", x);
    println!("Unsigned integer y: {}", y);

    let pi: f64 = 3.14259;
    println!("value of Pi is {}", pi);

    let is_snowing: bool = true;
    println!("is it snowing? {}", is_snowing);

    let letter: char = 'a';
    println!("first letter of alphabet is {}", letter);

    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Human Tuple: {:?}", human);

    // Slices
    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slice: {:?}", number_slice);

    let animal_slice: &[&str] = &["Dog", "Cat", "Elephant"];
    println!("Animal slice: {:?}", animal_slice);

    let mut stone_code = String::from("Hell, ");
    stone_code.push_str("Yeah!");
    println!("Stone Code: {}", stone_code);

    let string = String::from("Hello, World!");
    let slice = &string[0..5];
    println!("Slice value: {}", slice);

    hello_world();
    tell_height(176);
    human_id("Iche", 23, 176.4);

    let result = add(5, 10);
    println!("Addition result: {}", result);

    // Ownership & Borrowing
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);

    let s2 = String::from("RUST");
    let s3 = s2;
    let len2 = calculate_length_one_owner(s3);
    println!("Length is {}", len2);

    let mut x = 6;
    let y = &mut x;
    *y += 7;
    println!("Value of x is {}", x);

    let mut account = BankAccount {
        owner: "Vincent".to_string(),
        balance: 1000.55,
    };

    account.check_balance();
    account.withdraw(250.75);
    account.check_balance();

    const MAX_POINTS: i32 = 45;
    println!("Max points: {}", MAX_POINTS);

    let age = 20;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a child.");
    }

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner x: {}", x);
    }
    println!("Outer x: {}", x);

    let a = [10, 20, 30, 40, 50];
    for value in a {
        println!("{}", value);
    }

    let user1 = build_user(
        String::from("jazzman@g.com"),
        String::from("john_doe"),
    );

    let user2 = User {
        email: String::from("omasco@g.com"),
        ..user1
    };

    println!("User2 username: {}", user2.username);
    // Tuple Struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(2555, 255, 255);

    // Unit-like Struct
    struct AlwaysEquals;
    let subject = AlwaysEquals;



    let four = IpAddressKnd::V4;
    let six = IpAddressKnd::V6;

    route(&four);
    route(&six);
    println!("IP Address kinds: {:?}, {:?}", four, six);

    // Error Handling 
    // Approach 1
    // enum Option<T>{ // defining the generic Option enum
    //     Some(T), // Represents a value of type T
    //     None, // Represents the absence of a value
    // }

    // Approach 2
    // enum Result<T, E>{ // defining the generic Result enum
    //     Ok(T), // Represents a successful outcome with a value of type T
    //     Err(E), // Represents an error outcome with a value of type E
    // }

    let result1 = divideOption(10.0, 2.0);
    match result1 {
        Some(value) => println!("Result: {}", value),
        None => println!("Error: Cannot Divide by zero"),
    }

    let result2 = divideResult(10.0, 0.0);
    match result2 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let mut _v:Vec<i32> = Vec::new();
    let mut _v:Vec<i32> = vec![1,2,3,4,5];

    _v.push(6);
    _v.push(7);
    println!("Vector: {:?}", _v);

// HASHMAPS

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(&value) => println!("Score for {}: {}", team_name, value),
        None => println!("No score found for {}", team_name),
    }


}

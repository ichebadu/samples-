
fn hello_world() {
    println!("Hello, world!");
}

fn tell_height(height: u32){
    println!("My height is {} cm.", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}. I am {} years olg, and my height is {} cm.", name, age, height);
}

fn add(a: i32, b: i32) -> i32 {
        a + b
}



fn calculate_length(s: &String) -> usize{
    s.len()
}
 
fn calculate_length_one_owner(s: String) -> usize {
    s.len()
}


fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer x:  {}", x);
    println!("Unsigned integer y: {}", y);


    let pi: f64 = 3.14259;
    println!("values Pi is {}", pi);

    let is_snowing: bool = true;
    println!("is. it snowing? {}", is_snowing);

    let letter: char = 'a';
    println!("first letter of alphabet is {}", letter);

    // Arrays 
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers Arrays: {}", numbers[0]);
    println!("Numbers Arrays: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Arrays: {:?}", fruits);

    // Tuples 
    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Human Tuples: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5,6]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    let my_mix_tuples:  (String, i32, bool, [i32; 6]) = ("Kratos".to_string(), 23, true, [1,2,3,4,5,6]);
    println!("My Mix Tuple: {:?}", my_mix_tuples);

    // Slice:
    let number_slice: &[i32] = &[1,2,3,4,5];
    println!("number slice: {:?}", number_slice);

    let animal_slice: &[&str] = &["Dog", "Cat", "Elephant"];
    println!("animal slice: {:?}", animal_slice);

    let bool_slice: &[&String] = &[&"IT".to_string(), &"IS".to_string(), &"RUST".to_string()];
    println!("bool slice: {:?}", bool_slice);

    let mut stone_code: String = String::from("Hell, " );
    stone_code.push_str("Yeah!");
    println!("Stone Code: {}", stone_code);
    
    // B - &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Values: {}", slice);

    hello_world();
    tell_height(176);
    human_id("Iche", 23, 176.4);

    let _X: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };

    println!("Result is  {}", _X);

    let y: i32 = add(5, 10);
    println!("Addition is {}", y);

    // concept if ownership and borrowing
    let s1: String = String::from("RUST");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 2. - there can be only one owner at a time
    let s2: String = String::from("RUST");
    let s3 = s2; // s2 is moved to s3, s2 is no longer valid
    // println!("s2 is {}", s2); // this line will cause an error because s2 is no longer valid
       println!("s3 is {}", s3);
    let len2: usize = calculate_length_one_owner(s3);
    println!("lens is {}", len2);

    // println!("The length of '{}' is {}.", s2, len2); // this line will cause an error because s2 is no longer valid
    //println!("The length is {} is {},", "RUST", len2);

    let mut _x: i32 = 6;
    let _y: &mut i32 =  &mut _x;
    *_y += 7;
    println!("The value of _x is {},", _x);

    // Demonstrating on one mutable reference or many immutable references
    let mut account: BankAccount = BankAccount {
        owner: "Vincent".to_string(),
        balance: 1000.55,
    };

    // Immutable borrow to check balance
    account.check_balance();

    // mutable borrow to withdraw amount
    account.withdraw(250.75);
    account.check_balance();

        // Variables and Immutability
    let _uy = 10;

    println!("The value of a is {}", _uy);
    // _uy = 20;
    // println!("The value of a is {}", _uy);


    // constants
    const MAX_POINTS: i32 =  45;
    println!("The maximum points is {}", MAX_POINTS);

    // control flow in RUST
    let age = 20;
    if age >= 3{
        println!("You are an adult.");
    } else {
        println!("You are a child.");
    }
    
    // shadowing is not th same as marking a variable as mutable
    let x = 5;
    let x = x + 1; // shadowing x
    // x = 1;

    {
        let x =  x * 2;
        println!("the value of x in the inner scope is {x}");
    }
    println!("the value. of x in main function is: {x}");

    let number = 6;
    if number % 4 ==0 {
        println!("number is divisible by 4");
    }else if number % 3 == 0{
        println!("number is divisible by 3");
    }else{
        println!("number is not divisible by 4 or 3");
    }

    // Using if in a let statement
    let condition = false;
    let number  = if condition {5} else {6};
    println!("Number: {}", number);

    // loop in RUST
    loop{
        println!("again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter ==10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer x:  {}", x);
    println!("Unsigned integer y: {}", y);
}

struct BankAccount {
    owner:  String,
    balance: f64,
}


impl BankAccount {
    fn withdraw(&mut self, amount: f64){
        println!("withdrawing {} from account owned by {} ", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has balance of {}", self.owner, self.balance)
    }

}
// S1 goes out of scope and is dropped here: comment this code out to try it out
// fn printLost(s: String) {
//     println!("{}", s1)
// }


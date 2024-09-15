


/* 
the {:?} signifies the below -
The :? indicates that the value should be formatted using the default debug representation. This means that the value will be displayed in a way that is most informative and readable for debugging purposes.
*/

/* 
Statements :- Instuctions that perform some action but don't produce a value, e.g., variable declarations and function declarations, etc.
Expressions :-- Evaluate to a resultant value, e.g., function calls, variables with final return values, etc.
*/


/* 
Struct :-- Its a compound type which groups together values of different datatypes in a <T> named data structure. It is similar to tuples 
          but each value can be accessed by its name as they are stored with their names.
-> Struct has to be instantiated with data, think of it as template for instances / objects that you create from it. 
> We can use 'mut' with a struct instance to alter the values within the names it stores.

-> Partial Move :- Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same time. Doing this will result in a partial move of the variable, which means that parts of the variable will be moved while other parts stay. In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.
*/

use std::fs;

struct UserLogIn {
    active: bool, user_name: String, user_email: String, sign_in_count: u64
}


/*
Unit-like Structs :- These are structs without any field. They don't store any data. Generally used with Traits.
                    (They don't have any fields. It can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.)
*/


/*
Tuple Structs :- It is a named tuple. It is a struct but using tuple-like sytaxes to define the fields.
    > It is instantiated with paranthesis instead of curly braces. 
    > Accessed through point notation.
  (Tuple struct looks similar to tuples, it has added meaning the struct name provides but has no named fields. It's useful when you want to give the whole tuple a name, but don't care about the fields's names.)
*/

struct TupleStructExp(i64, String, [char; 5]);

// #[derive(Debug)]  will help us to make a specific struct printable. It will print as < struct_name(< field1, field2, ...>) >, field == component.
#[derive(Debug)]
struct Point(i32, i32, i32);


/* 
Enum :-- It is a way of defining a type with only one of a possible set of values. Any other values being used will show error,
         as it accepts only the variants initiated here.
> We can only access one variant from an enum at a time.
> It can hold additional variables using tuples. Especially useful when using match statements.
*/

/* 
enum IpAddr {
    // V4(), V6()    // These two are variants of this enum 
    V4(String), V6(String)    
}
*/
/*
 Here the IpAddr (enum for IP addresses) can be of V4 type and V6 type. Here each variant holds a String value. 
 It is written as :==>>     let var = < Enum_type > :: < Variable_in_enum >(< Datatype_value >); 
*/

// enum for try-catch like functionalities 
/*
enum Result<T, E> {
    Ok(T), Err(E)
}
*/

/*
Option Enum :- It is an enum that represents a value that may/may not be present. Used where a case or function fails to return a value.
 > Use of a null or similar keyword to represent the absence of a value, Rust doesn't have null.
 > The Option enum was introduced in Rust to handle the concept of nullability in a safe and expressive way.
*/ 
/*
pub enum Option<T> {
    None, Some(T)
}
*/

// Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}




fn main() {
    func_one();
    println!("Main function -> Hello, world!");
    func_one();

    // add_num(25, 34);
    println!("{}", add_num_fnc(25, 34));

    let num = {
        // num is a statement, and it returns the value of x from an expression within its braces assigned to num variable.
        let x = 6;
        x + 1          // No semi-colon since it returns the value of x..
    };
    println!("{}", num);

    let z = {
        let x = 4;
        2 + x
    };
    println!("z = {:?}", z);


    
    // Accessing and mutating struct objects 
    let mut user1 = UserLogIn {
        active: true, user_name: String::from("Bully Walker"), user_email: String::from("magiretobby.213@gmail.com"), sign_in_count: 1,
    };
    println!("For user1 :-\nActive: {:?}\nUser Name: {:?}\nUser Email: {:?}\nSign In counts: {:?}", 
    user1.active, user1.user_name, user1.user_email, user1.sign_in_count);

    user1.user_name = String::from("Bhai Lander");
    user1.user_email = String::from("bhailander.333@gmail.com");
    println!("New user name and user email :-\nName: {:?}\nEmail: {:?}", user1.user_name, user1.user_email);


    // struct update syntax 
    let user2 = UserLogIn {
        active: true, user_name: user1.user_name, user_email: String::from("cherrymitani@gmail.com"), sign_in_count: 2
    };
    println!("For user2 :-\nActive: {:?}\nUser Name: {:?}\nUser Email: {:?}\nSign In counts: {:?}", 
    user2.active, user2.user_name, user2.user_email, user2.sign_in_count);

    let user3 = build_user(String::from("Selmon boi"), String::from("hiranSlay@2k2.com"), 3);
    println!("For user3 :-\nActive: {:?}\nUser Name: {:?}\nUser Email: {:?}\nSign In counts: {:?}", 
    user3.active, user3.user_name, user3.user_email, user3.sign_in_count);

    let _user4 = UserLogIn {
        active: true,
        user_name: String::from("Heisenberg wltuh"), 
        ..user3
    };        // for using ..struct_object, you need to use that stuct_object before including it in another object. 



    // We can use '.' operator to access the tuple_struct element as done with accessing elements of tuples. 
    let tpl1 = TupleStructExp(356, String::from("Kaisa laga mera mazak.?"), ['t','u','p','l','e']);
    print!("Tuple structs :: \n( {}, {}, [ ", tpl1.0, tpl1.1);
    for c in tpl1.2.iter() {
        print!("{}, ", c);
    }
    println!("] )");

    let vt = Point(0, 127, 255);
    check_point(vt);



    // Enum IpAddr variables
    // let _hm_addr = IpAddr::V4(String::from("127.0.0.1"));
    // let _loopback = IpAddr::V6(String::from("::1"));
    // println!("{:?} --- {:?}", hm_addr, loopback);

    // Create instances of different shapes
    let circle = Shape::Circle(2375.4405);
    let square = Shape::Square(5394.0103);
    let rectangle = Shape::Rectangle(653.0902, 135.068);

    println!("Area of the Circle : {:?}", calculate_area(circle));
    println!("Area of the Square : {:?}", calculate_area(square));
    println!("Area of the Rectangle : {:?}", calculate_area(rectangle));


    // Usage of Result enum. If there is a true value, it will got with Ok otherwise with the Err. 
    let res = fs::read_to_string("sample1.txt");

    match res {
        Ok(content) => {
            println!("Content of the file : {}", content);
        },
        Err(e) => {
            println!("An Error occured : {}", e);
        },
    }


    let my_string = String::from("raman");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}



// fn <function_name>(<param>: <datatype>, .... , <param_n>: <datatype_n>) -> <returning_datatype> {}

// Snake-case for naming of functions
fn func_one() {
    println!("Function 1 is called ....");
} 

// fn add_num_fnc(x: i32, y: i32) -> i32 {
//     // println!("{} + {} = {}", x, y, x+y);
//     // return x + y;       // Both type of return works well
//     x + y
// } 

fn add_num_fnc(x: i32, y: i32) -> i32 {
    let rs = x + y;
    if rs > 50_i32 {
        return rs - 25_i32;
    }
    rs
} 

// Creating struct objects with functions 
fn build_user(name: String, email: String, sign_in_c: u64) -> UserLogIn {
    UserLogIn {
        active: true, 
        user_name: name, 
        user_email: email,
        sign_in_count: sign_in_c
    }
}

fn check_point(p: Point) {
    let Point(_, _, _) = p;    // Here the '_' are just for reference purpose 
    assert_eq!(p.0, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);

    dbg!(&p);  // -----> Print debug info to stderr  
    println!("{:?}", p);
    // println!("( {}, {}, {} )", p.0, p.1, p.2);
}

// Function to calculate area based on the shape for Shape enum
fn calculate_area(shape: Shape) -> f64 {
    let ansr = match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side* side,
        Shape::Rectangle(length, breadth) => length * breadth,
    };
    ansr
}

// If you ever have a function that should return null, return an Option instead. 
fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
    // Err("The character 'a' is not present here..".to_string())
}


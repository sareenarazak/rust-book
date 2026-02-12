fn main() {
    // variables are immutable unless 'mut'
    let x = 5; // immutable 
    println!("The value of x is: {x}");

    // error
    //x = 6;

    // shadowing -- using 'let' to reuse variable name, type can be diff too
    let x = "this is x as string";
    println!("The value of x is: {x}");

    // mutable variables
    let mut y = 9;
    println!("the value of mutable variable y is : {y}");

    y = 1000;
    println!("the value of mutable variable y is : {y}");

    // no shadowing for mut 
    let mut y = "s";
    println!("the value of mutable variable y is : {y}");

    // error since type of a mut cannot be changed, only value 
    // y = 5 
    y = "5";
    println!("the value of mutable variable y is : {y}");

    // constants are bound to a name and are not allowed to change
    // cannot be made mutable 
    // unlike `let`, const needs type annotated 
    const FIVE_MIN_IN_SECONDS : u16 = 5 * 60;
    println!("five min in seconds {FIVE_MIN_IN_SECONDS}");

    // data types
    // integer signed , unsigned --> u8, u16 ,u32, u64, u128, usize, i8, i16 ,i32, i64, i128, isize
    // floating point --> f32, f64(default)
    // boolean true, false --> bool
    // character in ' ' --> char
    
    // tuple -- collection of diff types of variables 
    let tup: (i32, f64, u8) = (500, 6.4, 1);
   
    println!("second element of tup is : {0}",tup.1);
    // destructuring tuple 
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // list ==> let name : [type; size] = [values];
    let a_list: [i32; 5] = [1, 2, 3, 4, 5];
    let another_list = [1, 2, 3];

    println!("second element of a_list: {0}",a_list[1]);


    // arithmetic operations 
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("function 1 {0}", a_function(4, 38));
    println!("function 2{0}", returns_input(42));

    // Control flow 

    let number = 10;
    if number < 15 {
        println!("if condition met for number < 15 ");
    }
    else if number > 20 {
        println!("eh");
    } else {
        println!("blah");
    }

    // keyword loop is an infinite loop, use break to exit 
    let mut count = 0;

    let result = loop {
        if count < 4 {
            count += 1;
            println!("count is {count}");
        } else {
          // return a value by putting it after break stateme t
          break count * 2 ;  
    
        }
    };

    println!("result is {result}");
    
    // while loop same as java
    while count < 6 {
        println!("count is now  {count}");
        count += 1;
    }

    let inji_loves = ["beef_liver", "squeaky_toys", "raw_carrot", "water", "sareena", "tennis_balls", "chasing_squirrels"];

    for inj_lo in inji_loves {
        println!("inji loves {inj_lo}");
    }

}
// Functions
    
fn a_function(input1: u32, input2: u32) -> u32 {
   
    // no return statement needed / skip ;
    input1 + input2
}

fn returns_input(input: u32) -> u32 {
    return input;
}

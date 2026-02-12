fn main() {
    // Ownership rules
    // 1. each value has an owner
    // 2. there can only be one owner at a time 
    // 3. the owner goes out of scope, the value is dropped

    // scope can be a function curly braces {} 
 
    {
        let s = "hello";
        println!("s is {s}");
    }
    // error for line below coz s is out of scope 
    // println!("s is {s}");

    let s = "world";
    let s2 = s;
    

    // string literal is fine to be referenced after reassign
    println!("s is {s}");

    
    let s3 = String::from("world");
    let s4 = s3; // ownership is moved to s4 

    // error because s3 is moved on line let s4 = s3
    //println!("s3 is {s3}");
    println!("S4 IS {s4}"); 

    // integers, booleans etc ( fixed size) are allocated on stack
    // Objects (including) strings are alocated in heap, then the reference is what is stored in
    // stack 
    
    //Rust will never automatically create “deep” copies of your data.
    // clone can be used to deep copy
    let s5 = s4.clone();

    println!("s4 IS {s4} , s5 is {s5}"); 

    // stack only data does not need to have clone called for copying fully
    // Copy trait is implemented for these types and any other type that needs the data copied on
    // to stack rather than heap.
    //
    // can't annotate Copy trait if Drop trait is implemented for the type/ part of the type 
    // integer, boolean, char etc has Copy trait implemented 

    // NOTE: function ownerships work similarly to variables --> ownership is moved to function
    // arugument or return assigned value 
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...

    // error 
    // println!("s is {s}");
    let x = 5;                      // x comes into scope

    makes_copy(x);                 
    
    println!("no error for x since it is integer x is : {x}");

    let s = String::from("meh");
    let s = returns_ownership(s);
    
    println!("s is returned - value is  : {s}");

    // ### REFERENCES AND BORROWING 
    // references are pointers, use &s to reference, *s to deref
    // but if we want to change the object/string a mutable reference need to be passed 
    let mut s1 = String::from("inji is a good boy");
    mutable_string_ref(&mut s1);
    println!("changed string is : {s1}");
   
    // using reference means ownership is not moved, only the reference ownership is moved
    // only one mutable reference allowed
    // multiple immutable references allowed 
    let mut  s1 = String::from("is inji a good boy?");
    let r1 = &mut s1;
    let r2 = &mut s1;
    
    //println!("error : {r1}");

    println!("fine : {r2}");
   
    // cannot have multiple mutable references
    // we can have multiple non mutable references
    let r3 = &s1;
    let r4 = &s1;
    println!("fine - r3: {r3} r4 : {r4}");

   // Slices => copy of part of the object( applicable to string and list )
    let name = &s1[3..8];
    println!("doggo name is : {name}");
}

fn mutable_string_ref(input: &mut String) {
    input.push_str(" adding this part");
}
fn returns_ownership(input: String) -> String {
    input
}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
}

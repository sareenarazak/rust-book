fn main() {
    // structs -- simialr to classes in java 
    // tuples with named members
    let treat = Treat {
        name: String::from("beef liver"),
        rank: 0,
    };

    let inji = Dog {
        name: String::from("inji"),
        age: 8,
        fav_treat: treat,
        is_a_good_doggo: true,
    };

    println!("doggo is {inji:?}");

    let another_inji = Dog {
        name : String::from("inji-imposter"),
        ..inji
    };

    // ownership rules apply
    //println!("this is gonna error :{inji:?}");

    println!("this is gonna error :{another_inji:?}");

    // if all members of struct has Copy trait , then the ownershsip will not be moved

    let new_treat = create_treat(String::from("carrot"), 1);
    println!("new treat is {new_treat:?}");

    // Tuple structs
    struct Point(i32, i32, i32, String);

    let point1  = Point(1,2,3, String::from("123"));

    println!("{0} says", another_inji.name);
    another_inji.speak();

}

fn create_treat(name: String, rank: u32) -> Treat {
    // short hand instead of repeating the member names 
    Treat {
        name,
        rank,
    }
}


// use derive to use trait Debug, which lets us print the object
#[derive(Debug)]

struct Dog {
    name: String,
    age: u16,
    fav_treat: Treat,
    is_a_good_doggo: bool,
}

// methods on Dog struct 
impl Dog {

    fn speak(&self) {
        println!("woof woof");
    }

    fn is_he_good(&self) -> bool {
        self.is_a_good_doggo
    }

}

#[derive(Debug)]
struct Treat {
    name: String,
    rank: u32,
}

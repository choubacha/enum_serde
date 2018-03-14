extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::{self, Read};

#[derive(Deserialize, Debug)]
enum Detail {
    Foo = 0,
    Bar = 1,
    Boo = 2,
    Baa = 3,
}

#[derive(Deserialize, Debug)]
struct Test {
    detail: Detail
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading stdin");
    println!("You said:   {}", buffer);
    let t: Test = serde_json::from_str(&buffer).expect("Bad json format");
    // Does not work as expected. Requires the `string` representation
    println!("You choose: {:?}", t.detail);
}

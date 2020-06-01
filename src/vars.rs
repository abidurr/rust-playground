// Variables: hold primitave data or references to data, are immutable by default, are block-scoped

pub fn run() {
    let mut name = "Abidur";
    println!("My name is {}", name);
    name = "Abi";
    println!("But some call me {}", name);

    // Using const involves types
    const ID: i32 = 011;
    println!("ID: {}", ID);

    // Assigning multiple variables
    let (my_name, fav_num) = ("Abi", 11);
    println!("{}'s fav num is {}", my_name, fav_num)
}

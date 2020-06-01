pub fn run() {
    // Print to console
    println!("Hello from print.rs!");

    // Basic formatting
    println!("{}'s favorite number is {}", "Abi", 11);

    //Positional and named arguments
    println!(
        "{name} is learning {0} just because {name} thought {0} seemed cool",
        "Rust", name = "Abi"
    );


}

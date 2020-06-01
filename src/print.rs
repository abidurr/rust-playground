pub fn run() {
    // Print to console
    println!("Hello from print.rs!");

    // Positional and named arguments
    println!(
        "{name} is learning {0} just because {name} thought {0} seemed cool",
        "Rust",
        name = "Abi"
    );

    // Basic formatting
    println!("{}'s favorite number is {}", "Abi", 11);

    // Placeholder traits
    println!("Binary: {:b} \nHex: {:x} \nOctal: {:o}\n", 11, 11, 11);

    // Placeholder for debugger trait - tuple
    println!("{:?}", (11, true, "hi"));

    // Basic Math
    println!("5 + 6 = {}", 5 + 6);

// Instructions: cargo run
}

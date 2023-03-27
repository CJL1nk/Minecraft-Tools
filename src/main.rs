use std::io::{stdin,stdout,Write};
use rand::Rng;

fn to_stack() {

    let mut input = String::new();

    print!("\nEnter the amount of blocks you have: ");

    let _ = stdout().flush();
    stdin().read_line(&mut input)
        .expect("Did not enter valid stuff.");
    let blocks: i64 = input.trim().parse()
        .expect("Please enter a number");

    let stacks = blocks / 64;
    let leftover_blocks = blocks % 64;

    println!("{stacks} Stacks and {leftover_blocks} Blocks");
}

fn generate_seed() {

    let seed: i128 = rand::thread_rng().gen_range(-9223372036854775808..9223372036854775808);

    println!("\n Lucky Seed: {seed}");
}

fn main() {

    println!("Enter an option below:");

    loop {

        let mut input = String::new();

        println!("\n\n[1] Blocks to Stacks Converter");
        println!("[2] Lucky Seed Generator");
        println!("[3] Nothing yet lol");

        print!("\n>> ");

        let _ = stdout().flush();
        stdin().read_line(&mut input)
            .expect("Did not enter a string.");
        let menu_choice = input.trim().parse()
            .expect("Please enter a number");

        match menu_choice {

            1 => to_stack(),
            2 => generate_seed(),
            _ => println!("Invalid Choice")
        }
    }


}

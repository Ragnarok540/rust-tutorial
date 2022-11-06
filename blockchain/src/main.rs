use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");

    io::stdout().flush().expect("could not flush");
    io::stdin().read_line(&mut miner_addr).expect("could not read");
    print!("Difficulty: ");
    io::stdout().flush().expect("could not flush");
    io::stdin().read_line(&mut difficulty).expect("could not read");
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("we need an integer");
    println!("generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().expect("could not flush");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("could not read");
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address:");
                io::stdout().flush().expect("could not flush");
                io::stdin().read_line(&mut sender).expect("could not read");
                print!("enter receiver address: ");
                io::stdout().flush().expect("could not flush");
                io::stdin().read_line(&mut receiver).expect("could not read");
                print!("Enter amount: ");
                io::stdout().flush().expect("could not flush");
                io::stdin().read_line(&mut amount).expect("could not read");

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("enter new difficulty: ");
                io::stdout().flush().expect("could not flush");
                io::stdin().read_line(&mut new_diff).expect("could not read");
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty"),
                    false => println!("Failed Update Difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().expect("could not flush");
                io::stdin().read_line(&mut new_reward).expect("could not read");
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed Update reward"),
                }
            }
            _ => println!("Invalid option please retry"),
        }
    }
}

use std::io;

fn main() {
    println!("Welcome to RuneScape counter: Please select a skill to track");

    let mut skill_choice: String = String::new();

    // std::io::stdin().read_line(&mut skill_choice);
    // correct because we handle the Result with expect
    io::stdin().read_line(&mut skill_choice).expect("Failed");
    let skill_choice = skill_choice.trim();
    // skill_choice = String::from(skill_choice.trim());

    println!("{skill_choice}")
}

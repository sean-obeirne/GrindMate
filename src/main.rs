use std::io;

#[derive(Debug)]
enum ActionType {
    Item,
    Inventory,
}
impl Default for ActionType {
    fn default() -> Self {
        ActionType::Item
    }
}

#[derive(Default, Debug)]
struct Method {
    name: String,
    skill: String,
    actions_left: i32,
    actions_done: i32,
    total_actions: i32,
    method: ActionType,   
}

const SKILLS: [&str; 23] = [
    "Attack",
    "Hitpoints",
    "Mining",
    "Strength",
    "Agility",
    "Smithing",
    "Defence",
    "Herblore",
    "Fishing",
    "Ranged",
    "Thieving",
    "Cooking",
    "Prayer",
    "Crafting",
    "Firemaking",
    "Magic",
    "Fletching",
    "Woodcutting",
    "Runecrafting",
    "Slayer",
    "Farming",
    "Construction",
    "Hunter",
    ];

fn run_skill_selection() {
    for (i, skill) in SKILLS.iter().enumerate() {
        let num_str = format!("{}: ", i + 1);
        print!("{:4}{:14}", num_str, skill);
        if i % 3 == 2 {
            print!("\n");
        }
    }
    print!("\n");
}


fn main() {
    
    run_skill_selection();

    println!("Welcome to RuneScape counter: Please select a skill to track");

    let mut skill_choice: String = String::new();
    let mut active_methods: Vec<Method> = Vec::new();

    // correct because we handle the Result with expect
    io::stdin().read_line(&mut skill_choice).expect("Failed");
    let skill_choice = skill_choice.trim();
    if SKILLS.contains(&skill_choice) {
        active_methods.push(Method {
            name: String::from(skill_choice),
            ..Default::default()
        });
    }
    println!("{:?}", active_methods)
}

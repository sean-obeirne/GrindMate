use crossterm::{
    cursor, execute, event::{self, read, Event, KeyCode}, 
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType}};


use std::{collections::HashSet, fmt::Display, fs, io::{self, Write}};
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use serde_json::Result;


enum AppMode {
    Menu,
    ViewMethods,
    AddMethod,
    RemoveMethod,
    MethodTracking,
    Quitting,
}

struct AppState {
    mode: AppMode,
    methods: Vec<Method>,
}

impl AppState {
    fn new() -> Self {
        Self {
            mode: AppMode::Menu,
            methods: Vec::new(),
        }
    }

    fn print_menu(&mut self) {
        println!("Welcome to GrindMate! What would you like to do?");
        println!("  [p]rint methods");
        println!("  [a]dd method");
        println!("  [d]elete method");
        println!("  [t]rack progress");
        println!("  [h]elp");
        println!("  [q]uit");

        let input = get_char(HashSet::from(CMDS)).unwrap();

        self.mode = match input {
            'p' => AppMode::ViewMethods,
            'a' => AppMode::AddMethod,
            'd' => AppMode::RemoveMethod,
            't' => AppMode::MethodTracking,
            'h' => AppMode::Menu,
            'q' => AppMode::Quitting,
            _ => std::process::exit(0),
        }
    }
}


#[derive(Default, Debug, Serialize, Deserialize)]
struct Method {
    name: String,
    skill: String,
    actions_left: i32,
    actions_done: i32,
    total_actions: i32,
    // method: ActionType,
}

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


impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  Method - {}:\n    Skill - {},\n    Action Details: {} left, {} done, {} total", self.name, self.skill, self.actions_left, self.actions_done, self.total_actions)
    }
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

const CMDS: [char; 6] = [
    'p',
    'a',
    'd',
    't',
    'h',
    'q',
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

fn reset_cursor() -> io::Result<()> {
// Move the cursor to the start of the line and clear it
    execute!(io::stdout(), cursor::MoveToColumn(0), Clear(ClearType::CurrentLine))?;
    io::stdout().flush()?;
    Ok(())
}

fn get_char(valid_chars: HashSet<char>) -> io::Result<char> {
    enable_raw_mode()?;
    let c = loop {
        let event = event::read()?;
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc => {
                    println!("Exiting...");
                    disable_raw_mode()?;
                    std::process::exit(0)
                }
                KeyCode::Char(c) => {
                    // println!("You chose {}", c);
                    if valid_chars.contains(&c) {
                        break c
                    }
                }
                _ => {continue}
            }
        }
    };
    reset_cursor()?;
    disable_raw_mode()?;
    Ok(c)
}


fn main() -> io::Result<()> {
    let mut state = AppState::new();

    let data_dir = ProjectDirs::from("", "", "GrindMate").unwrap();
    let data_dir = data_dir.data_dir();
    fs::create_dir_all(data_dir)?;
    let data_file = data_dir.join("methods.json");
    // let data_file = File::create(data_dir.join("methods.json"))?;


    let data_file_content = fs::read_to_string(data_file)?;
    let methods: Vec<Method> = serde_json::from_str(&data_file_content)?;

    // println!("json: {}", data_file_content);
    println!("{}", methods[0]);
    println!("{}", methods[1]);


    loop {
        match state.mode {
            AppMode::ViewMethods => {println!("viewing")}
            AppMode::AddMethod => {println!("adding method")}
            AppMode::RemoveMethod => {println!("adding method")}
            AppMode::MethodTracking => {println!("adding method")}
            AppMode::Menu => {state.print_menu()}
            AppMode::Quitting => {std::process::exit(0)}
        }
    }

    // println!("[1] - Favorite 1 - {}", fav1.skill);
    println!("[2] - Favorite 2 - {}", "hello?");
    println!("[3] - Favorite 3 - {}", "hello?");

    println!();


    run_skill_selection();

    let num = get_char(HashSet::from(CMDS))?;
    let skill_choice: String = String::from(num);

    let mut active_methods: Vec<Method> = Vec::new();

    // correct because we handle the Result with expect
    let skill_choice = skill_choice.trim().parse::<usize>();
    if let Ok(index) = skill_choice {
        active_methods.push(Method {
            skill: SKILLS[index - 1].to_string(),
            ..Default::default()
        });
    }
    println!("{:?}", active_methods);
    Ok(())
}

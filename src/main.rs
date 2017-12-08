use std::io::{stdin,stdout,Write};

fn main() {
    println!(
        "
It's dark. You find yourself soaked, and laying in soiled dirt. The air around you is musty, and 
something is obscuring the view of the stars above. It's cold, and there's a drip echoing somewhere
in the distance. It's a cave. The cold draft of damp air is brushing your back, finding its way 
under your wet coat, to chill you to the bone.

There's a sound; a small fluttering, and a faint glow moving in the dark, allowing minute angles
of the cavernous walls to reflect the water running down, sporadically and seemingly randomly.
"
    );

    let mut s = String::new();
    print!("What would you like to do? ");
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("You seem muddled and confused.");

    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }

    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    println!("
Sleep is a good plan. Now is not the time for action.
");
/*
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed)
    };*/
}
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

    let sleep_command: &str = "sleep";
    let mut s = String::new();
    print!("What would you like to do? ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut s)
        .expect("You speak too softly."); 

    match &s {
        sleep_command => { 
            println!("
Sleep is a good plan. Now is not the time for action.
"
            ); 
        }
    }
}
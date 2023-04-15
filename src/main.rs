use rsnote::{cli, commands};
use clearscreen::clear;

fn main() {
    loop {
        let input = cli::prompt("/");
        
        if &input == "l" { 
            let table = commands::list();
            println!("{}", table);
        } else if &input == "n" {
            commands::create();
        } else if &input == "u" {
            commands::update();
        } else if &input == "f" {
            commands::filter();
        } else if &input == "cl" {
            clear().expect("Why I can't clear the SCREENNNNNNNNN!!111!");
        }
        else if &input == "" { continue }
        else if &input == "q" { break }
        else {
            println!("Command \"{}\" not found", input);
            continue
        }
    }

}

use crate::{Todo, cli, fileio, update};
use tabled::{Table, Style};
use std::{fs, num::IntErrorKind};

pub fn list() -> String {
    let todos = Todo::from_file();
    
    Todo::to_table(todos)
}

pub fn create() {
    println!("    \":q\" to quit");

    let mut content = String::new();

    let raw = fileio::read_or_create_file();

    let mut new_todo = Todo::new(raw.len(), &content);
    
    while new_todo.content.is_empty() {
        content = cli::prompt("    c");
        new_todo.content = content.clone();

        if &content == ":q" {
            return
        }

        new_todo.content = new_todo.content.as_str().replace("|", "");
    }

    let fmt = format!("{}\n{}",
                      fileio::read_file().unwrap(),
                      new_todo.encode());

    fileio::write_file(&fmt).unwrap_or_else(|e| {
        println!("{}", e);
    });

    let table = list();

    println!("{}", table);
}

pub fn filter() {
    let mut keyword = String::new();

    while keyword.is_empty() {
        keyword = cli::prompt("    f");

        if &keyword == ":q" {
            return
        }
    }
    
    let result = Todo::find(&keyword);

    let table = Table::new(result)
        .with(Style::blank())
        .to_string();

    println!("{}", table);
}

pub fn find() -> Option<Todo> {
    let todos = Todo::from_file();

    loop {
        let keyword = cli::prompt("    #");

        if &keyword == ":q" {
            break None
        } else if keyword.is_empty() {
            continue
        }

        let keyword = keyword.parse::<usize>();

        match keyword {
            Ok(_) => (),
            Err(e) => match e.clone().kind() {
                IntErrorKind::InvalidDigit => {
                    println!("The input must be a positive number");
                    continue
                },
                e => panic!("{:?}", e),
            }
        }

        let i = todos.iter()
            .position(|todo| todo.id == keyword.clone().unwrap());

        match &i {
            Some(_) => (),
            None => {
                println!("Cannot find this todo");
                continue
            }
        };
        let todo = todos.get(i.unwrap()).unwrap().clone();
        let table = Todo::to_table(vec![todo]);

        println!("{}", table);

        break todos.get(i.unwrap()).clone().cloned()
    }
}


pub fn update() {
    let help = fs::read_to_string("helps/update.txt")
        .unwrap_or(String::from("I cannot help you"));

    let result = find();

    match result {
        Some(_) => (),
        None => return,
    }

    let result = result.unwrap();

    println!("\n{}", help);

    loop {
        let input = cli::prompt("    u");
        let mut todos: Vec<Todo> = Todo::from_file();
        let mut out = String::new();

        if &input == "t" {
            update::tick(result, &mut todos);
        } else if &input == "r" {
            update::rename(result, &mut todos);
        } else if &input == "d" {
            update::delete(result, &mut todos);
        } else if input.is_empty() {
            break
        } else {
            println!("Command \"{}\" not found", input);
            continue
        }

        for (i, t) in todos.iter().enumerate() {
            let fmt = if i != todos.len() - 1 {
                format!("{}\n", Todo::encode(t))
            } else {
                format!("{}", Todo::encode(t))
            };
            
            out.push_str(&fmt);
        }

        break fileio::write_file(&out).unwrap_or_else(|e| panic!("{}", e));
    }
}


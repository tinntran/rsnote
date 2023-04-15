use crate::{cli, Todo};

pub fn tick(result: Todo, todos: &mut Vec<Todo>) {
    for t in todos.into_iter() {
        t.is_done = if t.id == result.id {
            true
        } else {
            t.is_done
        };

    }

    let table = Todo::to_table(todos.clone());

    println!("{}", table);
}

pub fn rename(result: Todo, todos: &mut Vec<Todo>) {
    loop {
        let input = cli::prompt("    r");

        for t in todos.into_iter() {
            t.content = if t.id == result.id {
                input.clone()
            } else {
                t.content.clone()
            };

        }
        break
    }
}

pub fn delete(result: Todo, todos: &mut Vec<Todo>) {
    for (i, t) in todos.clone().iter().enumerate() {
        if t.id == result.id {
            todos.remove(i);
        }
    }
}
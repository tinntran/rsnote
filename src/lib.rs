use tabled::{Tabled, Table, Style};

pub mod commands;
pub mod cli;
pub mod update;
pub mod fileio;

pub const DATAPATH: &str = "data.txt";

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub content: String,
    pub is_done: bool,
}


#[derive(Tabled)]
pub struct StringifyTodo {
    #[tabled(rename = "No.")]
    pub id: usize,

    #[tabled(rename = "Content")]
    pub content: String,

    #[tabled(rename = "")]
    pub is_done: String,
}


impl Todo {
    fn new(id: usize, content: &str) -> Todo {
        Todo { id, content: content.to_string(), is_done: false, }
    }

    pub fn from_file() -> Vec<Todo> {
        let l = fileio::read_or_create_file();
        let mut todos: Vec<Todo> = Vec::new();

        for todo in l {
            let split = todo.split('|').collect::<Vec<_>>();

            let id = split[0].parse::<usize>().unwrap();
            let content = split[1].to_string();
            let is_done = split[2].to_string();

            todos.push(Todo::decode(id, content, is_done));
        }

        todos
    }

    pub fn decode(id: usize, content: String, is_done: String) -> Todo {
        Todo {
            id,
            content,
            is_done: if is_done == "V" {
                true
            } else {
                false
            },
        }
    }

    pub fn encode(&self) -> String {
        let content = self.content.as_str().replace("|", "");
        let is_done = if self.is_done {
            "V"
        } else {
            "X"
        };

        format!("{}|{}|{}", self.id, content, is_done)
    }

    pub fn to_table(todos: Vec<Todo>) -> String {
        let mut stringify_todos = Vec::<StringifyTodo>::new();

        for todo in &todos {
            let is_done = if todo.is_done {
                String::from("[✔]")
            } else {
                String::from("[ ]")
            };

            stringify_todos.push(StringifyTodo {
                id: todo.id,
                content: todo.content.to_string(),
                is_done,
            })
        }

        Table::new(stringify_todos)
            .with(Style::blank())
            .to_string()
    }

    pub fn find(keyword: &str) -> Vec<StringifyTodo> { 
        let todos = Todo::from_file();

        let result = todos
            .iter()
            .filter(|&todo| todo.content.contains(keyword))
            .collect::<Vec<_>>();

        let mut stringify_todos = Vec::<StringifyTodo>::new();

        for todo in &result {
            let is_done = if todo.is_done {
                String::from("[✔]")
            } else {
                String::from("[ ]")
            };

            stringify_todos.push(StringifyTodo {
                id: todo.id,
                content: todo.content.to_string(),
                is_done,
            })
        }

        stringify_todos
    }
}


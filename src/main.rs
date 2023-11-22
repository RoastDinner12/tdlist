fn main() {
    let mut todo_list = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        match command {
            "quit" => break,
            "list" => list_tasks(&todo_list),
            "add" => add_task(&mut todo_list),
            "remove" => remove_task(&mut todo_list),
            "help" => println!("quit makes you exit \n list is pretty obvious \n add lets you add a task e.g add pee \n and remove delets a todo e.g remove pee"),
            _ => println!("Unknown command: {}  please run help", command),
        }
    }
}
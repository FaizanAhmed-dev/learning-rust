use std::io;

struct Task {
    id: usize,
    description: String,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description: description.clone(), // Clone the description
        };
        self.next_id += 1;
        self.tasks.push(task);
        println!("Task added: '{}'", description);
    }

    fn update_task(&mut self, task_id: usize, new_description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.description = new_description.clone(); // Clone the new_description
            println!("Task updated: '{}'", new_description);
        } else {
            println!("Task with id {} not found", task_id);
        }
    }

    fn remove_task(&mut self, task_id: usize) {
        if let Some(index) = self.tasks.iter().position(|t| t.id == task_id) {
            let removed_task = self.tasks.remove(index);
            println!("Task removed: '{}'", removed_task.description);
        } else {
            println!("Task with id {} not found", task_id);
        }
    }

    fn print_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            println!("Tasks:");
            for task in &self.tasks {
                println!("{}: {}", task.id, task.description);
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("\nChoose an option:");
        println!("1. Add Task");
        println!("2. Update Task");
        println!("3. Remove Task");
        println!("4. Show Tasks");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(1) => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                todo_list.add_task(description.trim().to_string());
            }
            Ok(2) => {
                println!("Enter task ID to update:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).expect("Failed to read input");

                println!("Enter new task description:");
                let mut new_description = String::new();
                io::stdin().read_line(&mut new_description).expect("Failed to read input");

                if let Ok(task_id) = task_id.trim().parse() {
                    todo_list.update_task(task_id, new_description.trim().to_string());
                } else {
                    println!("Invalid task ID");
                }
            }
            Ok(3) => {
                println!("Enter task ID to remove:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).expect("Failed to read input");

                if let Ok(task_id) = task_id.trim().parse() {
                    todo_list.remove_task(task_id);
                } else {
                    println!("Invalid task ID");
                }
            }
            Ok(4) => todo_list.print_tasks(),
            Ok(5) => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}

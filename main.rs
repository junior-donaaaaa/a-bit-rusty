use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;

#[derive(Debug, Clone)]
enum TaskStatus {
    Pending,
    Completed,
    Deleted,
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Pending,
        }
    }

    fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
    }

    fn mark_deleted(&mut self) {
        self.status = TaskStatus::Deleted;
    }
}

struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
    filename: String,
}

impl TaskManager {
    fn new(filename: String) -> Self {
        let mut manager = TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
            filename,
        };
        manager.load_from_file();
        manager
    }

    fn add_task(&mut self, title: String, description: String) {
        let task = Task::new(self.next_id, title, description);
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
        self.save_to_file();
    }

    fn list_tasks(&self) {
        for task in self.tasks.values() {
            match task.status {
                TaskStatus::Pending => println!("ID: {}, Title: {}, Status: Pending", task.id, task.title),
                TaskStatus::Completed => println!("ID: {}, Title: {}, Status: Completed", task.id, task.title),
                TaskStatus::Deleted => println!("ID: {}, Title: {}, Status: Deleted", task.id, task.title),
            }
        }
    }

    fn mark_completed(&mut self, task_id: u32) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.mark_completed();
            self.save_to_file();
        } else {
            println!("Task with ID {} not found.", task_id);
        }
    }

    fn mark_deleted(&mut self, task_id: u32) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.mark_deleted();
            self.save_to_file();
        } else {
            println!("Task with ID {} not found.", task_id);
        }
    }

    fn delete_task(&mut self, task_id: u32) {
        if self.tasks.remove(&task_id).is_some() {
            self.save_to_file();
        } else {
            println!("Task with ID {} not found.", task_id);
        }
    }

    fn load_from_file(&mut self) {
        if Path::new(&self.filename).exists() {
            let mut file = match OpenOptions::new().read(true).open(&self.filename) {
                Ok(file) => file,
                Err(_) => return,
            };

            let mut contents = String::new();
            if let Err(_) = file.read_to_string(&mut contents) {
                return;
            }

            let lines = contents.lines();
            for line in lines {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 4 {
                    if let Ok(id) = parts[0].parse::<u32>() {
                        let title = parts[1].to_string();
                        let description = parts[2].to_string();
                        let status = match parts[3] {
                            "Pending" => TaskStatus::Pending,
                            "Completed" => TaskStatus::Completed,
                            "Deleted" => TaskStatus::Deleted,
                            _ => continue,
                        };
                        let task = Task {
                            id,
                            title,
                            description,
                            status,
                        };
                        self.tasks.insert(id, task);
                        self.next_id = self.next_id.max(id + 1);
                    }
                }
            }
        }
    }

    fn save_to_file(&self) {
        let mut file = match OpenOptions::new().write(true).create(true).truncate(true).open(&self.filename) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening file: {}", e);
                return;
            }
        };

        for task in self.tasks.values() {
            let status_str = match task.status {
                TaskStatus::Pending => "Pending",
                TaskStatus::Completed => "Completed",
                TaskStatus::Deleted => "Deleted",
            };
            let line = format!("{}|{}|{}|{}\n", task.id, task.title, task.description, status_str);
            if let Err(e) = file.write_all(line.as_bytes()) {
                println!("Error writing to file: {}", e);
                return;
            }
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new("tasks.txt".to_string());

    loop {
        println!("\nTask Manager");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Mark Task as Deleted");
        println!("5. Delete Task");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice, please try again.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();

                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let description = description.trim().to_string();

                task_manager.add_task(title, description);
            }
            2 => task_manager.list_tasks(),
            3 => {
                println!("Enter task ID to mark as completed:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).unwrap();
                let task_id: u32 = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID, please try again.");
                        continue;
                    }
                };
                task_manager.mark_completed(task_id);
            }
            4 => {
                println!("Enter task ID to mark as deleted:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).unwrap();
                let task_id: u32 = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID, please try again.");
                        continue;
                    }
                };
                task_manager.mark_deleted(task_id);
            }
            5 => {
                println!("Enter task ID to delete:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).unwrap();
                let task_id: u32 = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID, please try again.");
                        continue;
                    }
                };
                task_manager.delete_task(task_id);
            }
            6 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

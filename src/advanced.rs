// More Rust Experiments: Structs, Enums, and Error Handling

// Define a struct
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

// Implementation block for Task
impl Task {
    // Constructor
    fn new(id: u32, title: &str) -> Task {
        Task {
            id,
            title: String::from(title),
            completed: false,
        }
    }
    
    // Method
    fn complete(&mut self) {
        self.completed = true;
    }
    
    // Method with output
    fn summary(&self) -> String {
        format!("[{}] Task #{}: {}", 
            if self.completed { "✓" } else { " " },
            self.id,
            self.title
        )
    }
}

// Enum for task status
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn to_string(&self) -> &str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "In Progress",
            Status::Done => "Done",
        }
    }
}

// Result type for error handling
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero!"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("🦀 More Rust Experiments\n");
    
    // Structs
    println!("=== Structs ===");
    let mut task = Task::new(1, "Learn Rust");
    println!("{}", task.summary());
    task.complete();
    println!("{}", task.summary());
    println!();
    
    // Enums and pattern matching
    println!("=== Enums ===");
    let statuses = [Status::Todo, Status::InProgress, Status::Done];
    for status in &statuses {
        println!("Status: {}", status.to_string());
    }
    println!();
    
    // Error handling with Result
    println!("=== Error Handling ===");
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // Vector experiment
    println!("=== Vectors ===");
    let mut tasks: Vec<Task> = Vec::new();
    tasks.push(Task::new(2, "Read Rust book"));
    tasks.push(Task::new(3, "Build project"));
    tasks.push(Task::new(4, "Share knowledge"));
    
    for task in &tasks {
        println!("{}", task.summary());
    }
    
    println!("\n✅ All advanced experiments completed!");
}

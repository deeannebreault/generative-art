// Rust Ownership & Borrowing Experiments
// Just playing around to understand the concepts

fn main() {
    println!("🦀 Rust Ownership Experiments\n");
    
    // Experiment 1: Ownership transfer
    println!("=== Experiment 1: Ownership Transfer ===");
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    
    let s2 = s1;  // Ownership moves to s2
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1);  // This would be a compile error!
    println!("✅ s1 is no longer valid after move to s2\n");
    
    // Experiment 2: Borrowing (references)
    println!("=== Experiment 2: Borrowing ===");
    let s3 = String::from("world");
    let len = calculate_length(&s3);  // Borrow s3
    println!("Length of '{}' is {}", s3, len);
    println!("✅ s3 is still valid after borrowing!\n");
    
    // Experiment 3: Mutable borrowing
    println!("=== Experiment 3: Mutable Borrowing ===");
    let mut s4 = String::from("foo");
    println!("Before: {}", s4);
    change_string(&mut s4);
    println!("After: {}", s4);
    println!("✅ Changed via mutable reference\n");
    
    // Experiment 4: Multiple immutable borrows (OK)
    println!("=== Experiment 4: Multiple Immutable Borrows ===");
    let s5 = String::from("shared");
    let r1 = &s5;
    let r2 = &s5;
    println!("r1 = {}, r2 = {}", r1, r2);
    println!("✅ Multiple immutable borrows are fine!\n");
    
    // Experiment 5: Slices
    println!("=== Experiment 5: Slices ===");
    let s6 = String::from("hello world");
    let hello = &s6[0..5];
    let world = &s6[6..11];
    println!("Original: '{}'", s6);
    println!("Slice 1: '{}'", hello);
    println!("Slice 2: '{}'", world);
    println!("✅ Slices reference part of a string without taking ownership\n");
    
    println!("All experiments completed! 🎉");
}

fn calculate_length(s: &String) -> usize {
    s.len()  // We borrow s, don't take ownership
}

fn change_string(s: &mut String) {
    s.push_str(" bar");  // Modify via mutable reference
}

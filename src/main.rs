use toy_green_thread::{schedule ,spawn, spawn_from_main};

fn main() {
    let _ = spawn_from_main(("hello".to_string(), hello), 2 * 1024 * 1024);
    println!("Done Task")
}

fn hello() {
    let _ = spawn(("morinig".to_string(), mornig), 2 * 1024 * 1024).unwrap();
    let _ = spawn(("noon".to_string(), noon), 2 * 1024 * 1024).unwrap();
    let _ = spawn(("afternoon".to_string(), afternoon), 2 * 1024 * 1024).unwrap();
}

fn mornig() {
    for _ in 0..5 {
        println!("Good Morning!");
        schedule("morinig").expect("[Error] morning")
    }
    
}

fn noon() {
    for _ in 0..3 {
        println!("Good Noon!");
        schedule("noon").expect("[Error] noon")
    }
}

fn afternoon() {
    for _ in 0..4 {
        println!("Good Afternoon!");
        schedule("afternoon").expect("[Error] afternoon")
    }
}

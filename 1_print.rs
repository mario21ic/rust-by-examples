// This is the main function
fn main() {
    println!("Hello World!");

    println!("Hello World {}!", "mario21ic");
    println!("Hello World {0} {1}!", "Mario", "Inga");

    println!("Hello World {name}!", name="mario21ic");

    let mario: &str = "mario21ic";
    println!("Hello World {name}!", name=mario);

    let mut name = "mario";
    println!("Hello World {name}!", name=name);
    name = "mario21ic";
    println!("Hello World {name}!", name=name);
}


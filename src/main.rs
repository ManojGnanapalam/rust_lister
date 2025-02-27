mod todos;
use std::io;

fn main() {
    let mut todo_list = todos::TodoList::new();

    loop{
        println!("1. Add Item");
        println!("2. List Items");
        println!("3. Complete Item");
        println!("4. Exit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        let choice = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1=>{
                println!("Enter the title of the new item:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read entered title");
                todo_list.add_item(title.trim().to_string());
            }

            2=>{
                todo_list.list_items();
            }
            3=>{
                println!("Enter the ID of the completed item:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.complete_item(id);
            }
            4=>{
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}

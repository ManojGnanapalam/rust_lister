pub struct TodoItem{
    pub id: u64,
    pub title: String,
    pub is_completed: bool,
}

pub struct TodoList{
    pub items: Vec<TodoItem>,
}

impl TodoList{
    pub fn new() -> TodoList{
        TodoList{items: Vec::new()}
    }

    pub fn add_item(&mut self, title: String){
        let id = self.items.len() as u64 + 1;
        let new_item = TodoItem{
            id,
            title: title.clone(),
            is_completed:false,
        };

        self.items.push(new_item);
        println!("Added: {}",title);
    }

    pub fn list_items(&self){
        if self.items.is_empty(){
            println!("to do list is empty....");
        }else{
            for item in &self.items{
                let status = if item.is_completed {"[X]"} else {"[]"};

                println!("{} : {}) {} ",status, item.id,item.title);
            }
        }
    }

    pub fn complete_item(&mut self, id: u64){
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id){
            item.is_completed = true;
            println!("Completed: {}",item.title);
        }else{
            println!("Item with ID {} not found",id);
        }
    }

    
}
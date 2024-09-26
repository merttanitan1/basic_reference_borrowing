fn main() {
    let mut texts = vec![String::from("Text 1"), String::from("Text 2")];
    
    println!("First list:");
    print_list(&texts);
    
    println!("List is updating...");
    update_text(&mut texts, 1, String::from("New Text 2"));
    
    println!("Updated list:");
    print_list(&texts);
}

fn print_list(texts: &Vec<String>) {
    for text in texts {
        println!("{}", text);
    }
}

fn update_text(texts: &mut Vec<String>, index: usize, new_text: String) {
    if let Some(text) = texts.get_mut(index) {
        *text = new_text;
    }
}

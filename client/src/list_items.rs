// list_items.rs
use egui::Ui;

pub fn show_list_items(ui: &mut Ui) {
    ui.label("Foo3");

    let request = ehttp::Request::get("https://jsonplaceholder.typicode.com/posts");
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        if let Ok(response) = result {
            if let Ok(posts_json) = std::str::from_utf8(&response.bytes) {
                println!("x");
                if let Ok(posts) = serde_json::from_str::<Vec<serde_json::Value>>(posts_json) {
                    for post in posts {
                        if let Some(title) = post.get("title").and_then(|t| t.as_str()) {
                            println!("{}", title);
                        }
                    }
                } else {
                    println!("Failed to parse posts JSON");
                }
            } else {
                println!("Failed to read posts response");
            }
        } else {
            println!("Failed to fetch posts");
        }
    });
}

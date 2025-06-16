// list_items.rs
use egui::Ui;

pub fn show_list_items(ui: &mut Ui) {
    ui.label("Foo");

    let request = ehttp::Request::get("https://jsonplaceholder.typicode.com/posts");
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        // println!("Status code: {:?}", result.unwrap().status);
        // if let Ok(response) = result {
        //     if let Ok(posts_json) = response.text() {
        //         if let Ok(posts) = serde_json::from_str::<Vec<serde_json::Value>>(&posts_json) {
        //             ui.heading("Posts:");
        //             for post in posts {
        //                 if let Some(title) = post.get("title").and_then(|t| t.as_str()) {
        //                     ui.label(title);
        //                 }
        //             }
        //         } else {
        //             ui.label("Failed to parse posts JSON");
        //         }
        //     } else {
        //         ui.label("Failed to read posts response");
        //     }
        // } else {
        //     ui.label("Failed to fetch posts");
        // }
    });

    // if let Ok(response) = ureq::get("https://jsonplaceholder.typicode.com/posts").call() {
    //     // if let Ok(posts_json) = response.into_string() {
    //     //     let posts_result: Result<Vec<serde_json::Value>, _> = serde_json::from_str(&posts_json);
    //     //     if let Ok(posts) = posts_result {
    //     //         ui.heading("Posts:");
    //     //         for post in posts {
    //     //             if let Some(title) = post.get("title").and_then(|t| t.as_str()) {
    //     //                 ui.label(title);
    //     //             }
    //     //         }
    //     //     } else {
    //     //         ui.label("Failed to parse posts JSON");
    //     //     }
    //     // } else {
    //     //     ui.label("Failed to read posts response");
    //     // }
    // } else {
    //     ui.label("Failed to fetch posts");
    // }
}

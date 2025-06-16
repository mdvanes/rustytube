// list_items.rs
use egui::Ui;
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Deserialize, Debug, Clone)]
struct Post {
    title: String,
}

thread_local! {
    static POSTS: Rc<RefCell<Option<Vec<Post>>>> = Rc::new(RefCell::new(None));
}

pub fn show_list_items(ui: &mut Ui) {
    // Fetch posts if not already fetched
    let mut need_fetch = false;
    POSTS.with(|posts| {
        if posts.borrow().is_none() {
            need_fetch = true;
        }
    });
    if need_fetch {
        let request = ehttp::Request::get("https://jsonplaceholder.typicode.com/posts");
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            if let Ok(response) = result {
                if let Ok(posts_json) = std::str::from_utf8(&response.bytes) {
                    if let Ok(posts) = serde_json::from_str::<Vec<Post>>(posts_json) {
                        POSTS.with(|cell| {
                            *cell.borrow_mut() = Some(posts);
                        });
                    }
                }
            }
        });
    }

    ui.heading("Posts:");
    POSTS.with(|posts| {
        if let Some(posts) = &*posts.borrow() {
            for post in posts.iter().take(10) {
                ui.label(&post.title);
            }
        } else {
            ui.label("Loading...");
        }
    });
}

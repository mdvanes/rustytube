// list_items.rs
use egui::Ui;
use serde::Deserialize;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use futures::stream::StreamExt;
use reqwest_eventsource::{Event, EventSource};

#[derive(Deserialize, Debug, Clone)]
struct Post {
    title: String,
}

thread_local! {
    static POSTS: Rc<RefCell<Option<Vec<Post>>>> = Rc::new(RefCell::new(None));
    static EVENT_SOURCE_POLLED: Cell<bool> = Cell::new(false);
}

async fn run_event_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut es = EventSource::get("http://localhost:8081/events");
    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => {
                // Try to parse as JSON and extract "message"
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&message.data) {
                    if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
                        println!("message: {}", msg);
                    } else {
                        println!("event data (json, no 'message'): {}", message.data);
                    }
                } else {
                    println!("event data: {}", message.data);
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                // es.close();
            }
        }
    }
    Ok(())
}

pub fn show_list_items_eventsourced(ui: &mut Ui) {
    // Poll run_event_source only once
    EVENT_SOURCE_POLLED.with(|polled| {
        if !polled.get() {
            polled.set(true);
            wasm_bindgen_futures::spawn_local(async {
                let _ = run_event_source().await;
            });
        }
    });

    // Fetch posts if not already fetched
    // let mut need_fetch = false;
    // POSTS.with(|posts| {
    //     if posts.borrow().is_none() {
    //         need_fetch = true;
    //     }
    // });
    // if need_fetch {
    //     let request = ehttp::Request::get("http://localhost:8081/api/posts");
    //     ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
    //         if let Ok(response) = result {
    //             if let Ok(posts_json) = std::str::from_utf8(&response.bytes) {
    //                 if let Ok(posts) = serde_json::from_str::<Vec<Post>>(posts_json) {
    //                     POSTS.with(|cell| {
    //                         *cell.borrow_mut() = Some(posts);
    //                     });
    //                 }
    //             }
    //         }
    //     });
    // }

    ui.heading("Posts (EventSource):");
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

// list_items.rs
use egui::Ui;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use chrono::Local;
use futures::stream::StreamExt;
use reqwest_eventsource::{Event, EventSource};

thread_local! {
    // static POSTS: Rc<RefCell<Option<Vec<Post>>>> = Rc::new(RefCell::new(None));
    static MESSAGES: Rc<RefCell<Vec<(String, chrono::NaiveTime)>>> = Rc::new(RefCell::new(Vec::new()));
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
                        let now = Local::now().time();
                        MESSAGES.with(|messages| {
                            messages.borrow_mut().push((msg.to_string(), now));
                        });
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

    ui.heading("Messages (EventSource):");
    MESSAGES.with(|messages| {
        let messages = messages.borrow();
        if messages.is_empty() {
            ui.label("No messages yet.");
        } else {
            for (msg, time) in messages.iter() {
                let timestamp = time.format("%H:%M:%S").to_string();
                ui.label(format!("[{timestamp}] {msg}"));
            }
        }
    });
}

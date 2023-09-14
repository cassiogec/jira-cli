use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db = JiraDatabase::new("data/db.json".to_owned());
    let mut navigator = Navigator::new(Rc::new(db));
    loop {
        clearscreen::clear().unwrap();
        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            };

            let input = get_user_input();

            match page.handle_input(input.trim()) {
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!(
                                "Error handling action: {}\nPress any key to continue...",
                                error
                            );
                            wait_for_key_press();
                        }
                    }
                }
                Err(error) => {
                    println!(
                        "Error handling the input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                }
            }
        } else {
            break;
        }
    }
}

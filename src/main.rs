use inputbot::{KeybdKey::*, MouseButton::*};
use std::{thread::sleep, time::Duration};

fn main() {
    X2Button.bind(move || {
        println!("Activated macro.");
        while !X1Button.is_pressed() {
            LeftButton.press();
            LeftButton.release();
            EKey.press();
            EKey.release();
            FKey.press();
            FKey.release();
            sleep(Duration::from_millis(4));
        }
        println!("Deactivated macro.");
    });

    println!("Macro is ready!");
    inputbot::handle_input_events();
}

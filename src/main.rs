use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};
use rustautogui::RustAutoGui;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut rustautogui = RustAutoGui::new(false);
    X1Button.bind(move || {
        while !X1Button.is_pressed() {
            LeftButton.press();
            LeftButton.release();
            EKey.press();
            EKey.release();
            FKey.press();
            FKey.release();
            sleep(Duration::from_millis(4));
        }
    });
}

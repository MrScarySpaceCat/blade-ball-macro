use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use enigo::{Direction::Press, Enigo, Key as EnigoKey, Keyboard, Settings};
use rdev::{Button, EventType, listen};

/// Are we currently running the macro?
static RUNNING: AtomicBool = AtomicBool::new(false);
/// “Stop” flag that the background thread checks each cycle.
static STOP: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ------------------------------------------------------------
    // Global listener – reacts to X2 (start) and X1 (stop)
    // ------------------------------------------------------------
    println!("Macro ready! Hold Mouse5? to use the macro.");

    listen(move |event| {
        match event.event_type {
            // ------------- X2: start -------------------------------------------------
            EventType::ButtonPress(Button::Unknown(1)) => {
                // only start if we’re not already running
                if !RUNNING.load(Ordering::SeqCst) {
                    RUNNING.store(true, Ordering::SeqCst);
                    STOP.store(false, Ordering::SeqCst);
                    println!("Activated macro.");

                    // Create the key‑sending thread.
                    thread::spawn(|| {
                        let mut enigo = Enigo::new(&Settings::default()).unwrap();

                        // Keep spamming E+F while X1 remains pressed
                        while !STOP.load(Ordering::SeqCst) {
                            // enigo.key(EnigoKey::Unicode('e'), Press).unwrap();
                            enigo.key(EnigoKey::Unicode('f'), Press).unwrap();
                            thread::yield_now();
                        }

                        // Clean‑up after the loop
                        RUNNING.store(false, Ordering::SeqCst);
                        println!("Deactivated macro.");
                    });
                }
            }

            EventType::ButtonRelease(Button::Unknown(1)) => {
                if RUNNING.load(Ordering::SeqCst) {
                    STOP.store(true, Ordering::SeqCst);
                }
            }

            _ => {}
        }
    });

    Ok(())
}

use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use enigo::{Enigo, Key as EnigoKey, KeyboardControllable};
use rdev::{Event, EventType, Key, listen};

/// Are we currently running the macro?
static RUNNING: AtomicBool = AtomicBool::new(false);
/// “Stop” flag that the background thread checks each cycle.
static STOP: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ------------------------------------------------------------
    // Global listener – reacts to X2 (start) and X1 (stop)
    // ------------------------------------------------------------
    println!("Macro ready! Press X2 to start, X1 to stop.");

    listen(move |event| {
        match event.event_type {
            // ------------- X2: start -------------------------------------------------
            EventType::KeyPress(Key::KeyX2) => {
                // only start if we’re not already running
                if !RUNNING.load(Ordering::SeqCst) {
                    RUNNING.store(true, Ordering::SeqCst);
                    STOP.store(false, Ordering::SeqCst);
                    println!("Activated macro.");

                    // Create the key‑sending thread.
                    thread::spawn(|| {
                        let mut enigo = Enigo::new({});

                        // Keep spamming E+F while X1 remains pressed
                        while !STOP.load(Ordering::SeqCst) {
                            enigo.key_click(EnigoKey::E);
                            enigo.key_click(EnigoKey::F);
                            thread::sleep(Duration::from_millis(4));
                        }

                        // Clean‑up after the loop
                        RUNNING.store(false, Ordering::SeqCst);
                        println!("Deactivated macro.");
                    });
                }
            }

            // ------------- X1: stop --------------------------------------------------
            EventType::KeyPress(Key::KeyX1) => {
                if RUNNING.load(Ordering::SeqCst) {
                    STOP.store(true, Ordering::SeqCst);
                }
            }

            _ => {}
        }

        // Let the event propagate unchanged (no suppression)
        Some(event)
    })?;

    Ok(())
}

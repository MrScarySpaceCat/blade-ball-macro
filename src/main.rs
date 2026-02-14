use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    thread,
    time::Duration,
};

use enigo::{Direction::Click, Enigo, Key as EnigoKey, Keyboard, Settings};
use rdev::{Button, EventType, listen};
use text_io::read;

/// Are we currently running the macro?
static RUNNING: AtomicBool = AtomicBool::new(false);
/// “Stop” flag that the background thread checks each cycle.
static STOP: AtomicBool = AtomicBool::new(false);

static PAUSE_MICROS: AtomicU64 = AtomicU64::new(10000);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter desired keys per second: ");

    let kps: u64 = read!("{}\n");
    let wait_micros: u64 = 1000000 / kps;

    PAUSE_MICROS.store(wait_micros, Ordering::SeqCst);
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

                        let micros = PAUSE_MICROS.load(Ordering::SeqCst);

                        // Keep spamming E+F while X1 remains pressed
                        while !STOP.load(Ordering::SeqCst) {
                            // enigo.key(EnigoKey::Unicode('e'), Click).unwrap();
                            // thread::sleep(Duration::from_millis(PAUSE_MILLIS));
                            enigo.key(EnigoKey::Unicode('f'), Click).unwrap();
                            thread::sleep(Duration::from_micros(micros));
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
    })
    .unwrap();

    Ok(())
}

/*!
* gash : catch interrupt signal
* cs4414 ps2
* Jeremy Letang / free student
*/

use std::io::signal::{Listener, Interrupt};

pub fn catch() {
    spawn(proc() {
        let mut listener = Listener::new();
        match listener.register(Interrupt) {
            true => {
                loop {
                    match listener.port.recv() {
                        Interrupt => {/* Nothing to do */},
                        _         => {/* Nothing to do */}
                    }
                }
            }
            false => println!("Error: cannot register Interrupt signal")
        }
    });
}
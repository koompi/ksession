use self::calloop::signals::{Signal, Signals};
use self::calloop::{Dispatcher, EventLoop};
use self::nix::sys::signal::{kill, SigSet};
use self::nix::unistd::Pid;
use std::env;
use std::io;
extern crate calloop;
extern crate nix;
// use calloop::{generic::Generic, EventLoop, Interest, Mode};
use std::time::Duration;

pub fn run_loop() {
    let mut event_loop = EventLoop::try_new().expect("Failed to initialize the event loop!");
    // Retrieve an handle. It is used to insert new sources into the event loop
    // It can be cloned, allowing you to insert sources from within source callbacks
    let handle = event_loop.handle();
    let mut signal_received = true;
    // Inserting an event source takes this general form
    // it can also be done from within the callback of an other event source
    handle
        .insert_source(
            // a type implementing the EventSource trait
            Signals::new(&[Signal::SIGUSR1]).unwrap(),
            // a callback that is invoked whenever this source generates an event
            move |event, &mut (), rcv| {
                // This callback is given 3 values:
                // - the event generated by the source
                // - &mut access to some metadata, specific to the event source
                // - &mut access to the global shared data that was passed to EventLoop::dispatch
                assert_eq!(event.signal(), Signal::SIGUSR1);
                println!("event source called back");
                *rcv = true;
            },
        )
        .map_err(Into::<io::Error>::into)
        .unwrap();

    // Actual run of your loop
    //
    // Dispatch received events to their callbacks, waiting at most 20 ms for
    // new events between each invocation of the provided callback.
    //
    // The `&mut shared_data` is a mutable reference that will be forwarded to all
    // your callbacks, allowing them to share some state
    let result = dbus::dbus_call(
        "org.freedesktop.PowerManagement",
        "/org/freedesktop/PowerManagement",
        "CanSuspend",
    );
    match result {
        Ok(data) => {
            if data {
                println!("Can suspend {}", data);
            } else {
                println!("Cannot suspend");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    let mut count = 0;
    event_loop
        .run(
            Some(Duration::from_millis(2000)),
            &mut signal_received,
            |shared_data| {
                /*
                 * Insert here the processing you need to do do between each waiting session
                 * like your drawing logic if you're doing a GUI app for example.
                 */
                count += 1;
                if count == 5 {
                    std::process::exit(0);
                } else {
                    println!("Event loop");
                    println!("{:?}", env::var("XDG_SESSION_ID"));
                }
                // if env: {
                //     println!("Event looop");
                // } else {
                //     std::process::exit(0);
                // }
            },
        )
        .unwrap();
}
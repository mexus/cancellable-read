extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_signal;
extern crate tokio_stdin_stdout;

use futures::{Future, IntoFuture, Stream};
use std::io::BufReader;
use tokio_core::reactor::Core;
use tokio_io::io;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let reading = Box::new(
        io::lines(BufReader::new(tokio_stdin_stdout::stdin(0)))
            .for_each(|s| {
                println!("Received: {}", s);
                Ok(())
            })
            .into_future()
            .map_err(|e| println!("Error while reading: {}", e)),
    );
    let ctrl_c = Box::new(
        tokio_signal::ctrl_c(&handle)
            .flatten_stream()
            .map_err(|e| println!("Error while reading: {}", e))
            .take(1)
            .into_future()
            .map(|_| println!("Received ctrl_c")),
    );

    match core.run(ctrl_c.select2(reading)) {
        _ => {}
    }
}

#[cfg(feature = "rtu")]
pub fn main() {
    use futures::future::Future;
    use tokio_core::reactor::Core;
    use tokio_modbus::prelude::*;
    use tokio_serial::{Serial, SerialPortSettings};

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tty_path = "/dev/ttyUSB0";
    let server_addr = 0x01;

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 19200;
    let port = Serial::from_path_with_handle(tty_path, &settings, &handle.new_tokio_handle())
        .expect(&format!("Unable to open serial device '{}'", tty_path));

    // On Unix you might disable the `exclusive` flag:
    // port.set_exclusive(false)
    //     .expect("Unable to set serial port exlusive");

    let task = rtu::connect(&handle, port, server_addr).and_then(|ctx| {
        println!("Reading a sensor value");
        ctx.read_holding_registers(0x082B, 2).and_then(move |rsp| {
            println!("Sensor value is: {:?}", rsp);
            Ok(())
        })
    });

    core.run(task).unwrap();
}

#[cfg(not(feature = "rtu"))]
pub fn main() {
    println!("feature `rtu` is required to run this example");
    std::process::exit(1);
}

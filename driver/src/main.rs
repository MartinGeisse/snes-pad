use std::{env, error, io, time, fs};
use io::Error;
use input_linux::sys;

const BAUD_RATE: u32 = 38400;

fn main() -> Result<(), Box<dyn error::Error>> {

    // create serial port
    let serial_port_result: Result<Box<dyn serialport::SerialPort>, Error> = {
        let args: Vec<_> = env::args().collect();
        let serial_port_device_file_path: String = if args.len() > 1 {
            args[1].clone()
        } else {
            "/dev/ttyUSB1".to_owned()
        };
        println!("Connecting to serial port at {}", serial_port_device_file_path);
        let settings = serialport::SerialPortSettings {
            baud_rate: BAUD_RATE,
            data_bits: serialport::DataBits::Eight,
            flow_control: serialport::FlowControl::None,
            parity: serialport::Parity::None,
            stop_bits: serialport::StopBits::One,
            timeout: time::Duration::from_millis(5000),
        };
        Ok(serialport::open_with_settings::<String>(&serial_port_device_file_path.into(), &settings)?)
    };
    let mut serial_port: Box<dyn serialport::SerialPort> = serial_port_result?;

    // create joystick device
    let joystick_device_result: Result<input_linux::UInputHandle<fs::File>, Error> = {
        let uinput_file = fs::File::create("/dev/uinput")?;
        let device = input_linux::UInputHandle::new(uinput_file);

        let input_id = input_linux::InputId {
            bustype: sys::BUS_VIRTUAL,
            vendor: 34,
            product: 10,
            version: 1,
        };

        device.set_evbit(input_linux::EventKind::Absolute)?; // TODO remove?
        device.set_evbit(input_linux::EventKind::Key)?;
        device.set_keybit(input_linux::Key::ButtonTrigger)?; // needed -- informs linux that this is a joystick

        device.set_keybit(input_linux::Key::ButtonSouth)?;
        device.set_keybit(input_linux::Key::ButtonWest)?;
        device.set_keybit(input_linux::Key::ButtonSelect)?;
        device.set_keybit(input_linux::Key::ButtonStart)?;
        device.set_keybit(input_linux::Key::ButtonDpadUp)?;
        device.set_keybit(input_linux::Key::ButtonDpadDown)?;
        device.set_keybit(input_linux::Key::ButtonDpadLeft)?;
        device.set_keybit(input_linux::Key::ButtonDpadRight)?;
        device.set_keybit(input_linux::Key::ButtonEast)?;
        device.set_keybit(input_linux::Key::ButtonNorth)?;
        device.set_keybit(input_linux::Key::ButtonTL)?;
        device.set_keybit(input_linux::Key::ButtonTR)?;

        device.create(&input_id, b"arduino-snes-pad", 0, &([] as [input_linux::AbsoluteInfoSetup; 0]))?;

        println!("Created joystick with device path {}", device.evdev_path()?.to_string_lossy());
        Ok(device)
    };
    let joystick_device: input_linux::UInputHandle<fs::File> = joystick_device_result?;

    // main loop
    let mut state: u16 = 0;
    loop {
        let mut buffer: [u8; 1] = [0; 1];
        if serial_port.read(&mut buffer)? == 0 {
            continue;
        }
        state = (state >> 8) + ((buffer[0] as u16) << 8);
        if (state & 32768) != 0 {

            // B, Y, SELECT, START, UP, DOWN, LEFT, RIGHT, A, X, L, R

            handle_button(&joystick_device, state, 0, input_linux::Key::ButtonSouth)?;
            handle_button(&joystick_device, state, 1, input_linux::Key::ButtonWest)?;
            handle_button(&joystick_device, state, 2, input_linux::Key::ButtonSelect)?;
            handle_button(&joystick_device, state, 3, input_linux::Key::ButtonStart)?;
            handle_button(&joystick_device, state, 4, input_linux::Key::ButtonDpadUp)?;
            handle_button(&joystick_device, state, 5, input_linux::Key::ButtonDpadDown)?;

            handle_button(&joystick_device, state, 8, input_linux::Key::ButtonDpadLeft)?;
            handle_button(&joystick_device, state, 9, input_linux::Key::ButtonDpadRight)?;
            handle_button(&joystick_device, state, 10, input_linux::Key::ButtonEast)?;
            handle_button(&joystick_device, state, 11, input_linux::Key::ButtonNorth)?;
            handle_button(&joystick_device, state, 12, input_linux::Key::ButtonTL)?;
            handle_button(&joystick_device, state, 13, input_linux::Key::ButtonTR)?;

            write_event(&joystick_device, input_linux::SynchronizeEvent::report(empty_event_time()))?;

        }
    }
}

fn empty_event_time() -> input_linux::EventTime {
    input_linux::EventTime::new(0, 0)
}

fn write_event(
    joystick_device: &input_linux::UInputHandle<fs::File>,
    event: impl std::convert::AsRef<sys::input_event>
) -> Result<(), Error> {
    joystick_device.write(&[*event.as_ref()])?;
    Ok(())
}

fn handle_button(
    joystick_device: &input_linux::UInputHandle<fs::File>,
    state: u16,
    bit: u32,
    key: input_linux::Key
) -> Result<(), Error> {
    let key_state = if ((state >> bit) & 1) == 1 {input_linux::KeyState::PRESSED} else {input_linux::KeyState::RELEASED};
    write_event(joystick_device, input_linux::KeyEvent::new(empty_event_time(), key, key_state))?;
    Ok(())
}

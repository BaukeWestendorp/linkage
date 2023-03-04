use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

mod settings;

fn main() -> io::Result<()> {
    // FIXME: Use data from config file.
    ws::listen("0.0.0.0:3012", move |frontend| {
        let runtime_stream =
            TcpStream::connect("raspberrypi:8009").expect("should connect to runtime");
        eprintln!("Connected to Runtime on address {}.", "0.0.0.0:8009");

        move |msg| {
            let mut runtime_stream = runtime_stream.try_clone().unwrap();

            eprintln!("Received message from frontend: {msg:?}");

            match msg {
                ws::Message::Text(_) => todo!(),
                ws::Message::Binary(bin) => {
                    if bin.len() != 8 {
                        eprintln!("Binary data should have 8 bytes, but found {}", bin.len());
                        return Ok(());
                    }

                    match bin.first() {
                        Some(instruction) => match instruction {
                            0x00 => {
                                enable_linkage(&mut runtime_stream)?;
                                frontend.send(ws::Message::Binary(
                                    BACKEND_TX_MESSAGE_ENABLED_LINKAGE.into(),
                                ))?;
                            }
                            0x01 => {
                                disable_linkage(&mut runtime_stream)?;
                                frontend.send(ws::Message::Binary(
                                    BACKEND_TX_MESSAGE_DISABLED_LINKAGE.into(),
                                ))?;
                            }
                            _ => {
                                eprintln!("Unknown Instuction {instruction}");
                            }
                        },
                        None => unreachable!("Binary message should have 8 bytes."),
                    };

                    Ok(())
                }
            }
        }
    })
    .unwrap();

    Ok(())
}

fn enable_linkage(runtime_stream: &mut TcpStream) -> io::Result<()> {
    runtime_stream.write(&RUNTIME_RX_MESSAGE_ENABLE_LINKAGE)?;

    let mut buffer = MessageBytes::default();
    runtime_stream.read_exact(&mut buffer)?;
    if buffer.len() != 8 {
        eprintln!(
            "Binary data should have 8 bytes, but found {}",
            buffer.len()
        );
        return Ok(());
    }

    match buffer.first() {
        Some(instruction) => {
            if instruction == &LINKAGE_TX_MESSAGE_ENABLED[0] {
                eprintln!("Linkage has been enabled");
                // FIXME: Start sending controller input events to linkage.
            }
        }
        None => unreachable!("Binary message should have 8 bytes."),
    }

    Ok(())
}

fn disable_linkage(runtime_stream: &mut TcpStream) -> io::Result<()> {
    runtime_stream.write(&RUNTIME_RX_MESSAGE_DISABLE_LINKAGE)?;

    let mut buffer = MessageBytes::default();
    runtime_stream.read_exact(&mut buffer)?;
    if buffer.len() != 8 {
        eprintln!(
            "Binary data should have 8 bytes, but found {}",
            buffer.len()
        );
        return Ok(());
    }

    match buffer.first() {
        Some(instruction) => {
            if instruction == &LINKAGE_TX_MESSAGE_DISABLED[0] {
                eprintln!("Linkage has been disabled");
                // FIXME: Start sending controller input events to linkage.
            }
        }
        None => unreachable!("Binary message should have 8 bytes."),
    }

    Ok(())
}

type MessageBytes = [u8; 8];

const BACKEND_TX_MESSAGE_ENABLED_LINKAGE: MessageBytes = [0x00, 0, 0, 0, 0, 0, 0, 0];
const BACKEND_TX_MESSAGE_DISABLED_LINKAGE: MessageBytes = [0x01, 0, 0, 0, 0, 0, 0, 0];

const RUNTIME_RX_MESSAGE_ENABLE_LINKAGE: MessageBytes = [0x00, 0, 0, 0, 0, 0, 0, 0];
const RUNTIME_RX_MESSAGE_DISABLE_LINKAGE: MessageBytes = [0x01, 0, 0, 0, 0, 0, 0, 0];

const LINKAGE_TX_MESSAGE_ENABLED: MessageBytes = [0x00, 0, 0, 0, 0, 0, 0, 0];
const LINKAGE_TX_MESSAGE_DISABLED: MessageBytes = [0x01, 0, 0, 0, 0, 0, 0, 0];

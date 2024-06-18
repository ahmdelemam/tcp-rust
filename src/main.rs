use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    let mut buf = [0u8; 1504];
    let nbytes = nic.recv(&mut buf[..])?;

    eprint!("Read {} bytes: {:02x?}\n", nbytes, &buf[..nbytes]);
    Ok(())
}

 // sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/tcp-rust
 
use std::io::Read;
extern crate tun;

fn main(){
    let mut config = tun::Configuration::default();
	config.address((10, 0, 0, 1))
	       .netmask((255, 255, 255, 0))
	       .up();

	#[cfg(target_os = "linux")]
	config.platform(|config| {
		config.packet_information(true);
	});

	let mut dev = tun::create(&config).unwrap();
	let mut buf = [0; 4096];

	loop {
		let amount = dev.read(&mut buf).unwrap();
		println!("{:?}", &buf[0 .. amount]);
	}
}
 
// It just works, but you have to set up routing manually. For example:

// sudo route -n add -net 10.0.0.0/24 10.0.0.1


// sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/tcp-rust
// sudo ip addr set 192.168.0.1/24 dev tun0
// sudo ip link set up dev tun0
// ping -I tun0 192.168.0.2
// tshark -i tun0


const TUX : [&str; 7] = [
    "\x1b[37m       ,--.      ",
    "\x1b[37m      |0\x1b[33m_\x1b[37m0 |     ",
    "\x1b[37m      |\x1b[33mL_/\x1b[37m |     ",
    "\x1b[37m     //   \\ \\    ",
    "\x1b[37m    ((     ) )   ",
    "\x1b[33m   /`\\     /`\\ \x1b[37m  ",
    "\x1b[33m   \\__)\x1b[37m===\x1b[33m(__/\x1b[37m   "
];

const ALPINE : [&str; 7] = [
    "\x1b[31m   .8BBBBBBBBB.  \x1b[37m",
    "\x1b[33m  .888888888888. \x1b[37m",
    "\x1b[32m .8888* *88*8888.\x1b[37m",
    "\x1b[36m 888* ,8, `. *888\x1b[37m",
    "\x1b[36m `* ,88888, `, *`\x1b[37m",
    "\x1b[34m  `888888888888` \x1b[37m",
    "\x1b[35m   `8BBBBBBBBB`  \x1b[37m"
];

const ARCH : [&str; 7] = [
    "\x1b[31m        A        \x1b[37m",
    "\x1b[31m       a8s       \x1b[37m",
    "\x1b[33m      ao88s      \x1b[37m",
    "\x1b[32m     a88888s     \x1b[37m",
    "\x1b[36m    a88Y*Y88s    \x1b[37m",
    "\x1b[34m   a88H   B8bs   \x1b[37m",
    "\x1b[35m  /*`       `*\\  \x1b[37m"
];

const DEBIAN : [&str; 7] = [
    "\x1b[31m    ,gA88bq.     \x1b[37m",
    "\x1b[33m   dP      `9.   \x1b[37m",
    "\x1b[32m  d7  ,*`.  )8   \x1b[37m",
    "\x1b[36m  9:  A    ,Q*   \x1b[37m",
    "\x1b[34m  *1  `^vsv\"     \x1b[37m",
    "\x1b[35m   *b            \x1b[37m",
    "\x1b[35m     \"~.         \x1b[37m"
];

const UBUNTU : [&str; 7] = [
    "\x1b[31m          \x1b[35m($)    \x1b[37m",
    "\x1b[31m    \x1b[33m .\x1b[31m s88~..    \x1b[37m",
    "\x1b[31m    \x1b[33m.8*  \x1b[31m `*9.   \x1b[37m",
    "\x1b[31m \x1b[31m($) \x1b[33m8           \x1b[37m",
    "\x1b[31m    \x1b[33m`6-  \x1b[35m _-8`   \x1b[37m",
    "\x1b[31m     \x1b[33m`\x1b[35m ^88*``    \x1b[37m",
    "\x1b[31m          \x1b[33m($)    \x1b[37m"
];

fn fetch_packages_apk() -> Result<String, std::io::Error> {
    let buf = std::fs::read("/lib/apk/db/installed")?;
	let mut packages = 0;
	let buf0 = &buf[..buf.len()-1];
	let buf1 = &buf[1..];
	let mut chunks0 = buf0.chunks_exact(512);
	let mut chunks1 = buf1.chunks_exact(512);

	while let (Some(chunk0), Some(chunk1)) = (chunks0.next(), chunks1.next()) {
		let mut counter = 0;
		for j in 0..512 {
			counter += (chunk0[j] == b'\n') as u8 & (chunk1[j] == b'P') as u8;
		}
		packages += counter as usize;
	}

	for (a,b) in chunks0.remainder().iter().zip(chunks1.remainder()) {
		packages += (*a == b'\n') as usize & (*b == b'P') as usize;
	}

    Ok(packages.to_string())
}

fn main() {
    let user = std::env::var("LOGNAME").unwrap_or("Unknown".to_string());
    let host = nixinfo::hostname().unwrap_or("Unknown".to_string());
    let distro = nixinfo::distro().unwrap_or("Unknown".to_string());
    let kernel = nixinfo::kernel().unwrap_or("Unknown".to_string());
    let uptime = nixinfo::uptime().unwrap_or("Unknown".to_string());
    let shell = std::env::var("SHELL").unwrap_or("Unknown".to_string());
    let terminal = nixinfo::terminal().unwrap_or("Unknown".to_string());
    let cpu = nixinfo::cpu().unwrap_or("Unknown".to_string());
    let gpu = nixinfo::gpu().unwrap_or("Unknown".to_string());
    let memory = nixinfo::memory().unwrap_or("Unknown".to_string());

    let mut packages = "".to_string();
    match nixinfo::packages("apt") {
        Ok(p) => {
            packages += &p;
            packages += " (apt) ";
        }
        Err(_) => {}
    }
    match nixinfo::packages("pacman") {
        Ok(p) => {
            packages += &p;
            packages += " (pacman) ";
        }
        Err(_) => {}
    }
    match fetch_packages_apk() {
        Ok(p) => {
            packages += &p;
            packages += " (apk) ";
        }
        Err(_) => {}
    }

    //todo: flatpak, snap, etc

    let art = {
        if distro.contains("Ubuntu") {
            UBUNTU
        }
        else if distro.contains("Arch") {
            ARCH
        }
        else if distro.contains("Debian") {
            DEBIAN
        }
        else if distro.contains("Alpine") {
            ALPINE
        }
        else {
            TUX
        }
    };

    let output = " ".repeat(17) + &user + "@" + &host + "\n" +
                 art[0] + "Distro: " + &distro + "\n" +
                 art[1] + "Kernel: " + &kernel + "\n" +
                 art[2] + "Uptime: " + &uptime + "\n" +
                 art[3] + "Packages: " + &packages + "\n" +
                 art[4] + "Shell: " + &shell + "\n" +
                 art[5] + "Terminal: " + &terminal + "\n" +
                 art[6] + "CPU: " + &cpu + "\n" +
                 &" ".repeat(17) + "GPU: " + &gpu + "\n" +
                 &" ".repeat(17) + "Memory: " + &memory + "\n";
    print!("{}", output);
}

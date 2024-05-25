use std::io::{self, BufRead};
use std::ffi::CStr;

use libc::{utsname, c_char};

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

fn host () -> String {
    let mut utsname: utsname = unsafe { std::mem::zeroed() };
    unsafe {
        libc::uname(&mut utsname);
    }
    String::from_utf8_lossy(unsafe { CStr::from_ptr(utsname.nodename.as_ptr() as *const c_char).to_bytes() }).to_string()
}

fn distro () -> String {
    std::fs::read_to_string("/etc/os-release")
        .unwrap_or("".to_string())
        .lines()
        .find(|line| line.starts_with("PRETTY_NAME="))
        .map(|line| line.split('=').nth(1).unwrap_or("").to_string())
        .unwrap_or("Unknown".to_string())
}

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
fn fetch_packages_apt() -> Result<String, std::io::Error> {
    let buf = std::fs::read("/var/lib/dpkg/status")?;
    let mut packages = 0;
	let buf0 = &buf[..buf.len()-1];
	let buf1 = &buf[1..];
	let mut chunks0 = buf0.chunks_exact(512);
	let mut chunks1 = buf1.chunks_exact(512);

	while let (Some(chunk0), Some(chunk1)) = (chunks0.next(), chunks1.next()) {
		let mut counter = 0;
		for j in 0..512 {
			counter += (chunk0[j] == b'\n') as u8 & (chunk1[j] == b'I') as u8;
		}
		packages += counter as usize;
	}

	for (a,b) in chunks0.remainder().iter().zip(chunks1.remainder()) {
		packages += (*a == b'\n') as usize & (*b == b'I') as usize;
	}

    Ok(packages.to_string())
}

#[cfg(target_arch = "x86_64")]
fn cpuid() -> String {
    let a = unsafe{core::arch::x86_64::__cpuid(0x80000002)};
    let b = unsafe{core::arch::x86_64::__cpuid(0x80000003)};
    let c = unsafe{core::arch::x86_64::__cpuid(0x80000004)};
    let mut s = Vec::new();
    s.extend_from_slice(&a.eax.to_le_bytes());
    s.extend_from_slice(&a.ebx.to_le_bytes());
    s.extend_from_slice(&a.ecx.to_le_bytes());
    s.extend_from_slice(&a.edx.to_le_bytes());
    s.extend_from_slice(&b.eax.to_le_bytes());
    s.extend_from_slice(&b.ebx.to_le_bytes());
    s.extend_from_slice(&b.ecx.to_le_bytes());
    s.extend_from_slice(&b.edx.to_le_bytes());
    s.extend_from_slice(&c.eax.to_le_bytes());
    s.extend_from_slice(&c.ebx.to_le_bytes());
    s.extend_from_slice(&c.ecx.to_le_bytes());
    s.extend_from_slice(&c.edx.to_le_bytes());
    let s = String::from_utf8(s).unwrap();
    s.trim().to_string()
}
#[cfg(not(target_arch = "x86_64"))]
fn cpuid() -> String {
    nixinfo::cpu().unwrap_or("Unknown".to_string())
}

fn open_pci_ids() -> Option<std::fs::File> {
    [
        "/usr/share/hwdata/pci.ids",
        "/usr/share/misc/pci.ids",
        "/usr/share/pci.ids",
    ]
    .iter()
    .find_map(|path| std::fs::File::open(path).ok())
}

fn lines_find(
    reader: &mut io::BufReader<std::fs::File>,
    f: &dyn Fn(&Vec<u8>) -> bool,
) -> Result<Option<Vec<u8>>, io::Error> {
    let mut line = Vec::new();
    loop {
        line.clear();
        reader.read_until(b'\n', &mut line)?;
        if line.is_empty() {
            break;
        }
        if f(&line) {
            return Ok(Some(line));
        }
    }
    Ok(None)
}

pub fn gpu() -> Result<String, std::io::Error> {
    let mut vendor = std::fs::read("/sys/class/drm/card0/device/vendor")?;
    let mut device = std::fs::read("/sys/class/drm/card0/device/device")?;

    if let Some(file) = open_pci_ids() {
        let mut reader = io::BufReader::new(file);

        if let Some(line) = lines_find(&mut reader, &|line| line.starts_with(&vendor[2..6]))? {
            vendor = line[6..].to_vec();
        }

        if let Some(line) = lines_find(&mut reader, &|line| {
            !line[0] == b'\t' || device.get(2..6) == line.get(1..5)
        })? {
            if line[0] == b'\t' {
                device = line[7..].to_vec();
            }
        }
    }
    //remove newlines
    vendor.pop();
    device.pop();
    let mut result = vendor;
    result.push(b' ');
    result.append(&mut device);
    Ok(String::from_utf8_lossy(&result).to_string())
}

fn fetch_packages_pacman() -> Result<String, std::io::Error> {
    let count = std::fs::read_dir("/var/lib/pacman/local")?.into_iter().filter(|entry| entry.as_ref().unwrap().path().is_dir()).count();
    Ok(count.to_string())
}
fn fetch_packages_flatpak() -> Result<String, std::io::Error> {
    let mut count = std::fs::read_dir("/var/lib/flatpak/app")?.into_iter().filter(|entry| entry.as_ref().unwrap().path().is_dir()).count();
    count += std::fs::read_dir("/var/lib/flatpak/runtime")?.into_iter().filter(|entry| entry.as_ref().unwrap().path().is_dir()).count();
    Ok(count.to_string())
}

fn main() {
    let user = std::env::var("LOGNAME").unwrap_or("Unknown".to_string());
    let host = host();
    let distro = distro();
    let kernel = nixinfo::kernel().unwrap_or("Unknown".to_string());
    let uptime = nixinfo::uptime().unwrap_or("Unknown".to_string());
    let shell = std::env::var("SHELL").unwrap_or("Unknown".to_string());
    let terminal = nixinfo::terminal().unwrap_or("Unknown".to_string());
    let cpu = cpuid();
    let gpu = gpu().unwrap_or("Unknown".to_string());//nixinfo::gpu().unwrap_or("Unknown".to_string());
    let memory = nixinfo::memory().unwrap_or("Unknown".to_string());

    let mut packages = "".to_string();
    match fetch_packages_apt() {
        Ok(p) => {
            packages += &p;
            packages += " (apt) ";
        }
        Err(_) => {}
    }

    match fetch_packages_pacman() {
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

    match fetch_packages_flatpak() {
        Ok(p) => {
            packages += &p;
            packages += " (flatpak) ";
        }
        Err(_) => {}
    }

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

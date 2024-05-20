const TUX : [&str; 7] = [
    "\x1b[37m       ,--.      ",
    "\x1b[37m      |0\x1b[33m_\x1b[37m0 |     ",
    "\x1b[37m      |\x1b[33mL_/\x1b[37m |     ",
    "\x1b[37m     //   \\ \\    ",
    "\x1b[37m    ((     ) )   ",
    "\x1b[33m   /`\\     /`\\ \x1b[37m  ",
    "\x1b[33m   \\__)\x1b[37m===\x1b[33m(__/\x1b[37m   "
];
fn main() {
    let logname = std::env::var("LOGNAME").unwrap(); // user
    let host = nixinfo::hostname().unwrap();
    let distro = nixinfo::distro().unwrap();
    let kernel = nixinfo::kernel().unwrap();
    let uptime = nixinfo::uptime().unwrap();
    // todo: packages
    let shell = std::env::var("SHELL").unwrap();
    let terminal = nixinfo::terminal().unwrap();
    let cpu = nixinfo::cpu().unwrap();
    let gpu = nixinfo::gpu().unwrap();
    let memory = nixinfo::memory().unwrap();

    //construct string with 1 tux line, then pad, then the info
    let output = " ".repeat(17) + &logname + "@" + &host + "\n" +
                 TUX[0] + "Distro: " + &distro + "\n" +
                 TUX[1] + "Kernel: " + &kernel + "\n" +
                 TUX[2] + "Uptime: " + &uptime + "\n" +
                 TUX[3] + "Shell: " + &shell + "\n" +
                 TUX[4] + "Terminal: " + &terminal + "\n" +
                 TUX[5] + "CPU: " + &cpu + "\n" +
                 TUX[6] + "GPU: " + &gpu + "\n" +
                 &" ".repeat(17) + "Memory: " + &memory + "\n";
    print!("{}", output);
}

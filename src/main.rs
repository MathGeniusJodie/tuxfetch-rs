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
    let user = std::env::var("LOGNAME").unwrap_or("Unknown".to_string());
    let host = nixinfo::hostname().unwrap_or("Unknown".to_string());
    let distro = nixinfo::distro().unwrap_or("Unknown".to_string());
    let kernel = nixinfo::kernel().unwrap_or("Unknown".to_string());
    let uptime = nixinfo::uptime().unwrap_or("Unknown".to_string());
    let packages = nixinfo::packages("apt").unwrap_or(nixinfo::packages("pacman").unwrap_or(nixinfo::packages("dnf").unwrap_or(nixinfo::packages("apk").unwrap_or("Unknown".to_string()))));
    let shell = std::env::var("SHELL").unwrap_or("Unknown".to_string());
    let terminal = nixinfo::terminal().unwrap_or("Unknown".to_string());
    let cpu = nixinfo::cpu().unwrap_or("Unknown".to_string());
    let gpu = nixinfo::gpu().unwrap_or("Unknown".to_string());
    let memory = nixinfo::memory().unwrap_or("Unknown".to_string());

    //construct string with 1 tux line, then pad, then the info
    let output = " ".repeat(17) + &user + "@" + &host + "\n" +
                 TUX[0] + "Distro: " + &distro + "\n" +
                 TUX[1] + "Kernel: " + &kernel + "\n" +
                 TUX[2] + "Uptime: " + &uptime + "\n" +
                 TUX[3] + "Packages: " + &packages + "\n" +
                 TUX[4] + "Shell: " + &shell + "\n" +
                 TUX[5] + "Terminal: " + &terminal + "\n" +
                 TUX[6] + "CPU: " + &cpu + "\n" +
                 &" ".repeat(17) + "GPU: " + &gpu + "\n" +
                 &" ".repeat(17) + "Memory: " + &memory + "\n";
    print!("{}", output);
}

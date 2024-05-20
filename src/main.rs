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
    "\x1b[31m   .8BBBBBBBBB.   ",
    "\x1b[33m  .888888888888.  ",
    "\x1b[32m .8888* *88*8888. ",
    "\x1b[36m 888* ,8, `. *888 ",
    "\x1b[36m `* ,88888, `, *` ",
    "\x1b[34m  `888888888888`  ",
    "\x1b[35m   `8BBBBBBBBB`   "
];

const ARCH : [&str; 7] = [
    "\x1b[31m        A         ",
    "\x1b[31m       a8s        ",
    "\x1b[33m      ao88s       ",
    "\x1b[32m     a88888s      ",
    "\x1b[36m    a88Y*Y88s     ",
    "\x1b[34m   a88H   B8bs    ",
    "\x1b[35m  /*`       `*\\   "
];

const DEBIAN : [&str; 7] = [
    "\x1b[31m    ,gA88bq.   ",
    "\x1b[33m   dP      `9. ",
    "\x1b[32m  d7  ,*`.  )8 ",
    "\x1b[36m  9:  A    ,Q* ",
    "\x1b[34m  *1  `^vsv\"   ",
    "\x1b[35m   *b          ",
    "\x1b[35m     \"~.       "
];

const MINT : [&str; 7] = [
    "\x1b[32m BBBBBBBBBBBs~.  ",
    "\x1b[32m BB8  88**88**8, ",
    "\x1b[32m   H  H  a  a  l ",
    "\x1b[32m   H  H  H  H  H ",
    "\x1b[32m   V, *88888* .H ",
    "\x1b[32m   `8,_______.=H ",
    "\x1b[32m     *YBBBBBBBBH ",
];
const UBUNTU : [&str; 7] = [
    "\x1b[31m          \x1b[35m($)",
    "\x1b[31m    \x1b[33m .\x1b[31m s88~..",
    "\x1b[31m    \x1b[33m.8*  \x1b[31m `*9.",
    "\x1b[31m \x1b[31m($) \x1b[33m8",
    "\x1b[31m    \x1b[33m`6-  \x1b[35m _-8`",
    "\x1b[31m     \x1b[33m`\x1b[35m ^88*``",
    "\x1b[31m          \x1b[33m($) "
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

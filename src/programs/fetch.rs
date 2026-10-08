use crate::{arch::cpu, drivers::vga::writer, format, print, std::string::String};
use core::fmt::Write;

pub fn fetch() {
    print!(
        r#"
    ____  _____       ____  ____  _____ user@RS-DOS
   / __ \/ ___/      / __ \/ __ \/ ___/ -----------
  / /_/ /\__ \______/ / / / / / /\__ \  OS: RS-DOS
 / _, _/___/ /_____/ /_/ / /_/ /___/ /  Arch: i686
/_/ |_|/____/     /_____/\____//___/    Vendor: {}
                                        
"#,
        cpu_vendor()
    );
}

fn cpu_vendor() -> String {
    let bytes = cpu::vendor();
    let mut vendor = String::new();

    for part in bytes {
        for byte in part {
            vendor.push(byte);
        }
    }

    vendor
}

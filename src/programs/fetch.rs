use crate::{drivers::vga::writer, print};
use core::fmt::Write;

pub fn fetch() {
    print!(
        r#"
    ____  _____       ____  ____  _____ user@RS-DOS
   / __ \/ ___/      / __ \/ __ \/ ___/ -----------
  / /_/ /\__ \______/ / / / / / /\__ \  OS: RS-DOS
 / _, _/___/ /_____/ /_/ / /_/ /___/ /  Arch: i686
/_/ |_|/____/     /_____/\____//___/    Resoloution: 80x25
                                        
"#
    );
}

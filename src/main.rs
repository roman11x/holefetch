use crate::info::SystemInfo;

mod info;
mod desktop;

fn main() {
    let info = SystemInfo::new();
    info.display();

}



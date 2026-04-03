use crate::info::SystemInfo;

mod info;

fn main() {
    let info = SystemInfo::new();
    info.display();

}



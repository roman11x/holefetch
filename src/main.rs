mod info;

fn main() {
    println!("{}", info::read_os());
    println!("{}", info::read_kernel());
    println!("{}", info::read_uptime());
    println!("{}", info::read_cpu());
    println!("{}", info::read_memory());
    println!("Shell: {}", info::read_shell());
    println!("DE: {}", info::read_desktop_environment());
}



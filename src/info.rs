use std::fs;

#[derive(Debug)]
 pub struct SystemInfo {
    os: String,
    kernel: String,
    uptime: String,
    cpu: String,
    memory: String,
    shell: String,
    desktop_environment: String,
}
impl SystemInfo {
   pub fn new() -> SystemInfo {
        SystemInfo {
            os: read_os(),
            kernel: read_kernel(),
            uptime: read_uptime(),
            cpu: read_cpu(),
            memory: read_memory(),
            shell: read_shell(),
            desktop_environment: read_desktop_environment(),
        }
    }
    pub fn display(&self) {
        println!("OS: {}", self.os);
        println!("Kernel: {}", self.kernel);
        println!("Uptime: {}", self.uptime);
        println!("CPU: {}", self.cpu);
        println!("Memory: {}", self.memory);
        println!("Shell: {}", self.shell);
        println!("Desktop Environment: {}", self.desktop_environment);
    }
}
// Returns the pretty name of the OS
pub fn read_os() -> String {
    let contents = fs::read_to_string("/etc/os-release").unwrap_or_else(|_| "Unknown".to_string());

    let os_name = contents.lines().find(|line| line.starts_with("PRETTY_NAME="))
        .unwrap_or(&"Unknown").split("=").nth(1).unwrap_or(&"Unknown");

    os_name.trim_matches('"').to_string()
}
// Returns the kernel version
pub fn read_kernel() -> String {
    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| "Unknown".to_string());
    format!("Linux {}", kernel.trim())
}
// Returns the uptime of the system
//as the uptime in /proc/uptime is in seconds, we convert it to hours and minutes like in Fastfetch
pub fn read_uptime() -> String {
    let uptime = fs::read_to_string("/proc/uptime")
        .unwrap_or_else(|_| "0.0 hours, 0.0 mins".to_string());

    let seconds = uptime.trim().split_whitespace().next().unwrap_or(&"0.0").parse::<f64>().unwrap_or(0.0);

    let minutes = seconds as u64 / 60;

    let hours = minutes / 60;
    format!("{} hours, {} mins",  hours, minutes % 60)
}
// Returns the CPU model and number of cores as well as the CPU speed
pub fn read_cpu() -> String {
    let cpu = fs::read_to_string("/proc/cpuinfo")
        .unwrap_or_else(|_| "Unknown".to_string());

    let cpu_model = cpu.lines().find(|line| line.starts_with("model name"))
        .unwrap_or(&"Unknown").split(":").nth(1).unwrap_or(&"Unknown").trim();

    let num_cores = cpu.lines().filter(|line| line.starts_with("processor")).count();

    let ghz = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        .unwrap_or_else(|_| "Unknown".to_string()).trim().parse::<f64>().unwrap_or(0.0) / 1000000.0;
    format!("{} ({}) @ {:.2} GHz", cpu_model, num_cores, ghz)
}
// Returns the total and used memory as well as the percentage of used memory
pub fn read_memory() -> String {
    let mem = fs::read_to_string("/proc/meminfo")
        .unwrap_or_else(|_| "Unknown".to_string());
// helper closure to parse the memory info
    let parse_label = |label: &str| -> f64 {
        let value = mem.lines().find(|line| line.starts_with(label))
            .unwrap_or(&"Unknown").split_whitespace().nth(1).unwrap_or(&"Unknown").trim() //get the numeric value
            .parse::<f64>().unwrap_or(0.0);
        value
    };
    let total = parse_label("MemTotal:") /1024.0 /1024.0;
    let used = total - parse_label("MemAvailable:") /1024.0 /1024.0;
    let percentage = (used / total * 100.0) as u64;
    format!("{:.2} GiB / {:.2} GiB ({}%)", used, total, percentage)

}
// Returns the shell used
pub fn read_shell() -> String {
      std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string())
        .split("/").last().unwrap_or(&"Unknown").to_string()
}
// Returns the desktop environment used
pub fn read_desktop_environment() -> String {
      std::env::var("DESKTOP_SESSION").unwrap_or_else(|_| "Unknown".to_string())

}
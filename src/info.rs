use std::fs;
use std::io::BufRead;
use std::process::Command;
use sysinfo::Disks;

#[derive(Debug)]
 pub struct SystemInfo {
    os: String,
    kernel: String,
    uptime: String,
    cpu: String,
    memory: String,
    shell: String,
    desktop_environment: String,
    disk: String,
    gpu: String,
    terminal: String,
    locale: String,
    packages: String,
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
            disk: read_disk(),
            gpu: read_gpu(),
            terminal: read_terminal(),
            locale: read_locale(),
            packages: read_packages(),
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
        println!("Disk: {}", self.disk);
        println!("GPU: {}", self.gpu);
        println!("Terminal: {}", self.terminal);
        println!("Locale: {}", self.locale);
        println!("Packages: {}", self.packages);
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
// Returns the desktop environment used and the display server
pub fn read_desktop_environment() -> String {
    // This project focuses on traditional desktop environments,
    // so the only possible display servers are Wayland and X11

    let desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_|std::env::var("DESKTOP_SESSION") //if it's not set, check for DESKTOP_SESSION
            .unwrap_or_else(|_| "Unknown".to_string()));

    let display = std::env::var("WAYLAND_DISPLAY")
        .map(|_| "Wayland").unwrap_or_else(|_| "X11"); //if it's not set, it's X11

    format!("{} ({})", desktop, display)
}
// Returns the disk usage of the root partition
pub fn read_disk() -> String {
    //get the list of disks
    let binding = Disks::new_with_refreshed_list();
    //find the root partition in the disks list
      binding.into_iter()
        .find(|disk| disk.mount_point() == std::path::Path::new("/"))
        .map(|disk|{ //if the root partition is found, calculate the disk usage and return it formatted
            let total = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;

            let available = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;

            let used = total - available;

            let file_system = disk.file_system().to_string_lossy();
            format!("{:.2} GiB / {:.2} GiB ({}%) - {}", used, total, (used / total * 100.0) as u64, file_system)

    }).unwrap_or_else(||"Unknown".to_string()) //if the root partition is not found, return "Unknown"

}
// returns the GPU model and whether it's integrated or discrete
pub fn read_gpu() -> String{
    let mut result: Vec<String> = Vec::new();

    let output = std::process::Command::new("lspci") //get the list of PCI devices
        .output().map_err(|_| "Unknown".to_string()).unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout); //convert the output to a string

    // helper closure to format the GPU name according to the vendor
    let clean_gpu_name = |name: &str| -> String {
        match name {
            n if n.contains("NVIDIA Corporation") => n.replace("NVIDIA Corporation", "NVIDIA").to_string(),
            n if n.contains("Advanced Micro Devices, Inc. [AMD/ATI]") => n.replace("Advanced Micro Devices, Inc. [AMD/ATI]", "AMD").to_string(),
            n if n.contains("Intel Corporation") => n.replace("Intel Corporation", "Intel").to_string(),
            _ => "Unknown".to_string()
        }
    };
    // helper closure to determine if the GPU is integrated or discrete
    let integrated_or_discrete = |name : &str| -> &str {
        match name {
            n if n.contains("VGA compatible controller") => "[Integrated]",
            n if n.contains("Discrete Graphics Controller") => "[Discrete]",
            _ => ""
        }
    };

    for line in  stdout.lines().filter(|line| line.contains("VGA") || line.contains("3D controller")) { //find the line containing "VGA" or "3D controller"
            let tag = integrated_or_discrete(line);

           if let Option::Some((_, model_info)) = line.split_once(": ") { //skip the 0x:00.0 header using the space, ignore the left side

              let raw_name =  if let Option::Some((clean_name, _)) = model_info.split_once(" (rev") { //remove the revision number if it exists, ignore the right side
                   clean_name.trim()
               }
               else {
                   model_info.trim()
               };
               let vendor_cleaned = clean_gpu_name(raw_name);
               result.push(format!("{} {}", vendor_cleaned, tag));

           }
    }

    result.join("| ")

}
// Returns the name of the terminal used
pub fn read_terminal() -> String {
    // helper closure to find the PIDs of the shell and terminal
    let pid_finder = |path: &str| -> Option<String> {
        let path = format!("/proc/{}/status", path);
        fs::read_to_string(path).ok()?
            .lines().find(|line| line.starts_with("PPid:"))?
            .split_once(":")?.1.trim().to_string().into()
    };

    let capitalize_first_letter = |s: &str| -> String {
        let mut c = s.chars();
        match c.next() {
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            None => "".to_string()
        }
    };

    let shell_pid = pid_finder("self").unwrap_or_else(|| "Unknown".to_string());


    let terminal_pid = pid_finder(&shell_pid).unwrap_or_else(|| "Unknown".to_string());


    // We check for 0 as well because it's not a user space program.
    if terminal_pid == "Unknown" || terminal_pid == "0" {
        return "Unknown".to_string();
    }

    let comm_path = format!("/proc/{}/comm", terminal_pid);

    let comm = fs::read_to_string(comm_path).unwrap_or_else(|_| "Unknown".to_string());

    let trimmed_comm = comm.trim();
    // Some terminals have -agent or -gui in their name
    let final_comm = match trimmed_comm.split_once("-agent") {
        Some((name, _)) => name,
        None => {
            match trimmed_comm.split_once("-gui") {
                Some((name, _)) => name,
                _ => trimmed_comm
            }
        }
    };

   capitalize_first_letter(final_comm)
}
// Returns the locale of the system
pub fn read_locale() -> String {
    std::env::var("LANG").unwrap_or_else(|_| "Unknown".to_string())
}
// Returns the number of installed packages (both native and non-native) as well as the package manager used
pub fn read_packages() -> String {
    // commands to get the number of packages using the native package manager
    let commands = vec![("pacman", "-Qq"), ("dpkg", "--get-selections"), ("rpm", "-qa")];

    // variables to store the number of packages and the package manager used
    let mut package_manager = "";
    let mut flatpak_user_packages = 0;
    let mut flatpak_system_packages = 0;
    let mut snap_packages = 0;

    // helper closure to count the number of native packages depending on the package manager used
    let mut native_packages = || -> u64 {
        for (command, arg) in &commands {
            if let Ok(output) = Command::new(command).arg(arg).output() {
                package_manager = command;
                return output.stdout.lines().count() as u64;
            }
            else {
                continue;
            }
        }
        0
    };

    let native_packages_count = native_packages();

    if let Ok(output) = Command::new("flatpak").arg("list").arg("--system").output() {
        flatpak_system_packages = output.stdout.lines().count() as u64;
    }
    if let Ok(output) = Command::new("flatpak").arg("list").arg("--user").output() {
        flatpak_user_packages = output.stdout.lines().count() as u64;
    }
    if let Ok(output) = Command::new("snap").arg("list").output() {
        let count = output.stdout.lines().count() as u64;
        snap_packages = if count > 0 {
            count - 1
        }
        else {
            0
        };
    }

    let mut result = Vec::new();
    if native_packages_count > 0 {
        result.push(format!("{} ({})", native_packages_count, package_manager));
    }
    if flatpak_system_packages > 0 {
        result.push(format!("{} ({})", flatpak_system_packages, "flatpak-system"));
    }
    if flatpak_user_packages > 0 {
        result.push(format!("{} ({})", flatpak_user_packages, "flatpak-user"));
    }
    if snap_packages > 0 {
        result.push(format!("{} ({})", snap_packages, "snap"));
    }
    result.join(", ")

}
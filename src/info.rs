use std::fs;
use std::io::BufRead;
use std::process::Command;
use sysinfo::Disks;
use crate::desktop::DesktopEnvironment;

// Group all the relevant fields of the system into a struct
#[derive(Debug)]
 pub struct SystemInfo {
    user: String,
    hostname: String,
    os: String,
    kernel: String,
    uptime: String,
    cpu: String,
    memory: String,
    shell: String,
    desktop_environment: DesktopEnvironment,
    disk: String,
    gpu: String,
    terminal: String,
    locale: String,
    packages: String,
    host: String,
    swap: String,
    battery: String,
    ip: String,
}
impl SystemInfo {
   pub fn new() -> SystemInfo {
        SystemInfo {
            user: Self::read_user(),
            hostname: Self::read_hostname(),
            os: Self::read_os(),
            kernel: Self::read_kernel(),
            uptime: Self::read_uptime(),
            cpu: Self::read_cpu(),
            memory: Self::read_memory(),
            shell: Self::read_shell(),
            desktop_environment: DesktopEnvironment::new(),
            disk: Self::read_disk(),
            gpu: Self::read_gpu(),
            terminal: Self::read_terminal(),
            locale: Self::read_locale(),
            packages: Self::read_packages(),
            host: Self::read_host(),
            swap: Self::read_swap(),
            battery: Self::read_battery(),
            ip: Self::read_ip(),
        }
    }
    

    pub fn to_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("{}@{}", self.user, self.hostname));
        lines.push("-----------".to_string());
        lines.push(format!("OS: {}", self.os));
        lines.push(format!("Host: {}", self.host));
        lines.push(format!("Kernel: {}", self.kernel));
        lines.push(format!("Uptime: {}", self.uptime));
        lines.push(format!("CPU: {}", self.cpu));
        lines.push(format!("Memory: {}", self.memory));
        lines.push(format!("Swap: {}", self.swap));
        lines.push(format!("Shell: {}", self.shell));
        lines.extend(self.desktop_environment.to_lines());
        lines.push(format!("Disk: {}", self.disk));
        lines.push(format!("GPU: {}", self.gpu));
        lines.push(format!("Terminal: {}", self.terminal));
        lines.push(format!("Battery {}", self.battery));
        lines.push(format!("Locale: {}", self.locale));
        lines.push(format!("Packages: {}", self.packages));
        lines.push(format!("Local IP {}", self.ip));
        lines
    }

    // Returns the pretty name of the OS
    pub fn read_os() -> String {
        let contents = fs::read_to_string("/etc/os-release").unwrap_or_else(|_| "Unknown".to_string());

        let os_name = contents.lines().find(|line| line.starts_with("PRETTY_NAME="))
            .unwrap_or(&"Unknown").split("=").nth(1).unwrap_or(&"Unknown");

        os_name.trim_matches('"').to_string()
    }
    // returns the ID of the OS
    pub fn read_os_id() -> String {
        let contents = fs::read_to_string("/etc/os-release").unwrap_or_else(|_| "Unknown".to_string());
        let os_id = contents.lines().find(|line| line.starts_with("ID="))
            .unwrap_or(&"Unknown").split("=").nth(1).unwrap_or(&"Unknown");
        os_id.trim().to_string()
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
    // Returns the product name and product family of the host
    pub fn read_host() -> String {
        let product_name = fs::read_to_string("/sys/class/dmi/id/product_name")
            .unwrap_or_else(|_| "Unknown".to_string());
        let product_family = fs::read_to_string("/sys/class/dmi/id/product_family")
            .unwrap_or_else(|_| "Unknown".to_string());
        format!("{} ({})", product_name.trim(), product_family.trim())
    }
    // Returns the total and used swap space as well as the percentage of used swap space
    pub fn read_swap() -> String {
        let mem = fs::read_to_string("/proc/meminfo")
            .unwrap_or_else(|_| "Unknown".to_string());
        // helper closure to parse the memory info
        let parse_label = |label: &str| -> f64 {
            let value = mem.lines().find(|line| line.starts_with(label))
                .unwrap_or(&"Unknown").split_whitespace().nth(1).unwrap_or(&"Unknown").trim() //get the numeric value
                .parse::<f64>().unwrap_or(0.0);
            value
        };
        let total = parse_label("SwapTotal:") /1024.0 /1024.0;
        if total == 0.0 {
            return "Disabled".to_string();
        }
        let used = total - parse_label("SwapFree:") /1024.0 /1024.0;
        let percentage = (used / total * 100.0) as u64;
        format!("{:.2} GiB / {:.2} GiB ({}%)", used, total, percentage)
    }
    // Returns the model name and the percentage of battery charge as well as the status of the battery
    pub fn read_battery() -> String {
        let mut result = Vec::new();
        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() { // get the list of power supplies, skip failed entries
                let base = entry.path();
                let type_path = base.join("type");
                if fs::read_to_string(type_path).unwrap_or_default().contains("Battery") { // if the entry is a battery, get the model name, capacity and status
                    let model_path = base.join("model_name");
                    let capacity_path = base.join("capacity");
                    let status_path = base.join("status");
                    let model_name = fs::read_to_string(model_path).unwrap_or_else(|_| "Unknown".to_string());
                    let capacity = fs::read_to_string(capacity_path).unwrap_or_else(|_| "Unknown".to_string()).trim().parse::<u64>().unwrap_or(0);
                    let status = fs::read_to_string(status_path).unwrap_or_else(|_| "Unknown".to_string()).trim().to_string();
                    result.push(format!("({}): {}% [{}]", model_name.trim(), capacity, status.trim()));
                }
            }
        }
        if result.is_empty() {
            return "No battery".to_string();
        }
        result.join(", ")
    }
    // returns the user's ip and network interface
    pub fn read_ip() -> String {
        let mut ip = "offline".to_string();
        let mut network_interface = "none".to_string();

        if let Ok(output) = Command::new("ip").args(["route", "get", "1"]).output(){ //get the IP address of the default route
            let stdout = String::from_utf8_lossy(&output.stdout); //convert the output to a string

            for line in stdout.lines() {
                if let Some((interface, ip_address)) = line.split_once(" src ") {
                    ip = ip_address.split_whitespace().next().unwrap_or("offline").to_string();
                    network_interface = interface.split_whitespace().last().unwrap_or("none").to_string();
                    break;
                }
            }
        }
        format!("({}): {}", network_interface, ip)
    }


    pub fn read_user() -> String {
        std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string()).trim().to_string()
    }

    pub fn read_hostname() -> String {
        std::fs::read_to_string("/etc/hostname").unwrap_or_else(|_| "Unknown".to_string()).trim().to_string()
    }


}














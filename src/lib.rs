use ipnetwork::IpNetwork;
use sysinfo::{CpuExt, DiskExt, ProcessExt, System, SystemExt};

pub struct Device {
    systeminfo: SystemInfo,
    cpus: Vec<Cpu>,
    disks: Vec<Disk>,
    interfaces: Vec<Interface>,
    sys: System,
}

#[derive(Debug, Clone)]
struct SystemInfo {
    os_version: String,
    kernel_version: String,
    hostname: String,
    cores_nb: usize,
    total_ram: u64,
    free_ram: u64,
    used_ram: u64,
    total_swap: u64,
    free_swap: u64,
    used_swap: u64,
}

#[derive(Debug, Clone)]
pub struct Cpu {
    name: String,
    usage: f32,
    vendor_id: String,
    brand: String,
}

#[derive(Debug, Clone)]
pub struct Disk {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
}

#[derive(Clone, Debug)]
pub struct Interface {
    name: String,
    mac: String,
    ip: String,
    prefix: u8,
    version: u8,
}

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation for the struct Device.
/// This one is used to get system and network info
/// about the device.
impl Device {
    pub fn new() -> Self {
        // get all sys info of device
        let mut sys = System::new_all();

        // update info of sys struct
        sys.refresh_all();

        let os_version = format!("{} {}", sys.name().unwrap(), sys.os_version().unwrap());

        let systeminfo = SystemInfo {
            hostname: sys.host_name().unwrap(),
            os_version,
            kernel_version: sys.kernel_version().unwrap(),
            cores_nb: sys.cpus().len(),
            total_ram: sys.total_memory(),
            free_ram: sys.free_memory(),
            used_ram: sys.used_memory(),
            total_swap: sys.total_swap(),
            free_swap: sys.free_swap(),
            used_swap: sys.used_swap(),
        };

        // Get all disks and put them in a Vec of Disk structs
        let mut disks: Vec<Disk> = Vec::new();
        for disk in sys.disks() {
            let disk_struct = Disk {
                name: disk.name().to_str().unwrap().to_string(),
                mount_point: disk.mount_point().to_str().unwrap().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
            };

            disks.push(disk_struct);
        }

        // Get cpu info an put it in a Vec of Cpu struct
        let mut cpus: Vec<Cpu> = Vec::new();
        for cpu in sys.cpus() {
            let cpu_struct = Cpu {
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
                vendor_id: cpu.vendor_id().to_string(),
                brand: cpu.brand().to_string(),
            };

            cpus.push(cpu_struct);
        }

        // Get all network interfaces
        let mut interfaces: Vec<Interface> = Vec::new();
        for iface in pnet_datalink::interfaces() {
            let mut interface_struct: Interface;

            for ip in iface.ips {
                match ip {
                    IpNetwork::V4(ipv4) => {
                        interface_struct = Interface {
                            name: iface.name.clone(),
                            mac: iface.mac.unwrap().to_string().clone(),
                            ip: ipv4.ip().to_string(),
                            prefix: ipv4.prefix(),
                            version: 4,
                        };
                    }
                    IpNetwork::V6(ipv6) => {
                        interface_struct = Interface {
                            name: iface.name.clone(),
                            mac: iface.mac.unwrap().to_string().clone(),
                            ip: ipv6.ip().to_string(),
                            prefix: ipv6.prefix(),
                            version: 6,
                        };
                    }
                }
                interfaces.push(interface_struct);
            }
        }

        Self {
            systeminfo,
            cpus,
            disks,
            interfaces,
            sys,
        }
    }

    /// Refresh all from system
    pub fn refresh_all(&mut self) {
        self.sys.refresh_all();
    }

    /// Get CPUs
    pub fn get_cpus(&mut self) -> Vec<Cpu> {
        self.cpus.clone()
    }

    /// Get disks
    pub fn get_disks(&mut self) -> Vec<Disk> {
        self.disks.clone()
    }

    /// Get network interfaces
    pub fn get_interfaces(&mut self) -> Vec<Interface> {
        self.interfaces.clone()
    }

    /// Print system information
    pub fn print_system_info(&mut self, do_tab: bool) {
        let tab: String = if do_tab {
            "   ".to_string()
        } else {
            "".to_string()
        };

        println!("{tab}Hosname: {}", self.systeminfo.hostname);
        println!("{tab}OS : {}", self.systeminfo.os_version);
        println!("{tab}kernel : {}", self.systeminfo.kernel_version);
        println!("{tab}Cores : {}", self.systeminfo.cores_nb);
        println!("{tab}RAM");
        println!("   {tab}Total RAM: {}MB", self.systeminfo.total_ram / 1000);
        println!("   {tab}Free  RAM: {}MB", self.systeminfo.free_ram / 1000);
        println!("   {tab}Used  RAM: {}MB", self.systeminfo.used_ram / 1000);
        println!("{tab}Swap");
        println!(
            "   {tab}Total Swap: {}MB",
            self.systeminfo.total_swap / 1000
        );
        println!("   {tab}Free  Swap: {}MB", self.systeminfo.free_swap / 1000);
        println!("   {tab}Used  Swap: {}MB", self.systeminfo.used_swap / 1000);
    }

    /// Print network info
    pub fn print_network_info(&mut self) {
        for iface in self.get_interfaces() {
            if iface.version != 6 {
                println!("{}", iface.name);
            } else {
                println!();
            }
            println!("   {}/{}", iface.ip, iface.prefix);
            println!("   {}", iface.mac);
        }
    }

    /// Print CPUs info
    pub fn print_cpus_info(&mut self) {
        for cpu in self.get_cpus() {
            println!("{}", cpu.name);
            println!("   Usage: {}%", f32::trunc(cpu.usage * 100.0) / 100.0);
            println!("   Vendor: {}", cpu.vendor_id);
            println!("   Core: {}", cpu.brand);
        }
    }

    /// Print disks info
    pub fn print_disks_info(&mut self) {
        for disk in self.get_disks() {
            println!("{}", disk.name);
            println!("   Mount point : {}", disk.mount_point);
            println!(
                "   Available space : {}MB",
                (disk.available_space / (1024 * 1024))
            );
            println!("   Total space : {}MB", (disk.total_space / (1024 * 1024)));
        }
    }

    /// Print all process info I need, not in a struct yet, because la flemme
    /// TODO : order by pid
    pub fn print_processes(&mut self) {
        for (pid, process) in self.sys.processes() {
            println!("{} - {}", pid, process.name());
            println!("   CPU usage: {}%", process.cpu_usage());
            println!("   Mem usage: {}MB", process.virtual_memory() / 1000);
            println!("   Status: {}\n", process.status());
        }
    }

    /// Print all info about a device
    pub fn print_all_info(&mut self) {
        println!("System:");
        self.print_system_info(true);

        println!("\nNetwork interfaces:");
        self.print_network_info();

        println!("\nCPUs:");
        self.print_cpus_info();

        println!("\ndisks:");
        self.print_disks_info();
    }
}

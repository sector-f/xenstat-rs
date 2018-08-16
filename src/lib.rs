extern crate xenstat_sys as sys;
use std::ops::Drop;
use std::ffi::CStr;

// 0x1 | 0x2 | 0x4 | 0x8
const NODE_FLAGS: u32 = sys::XENSTAT_VCPU | sys::XENSTAT_NETWORK | sys::XENSTAT_XEN_VERSION | sys::XENSTAT_VBD;

pub struct Xen {
    handle: *mut sys::xenstat_handle,
    node: *mut sys::xenstat_node,
}

impl Xen {
    /// Get a handle to the xenstat library. Returns None if an error occurs
    pub fn new() -> Option<Self> {
        unsafe {
            let handle = sys::xenstat_init();

            if handle.is_null() {
                return None;
            }

            let node = sys::xenstat_get_node(handle, NODE_FLAGS);

            Some(
                Xen {
                    handle: handle,
                    node: node,
                }
            )
        }
    }

    /// Get Domain with the given domain ID
    pub fn domain(&self, domid: u32) -> Domain {
        unsafe {
            Domain {
                ptr: sys::xenstat_node_domain(self.node, domid),
            }
        }
    }

    /// Get Domain with given index; used to loop over all domains
    pub fn domain_by_index(&self, index: u32) -> Domain {
        unsafe {
            Domain {
                ptr: sys::xenstat_node_domain_by_index(self.node, index),
            }
        }
    }

    /// Get Xen version
    pub fn xen_version(&self) -> String {
        unsafe {
            String::from_utf8_lossy(CStr::from_ptr(sys::xenstat_node_xen_version(self.node)).to_bytes()).into_owned()
        }
    }

    /// Get amount of total memory
    pub fn total_memory(&self) -> u64 {
        unsafe {
            sys::xenstat_node_tot_mem(self.node)
        }
    }

    /// Get amount of free memory
    pub fn free_memory(&self) -> u64 {
        unsafe {
            sys::xenstat_node_free_mem(self.node)
        }
    }

    /// Get amount of tmem freeable memory (in MiB)
    pub fn freeable_memory(&self) -> i64 {
        unsafe {
            sys::xenstat_node_freeable_mb(self.node)
        }
    }

    /// Get number of existing domains
    pub fn num_domains(&self) -> u32 {
        unsafe {
            sys::xenstat_node_num_domains(self.node)
        }
    }

    /// Get number of CPUs
    pub fn num_cpus(&self) -> u32 {
        unsafe {
            sys::xenstat_node_num_cpus(self.node)
        }
    }

    /// Get CPU speed
    pub fn cpu_hz(&self) -> u64 {
        unsafe {
            sys::xenstat_node_cpu_hz(self.node)
        }
    }
}

impl Drop for Xen {
    fn drop(&mut self) {
        unsafe {
            sys::xenstat_free_node(self.node);
            sys::xenstat_uninit(self.handle);
        }
    }
}

pub struct Domain {
    ptr: *mut sys::xenstat_domain,
}

impl Domain {
    /// Get the domain ID
    pub fn id(&self) -> u32 {
        unsafe {
            sys::xenstat_domain_id(self.ptr)
        }
    }

    /// Get the domain name
    pub fn name(&self) -> String {
        unsafe {
            String::from_utf8_lossy(CStr::from_ptr(sys::xenstat_domain_name(self.ptr)).to_bytes()).into_owned()
        }
    }

    /// Get information about how much CPU time has been used
    pub fn cpu_ns(&self) -> u64 {
        unsafe {
            sys::xenstat_domain_cpu_ns(self.ptr)
        }
    }

    /// Get number of VCPUs allocated to the domain
    pub fn num_vcpus(&self) -> u32 {
        unsafe {
            sys::xenstat_domain_num_vcpus(self.ptr)
        }
    }

    /// Get handle to obtain VCPU stats
    pub fn vcpu(&self, cpu: u32) -> Vcpu {
        unsafe {
            Vcpu {
                ptr: sys::xenstat_domain_vcpu(self.ptr, cpu)
            }
        }
    }

    /// Get current memory reservation for this domain
    pub fn cur_mem(&self) -> u64 {
        unsafe {
            sys::xenstat_domain_cur_mem(self.ptr)
        }
    }

    /// Get maximum memory reservation for this domain
    pub fn max_mem(&self) -> u64 {
        unsafe {
            sys::xenstat_domain_max_mem(self.ptr)
        }
    }

    /// Get the domain's SSID
    pub fn ssid(&self) -> u32 {
        unsafe {
            sys::xenstat_domain_ssid(self.ptr)
        }
    }

    /// Get the domain state
    pub fn state(&self) -> DomainState {
        unsafe {
            DomainState {
                running: sys::xenstat_domain_running(self.ptr) == 0,
                blocked: sys::xenstat_domain_blocked(self.ptr) == 0,
                paused: sys::xenstat_domain_paused(self.ptr) == 0,
                shutdown: sys::xenstat_domain_shutdown(self.ptr) == 0,
                crashed: sys::xenstat_domain_crashed(self.ptr) == 0,
                dying: sys::xenstat_domain_dying(self.ptr) == 0,
            }
        }
    }

    /// Get the number of networks
    pub fn num_networks(&self) -> u32 {
        unsafe {
            sys::xenstat_domain_num_networks(self.ptr)
        }
    }

    /// Get the network handle for a given network
    pub fn network(&self, network: u32) -> Network {
        unsafe {
            Network {
                ptr: sys::xenstat_domain_network(self.ptr, network),
            }
        }
    }

    /// Get the number of VBDs
    pub fn num_vbds(&self) -> u32 {
        unsafe {
            sys::xenstat_domain_num_vbds(self.ptr)
        }
    }

    /// Get the VBD handle to obtain VBD stats
    pub fn vbd(&self, vbd: u32) -> Vbd {
        unsafe {
            Vbd {
                ptr: sys::xenstat_domain_vbd(self.ptr, vbd),
            }
        }
    }

    /// Get the tmem information for a given domain
    pub fn tmem(&self) -> Tmem {
        unsafe {
            Tmem {
                ptr: sys::xenstat_domain_tmem(self.ptr),
            }
        }
    }
}

pub struct DomainState {
    pub running: bool,
    pub blocked: bool,
    pub paused: bool,
    pub shutdown: bool,
    pub crashed: bool,
    pub dying: bool,
}

impl DomainState {
    /// Print the DomainState in the same format as `xm list`
    pub fn print(&self) -> String {
        format!("{}{}{}{}{}{}",
            if self.dying { "d" } else { "-" },
            if self.shutdown { "s" } else { "-" },
            if self.blocked { "b" } else { "-" },
            if self.crashed { "c" } else { "-" },
            if self.paused { "p" } else { "-" },
            if self.running { "r" } else { "-" }
        )
    }
}

pub struct Vcpu {
    ptr: *mut sys::xenstat_vcpu,
}

impl Vcpu {
    pub fn online(&self) -> u32 {
        unsafe {
            sys::xenstat_vcpu_online(self.ptr)
        }
    }

    pub fn ns(&self) -> u64 {
        unsafe {
            sys::xenstat_vcpu_ns(self.ptr)
        }
    }
}

pub struct Network {
    ptr: *mut sys::xenstat_network,
}

impl Network {
    /// Get the ID for this network
    pub fn id(&self) -> u32 {
        unsafe {
            sys::xenstat_network_id(self.ptr)
        }
    }

    /// Get the number of receive bytes for this network
    pub fn rbytes(&self) -> u64 {
        unsafe {
            sys::xenstat_network_rbytes(self.ptr)
        }
    }

    /// Get the number of receive packets for this network
    pub fn rpackets(&self) -> u64 {
        unsafe {
            sys::xenstat_network_rpackets(self.ptr)
        }
    }

    /// Get the number of receive errors for this network
    pub fn rerrs(&self) -> u64 {
        unsafe {
            sys::xenstat_network_rerrs(self.ptr)
        }
    }

    /// Get the number of receive drops for this network
    pub fn rdrop(&self) -> u64 {
        unsafe {
            sys::xenstat_network_rdrop(self.ptr)
        }
    }

    /// Get the number of transmit bytes for this network
    pub fn tbytes(&self) -> u64 {
        unsafe {
            sys::xenstat_network_tbytes(self.ptr)
        }
    }

    /// Get the number of transmit packets for this network
    pub fn tpackets(&self) -> u64 {
        unsafe {
            sys::xenstat_network_tpackets(self.ptr)
        }
    }

    /// Get the number of transmit errors for this network
    pub fn terrs(&self) -> u64 {
        unsafe {
            sys::xenstat_network_terrs(self.ptr)
        }
    }

    /// Get the number of transmit drops for this network
    pub fn tdrop(&self) -> u64 {
        unsafe {
            sys::xenstat_network_tdrop(self.ptr)
        }
    }
}

pub struct Vbd {
    ptr: *mut sys::xenstat_vbd,
}

impl Vbd {
    pub fn get_type(&self) -> VbdType {
        unsafe {
            match sys::xenstat_vbd_type(self.ptr) {
                0 => VbdType::Unidentified,
                1 => VbdType::BlkBack,
                2 => VbdType::BlkTap,
                _ => unreachable!("xenstat_vbd_type returned invalid value"),
            }
        }
    }

    /// Get the device number for the Virtual Block Device
    pub fn vbd_dev(&self) -> u32 {
        unsafe {
            sys::xenstat_vbd_dev(self.ptr)
        }
    }

    pub fn oo_reqs(&self) -> u64 {
        unsafe {
            sys::xenstat_vbd_oo_reqs(self.ptr)
        }
    }

    pub fn rd_reqs(&self) -> u64 {
        unsafe {
            sys::xenstat_vbd_rd_reqs(self.ptr)
        }
    }

    pub fn wr_reqs(&self) -> u64 {
        unsafe {
            sys::xenstat_vbd_wr_reqs(self.ptr)
        }
    }

    pub fn rd_sects(&self) -> u64 {
        unsafe {
            sys::xenstat_vbd_rd_sects(self.ptr)
        }
    }

    pub fn wr_sects(&self) -> u64 {
        unsafe {
            sys::xenstat_vbd_wr_sects(self.ptr)
        }
    }
}

pub enum VbdType {
    Unidentified,
    BlkBack,
    BlkTap,
}

pub struct Tmem {
    ptr: *mut sys::xenstat_tmem,
}

// TODO: Figure out what these do and give them better names
impl Tmem {
    pub fn curr_eph_pages(&self) -> u64 {
        unsafe {
            sys::xenstat_tmem_curr_eph_pages(self.ptr)
        }
    }

    pub fn succ_eph_gets(&self) -> u64 {
        unsafe {
            sys::xenstat_tmem_succ_eph_gets(self.ptr)
        }
    }

    pub fn succ_pers_puts(&self) -> u64 {
        unsafe {
            sys::xenstat_tmem_succ_pers_puts(self.ptr)
        }
    }

    pub fn succ_pers_gets(&self) -> u64 {
        unsafe {
            sys::xenstat_tmem_succ_pers_gets(self.ptr)
        }
    }
}

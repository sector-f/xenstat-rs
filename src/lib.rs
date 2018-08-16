extern crate xenstat_sys as sys;
use std::ops::Drop;
use std::ffi::CStr;

// #define XENSTAT_VCPU 0x1
// #define XENSTAT_NETWORK 0x2
// #define XENSTAT_XEN_VERSION 0x4
// #define XENSTAT_VBD 0x8

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
    pub fn networks(&self, network: u32) -> Network {
        unsafe {
            Network {
                ptr: sys::xenstat_domain_network(self.ptr, network),
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

pub struct Vbd {
    ptr: *mut sys::xenstat_vbd,
}

pub struct Tmem {
    ptr: *mut sys::xenstat_tmem,
}

use std::io;
use std::fs::File;
use crate::util;
use std::io::prelude::*;


// #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
// pub struct MemInfo {
//     pub MemTotal: u64,
//     pub MemFree: u64,
//     pub MemAvailable: u64,
//     pub Buffers: u64,
//     pub Cached: u64,
//     pub SwapCached: u64,
//     pub Active: u64,
//     pub Inactive: u64,
//     pub ActiveAnon: u64,
//     pub InactiveAnon: u64,
//     pub ActiveFile: u64,
//     pub InactiveFile: u64,
//     pub Unevictable: u64,
//     pub Mlocked: u64,
//     pub SwapTotal: u64,
//     pub SwapFree: u64,
//     pub Dirty: u64,
//     pub Writeback: u64,
//     pub AnonPages: u64,
//     pub Mapped: u64,
//     pub Shmem: u64,
//     pub KReclaimable: u64,
//     pub Slab: u64,
//     pub SReclaimable: u64,
//     pub SUnreclaim: u64,
//     pub KernelStack: u64,
//     pub PageTables: u64,
//     pub NFSUnstable: u64,
//     pub Bounce: u64,
//     pub WritebackTmp: u64,
//     pub CommitLimit: u64,
//     pub CommittedAS: u64,
//     pub VmallocTotal: u64,
//     pub VmallocUsed: u64,
//     pub VmallocChunk: u64,
//     pub Percpu: u64,
//     pub HardwareCorrupted: u64,
//     pub AnonHugePages: u64,
//     pub ShmemHugePages: u64,
//     pub ShmemPmdMapped: u64,
//     pub FileHugePages: u64,
//     pub FilePmdMapped: u64,
//     pub CmaTotal: u64,
//     pub CmaFree: u64,
//     pub HugePagesTotal: u64,
//     pub HugePagesFree: u64,
//     pub HugePagesRsvd: u64,
//     pub HugePagesSurp: u64,
//     pub Hugepagesize: u64,
//     pub Hugetlb: u64,
//     pub DirectMap4k: u64,
//     pub DirectMap2M: u64,
// }

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BriefMemInfo{
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_cached: u64,
}

impl BriefMemInfo{
    const PATH: &'static str = "/proc/meminfo";

    pub fn now() -> Result<Self,String>{
        let mut briefmeminfo = BriefMemInfo::default();
        let mut meminfobuf = io::BufReader::new(File::open(Self::PATH).expect("open file meminfo err"));
        let mut input = String::with_capacity(30);

        let read = meminfobuf.read_line(&mut input).unwrap();
        if read == 0{
            return Err("Err input".to_string());
        }
        if !input.starts_with("MemTotal"){
            return Err("Err data".to_string());
        }
        let (temp,_) = util::parse_token(&input).unwrap();
        let (_,memtotal) = util::parse_u64(&temp).unwrap();
        briefmeminfo.mem_total = memtotal;
        input.clear();

        let read = meminfobuf.read_line(&mut input).unwrap();
        if read == 0{
            return Err("Err input".to_string());
        }
        if !input.starts_with("MemFree"){
            return Err("Err data".to_string());
        }
        let (temp,_) = util::parse_token(&input).unwrap();
        let (_,memfree) = util::parse_u64(&temp).unwrap();
        briefmeminfo.mem_free = memfree;
        input.clear();

        let read = meminfobuf.read_line(&mut input).unwrap();
        if read == 0{
            return Err("Err input".to_string());
        }
        if !input.starts_with("MemAvailable"){
            return Err("Err data".to_string());
        }
        let (temp,_) = util::parse_token(&input).unwrap();
        let (_,mem_available) = util::parse_u64(&temp).unwrap();
        briefmeminfo.mem_available = mem_available;
        input.clear();

        
        let read = meminfobuf.read_line(&mut input).unwrap();
        if read == 0{
            return Err("Err input".to_string());
        }
        if !input.starts_with("Buffers"){
            return Err("Err data".to_string());
        }
        let (temp,_) = util::parse_token(&input).unwrap();
        let (_,buffers) = util::parse_u64(&temp).unwrap();
        briefmeminfo.buffers = buffers;
        input.clear();

        
        let read = meminfobuf.read_line(&mut input).unwrap();
        if read == 0{
            return Err("Err input".to_string());
        }
        if !input.starts_with("Cached"){
            return Err("Err data".to_string());
        }
        let (temp,_) = util::parse_token(&input).unwrap();
        let (_,cached) = util::parse_u64(&temp).unwrap();
        briefmeminfo.cached = cached;
        input.clear();

        let read = meminfobuf.read_line(&mut input).unwrap();
        if read == 0{
            return Err("Err input".to_string());
        }
        if !input.starts_with("SwapCached"){
            return Err("Err data".to_string());
        }
        let (temp,_) = util::parse_token(&input).unwrap();
        let (_,swap_cached) = util::parse_u64(&temp).unwrap();
        briefmeminfo.swap_cached = swap_cached;
        input.clear();


        Ok(briefmeminfo)
    }

    pub fn get_memused(&self) -> u64{
        self.mem_total - self.mem_available
    }

}

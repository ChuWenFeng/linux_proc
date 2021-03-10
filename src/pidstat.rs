use fs::OpenOptions;
use crate::util;
use crate::Error;
use std::{fs, io::Read};

macro_rules! parse_key_val_string {
    ($name:expr) => {
        |input|{
            let (input, name) = util::parse_token(input).ok_or(Error::from("cannot read name"))?;
            if name != $name {
                return Err(Error::from(format!(
                    "incorrect name, expected: {}, actual: {}",
                    $name, name
                )));
            }

            let (_,value) = util::parse_token(input).ok_or(Error::from("cannot read value"))?;
            let value = String::from(value);
            Ok(value)
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,Default)]
pub struct  PidStat{
    pub path:String,
    pub Name:String,
    pub Tgid: u64,
    pub Pid:u64,
    pub PPid:u64,
    pub VmPeak:u64, // KB
    pub VmSize:u64, // KB
    pub VmHWM:u64,  
    pub VmRSS:u64,  // KB
    pub VmSwap:u64,
    pub Threads:u64,
}

impl PidStat{
    // fn new(path:String)->Self{
    //     let mut pidstat =  PidStat::default();
    //     pidstat.path = path.clone();
    //     pidstat
    // }
    pub fn now(pid:u64)->Result<PidStat,std::io::Error>{
        let pidpath = format!("/proc/{}",pid);
        let dir = fs::metadata(pidpath.clone());
        if let Err(e) = dir{
            return Err(e);
        }
        let statpath = pidpath+"/status";
        let f =fs::OpenOptions::new().read(true).open(statpath);
        if let Err(e) = f{
            return Err(e);
        }
        let mut pidstat = PidStat::default();
        let mut reader = util::LineParser::new(f.unwrap());

        pidstat.Name  = reader.parse_line(parse_key_val_string!("Name:"))?;
        reader.next_line()?;// skip Umask
        reader.next_line()?;// skip State
        pidstat.Tgid = reader.parse_line(parse_key_val_string!("Tgid:")).unwrap().parse::<u64>().unwrap();
        reader.next_line()?;// skip Ngid
        pidstat.Pid = reader.parse_line(parse_key_val_string!("Pid:")).unwrap().parse::<u64>().unwrap();
        pidstat.PPid = reader.parse_line(parse_key_val_string!("PPid:")).unwrap().parse::<u64>().unwrap();
        reader.next_line()?;// skip TracerPid
        reader.next_line()?;// skip Uid
        reader.next_line()?;// skip Gid
        reader.next_line()?;// skip FDSize
        reader.next_line()?;// skip Groups
        reader.next_line()?;// skip NStgid
        reader.next_line()?;// skip NSpid
        reader.next_line()?;// skip NSpgid
        reader.next_line()?;// skip NSsid
        pidstat.VmPeak = reader.parse_line(parse_key_val_string!("VmPeak:")).unwrap().parse::<u64>().unwrap();
        pidstat.VmSize = reader.parse_line(parse_key_val_string!("VmSize:")).unwrap().parse::<u64>().unwrap();
        reader.next_line()?;// skip VmLck
        reader.next_line()?;// skip VmPin
        pidstat.VmHWM = reader.parse_line(parse_key_val_string!("VmHWM:")).unwrap().parse::<u64>().unwrap();
        pidstat.VmRSS = reader.parse_line(parse_key_val_string!("VmRSS:")).unwrap().parse::<u64>().unwrap();
        reader.next_line()?;// skip RssAnon
        reader.next_line()?;// skip RssFile
        reader.next_line()?;// skip RssShmem
        reader.next_line()?;// skip VmData
        reader.next_line()?;// skip VmStk
        reader.next_line()?;// skip VmExe
        reader.next_line()?;// skip VmLib
        reader.next_line()?;// skip VmPTE
        pidstat.VmSwap = reader.parse_line(parse_key_val_string!("VmSwap:")).unwrap().parse::<u64>().unwrap();

        Ok(pidstat)
    }

    pub fn get_name(&self)->String{
        self.Name.clone()
    }

    pub fn get_memused(&self)->u64{
        self.VmRSS
    }

    pub fn get_memused_peak(&self)->u64{
        self.VmHWM
    }

    pub fn get_swap(&self)->u64{
        self.VmSwap
    }

}
use linux_proc::meminfo;
use linux_proc::pidstat;
fn main(){
    // let meminfo = meminfo::BriefMemInfo::now().unwrap();
    // println!("{:?}",meminfo);
    // println!("memused:{}",meminfo.get_memused());

    let pidstat = pidstat::PidStat::now(1).unwrap();
    println!("{:?}",pidstat);
}
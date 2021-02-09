use linux_proc::meminfo;

fn main(){
    let meminfo = meminfo::BriefMemInfo::now().unwrap();
    println!("{:?}",meminfo);
    println!("memused:{}",meminfo.get_memused());
}
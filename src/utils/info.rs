use sysinfo::System;
use sysinfo::SystemExt;

// struct sys_info_ {
//     Op_name: String,
//     Num_cpu: i32,
// }

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
pub fn get_base_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    if System::IS_SUPPORTED {
        println!("supper current system");
    } else {
        println!("supper current system");
    }
    println!("System name:{:?}", sys.name());
    println!("memory:{}", sys.total_memory());
}

#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
pub fn get_base_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    if System::IS_SUPPORTED {
        println!("supper current system");
    } else {
        println!("unsupper current system");
    }
    println!("System name:{:?}", sys.name());
    println!("memory:{}", sys.total_memory());
}

#[cfg(target_os = "linux")]
#[cfg(target_arch = "i686")]
pub fn get_base_info() {
    println!("{}", "linux x32");
    let mut sys = System::new_all();
    sys.refresh_all();
    if System::IS_SUPPORTED {
        println!("yes");
    } else {
        println!("no");
    }
    println!("System name:{:?}", sys.name());
    sys.total_memory();
}

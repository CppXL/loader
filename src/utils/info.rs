use sysinfo::System;
use sysinfo::SystemExt;

// struct sys_info_ {
//     Op_name: String,
//     Num_cpu: i32,
// }

#[cfg(target_os = "windows")]
pub fn get_base_info() {
    let mut sys = System::new_all();
    sys.refresh_all();
    if System::IS_SUPPORTED {
        println!("yes");
    } else {
        println!("no");
    }
    println!("{}", "windows");
    println!("System name:{:?}", sys.name());
    sys.total_memory();
}

#[cfg(target_os = "linux")]
pub fn get_base_info() {
    println!("{}", "linux")
}

#[cfg(target_os = "windows")]
use core::arch::asm;
use sysinfo::System;
use sysinfo::SystemExt;

// struct sys_info_ {
//     Op_name: String,
//     Num_cpu: i32,
// }

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
pub fn get_base_info() {
    use libloading::Symbol;
    use libloading::{library_filename, Library};
    use std::ptr;

    let mut sys = System::new_all();
    sys.refresh_all();

    if System::IS_SUPPORTED {
        println!("supper current system");
    } else {
        println!("supper current system");
    }
    println!("System name:{:?}", sys.name());
    println!("System version:{:?}", sys.os_version());
    println!("memory:{} bytes", sys.total_memory());
    println!("number of cores:{:?}", sys.cpus());
    println!("networkd:{:?}", sys.networks());
    println!("cpu info:{:?}", sys.global_cpu_info());
    println!("kernel version:{:?}", sys.kernel_version());
    unsafe {
        let mut size = 0;
        let retval = advapi32::GetUserNameW(ptr::null_mut(), &mut size);

        assert_eq!(retval, 0, "Should have failed");

        let mut username = Vec::with_capacity(size as usize);
        let retval = advapi32::GetUserNameW(username.as_mut_ptr(), &mut size);
        assert_ne!(retval, 0, "Perform better error handling");
        assert!((size as usize) <= username.capacity());
        username.set_len(size as usize);

        // Beware: This leaves the trailing NUL character in the final string,
        // you may want to remove it!
        println!("running user:{}", String::from_utf16(&username).unwrap());
    }

    unsafe {
        let library = Library::new(library_filename("testdll")).unwrap();
        let add: Symbol<unsafe extern "C" fn(i32, i32) -> i32> = library.get(b"add").unwrap();
        println!("address of add:{:?}", add.to_owned());
        println!("result of 1 + 2 = {}", add(1, 2));
    }
}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
pub fn get_kernel32_addr() {
    unsafe {
        let mut p: i64;
        asm!(
            "mov r12, gs:[60h]",
            "mov r12, [r12 + 0x18]",
            "mov r12, [r12 + 0x20]",
            "mov r12, [r12]",
            "mov r15, [r12 + 0x20]",
            "mov r12, [r12]",
            "mov r12, [r12 + 0x20]",

            out("r12") p,
        );
        println!("addr:{}", p)
    }
}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86")]
pub fn get_kernel32_addr() {
    unsafe {
        let mut p: i32;
        asm!(
            "mov eax, fs:[30h]",
            "mov eax, [eax + 0ch]",
            "mov eax, [eax + 14h]",
            "mov eax, [eax]",
            "mov eax, [eax]",
            "mov eax, [eax -8h + 18h]",
            out("eax") p,
        );
        println!("addr:{}", p)
    }
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

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
pub fn process_list() {}

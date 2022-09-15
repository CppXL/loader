mod utils;
// extern crate advapi32;
// #[cfg(target_os = "linux")]
// extern crate nix;
// extern crate winapi;

#[cfg(target_os = "linux")]
use nix::libc::getuid;
// use nix::unistd::getuid;

use core::arch::asm;
use std::env::current_dir;
use utils::*;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let name = String::from("Global\\73E21C80-1960-472F-BF0B-3EE7CC7AF17E");
        if !single(name) {
            println!("sorry, running");
            return;
        }
    }

    println!(
        "check_in_virtual_machine retn:{}",
        check_in_virtual_machine()
    );
    println!("running dir:{:?}", current_dir().unwrap());
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    println!("line:{}", line);

    #[cfg(target_os = "linux")]
    unsafe {
        let id = getuid();
        println!("uid:{}", id);
    }
    reg_fs();
}

/// windows下的互斥锁
#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
fn single(name: String) -> bool {
    use log::{info, trace};

    use std::{ffi::OsStr, iter::once, os::windows::prelude::OsStrExt};
    use winapi::shared::{minwindef::FALSE, ntdef::NULL, winerror::ERROR_ALREADY_EXISTS};
    use winapi::um::{
        errhandlingapi::GetLastError, handleapi::CloseHandle, minwinbase::LPSECURITY_ATTRIBUTES,
        synchapi::CreateMutexW,
    };

    trace!("test trace");
    unsafe {
        let mut mutex_name: Vec<u16> = OsStr::new(&name).encode_wide().chain(once(0)).collect();
        let handle = CreateMutexW(0 as LPSECURITY_ATTRIBUTES, FALSE, mutex_name.as_mut_ptr());
        if handle == NULL {
            return false;
        }
        let error = GetLastError();
        if error == ERROR_ALREADY_EXISTS {
            CloseHandle(handle);
            return false;
        }
        info!("success create mutex lock ");
    }
    true
}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
fn reg_fs() {
    unsafe {
        asm!(
            "mov eax,large fs:[30h]",
            out("eax") _,
        );
    }
}

/// Linux下的互斥锁
#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
fn single() -> bool {
    false
}

mod utils;
// extern crate advapi32;
// #[cfg(target_os = "linux")]
// extern crate nix;
// extern crate winapi;

#[cfg(target_os = "linux")]
mod libcs {
    pub use libc;
    pub use nix::libc::getuid;
    // use nix::unistd::getuid;
}
#[cfg(target_os = "linux")]
use libsc::*;

// use core::arch::asm;
use std::env::current_dir;
use std::{ffi::OsStr, iter::once, os::windows::prelude::OsStrExt};
use utils::*;

#[cfg(target_os = "windows")]
mod winapis {
    pub use winapi::shared::{minwindef::FALSE, ntdef::NULL, winerror::ERROR_ALREADY_EXISTS};
    pub use winapi::um::{
        errhandlingapi::GetLastError, handleapi::CloseHandle, minwinbase::LPSECURITY_ATTRIBUTES,
        synchapi::CreateMutexW,
    };
}

#[cfg(target_os = "windows")]
use winapis::*;

fn main() {
    if !single() {
        println!("sorry, running");
        return;
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
}

/// windows下的互斥锁
#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
fn single() -> bool {
    use log::{info, trace};

    // aa

    let name = "Global\\73E21C80-1960-472F-BF0B-3EE7CC7AF17E";
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

/// Linux下的互斥锁
#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
fn single() -> bool {
    let singal_name = String::from("gdpRAIbgPS");
    unsafe {
        let sem = libc::sem_open(singal_name.as_ptr() as *const libc::c_char, 02 | 0100, 1);
        println!("sem:{:?}", *sem);
        // let mut m = 0;
        // let j = &mut m;
        // let i = libc::sem_getvalue(sem, j);
        // println!("i:{}\tj:{}", i, j);
    }
    false
}

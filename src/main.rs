mod utils;
extern crate nix;
extern crate advapi32;
extern crate winapi;

// use nix::unistd::getuid;
use nix::libc::getuid;
use std::env::*;
use utils::*;
use windows::Win32::Storage::Packaging::Opc::;

fn main() {
    println!(
        "check_in_virtual_machine retn:{}",
        check_in_virtual_machine()
    );
    println!("run dir:{:?}", current_dir().unwrap());
    if cfg!(target_os = "linux") {
        unsafe {
            let id = getuid();
            println!("uid:{}", id);
        }
    }
    if cfg!(target_os = "windows") {
        unsafe {
            let result = GetUserName();
            
        }
    }
}

use crate::emu;
use crate::emu::winapi32::kernel32;
//use crate::emu::winapi32::helper;
//use crate::emu::endpoint;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        _ => {
            println!("calling unimplemented shlwapi API 0x{:x} {}", addr, api);
            return api;
        }
    }
}

use crate::emu;
//use crate::emu::constants::*;
//use crate::emu::winapi32::helper;
use crate::emu::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "_set_invalid_parameter_handler" => set_invalid_parameter_handler(emu),


        _ => {
            println!("calling unimplemented wincrt API 0x{:x} {}", addr, api);
            return api;
        }
    }

    return String::new();
}

fn set_invalid_parameter_handler(emu: &mut emu::Emu) {
    println!("{}** {} wincrt!_set_invalid_parameter_handler {}", emu.colors.light_red, emu.pos, emu.colors.nc);
    emu.regs.rax = 0;
}

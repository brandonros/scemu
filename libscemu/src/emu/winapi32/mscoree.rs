use crate::emu;
use crate::emu::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "_CorExeMain" => _CorExeMain(emu),

        _ => {
            println!("calling unimplemented mscoree API 0x{:x} {}", addr, api);
            return api;
        }
    }

    return String::new();
}

pub fn _CorExeMain(emu: &mut emu::Emu) {
    println!(
        "{}** {} mscoree!_CorExeMain {}",
        emu.colors.light_red, emu.pos, emu.colors.nc
    );
    emu.regs.rax = 1;
    unimplemented!();
}

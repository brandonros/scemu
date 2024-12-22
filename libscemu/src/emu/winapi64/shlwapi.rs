use crate::emu;
//use crate::emu::constants;
//use crate::emu::winapi32::helper;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let apiname = emu::winapi64::kernel32::guess_api_name(emu, addr);
    match apiname.as_str() {
        "PathIsContentTypeW" => PathIsContentTypeW(emu),
        "PathFindSuffixArrayA" => PathFindSuffixArrayA(emu),

        _ => {
            println!(
                "calling unimplemented shlwapi API 0x{:x} {}",
                addr, apiname
            );
            return apiname;
        }
    }

    return String::new();
}

pub fn PathIsContentTypeW(emu: &mut emu::Emu) {
    let path = emu.maps.read_wide_string(emu.regs.rcx);
    let content_type = emu.maps.read_wide_string(emu.regs.rdx);

    println!(
        "{}** {} shlwapi!PathIsContentTypeW path: {} content-type: {} {}",
        emu.colors.light_red, emu.pos, path, content_type, emu.colors.nc
    );

    emu.regs.rax = 1;
}

pub fn PathFindSuffixArrayA(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs.rcx);
    let suffixes = emu.maps.read_string(emu.regs.rdx);

    println!(
        "{}** {} shlwapi!PathFindSuffixArrayA path: {} suffixes: {} {}",
        emu.colors.light_red, emu.pos, path, suffixes, emu.colors.nc
    );

    emu.regs.rax = emu.regs.rdx;
}

// TODO documentation...

use std::arch::x86_64::{CpuidResult, __cpuid};

fn check_hypervisor() -> bool {
    let leaf = 0x1;
    let cpuid_result: CpuidResult;
    unsafe {
        cpuid_result = __cpuid(leaf);
    }
    cpuid_result.ecx >> 30 & 1 == 1
}

fn get_manufacturer_str() -> String {
    let leaf = 0x40000000;
    let cpuid_result: CpuidResult;
    unsafe {
        cpuid_result = __cpuid(leaf);
    }
    let mut manufacturer_str = String::new();
    manufacturer_str
        .push_str(std::str::from_utf8(cpuid_result.ebx.to_le_bytes().as_ref()).unwrap());
    manufacturer_str
        .push_str(std::str::from_utf8(cpuid_result.ecx.to_le_bytes().as_ref()).unwrap());
    manufacturer_str
        .push_str(std::str::from_utf8(cpuid_result.edx.to_le_bytes().as_ref()).unwrap());

    manufacturer_str
}

fn main() {
    let manufacturer_str = get_manufacturer_str();
    let hyperbit = check_hypervisor();

    println!("Manufacturer str: {}", manufacturer_str);
    println!("Hypervisor: {}", hyperbit);
}

#[cfg(target_arch = "x86")]
use std::arch::x86;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64 as x86;

#[inline]
pub fn __cpuid(addr: u32) -> x86::CpuidResult {
    unsafe { x86::__cpuid(addr) }
}

#[inline]
pub fn has_feature(features: &[u32; 2], feature: usize) -> bool {
    (features[(feature > 31) as usize] & (1 << (feature % 32))) > 0
}

pub struct Info {
    pub stepping: u8,
    pub model: u8,
    pub family_id: u8,
    pub processor_type: u8,
    #[allow(unused)]
    reserved1: u8,
    pub extended_model_id: u8,
    pub extended_family_id: u8,
    #[allow(unused)]
    reserved2: u8,

    pub features: [u32; 2],
}

impl Default for Info {
    fn default() -> Self {
        let cpuid = __cpuid(0x1);
        Self {
            stepping: (cpuid.eax & 0xF) as u8,
            model: ((cpuid.eax >> 4) & 0xF) as u8,
            family_id: ((cpuid.eax >> 8) & 0xF) as u8,
            processor_type: ((cpuid.eax >> 12) & 0x3) as u8,
            reserved1: ((cpuid.eax >> 14) & 0x3) as u8,
            extended_model_id: ((cpuid.eax >> 16) & 0xF) as u8,
            extended_family_id: ((cpuid.eax >> 20) & 0xFF) as u8,
            reserved2: ((cpuid.eax >> 28) & 0xF) as u8,

            features: [cpuid.edx, cpuid.ecx],
        }
    }
}

pub fn vendor() -> Option<Vec<u8>> {
    let mut cpuid = __cpuid(0x80000000);
    if cpuid.eax < 0x80000004 {
        return None;
    }
    
    // As an example, it could be something like this:
    // 12th Gen Intel(R) Core(TM) i5-12400F
    let mut result = Vec::<u8>::with_capacity(64);

    cpuid = __cpuid(0x80000002);
    result.extend_from_slice(&cpuid.eax.to_le_bytes());
    result.extend_from_slice(&cpuid.ebx.to_le_bytes());
    result.extend_from_slice(&cpuid.ecx.to_le_bytes());
    result.extend_from_slice(&cpuid.edx.to_le_bytes());

    cpuid = __cpuid(0x80000003);
    result.extend_from_slice(&cpuid.eax.to_le_bytes());
    result.extend_from_slice(&cpuid.ebx.to_le_bytes());
    result.extend_from_slice(&cpuid.ecx.to_le_bytes());
    result.extend_from_slice(&cpuid.edx.to_le_bytes());

    cpuid = __cpuid(0x80000004);
    result.extend_from_slice(&cpuid.eax.to_le_bytes());
    result.extend_from_slice(&cpuid.ebx.to_le_bytes());
    result.extend_from_slice(&cpuid.ecx.to_le_bytes());
    result.extend_from_slice(&cpuid.edx.to_le_bytes());

    Some(result)
}

pub fn frequencies(max_lvl: u8) -> Option<[u32; 3]> {
    if max_lvl < 0x16 {
        return None;
    }

    // Base=EAX, Max=EBX, Bus=ECX
    let cpuid = __cpuid(0x16);
    Some([cpuid.eax, cpuid.ebx, cpuid.ecx])
}

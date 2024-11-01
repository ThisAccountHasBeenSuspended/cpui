use strum_macros::{AsRefStr, EnumIter};

/// https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits
#[derive(Clone, AsRefStr, EnumIter)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Feature {
    Fpu = 0,
    Sse3 = 32,

    Vme = 1,
    Pclmulqdq = 33,

    De = 2,
    Dtes64 = 34,

    Pse = 3,
    Monitor = 35,

    Tsc = 4,
    DsCpl = 36,

    Msr = 5,
    Vmx = 37,

    Pae = 6,
    Smx = 38,

    Mce = 7,
    Est = 39,

    Cx8 = 8,
    Tm2 = 40,

    Apic = 9,
    Ssse3 = 41,

    MtrrReserved = 10,
    CnxtId = 42,

    Sep = 11,
    Sdbg = 43,

    Mtrr = 12,
    Fma = 44,

    Pge = 13,
    Cx16 = 45,

    Mca = 14,
    Xtpr = 46,

    Cmov = 15,
    Pdcm = 47,

    Pat = 16,
    Reserved1 = 48,

    Pse36 = 17,
    Pcid = 49,

    Psn = 18,
    Dca = 50,

    Clfsh = 19,
    Sse41 = 51,

    Nx = 20,
    Sse42 = 52,

    Ds = 21,
    X2Apic = 53,

    Acpi = 22,
    Movbe = 54,

    Mmx = 23,
    Popcnt = 55,

    Fxsr = 24,
    TscDeadline = 56,

    Sse = 25,
    AesNi = 57,

    Sse2 = 26,
    Xsave = 58,

    Ss = 27,
    Osxsave = 59,

    Htt = 28,
    Avx = 60,

    Tm = 29,
    F16c = 61,

    Ia64 = 30,
    Rdrand = 62,

    Pbe = 31,
    Hypervisor = 63,
}

pub enum Family {
    Unknown,
    Intel(u8),
    Amd(u8),
}

impl Family {
    pub fn max_lvl(&self) -> u8 {
        match *self {
            Family::Unknown => 0,
            Family::Intel(v) => v,
            Family::Amd(v) => v,
        }
    }
}

impl Default for Family {
    fn default() -> Self {
        let cpuid = crate::cpu::__cpuid(0x0);
        if cpuid.ebx == 0x756e6547 && cpuid.ecx == 0x6c65746e && cpuid.edx == 0x49656e69 { // INTEL
            Self::Intel(cpuid.eax as u8)
        } else if cpuid.ebx == 0x68747541 && cpuid.ecx == 0x444d4163 && cpuid.edx == 0x69746e65 { // AMD
            Self::Amd(cpuid.eax as u8)
        } else {
            Self::Unknown
        }
    }
}

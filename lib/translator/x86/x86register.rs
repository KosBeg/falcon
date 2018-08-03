use falcon_capstone::capstone_sys::x86_reg;
use error::*;
use il::*;
use il::Expression as Expr;
use translator::x86::mode::Mode;


const X86REGISTERS : &'static [X86Register] = &[
    X86Register { name: "ah",  capstone_reg: x86_reg::X86_REG_AH,  full_reg: x86_reg::X86_REG_EAX, offset: 8, bits: 8,  mode: Mode::X86 },
    X86Register { name: "al",  capstone_reg: x86_reg::X86_REG_AL,  full_reg: x86_reg::X86_REG_EAX, offset: 0, bits: 8,  mode: Mode::X86 },
    X86Register { name: "ax",  capstone_reg: x86_reg::X86_REG_AX,  full_reg: x86_reg::X86_REG_EAX, offset: 0, bits: 16, mode: Mode::X86 },
    X86Register { name: "eax", capstone_reg: x86_reg::X86_REG_EAX, full_reg: x86_reg::X86_REG_EAX, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "bh",  capstone_reg: x86_reg::X86_REG_BH,  full_reg: x86_reg::X86_REG_EBX, offset: 8, bits: 8,  mode: Mode::X86 },
    X86Register { name: "bl",  capstone_reg: x86_reg::X86_REG_BL,  full_reg: x86_reg::X86_REG_EBX, offset: 0, bits: 8,  mode: Mode::X86 },
    X86Register { name: "bx",  capstone_reg: x86_reg::X86_REG_BX,  full_reg: x86_reg::X86_REG_EBX, offset: 0, bits: 16, mode: Mode::X86 },
    X86Register { name: "ebx", capstone_reg: x86_reg::X86_REG_EBX, full_reg: x86_reg::X86_REG_EBX, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "ch",  capstone_reg: x86_reg::X86_REG_CH,  full_reg: x86_reg::X86_REG_ECX, offset: 8, bits: 8,  mode: Mode::X86 },
    X86Register { name: "cl",  capstone_reg: x86_reg::X86_REG_CL,  full_reg: x86_reg::X86_REG_ECX, offset: 0, bits: 8,  mode: Mode::X86 },
    X86Register { name: "cx",  capstone_reg: x86_reg::X86_REG_CX,  full_reg: x86_reg::X86_REG_ECX, offset: 0, bits: 16, mode: Mode::X86 },
    X86Register { name: "ecx", capstone_reg: x86_reg::X86_REG_ECX, full_reg: x86_reg::X86_REG_ECX, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "dh",  capstone_reg: x86_reg::X86_REG_DH,  full_reg: x86_reg::X86_REG_EDX, offset: 8, bits: 8,  mode: Mode::X86 },
    X86Register { name: "dl",  capstone_reg: x86_reg::X86_REG_DL,  full_reg: x86_reg::X86_REG_EDX, offset: 0, bits: 8,  mode: Mode::X86 },
    X86Register { name: "dx",  capstone_reg: x86_reg::X86_REG_DX,  full_reg: x86_reg::X86_REG_EDX, offset: 0, bits: 16, mode: Mode::X86 },
    X86Register { name: "edx", capstone_reg: x86_reg::X86_REG_EDX, full_reg: x86_reg::X86_REG_EDX, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "si",  capstone_reg: x86_reg::X86_REG_SI,  full_reg: x86_reg::X86_REG_ESI, offset: 0, bits: 16, mode: Mode::X86 },
    X86Register { name: "esi", capstone_reg: x86_reg::X86_REG_ESI, full_reg: x86_reg::X86_REG_ESI, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "di",  capstone_reg: x86_reg::X86_REG_DI,  full_reg: x86_reg::X86_REG_EDI, offset: 0, bits: 16, mode: Mode::X86 },
    X86Register { name: "edi", capstone_reg: x86_reg::X86_REG_EDI, full_reg: x86_reg::X86_REG_EDI, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "sp",  capstone_reg: x86_reg::X86_REG_SP,  full_reg: x86_reg::X86_REG_ESP, offset: 0, bits: 16, mode: Mode::X86 },
    X86Register { name: "esp", capstone_reg: x86_reg::X86_REG_ESP, full_reg: x86_reg::X86_REG_ESP, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "bp",  capstone_reg: x86_reg::X86_REG_BP,  full_reg: x86_reg::X86_REG_EBP, offset: 0, bits: 16, mode: Mode::X86 },
    X86Register { name: "ebp", capstone_reg: x86_reg::X86_REG_EBP, full_reg: x86_reg::X86_REG_EBP, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "fs_base", capstone_reg: x86_reg::X86_REG_FS, full_reg: x86_reg::X86_REG_FS, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "gs_base", capstone_reg: x86_reg::X86_REG_GS, full_reg: x86_reg::X86_REG_GS, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "ds_base", capstone_reg: x86_reg::X86_REG_DS, full_reg: x86_reg::X86_REG_DS, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "es_base", capstone_reg: x86_reg::X86_REG_ES, full_reg: x86_reg::X86_REG_ES, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "cs_base", capstone_reg: x86_reg::X86_REG_CS, full_reg: x86_reg::X86_REG_CS, offset: 0, bits: 32, mode: Mode::X86 },
    X86Register { name: "ss_base", capstone_reg: x86_reg::X86_REG_SS, full_reg: x86_reg::X86_REG_SS, offset: 0, bits: 32, mode: Mode::X86 },
];


const AMD64REGISTERS : &'static [X86Register] = &[
    X86Register { name: "ah",  capstone_reg: x86_reg::X86_REG_AH,  full_reg: x86_reg::X86_REG_RAX, offset: 8, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "al",  capstone_reg: x86_reg::X86_REG_AL,  full_reg: x86_reg::X86_REG_RAX, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "ax",  capstone_reg: x86_reg::X86_REG_AX,  full_reg: x86_reg::X86_REG_RAX, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "eax", capstone_reg: x86_reg::X86_REG_EAX, full_reg: x86_reg::X86_REG_RAX, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "rax", capstone_reg: x86_reg::X86_REG_RAX, full_reg: x86_reg::X86_REG_RAX, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "bh",  capstone_reg: x86_reg::X86_REG_BH,  full_reg: x86_reg::X86_REG_RBX, offset: 8, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "bl",  capstone_reg: x86_reg::X86_REG_BL,  full_reg: x86_reg::X86_REG_RBX, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "bx",  capstone_reg: x86_reg::X86_REG_BX,  full_reg: x86_reg::X86_REG_RBX, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "ebx", capstone_reg: x86_reg::X86_REG_EBX, full_reg: x86_reg::X86_REG_RBX, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "rbx", capstone_reg: x86_reg::X86_REG_RBX, full_reg: x86_reg::X86_REG_RBX, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "ch",  capstone_reg: x86_reg::X86_REG_CH,  full_reg: x86_reg::X86_REG_RCX, offset: 8, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "cl",  capstone_reg: x86_reg::X86_REG_CL,  full_reg: x86_reg::X86_REG_RCX, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "cx",  capstone_reg: x86_reg::X86_REG_CX,  full_reg: x86_reg::X86_REG_RCX, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "ecx", capstone_reg: x86_reg::X86_REG_ECX, full_reg: x86_reg::X86_REG_RCX, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "rcx", capstone_reg: x86_reg::X86_REG_RCX, full_reg: x86_reg::X86_REG_RCX, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "dh",  capstone_reg: x86_reg::X86_REG_DH,  full_reg: x86_reg::X86_REG_RDX, offset: 8, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "dl",  capstone_reg: x86_reg::X86_REG_DL,  full_reg: x86_reg::X86_REG_RDX, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "dx",  capstone_reg: x86_reg::X86_REG_DX,  full_reg: x86_reg::X86_REG_RDX, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "edx", capstone_reg: x86_reg::X86_REG_EDX, full_reg: x86_reg::X86_REG_RDX, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "rdx", capstone_reg: x86_reg::X86_REG_RDX, full_reg: x86_reg::X86_REG_RDX, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "sil", capstone_reg: x86_reg::X86_REG_SIL, full_reg: x86_reg::X86_REG_RSI, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "si",  capstone_reg: x86_reg::X86_REG_SI,  full_reg: x86_reg::X86_REG_RSI, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "esi", capstone_reg: x86_reg::X86_REG_ESI, full_reg: x86_reg::X86_REG_RSI, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "rsi", capstone_reg: x86_reg::X86_REG_RSI, full_reg: x86_reg::X86_REG_RSI, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "dil", capstone_reg: x86_reg::X86_REG_DIL, full_reg: x86_reg::X86_REG_RDI, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "di",  capstone_reg: x86_reg::X86_REG_DI,  full_reg: x86_reg::X86_REG_RDI, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "edi", capstone_reg: x86_reg::X86_REG_EDI, full_reg: x86_reg::X86_REG_RDI, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "rdi", capstone_reg: x86_reg::X86_REG_RDI, full_reg: x86_reg::X86_REG_RDI, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "sp",  capstone_reg: x86_reg::X86_REG_SP,  full_reg: x86_reg::X86_REG_RSP, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "esp", capstone_reg: x86_reg::X86_REG_ESP, full_reg: x86_reg::X86_REG_RSP, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "rsp", capstone_reg: x86_reg::X86_REG_RSP, full_reg: x86_reg::X86_REG_RSP, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "bpl", capstone_reg: x86_reg::X86_REG_BPL, full_reg: x86_reg::X86_REG_RBP, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "bp",  capstone_reg: x86_reg::X86_REG_BP,  full_reg: x86_reg::X86_REG_RBP, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "ebp", capstone_reg: x86_reg::X86_REG_EBP, full_reg: x86_reg::X86_REG_RBP, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "rbp", capstone_reg: x86_reg::X86_REG_RBP, full_reg: x86_reg::X86_REG_RBP, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "r8b", capstone_reg: x86_reg::X86_REG_R8B, full_reg: x86_reg::X86_REG_R8 , offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "r8w", capstone_reg: x86_reg::X86_REG_R8W, full_reg: x86_reg::X86_REG_R8 , offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "r8d", capstone_reg: x86_reg::X86_REG_R8D, full_reg: x86_reg::X86_REG_R8 , offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "r8",  capstone_reg: x86_reg::X86_REG_R8,  full_reg: x86_reg::X86_REG_R8,  offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "r9b", capstone_reg: x86_reg::X86_REG_R9B, full_reg: x86_reg::X86_REG_R9,  offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "r9w", capstone_reg: x86_reg::X86_REG_R9W, full_reg: x86_reg::X86_REG_R9,  offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "r9d", capstone_reg: x86_reg::X86_REG_R9D, full_reg: x86_reg::X86_REG_R9,  offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "r9",  capstone_reg: x86_reg::X86_REG_R9,  full_reg: x86_reg::X86_REG_R9,  offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "r10b", capstone_reg: x86_reg::X86_REG_R10B, full_reg: x86_reg::X86_REG_R10, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "r10w", capstone_reg: x86_reg::X86_REG_R10W, full_reg: x86_reg::X86_REG_R10, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "r10d", capstone_reg: x86_reg::X86_REG_R10D, full_reg: x86_reg::X86_REG_R10, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "r10",  capstone_reg: x86_reg::X86_REG_R10,  full_reg: x86_reg::X86_REG_R10, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "r11b", capstone_reg: x86_reg::X86_REG_R11B, full_reg: x86_reg::X86_REG_R11, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "r11w", capstone_reg: x86_reg::X86_REG_R11W, full_reg: x86_reg::X86_REG_R11, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "r11d", capstone_reg: x86_reg::X86_REG_R11D, full_reg: x86_reg::X86_REG_R11, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "r11",  capstone_reg: x86_reg::X86_REG_R11,  full_reg: x86_reg::X86_REG_R11, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "r12b", capstone_reg: x86_reg::X86_REG_R12B, full_reg: x86_reg::X86_REG_R12, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "r12w", capstone_reg: x86_reg::X86_REG_R12W, full_reg: x86_reg::X86_REG_R12, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "r12d", capstone_reg: x86_reg::X86_REG_R12D, full_reg: x86_reg::X86_REG_R12, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "r12",  capstone_reg: x86_reg::X86_REG_R12,  full_reg: x86_reg::X86_REG_R12, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "r13b", capstone_reg: x86_reg::X86_REG_R13B, full_reg: x86_reg::X86_REG_R13, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "r13w", capstone_reg: x86_reg::X86_REG_R13W, full_reg: x86_reg::X86_REG_R13, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "r13d", capstone_reg: x86_reg::X86_REG_R13D, full_reg: x86_reg::X86_REG_R13, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "r13",  capstone_reg: x86_reg::X86_REG_R13,  full_reg: x86_reg::X86_REG_R13, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "r14b", capstone_reg: x86_reg::X86_REG_R14B, full_reg: x86_reg::X86_REG_R14, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "r14w", capstone_reg: x86_reg::X86_REG_R14W, full_reg: x86_reg::X86_REG_R14, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "r14d", capstone_reg: x86_reg::X86_REG_R14D, full_reg: x86_reg::X86_REG_R14, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "r14",  capstone_reg: x86_reg::X86_REG_R14,  full_reg: x86_reg::X86_REG_R14, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "r15b", capstone_reg: x86_reg::X86_REG_R15B, full_reg: x86_reg::X86_REG_R15, offset: 0, bits:  8, mode: Mode::Amd64 },
    X86Register { name: "r15w", capstone_reg: x86_reg::X86_REG_R15W, full_reg: x86_reg::X86_REG_R15, offset: 0, bits: 16, mode: Mode::Amd64 },
    X86Register { name: "r15d", capstone_reg: x86_reg::X86_REG_R15D, full_reg: x86_reg::X86_REG_R15, offset: 0, bits: 32, mode: Mode::Amd64 },
    X86Register { name: "r15",  capstone_reg: x86_reg::X86_REG_R15,  full_reg: x86_reg::X86_REG_R15, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "fs_base", capstone_reg: x86_reg::X86_REG_FS, full_reg: x86_reg::X86_REG_FS, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "gs_base", capstone_reg: x86_reg::X86_REG_GS, full_reg: x86_reg::X86_REG_GS, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "ds_base", capstone_reg: x86_reg::X86_REG_DS, full_reg: x86_reg::X86_REG_DS, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "es_base", capstone_reg: x86_reg::X86_REG_ES, full_reg: x86_reg::X86_REG_ES, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "cs_base", capstone_reg: x86_reg::X86_REG_CS, full_reg: x86_reg::X86_REG_CS, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "ss_base", capstone_reg: x86_reg::X86_REG_SS, full_reg: x86_reg::X86_REG_SS, offset: 0, bits: 64, mode: Mode::Amd64 },
    X86Register { name: "xmm0",  capstone_reg: x86_reg::X86_REG_XMM0, full_reg: x86_reg::X86_REG_XMM0, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm1",  capstone_reg: x86_reg::X86_REG_XMM1, full_reg: x86_reg::X86_REG_XMM1, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm2",  capstone_reg: x86_reg::X86_REG_XMM2, full_reg: x86_reg::X86_REG_XMM2, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm3",  capstone_reg: x86_reg::X86_REG_XMM3, full_reg: x86_reg::X86_REG_XMM3, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm4",  capstone_reg: x86_reg::X86_REG_XMM4, full_reg: x86_reg::X86_REG_XMM4, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm5",  capstone_reg: x86_reg::X86_REG_XMM5, full_reg: x86_reg::X86_REG_XMM5, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm6",  capstone_reg: x86_reg::X86_REG_XMM6, full_reg: x86_reg::X86_REG_XMM6, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm7",  capstone_reg: x86_reg::X86_REG_XMM7, full_reg: x86_reg::X86_REG_XMM7, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm8",  capstone_reg: x86_reg::X86_REG_XMM8, full_reg: x86_reg::X86_REG_XMM8, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm9",  capstone_reg: x86_reg::X86_REG_XMM9, full_reg: x86_reg::X86_REG_XMM9, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm10", capstone_reg: x86_reg::X86_REG_XMM10, full_reg: x86_reg::X86_REG_XMM10, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm11", capstone_reg: x86_reg::X86_REG_XMM11, full_reg: x86_reg::X86_REG_XMM11, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm12", capstone_reg: x86_reg::X86_REG_XMM12, full_reg: x86_reg::X86_REG_XMM12, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm13", capstone_reg: x86_reg::X86_REG_XMM13, full_reg: x86_reg::X86_REG_XMM13, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm14", capstone_reg: x86_reg::X86_REG_XMM14, full_reg: x86_reg::X86_REG_XMM14, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm15", capstone_reg: x86_reg::X86_REG_XMM15, full_reg: x86_reg::X86_REG_XMM15, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm16", capstone_reg: x86_reg::X86_REG_XMM16, full_reg: x86_reg::X86_REG_XMM16, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm17", capstone_reg: x86_reg::X86_REG_XMM17, full_reg: x86_reg::X86_REG_XMM17, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm18", capstone_reg: x86_reg::X86_REG_XMM18, full_reg: x86_reg::X86_REG_XMM18, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm19", capstone_reg: x86_reg::X86_REG_XMM19, full_reg: x86_reg::X86_REG_XMM19, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm20", capstone_reg: x86_reg::X86_REG_XMM20, full_reg: x86_reg::X86_REG_XMM20, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm21", capstone_reg: x86_reg::X86_REG_XMM21, full_reg: x86_reg::X86_REG_XMM21, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm22", capstone_reg: x86_reg::X86_REG_XMM22, full_reg: x86_reg::X86_REG_XMM22, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm23", capstone_reg: x86_reg::X86_REG_XMM23, full_reg: x86_reg::X86_REG_XMM23, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm24", capstone_reg: x86_reg::X86_REG_XMM24, full_reg: x86_reg::X86_REG_XMM24, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm25", capstone_reg: x86_reg::X86_REG_XMM25, full_reg: x86_reg::X86_REG_XMM25, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm26", capstone_reg: x86_reg::X86_REG_XMM26, full_reg: x86_reg::X86_REG_XMM26, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm27", capstone_reg: x86_reg::X86_REG_XMM27, full_reg: x86_reg::X86_REG_XMM27, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm28", capstone_reg: x86_reg::X86_REG_XMM28, full_reg: x86_reg::X86_REG_XMM28, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm29", capstone_reg: x86_reg::X86_REG_XMM29, full_reg: x86_reg::X86_REG_XMM29, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm30", capstone_reg: x86_reg::X86_REG_XMM30, full_reg: x86_reg::X86_REG_XMM30, offset: 0, bits: 128, mode: Mode::Amd64 },
    X86Register { name: "xmm31", capstone_reg: x86_reg::X86_REG_XMM31, full_reg: x86_reg::X86_REG_XMM31, offset: 0, bits: 128, mode: Mode::Amd64 },
];


/// Struct for dealing with x86 registers
pub(crate) struct X86Register {
    name: &'static str,
    // The capstone enum value for this register.
    capstone_reg: x86_reg,
    /// The full register. For example, eax is the full register for al.
    full_reg: x86_reg,
    /// The offset of this register. For example, ah is offset 8 bit into eax.
    offset: usize,
    /// The size of this register in bits
    bits: usize,
    /// The mode for this register
    mode: Mode
}


impl X86Register {
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns true if this is a full-width register (i.e. eax, ebx, etc)
    pub fn is_full(&self) -> bool {
        if self.capstone_reg == self.full_reg {
            true
        }
        else {
            false
        }
    }

    /// Returns the full-width register for this register
    pub fn get_full(&self) -> Result<&'static X86Register> {
        get_register(&self.mode, self.full_reg)
    }

    /// Returns an expression which evaluates to the value of the register.
    ///
    /// This handles things like al/ah/ax/eax
    pub fn get(&self) -> Result<Expression> {
        if self.is_full() {
            Ok(expr_scalar(self.name, self.bits))
        }
        else if self.offset == 0 {
            Expr::trun(self.bits, self.get_full()?.get()?)
        }
        else {
            let full_reg = self.get_full()?;
            let expr = Expr::shr(full_reg.get()?, expr_const(self.offset as u64, full_reg.bits))?;
            Expr::trun(self.bits, expr)
        }
    }

    /// Sets the value of this register.
    ///
    /// This handles things like al/ah/ax/eax
    pub fn set(&self, block: &mut Block, value: Expression) -> Result<()> {
        if self.is_full() {
            block.assign(scalar(self.name, self.bits), value);
            Ok(())
        }
        else if self.offset == 0 {
            let full_reg = self.get_full()?;
            if full_reg.bits() < 64 {
                let mask = !0 << self.bits;
                let expr = Expr::and(full_reg.get()?, expr_const(mask, full_reg.bits))?;
                let expr = Expr::or(expr, Expr::zext(full_reg.bits, value)?)?;
                full_reg.set(block, expr)
            } else {
                full_reg.set(block, Expr::zext(full_reg.bits, value)?)
            }
        }
        else {
            let full_reg = self.get_full()?;
            let mask = ((1 << self.bits) - 1) << self.offset;
            let expr = Expr::and(full_reg.get()?, expr_const(mask, full_reg.bits))?;
            let value = Expr::zext(full_reg.bits, value)?;
            let expr = Expr::or(expr, Expr::shl(value, expr_const(self.offset as u64, full_reg.bits))?)?;
            full_reg.set(block, expr)
        }
    }
}


/// Takes a capstone register enum and returns an `X86Register`
pub(crate) fn get_register(mode: &Mode, capstone_id: x86_reg)
    -> Result<&'static X86Register> {
        
    let registers: &[X86Register] = match *mode {
        Mode::X86 => X86REGISTERS,
        Mode::Amd64 => AMD64REGISTERS
    };

    for register in registers.iter() {
        if register.capstone_reg == capstone_id {
            return Ok(&register);
        }
    }
    Err(format!("Could not find register {:?}", capstone_id).into())
}
use crate::cpu::opcodes::*;

macro_rules! match_opcode {
    (
        use $opcode_ident:ident;

        $($opcode:ident => (
            $operation:ident,
            $mode:ident,
            $len:literal,
            $cycles_base:literal
        ),)+
    ) => {
        match $opcode_ident {
            $($opcode => Instruction {
                opcode: $opcode,
                operation: InstructionOperation::$operation,
                mode: InstructionMode::$mode,
                len: $len,
                cycles_base: $cycles_base,
            },)+
            _ => unimplemented!("no instruction found for opcode `${:02X}`", $opcode_ident),
        }
    };
}

#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Instruction {
    opcode: u8,
    operation: InstructionOperation,
    mode: InstructionMode,
    len: u8,
    cycles_base: u8,
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Self {
        match_opcode! {
            use opcode;

            // opcode => (operation, mode, len, cycles_base)
            ADC_IMMEDIATE   => (Adc, Immediate,   2, 2),
            ASL_ACCUMULATOR => (Asl, Accumulator, 1, 2),
            ASL_ZERO_PAGE_X => (Asl, ZeroPageX,   2, 6),
            CLC_IMPLIED     => (Clc, Implied,     1, 2),
            CLD_IMPLIED     => (Cld, Implied,     1, 2),
            CLI_IMPLIED     => (Cli, Implied,     1, 2),
            CLV_IMPLIED     => (Clv, Implied,     1, 2),
            INX_IMPLIED     => (Inx, Implied,     1, 2),
            INY_IMPLIED     => (Iny, Implied,     1, 2),
            LDA_ABSOLUTE    => (Lda, Absolute,    3, 4),
            LDX_IMMEDIATE   => (Ldx, Immediate,   2, 2),
            NOP_IMPLIED     => (Nop, Implied,     1, 2),
            SEC_IMPLIED     => (Sec, Implied,     1, 2),
            SED_IMPLIED     => (Sed, Implied,     1, 2),
            SEI_IMPLIED     => (Sei, Implied,     1, 2),
            TAX_IMPLIED     => (Tax, Implied,     1, 2),
            TAY_IMPLIED     => (Tay, Implied,     1, 2),
            TXA_IMPLIED     => (Txa, Implied,     1, 2),
            TYA_IMPLIED     => (Tya, Implied,     1, 2),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionOperation {
    Adc, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc,
    Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Jmp,
    Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti,
    Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Tya,
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionMode {
    Implied,
    Accumulator,
    Immediate,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

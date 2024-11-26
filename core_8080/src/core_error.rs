use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("invalid ROM size")]
    RomSizeError,

    #[error("program counter out of bounds\n index: {index}")]
    ProgramCounterError { index: u16 },

    #[error("program pointer overflow")]
    ProgramCounterOverflow,

    #[error("attempted to read outside of RAM\n index: {index}")]
    IndexError { index: u16 },
    
    #[error("invalid opcode: {opcode}")]
    OpcodeError { opcode: u8 },

    #[error("stack pointer overflow")]
    StackPointerOverflow,
}
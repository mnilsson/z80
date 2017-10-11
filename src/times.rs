/// internal cpu operation
pub const IO: u8 = 5; 
pub const MR: u8 = 3; // Memory read
pub const MRH: u8 = 3; // Memory read of high byte
pub const MRL: u8 = 3; // Memory read of low byte

pub const MW: u8 = 3; // Memory write
pub const MWH: u8 = 3; // Memory write of high byte
pub const MWL: u8 = 3; // Memory write of low byte


pub const OCF: u8 = 4; // Op Code Fetch

pub const OD: u8 = 3; // Operand data read
pub const ODH: u8 = 3; // Operand data read of high byte
pub const ODL: u8 = 3; // Operand data read of low byte

pub const PR: u8 = 4; // port read
pub const PW: u8 = 4; // port read

pub const SRH: u8 = 3; /// Stack read of high byte
pub const SRL: u8 = 3; // Stack read of low byte

pub const SWH: u8 = 3; /// Stack write of high byte
pub const SWL: u8 = 3; // Stack write of low byte
/// CPU of this virtual machine.
/// CPU has 3 quick memories: X, Y, and Z.
/// Flags field:
///     0th: Zero flag
///     1st: Sign bit
///     2nd: Overflow flag
///     3rd: Carry flag
#[derive(Debug)]
pub struct CPU
{
     pub qm_x: u16,
     pub qm_y: u16,
     pub qm_z: u16,
     code_ptr: u16,
     data_ptr: u16,
     flags: u8
}

impl Default for CPU
{
    fn default() -> Self
    {
        CPU {
            qm_x: 0,
            qm_y: 0,
            qm_z: 0,
            code_ptr: 0,
            data_ptr: 0,
            flags: 0
        }
    }
}

impl CPU
{
    fn _fetch_from_code_mem(self, _loc: u16) -> u8
    {
        return 0;
    }

    pub fn exec(&mut self, instr: &[u8])
    {
        for i in instr
        {
            match i {
                0xDE => self.qm_y = self.qm_x,
                0xAD => self.qm_x = self.qm_y,
                _ => println!("invalid opcode")
            }
        }
    }

    pub fn dump_qms(&self)
    {
        println!("Quick Memories:");
        println!("X -> {}", self.qm_x);
        println!("Y -> {}", self.qm_y);
        println!("Z -> {}", self.qm_z);
    }
}

pub trait Mem
{
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, data: u8, addr: u16);
}

pub struct RawMem
{
    pub data: Box<[u8]>
}

impl RawMem
{
    pub fn new() -> Self
    {
        Self {
            data: Box::new([0; (2^16)-1])
        }
    }
}

impl Mem for RawMem
{
    fn read(&self, addr: u16) -> u8
    {
        return self.data[addr as usize];
    }

    fn write(&mut self, byt: u8, addr: u16)
    {
        self.data[addr as usize] = byt;
    }
}

fn main() {}

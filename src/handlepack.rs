
pub struct BytePackerBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}


impl BytePackerBuffer {

    /// This gives us a fresh buffer for holding the packet contents, and a
    /// field for keeping track of where we are.
    pub fn new() -> BytePackerBuffer{
        BytePackerBuffer {
            buf:[0; 512],
            pos:0
        }
    }

    /// Current position within buffer
    fn pos(&self) -> usize{
        self.pos
    }

    /// Step the buffer position forward a specific number of steps
    fn step(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;
        
        Ok(())
    }

    /// Change the buffer position
    fn seek(&mut self, pos:usize)-> Result<()>{
        self.pos = pos;

        Ok(())
    }

    /// Read a single byte and move the position one step forward
    fn read(&mut self) -> Result<()> {
        if self.pos >= 512 {
            return Err("End of vbuffer".into());
        }
        let res = self.buf[self.pos];
        self.pos +=1;

        Ok(res)
    }

    /// Get a single byte, without changing the buffer position
    fn get(&mut self, pos: usize) -> Result<u8>{
        if pos >= 512{
            return Err("End of buffer".into());
        }

        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    fn get_range(&mut self, start: usize, len:usize) -> Result<&[u8]> {
        if start + len >= 512{
            return Err("End of buffer".into());
        }

        Ok(&self.buf[start..start + len as usize])
    }

    /// Read two bytes, stepping two steps forward
    fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    /// Read four bytes, stepping four steps forward
    fn read_u32(&mut self) -> Result<u32>{
        let res = ((self.read()? as u32) << 24) 
            | ((self.read()? as u32) << 16) 
            | ((self.read()? as u32) << 8) 
            | ((self.read()? as u32) << 0);

        Ok(res)
    }

}
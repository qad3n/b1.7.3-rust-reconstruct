use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;

#[derive(Default)]
pub struct Packet17Sleep
{
    m_base: PacketBase,
    pub m_id: i32,
    pub m_x: i32,
    pub m_y: i32,
    pub m_z: i32,
    pub m_action: i32,
}

impl Packet for Packet17Sleep
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_id = input.readInt()?;
        self.m_action = input.readByte()? as i32;
        self.m_x = input.readInt()?;
        self.m_y = input.readByte()? as i32;
        self.m_z = input.readInt()?;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_id)?;
        output.writeByte(self.m_action)?;
        output.writeInt(self.m_x)?;
        output.writeByte(self.m_y)?;
        output.writeInt(self.m_z)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handlePlayerSleep(self); }

    fn getPacketSize(&self) -> i32 { 14 }
}

use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;

#[derive(Default)]
pub struct Packet200Statistic
{
    m_base: PacketBase,
    pub m_statId: i32,
    pub m_value: i32,
}

impl Packet for Packet200Statistic
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_statId = input.readInt()?;
        self.m_value = input.readByte()? as i32;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_statId)?;
        output.writeByte(self.m_value)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleStat(self); }

    fn getPacketSize(&self) -> i32 { 6 }
}

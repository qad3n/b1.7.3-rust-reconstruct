use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;

#[derive(Default)]
pub struct Packet105UpdateProgressbar
{
    m_base: PacketBase,
    pub m_windowId: i32,
    pub m_progressBar: i32,
    pub m_progressBarValue: i32,
}

impl Packet for Packet105UpdateProgressbar
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_windowId = input.readByte()? as i32;
        self.m_progressBar = input.readShort()? as i32;
        self.m_progressBarValue = input.readShort()? as i32;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeByte(self.m_windowId)?;
        output.writeShort(self.m_progressBar)?;
        output.writeShort(self.m_progressBarValue)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleInventoryMenuData(self); }

    fn getPacketSize(&self) -> i32 { 5 }
}

use crate::network::net_handler::NetHandler;
use crate::network::packet::{self, Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;

#[derive(Default)]
pub struct Packet54PlayNoteBlock
{
    m_base: PacketBase,
    pub m_xLocation: i32,
    pub m_yLocation: i32,
    pub m_zLocation: i32,
    pub m_instrumentType: i32,
    pub m_pitch: i32,
}

impl Packet for Packet54PlayNoteBlock
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_xLocation = input.readInt()?;
        self.m_yLocation = input.readShort()? as i32;
        self.m_zLocation = input.readInt()?;
        self.m_instrumentType = packet::read(input)?;
        self.m_pitch = packet::read(input)?;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_xLocation)?;
        output.writeShort(self.m_yLocation)?;
        output.writeInt(self.m_zLocation)?;
        output.write(self.m_instrumentType)?;
        output.write(self.m_pitch)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleNotePlay(self); }

    fn getPacketSize(&self) -> i32 { 12 }
}

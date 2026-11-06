use crate::network::net_handler::NetHandler;
use crate::network::packet::{self, Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;

pub struct Packet53BlockChange
{
    m_base: PacketBase,
    pub m_xPosition: i32,
    pub m_yPosition: i32,
    pub m_zPosition: i32,
    pub m_type: i32,
    pub m_metadata: i32,
}

impl Default for Packet53BlockChange
{
    fn default() -> Self
    {
        Self {
            m_base: PacketBase::chunkData(),
            m_xPosition: Default::default(),
            m_yPosition: Default::default(),
            m_zPosition: Default::default(),
            m_type: Default::default(),
            m_metadata: Default::default(),
        }
    }
}

impl Packet for Packet53BlockChange
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_xPosition = input.readInt()?;
        self.m_yPosition = packet::read(input)?;
        self.m_zPosition = input.readInt()?;
        self.m_type = packet::read(input)?;
        self.m_metadata = packet::read(input)?;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_xPosition)?;
        output.write(self.m_yPosition)?;
        output.writeInt(self.m_zPosition)?;
        output.write(self.m_type)?;
        output.write(self.m_metadata)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleBlockChange(self); }

    fn getPacketSize(&self) -> i32 { 11 }
}

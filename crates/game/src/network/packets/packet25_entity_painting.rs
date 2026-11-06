use crate::entity::entity_painting::EnumArt;
use crate::network::net_handler::NetHandler;
use crate::network::packet::{self, Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;
use jvm::string::JString;
use jvm::throwable::OrNpe;

#[derive(Default)]
pub struct Packet25EntityPainting
{
    m_base: PacketBase,
    pub m_entityId: i32,
    pub m_xPosition: i32,
    pub m_yPosition: i32,
    pub m_zPosition: i32,
    pub m_direction: i32,
    pub m_title: Option<JString>,
}

impl Packet for Packet25EntityPainting
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_entityId = input.readInt()?;
        self.m_title = Some(packet::readString(input, EnumArt::maxArtTitleLength())?);
        self.m_xPosition = input.readInt()?;
        self.m_yPosition = input.readInt()?;
        self.m_zPosition = input.readInt()?;
        self.m_direction = input.readInt()?;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_entityId)?;
        packet::writeString(self.m_title.as_ref().orNpe(), output)?;
        output.writeInt(self.m_xPosition)?;
        output.writeInt(self.m_yPosition)?;
        output.writeInt(self.m_zPosition)?;
        output.writeInt(self.m_direction)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleAddPainting(self); }

    fn getPacketSize(&self) -> i32 { 24 }
}

use crate::network::net_handler::NetHandler;
use crate::network::packet::{self, Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;
use jvm::string::JString;
use jvm::throwable::OrNpe;

#[derive(Default)]
pub struct Packet20NamedEntitySpawn
{
    m_base: PacketBase,
    pub m_entityId: i32,
    pub m_name: Option<JString>,
    pub m_xPosition: i32,
    pub m_yPosition: i32,
    pub m_zPosition: i32,
    pub m_rotation: i8,
    pub m_pitch: i8,
    pub m_currentItem: i32,
}

impl Packet for Packet20NamedEntitySpawn
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_entityId = input.readInt()?;
        self.m_name = Some(packet::readString(input, 16)?);
        self.m_xPosition = input.readInt()?;
        self.m_yPosition = input.readInt()?;
        self.m_zPosition = input.readInt()?;
        self.m_rotation = input.readByte()?;
        self.m_pitch = input.readByte()?;
        self.m_currentItem = input.readShort()? as i32;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_entityId)?;
        packet::writeString(self.m_name.as_ref().orNpe(), output)?;
        output.writeInt(self.m_xPosition)?;
        output.writeInt(self.m_yPosition)?;
        output.writeInt(self.m_zPosition)?;
        output.writeByte(self.m_rotation as i32)?;
        output.writeByte(self.m_pitch as i32)?;
        output.writeShort(self.m_currentItem)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleNamedEntitySpawn(self); }

    fn getPacketSize(&self) -> i32 { 28 }
}

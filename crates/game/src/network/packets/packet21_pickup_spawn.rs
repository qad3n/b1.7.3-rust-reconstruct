use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;

#[derive(Default)]
pub struct Packet21PickupSpawn
{
    m_base: PacketBase,
    pub m_entityId: i32,
    pub m_xPosition: i32,
    pub m_yPosition: i32,
    pub m_zPosition: i32,
    pub m_rotation: i8,
    pub m_pitch: i8,
    pub m_roll: i8,
    pub m_itemID: i32,
    pub m_count: i32,
    pub m_itemDamage: i32,
}

impl Packet for Packet21PickupSpawn
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_entityId = input.readInt()?;
        self.m_itemID = input.readShort()? as i32;
        self.m_count = input.readByte()? as i32;
        self.m_itemDamage = input.readShort()? as i32;
        self.m_xPosition = input.readInt()?;
        self.m_yPosition = input.readInt()?;
        self.m_zPosition = input.readInt()?;
        self.m_rotation = input.readByte()?;
        self.m_pitch = input.readByte()?;
        self.m_roll = input.readByte()?;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_entityId)?;
        output.writeShort(self.m_itemID)?;
        output.writeByte(self.m_count)?;
        output.writeShort(self.m_itemDamage)?;
        output.writeInt(self.m_xPosition)?;
        output.writeInt(self.m_yPosition)?;
        output.writeInt(self.m_zPosition)?;
        output.writeByte(self.m_rotation as i32)?;
        output.writeByte(self.m_pitch as i32)?;
        output.writeByte(self.m_roll as i32)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handlePickupSpawn(self); }

    fn getPacketSize(&self) -> i32 { 24 }
}

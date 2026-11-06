use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;

#[derive(Default)]
pub struct Packet23VehicleSpawn
{
    m_base: PacketBase,
    pub m_entityId: i32,
    pub m_xPosition: i32,
    pub m_yPosition: i32,
    pub m_zPosition: i32,
    pub m_velocityX: i32,
    pub m_velocityY: i32,
    pub m_velocityZ: i32,
    pub m_type: i32,
    pub m_data: i32,
}

impl Packet for Packet23VehicleSpawn
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_entityId = input.readInt()?;
        self.m_type = input.readByte()? as i32;
        self.m_xPosition = input.readInt()?;
        self.m_yPosition = input.readInt()?;
        self.m_zPosition = input.readInt()?;
        self.m_data = input.readInt()?;
        if self.m_data > 0
        {
            self.m_velocityX = input.readShort()? as i32;
            self.m_velocityY = input.readShort()? as i32;
            self.m_velocityZ = input.readShort()? as i32;
        }
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_entityId)?;
        output.writeByte(self.m_type)?;
        output.writeInt(self.m_xPosition)?;
        output.writeInt(self.m_yPosition)?;
        output.writeInt(self.m_zPosition)?;
        output.writeInt(self.m_data)?;
        if self.m_data > 0
        {
            output.writeShort(self.m_velocityX)?;
            output.writeShort(self.m_velocityY)?;
            output.writeShort(self.m_velocityZ)?;
        }
        Ok(())
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleVehicleSpawn(self); }

    fn getPacketSize(&self) -> i32
    {
        if 21i32.wrapping_add(self.m_data) > 0
        {
            6
        }
        else
        {
            0
        }
    }
}

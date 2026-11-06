use crate::entity::WatchableObject;
use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use crate::port::watchable_data::WatchableData;
use jvm::io::IoResult;
use std::rc::Rc;

#[derive(Default)]
pub struct Packet24MobSpawn
{
    m_base: PacketBase,
    pub m_entityId: i32,
    pub m_type: i8,
    pub m_xPosition: i32,
    pub m_yPosition: i32,
    pub m_zPosition: i32,
    pub m_yaw: i8,
    pub m_pitch: i8,
    m_receivedMetadata: Option<Vec<WatchableData>>,
}

impl Packet24MobSpawn
{
    pub fn getMetadata(&self) -> Option<Vec<Rc<WatchableObject>>>
    {
        WatchableData::toWatchableObjects(self.m_receivedMetadata.as_deref())
    }
}

impl Packet for Packet24MobSpawn
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_entityId = input.readInt()?;
        self.m_type = input.readByte()?;
        self.m_xPosition = input.readInt()?;
        self.m_yPosition = input.readInt()?;
        self.m_zPosition = input.readInt()?;
        self.m_yaw = input.readByte()?;
        self.m_pitch = input.readByte()?;
        self.m_receivedMetadata = WatchableData::readWatchableObjects(input)?;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_entityId)?;
        output.writeByte(self.m_type as i32)?;
        output.writeInt(self.m_xPosition)?;
        output.writeInt(self.m_yPosition)?;
        output.writeInt(self.m_zPosition)?;
        output.writeByte(self.m_yaw as i32)?;
        output.writeByte(self.m_pitch as i32)?;
        jvm::throwable::npe()
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleMobSpawn(self); }

    fn getPacketSize(&self) -> i32 { 20 }
}

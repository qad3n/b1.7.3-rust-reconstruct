use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use jvm::io::IoResult;

#[derive(Default)]
pub struct Packet5PlayerInventory
{
    m_base: PacketBase,
    pub m_entityID: i32,
    pub m_slot: i32,
    pub m_itemID: i32,
    pub m_itemDamage: i32,
}

impl Packet for Packet5PlayerInventory
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_entityID = input.readInt()?;
        self.m_slot = input.readShort()? as i32;
        self.m_itemID = input.readShort()? as i32;
        self.m_itemDamage = input.readShort()? as i32;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeInt(self.m_entityID)?;
        output.writeShort(self.m_slot)?;
        output.writeShort(self.m_itemID)?;
        output.writeShort(self.m_itemDamage)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handlePlayerInventory(self); }

    fn getPacketSize(&self) -> i32 { 8 }
}

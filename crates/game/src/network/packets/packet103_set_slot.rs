use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use crate::port::packet_item_stack::PacketItemStack;
use jvm::io::IoResult;

#[derive(Default)]
pub struct Packet103SetSlot
{
    m_base: PacketBase,
    pub m_windowId: i32,
    pub m_itemSlot: i32,
    pub m_myItemStack: Option<PacketItemStack>,
}

impl Packet for Packet103SetSlot
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_windowId = input.readByte()? as i32;
        self.m_itemSlot = input.readShort()? as i32;
        self.m_myItemStack = PacketItemStack::read(input)?;
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        output.writeByte(self.m_windowId)?;
        output.writeShort(self.m_itemSlot)?;
        PacketItemStack::write(self.m_myItemStack.as_ref(), output)
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleInventoryMenuSlotContent(self); }

    fn getPacketSize(&self) -> i32 { 8 }
}

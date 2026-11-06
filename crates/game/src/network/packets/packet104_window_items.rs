use crate::network::net_handler::NetHandler;
use crate::network::packet::{Packet, PacketBase, PacketInput, PacketOutput};
use crate::port::packet_item_stack::PacketItemStack;
use jvm::io::IoResult;
use jvm::throwable::OrNpe;

#[derive(Default)]
pub struct Packet104WindowItems
{
    m_base: PacketBase,
    pub m_windowId: i32,
    pub m_itemStack: Option<Vec<Option<PacketItemStack>>>,
}

impl Packet for Packet104WindowItems
{
    fn base(&self) -> &PacketBase { &self.m_base }

    fn readPacketData(&mut self, input: &mut PacketInput) -> IoResult<()>
    {
        self.m_windowId = input.readByte()? as i32;
        let short1: i16 = input.readShort()?;
        if short1 < 0
        {
            jvm::throwable::throwBare(&jvm::throwable::NEGATIVE_ARRAY_SIZE);
        }
        let mut items: Vec<Option<PacketItemStack>> = vec![None; short1 as usize];
        for item in &mut items
        {
            let short2: i16 = input.readShort()?;
            if short2 >= 0
            {
                let b0: i8 = input.readByte()?;
                let short3: i16 = input.readShort()?;
                *item = Some(PacketItemStack {
                    m_itemID: short2 as i32,
                    m_stackSize: b0 as i32,
                    m_itemDamage: short3 as i32,
                });
            }
        }
        self.m_itemStack = Some(items);
        Ok(())
    }

    fn writePacketData(&self, output: &mut PacketOutput) -> IoResult<()>
    {
        let items: &Vec<Option<PacketItemStack>> = self.m_itemStack.as_ref().orNpe();
        output.writeByte(self.m_windowId)?;
        output.writeShort(items.len() as i32)?;
        for item in items
        {
            match item
            {
                None => output.writeShort(-1)?,
                Some(item) =>
                {
                    output.writeShort(item.m_itemID as i16 as i32)?;
                    output.writeByte(item.m_stackSize as i8 as i32)?;
                    output.writeShort(item.m_itemDamage as i16 as i32)?;
                }
            }
        }
        Ok(())
    }

    fn processPacket(&self, handler: &dyn NetHandler) { handler.handleInventoryMenuContent(self); }

    fn getPacketSize(&self) -> i32 { 3 + self.m_itemStack.as_ref().orNpe().len() as i32 * 5 }
}

use crate::container::Container;
use crate::item::ItemStack;
use std::rc::Rc;

pub trait ICrafting
{
    fn onSlotChanged(&self, container: &Rc<Container>, slot: i32, item: Option<Rc<ItemStack>>);

    fn onDataChanged(&self, container: &Rc<Container>, id: i32, value: i32);
}

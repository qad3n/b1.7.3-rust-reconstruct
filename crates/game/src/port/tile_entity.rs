// the (TileEntityX) casts of world.getBlockTileEntity results, a wrong class throws ClassCastException like java
use crate::tileentity::TileEntity;
use jvm::throwable::OrNpe;
use std::rc::Rc;

// a cast whose result the caller dereferences right away, so a null throws there
pub fn cast(tileEntity: Option<Rc<TileEntity>>, className: &str) -> Rc<TileEntity>
{
    castNullable(tileEntity, className).orNpe()
}

pub fn castNullable(tileEntity: Option<Rc<TileEntity>>, className: &str) -> Option<Rc<TileEntity>>
{
    if let Some(entity) = &tileEntity
        && !entity.instanceOf(className)
    {
        jvm::throwable::throw(
            &jvm::throwable::CLASS_CAST,
            &format!("net.minecraft.src.{} cannot be cast to net.minecraft.src.{}", entity.className(), className),
        );
    }
    tileEntity
}

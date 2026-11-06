use crate::entity::{Entity, WatchedValue};
use crate::world::World;
use jvm::throwable::{self, CLASS_CAST};
use std::rc::Rc;

// the AbstractMethodError a call into an abstract java method throws
pub fn abstractMethod(name: &str) -> !
{
    throwable::throw(&throwable::ERROR, &format!("java.lang.AbstractMethodError: {name}"))
}

// java reference equality where either side may be null
pub fn sameObject<T>(a: Option<&Rc<T>>, b: Option<&Rc<T>>) -> bool
{
    match (a, b)
    {
        (None, None) => true,
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        _ => false,
    }
}

// the ClassCastException of a DataWatcher getter whose slot holds another boxed type
pub fn watchedCastError(found: &WatchedValue, target: &str) -> !
{
    let name: &str = match found
    {
        WatchedValue::Byte(_) => "java.lang.Byte",
        WatchedValue::Short(_) => "java.lang.Short",
        WatchedValue::Int(_) => "java.lang.Integer",
        WatchedValue::Float(_) => "java.lang.Float",
        WatchedValue::String(_) => "java.lang.String",
        WatchedValue::ItemStack(_) => "net.minecraft.src.ItemStack",
        WatchedValue::ChunkCoordinates(_) => "net.minecraft.src.ChunkCoordinates",
    };
    throwable::throw(&CLASS_CAST, &format!("{name} cannot be cast to {target}"))
}

// playSoundAtEntity takes a sound name java may pass as null, the sound pool finds nothing for null or an empty name
pub fn playSoundAtEntity(world: &World, entity: &Rc<Entity>, sound: Option<String>, volume: f32, pitch: f32)
{
    world.playSoundAtEntity(entity, sound.as_deref().unwrap_or(""), volume, pitch);
}

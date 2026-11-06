use crate::entity::Entity;
use crate::world::{IChunkLoader, WorldInfo, WorldProvider};
use std::path::PathBuf;
use std::rc::Rc;

pub trait ISaveHandler
{
    fn loadWorldInfo(&self) -> Option<Rc<WorldInfo>>;
    fn checkSessionLock(&self);
    fn getChunkLoader(&self, provider: &Rc<WorldProvider>) -> Rc<dyn IChunkLoader>;
    fn saveWorldInfoAndPlayer(&self, info: &Rc<WorldInfo>, players: &[Rc<Entity>]);
    fn saveWorldInfo(&self, info: &Rc<WorldInfo>);
    fn getDataFile(&self, name: &str) -> Option<PathBuf>;
}

use game::entity::{Entity, entity_player};

// the (EntityPlayer) cast java applies before calling a player method
pub fn castToPlayer(entity: &Entity) { entity_player::fields(entity); }

#[deriving(Eq, TotalEq, Hash, Ord, Show)]
pub struct Entity {
    pub id: u64
}

impl Entity {
    pub fn new(id: u64)-> Entity {
        Entity { id: id }
    }
}

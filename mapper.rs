extern crate collections;

use entity::Entity;
use collections::hashmap::HashMap;

mod entity;

pub struct Mapper<C> {
    map: HashMap<Entity, C>
}

impl <C> Mapper <C> {
    pub fn new()-> Mapper<C> {
        Mapper {
            map: HashMap::new()
        }
    }
}
impl <C: 'static> Mapper<C> {
    /// Maps a component to an entity.
    /// Returns true if a component was already
    /// mapped to by this Entity.
    pub fn put(&mut self, e: Entity, c: C)-> bool {
        !self.map.insert(e, c)
    }
    /// Retrieves an entity from the mapping.
    pub fn get<'a> (&'a mut self, e: Entity) -> Option<&'a mut C> {
        self.map.find_mut(&e)
    }
    /// Removes an entity from the mapping.
    /// Returns true if an entity was actually removed.
    pub fn del(&mut self, e: Entity)-> bool {
        self.map.remove(&e)
    }
}

#[cfg(test)]
#[deriving(Eq)]
struct Comp {
    x: uint,
    y: uint
}

#[test]
fn test_insert() {
    let mut mapper = Mapper::new();

    assert!(!mapper.put(Entity{id: 0}, Comp{x: 0, y:1}));

    {
        let g1 = mapper.get(Entity{id: 0});
        assert!(g1.is_some());
        assert!(*g1.unwrap() == Comp{x: 0, y: 1});
    }

    assert!(mapper.get(Entity{id: 1}).is_none());
}

#[test]
fn test_override() {
    let mut mapper = Mapper::new();
    {
        assert!(!mapper.put(Entity{id: 0}, Comp{x: 0, y:1}));
        let g1 = mapper.get(Entity{id: 0});
        assert!(g1.is_some());
        assert!(*g1.unwrap() == Comp{x: 0, y: 1});
    } {
        assert!(mapper.put(Entity{id: 0}, Comp{x: 1, y:2}));
        let g2 = mapper.get(Entity{id: 0});
        assert!(g2.is_some());
        assert!(*g2.unwrap() == Comp{x: 1, y: 2});
    }
}

#[test]
fn test_remove() {
    let mut mapper = Mapper::new();
    {
        assert!(!mapper.put(Entity{id: 0}, Comp{x: 0, y:1}));
        let g1 = mapper.get(Entity{id: 0});
        assert!(g1.is_some());
        assert!(*g1.unwrap() == Comp{x: 0, y: 1});
    } {
        assert!(mapper.del(Entity{id: 0}));
        let g2 = mapper.get(Entity{id: 0});
        assert!(g2.is_none());
    }
}

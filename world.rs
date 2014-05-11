#![allow(dead_code)]
extern crate core;
extern crate collections;

use entity::Entity;
use router::Router;
use collections::HashSet;

mod entity;
mod router;
mod mapper;

pub struct World {
    router: Router,
    e_id_counter: u64,
    ids: Vec<u64>,
    id_set: HashSet<u64>
}

impl World {
    pub fn new()-> World {
        World {
            router: Router::new(),
            e_id_counter: 0,
            ids: Vec::new(),
            id_set: HashSet::new()
        }
    }

    pub fn spawn(&mut self)-> Entity {
        self.e_id_counter += 1;
        self.id_set.insert(self.e_id_counter);
        Entity::new(self.e_id_counter)
    }

    pub fn spawn_with_id(&mut self, id: u64)-> Entity {
        Entity::new(id)
    }

    pub fn router<'a>(&'a mut self)-> &'a mut Router {
        &mut self.router
    }
}

fn main() {}

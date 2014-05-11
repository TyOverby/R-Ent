#![allow(dead_code)]
extern crate collections;
extern crate core;

use entity::Entity;
use mapper::Mapper;
use collections::hashmap::HashMap;
use core::intrinsics::TypeId;
use core::any::Any;
use core::any::AnyMutRefExt;

mod entity;
mod mapper;

pub struct Router {
    map: HashMap<TypeId, Box<Any>>
}

impl Router {
    pub fn new()-> Router {
        return Router {map: HashMap::new()}
    }

    fn get_mapper<'a, C: 'static>(&'a mut self)-> Option<&'a mut Mapper<C>> {
        let found : Option<&'a mut Box<Any>>  = self.map.find_mut(& TypeId::of::<C>());
        match found {
            Some(mapper) => mapper.as_mut::<Mapper<C>>(),
            None => None
        }
    }

    fn init_for<C: 'static>(&mut self){
        let m: Box<Any> = box Mapper::<C>::new();
        self.map.insert(TypeId::of::<C>(), m);
    }

    pub fn set<'a, C: 'static>(&'a mut self, e: Entity, c: C) -> bool {
        if !self.map.contains_key(&TypeId::of::<C>()) {
            self.init_for::<C>();
        }
        match self.get_mapper::<C>() {
            Some(mapper) => {
                mapper.set(e, c);
                true
            },
            None => {
                false
            }
        }
    }

    pub fn get<'a, C: 'static>(&'a mut self, e: Entity) -> Option<&'a mut C> {
        let mapper_opt = self.get_mapper::<C>();
        match mapper_opt {
            Some(mapper) => mapper.get(e),
            None => None
        }
    }

    pub fn del<C: 'static>(&mut self, e: Entity) {
        match self.get_mapper::<C>() {
            Some(mapper) => { mapper.del(e); }
            None => { }
        };
    }
}

#[cfg(test)]
mod router_tests{
    use entity::Entity;
    use super::Router;
    #[deriving(Eq)]
    struct Vel {
        vx: uint,
        vy: uint
    }

    impl Vel {
        fn new(vx: uint, vy: uint)-> Vel {
            Vel {vx: vx, vy: vy}
        }
    }

    #[deriving(Eq)]
    struct Hidden {
        hidden: bool
    }

    impl Hidden {
        fn new(h: bool)-> Hidden {
            Hidden { hidden: h }
        }
    }


    #[test]
    fn test_insert() {
        let mut router = Router::new();
        {
            router.set(Entity::new(0), Vel::new(0,5));
            let v0 = router.get::<Vel>(Entity::new(0));
            assert!(v0.is_some());
            assert!(*v0.unwrap() == Vel::new(0,5));
        }
        {
            router.set(Entity::new(0), Hidden::new(true));
            let v0 = router.get::<Hidden>(Entity::new(0));
            assert!(v0.is_some());
            assert!(*v0.unwrap() == Hidden::new(true));
        }
    }

    #[test]
    fn test_replace() {
        let mut router = Router::new();
        {
            router.set(Entity::new(0), Vel::new(0,5));
            let v0 = router.get::<Vel>(Entity::new(0));
            assert!(v0.is_some());
            assert!(*v0.unwrap() == Vel::new(0,5));
        }
        {
            router.set(Entity::new(0), Vel::new(1,1));
            let v0 = router.get::<Vel>(Entity::new(0));
            assert!(v0.is_some());
            assert!(*v0.unwrap() == Vel::new(1,1));
        }
    }

    #[test]
    fn test_remove() {
        let mut router = Router::new();
        {
            router.set(Entity::new(0), Vel::new(0,5));
            let v0 = router.get::<Vel>(Entity::new(0));
            assert!(v0.is_some());
            assert!(*v0.unwrap() == Vel::new(0,5));
        }
        {
            router.del::<Vel>(Entity::new(0));
            let v0 = router.get::<Vel>(Entity::new(0));
            assert!(v0.is_none());
        }
    }
}

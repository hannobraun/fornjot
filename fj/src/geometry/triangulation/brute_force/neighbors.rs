use std::collections::{BTreeMap, BTreeSet};

use crate::geometry::point::Pnt2;

pub struct Neighbors(pub BTreeMap<Pnt2, BTreeSet<Pnt2>>);

impl Neighbors {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, a: impl Into<Pnt2>, b: impl Into<Pnt2>) {
        let a = a.into();
        let b = b.into();

        self.0.entry(a).or_insert(BTreeSet::new()).insert(b);
        self.0.entry(b).or_insert(BTreeSet::new()).insert(a);
    }

    pub fn remove(&mut self, p: Pnt2) {
        self.0.remove(&p);
        for neighbors in self.0.values_mut() {
            neighbors.remove(&p);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> Pnt2 {
        *self.0.keys().next().unwrap()
    }

    pub fn of(&self, point: Pnt2) -> impl Iterator<Item = Pnt2> + '_ {
        self.0.get(&point).unwrap().iter().map(|&point| point)
    }
}

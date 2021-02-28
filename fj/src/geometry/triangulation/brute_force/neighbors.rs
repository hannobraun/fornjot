use std::collections::{BTreeMap, BTreeSet};

use decorum::R32;
use nalgebra::Point2;

pub struct Neighbors(pub BTreeMap<Point, BTreeSet<Point>>);

impl Neighbors {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, a: Point2<f32>, b: Point2<f32>) {
        let a = a.map(|value| R32::from_inner(value));
        let b = b.map(|value| R32::from_inner(value));

        let a = (a.x, a.y);
        let b = (b.x, b.y);

        self.0.entry(a).or_insert(BTreeSet::new()).insert(b);
        self.0.entry(b).or_insert(BTreeSet::new()).insert(a);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> Point {
        *self.0.keys().next().unwrap()
    }

    pub fn of(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        self.0.get(&point).unwrap().iter().map(|&point| point)
    }
}

pub type Point = (R32, R32);

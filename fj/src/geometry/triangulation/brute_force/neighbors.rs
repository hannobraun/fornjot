use std::collections::{BTreeMap, BTreeSet};

use decorum::R32;
use nalgebra::Point2;

pub struct Neighbors(pub BTreeMap<Point, BTreeSet<Point>>);

impl Neighbors {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, a: impl Into<Point>, b: impl Into<Point>) {
        let a = a.into();
        let b = b.into();

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

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point(pub R32, pub R32);

impl From<Point2<f32>> for Point {
    fn from(p: Point2<f32>) -> Self {
        let p = p.map(|value| R32::from_inner(value));
        Point(p.x, p.y)
    }
}

impl From<Point> for Point2<f32> {
    fn from(p: Point) -> Self {
        Point2::new(p.0.into_inner(), p.1.into_inner())
    }
}

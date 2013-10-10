// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use point::Point2D;
use size::Size2D;
use std::cmp::{Eq, Ord};

#[deriving(Eq, Clone)]
pub struct Rect<T> {
    origin: Point2D<T>,
    size: Size2D<T>,
}

pub fn Rect<T:Clone + Ord + Add<T,T> + Sub<T,T>>(origin: Point2D<T>,
                                                 size: Size2D<T>)
        -> Rect<T> {
    return Rect {
        origin: origin,
        size: size
    }
}

impl<T: Clone + Ord + Add<T,T> + Sub<T,T>> Rect<T> {
    pub fn intersects(&self, other: &Rect<T>) -> bool {
        self.origin.x < other.origin.x + other.size.width &&
       other.origin.x <  self.origin.x + self.size.width &&
        self.origin.y < other.origin.y + other.size.height &&
       other.origin.y <  self.origin.y + self.size.height
    }

    pub fn intersection(&self, other: &Rect<T>) -> Option<Rect<T>> {
        if !self.intersects(other) {
            return None;
        }

        let upper_left = Point2D(max(self.origin.x.clone(), other.origin.x.clone()),
                                 max(self.origin.y.clone(), other.origin.y.clone()));
        
        let lower_right = Point2D(min(self.origin.x + self.size.width,
                                      other.origin.x + other.size.width),
                                  min(self.origin.y + self.size.height,
                                      other.origin.y + other.size.height));
            
        Some(Rect(upper_left.clone(), Size2D(lower_right.x - upper_left.x,
                                             lower_right.y - upper_left.y)))
    }

    pub fn union(&self, other: &Rect<T>) -> Rect<T> {
        let upper_left = Point2D(min(self.origin.x.clone(), other.origin.x.clone()),
                                 min(self.origin.y.clone(), other.origin.y.clone()));
        
        let lower_right = Point2D(max(self.origin.x + self.size.width,
                                      other.origin.x + other.size.width),
                                  max(self.origin.y + self.size.height,
                                      other.origin.y + other.size.height));
        
        Rect {
            origin: upper_left.clone(),
            size: Size2D(lower_right.x - upper_left.x, lower_right.y - upper_left.y)
        }
    }

    pub fn translate(&self, other: &Point2D<T>) -> Rect<T> {
        Rect {
            origin: Point2D(self.origin.x + other.x, self.origin.y + other.y),
            size: self.size.clone()
        }
    }
}

pub fn min<T:Clone + Ord>(x: T, y: T) -> T {
    if x <= y { x } else { y }
}

pub fn max<T:Clone + Ord>(x: T, y: T) -> T {
    if x >= y { x } else { y }
}

#[test]
fn test_min_max() {
    assert!(min(0, 1) == 0);
    assert!(min(-1.0, 0.0) == -1.0);

    assert!(max(0, 1) == 1);
    assert!(max(-1.0, 0.0) == 0.0);
}

#[test]
fn test_translate() {
    let p = Rect(Point2D(0, 0), Size2D(50, 40));
    let pp = p.translate(&Point2D(10,15));

    assert!(pp.size.width == 50);
    assert!(pp.size.height == 40);
    assert!(pp.origin.x == 10);
    assert!(pp.origin.y == 15);


    let r = Rect(Point2D(-10, -5), Size2D(50, 40));
    let rr = r.translate(&Point2D(0,-10));

    assert!(rr.size.width == 50);
    assert!(rr.size.height == 40);
    assert!(rr.origin.x == -10);
    assert!(rr.origin.y == -15);
}

#[test]
fn test_union() {
    let p = Rect(Point2D(0,0), Size2D(50, 40));
    let q = Rect(Point2D(20,20), Size2D(5, 5));
    let r = Rect(Point2D(-15, -30), Size2D(200, 15));
    let s = Rect(Point2D(20, -15), Size2D(250, 200));

    let pq = p.union(&q);
    assert!(pq.origin == Point2D(0, 0));
    assert!(pq.size == Size2D(50, 40));

    let pr = p.union(&r);
    assert!(pr.origin == Point2D(-15, -30));
    assert!(pr.size == Size2D(200, 70));

    let ps = p.union(&s);
    assert!(ps.origin == Point2D(0, -15));
    assert!(ps.size == Size2D(270, 200));

}

#[test]
fn test_intersection() {
    let p = Rect(Point2D(0, 0), Size2D(10, 20));
    let q = Rect(Point2D(5, 15), Size2D(10, 10));
    let r = Rect(Point2D(-5, -5), Size2D(8, 8));

    let pq = p.intersection(&q);
    assert!(pq.is_some());
    let pq = pq.unwrap();
    assert!(pq.origin == Point2D(5, 15));
    assert!(pq.size == Size2D(5, 5));
    
    let pr = p.intersection(&r);
    assert!(pr.is_some());
    let pr = pr.unwrap();
    assert!(pr.origin == Point2D(0, 0));
    assert!(pr.size == Size2D(3, 3));

    let qr = q.intersection(&r);
    assert!(qr.is_none());
}

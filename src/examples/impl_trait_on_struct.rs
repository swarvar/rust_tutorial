#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const PI: f32 = 3.142;

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        length: f32,
        width: f32,
    };

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area: {}", rec.area());
    println!("Circ Area: {}", circ.area());
}

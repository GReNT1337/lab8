use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;
use std::fmt;

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Relation {
    Inside,
    Border,
    Outside,
}

impl fmt::Display for Relation {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Outside => formatter.write_str("outside"),
            Inside => formatter.write_str("inside"),
            Border => formatter.write_str("border"),
        }
    }
}

use Relation::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MyPoint {
    #[serde(deserialize_with = "deserialize_coord")]
    pub x: i32,
    #[serde(deserialize_with = "deserialize_coord")]
    pub y: i32,
}

pub fn deserialize_coord<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let c = i32::deserialize(deserializer)?;
    if c.abs() >= 100 {
        Err(serde::de::Error::custom("ERROR: out of range"))
    } else {
        Ok(c)
    }
}

fn distance_relation(distance: i32, border: i32) -> Relation {
    use std::cmp::Ordering::*;

    match distance.cmp(&border) {
        Less => Inside,
        Equal => Border,
        Greater => Outside,
    }
}

fn box_calc(x: i32, y: i32, border: i32) -> Relation {
    let x = x.abs();
    let y = y.abs();
    let dist = if y > x { y } else { x };

    distance_relation(dist, border)
}

fn radii_calc(x: i32, y: i32, border: i32) -> Relation {
    distance_relation(x * x + y * y, border * border)
}

fn partition(
    lo: impl Fn(i32, i32, i32) -> Relation,
    h1: impl Fn(i32, i32, i32) -> Relation,
    x: i32,
    y: i32,
    border_inner: i32,
    border_outer: i32,
) -> Relation {
    match lo(x, y, border_inner) {
        Border => Border,
        Inside => Outside,
        Outside => match h1(x, y, border_outer) {
            Border => Border,
            Inside => Inside,
            Outside => Outside,
        },
    }
}

pub fn point_location1(x: i32, y: i32) -> Relation {
    let border_inner = 10;
    let border_outer = 20;
    #[allow(clippy::collapsible_else_if)]
    if x > 0 {
        if y > 0 {
            partition(box_calc, radii_calc, x, y, border_inner, border_outer)
        } else {
            partition(radii_calc, radii_calc, x, y, border_inner, border_outer)
        }
    } else {
        if y > 0 {
            partition(radii_calc, box_calc, x, y, border_inner, border_outer)
        } else {
            partition(box_calc, box_calc, x, y, border_inner, border_outer)
        }
    }
}

pub fn point_location2(x: i32, y: i32) -> Relation {
    let border_inner = 20;
    let border_outer = 40;
    #[allow(clippy::collapsible_else_if)]
    if x > 0 {
        if y > 0 {
            partition(box_calc, radii_calc, x, y, border_inner, border_outer)
        } else {
            partition(radii_calc, box_calc, x, y, border_inner, border_outer)
        }
    } else {
        if y > 0 {
            partition(radii_calc, box_calc, x, y, border_inner, border_outer)
        } else {
            partition(radii_calc, box_calc, x, y, border_inner, border_outer)
        }
    }
}

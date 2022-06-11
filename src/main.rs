use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;
use warp::{http::Response, Filter};

#[cfg_attr(test, derive(PartialEq, Debug))]
enum Relation {
    Inside,
    Border,
    Outside,
}

use Relation::*;

enum Error {
    BadFormat,
    OutOfRange,
    EmptyString,
    OneCoord,
    TooMuchCoords,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MyPoint {
    #[serde(deserialize_with = "deserialize_coord")]
    x: i32,
    #[serde(deserialize_with = "deserialize_coord")]
    y: i32,
}

pub fn deserialize_coord<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let c = i32::deserialize(deserializer)?;
    if c >= 100 {
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
) -> Relation {
    match lo(x, y, 10) {
        Border => Border,
        Inside => Outside,
        Outside => match h1(x, y, 20) {
            Border => Border,
            Inside => Inside,
            Outside => Outside,
        },
    }
}

fn point_location(x: i32, y: i32) -> Relation {
    #[allow(clippy::collapsible_else_if)]
    if x > 0 {
        if y > 0 {
            partition(box_calc, radii_calc, x, y)
        } else {
            partition(radii_calc, radii_calc, x, y)
        }
    } else {
        if y > 0 {
            partition(radii_calc, box_calc, x, y)
        } else {
            partition(box_calc, box_calc, x, y)
        }
    }
}

fn parse_coord(coord: &str) -> Result<i32, Error> {
    match coord.parse() {
        Ok(coord @ -100..=100) => Ok(coord),
        Ok(_) => Err(Error::OutOfRange),
        Err(_) => Err(Error::BadFormat),
    }
}

fn set_point_location(line: String) -> Result<Relation, Error> {
    let mut iter = line.split_ascii_whitespace();
    let x = iter.next().ok_or(Error::EmptyString)?;
    let x = parse_coord(x)?;

    let y = iter.next().ok_or(Error::OneCoord)?;
    let y = parse_coord(y)?;

    match iter.next() {
        Some(_) => Err(Error::TooMuchCoords),
        None => Ok(point_location(x, y)),
    }
}

fn format_result(result: Result<Relation, Error>) -> &'static str {
    match result {
        Ok(Outside) => "outside",
        Ok(Inside) => "inside",
        Ok(Border) => "border",
        Err(Error::BadFormat) => "error: bad format",
        Err(Error::EmptyString) => "error: empty string",
        Err(Error::OutOfRange) => "error: out of range",
        Err(Error::OneCoord) => "error: one coord",
        Err(Error::TooMuchCoords) => "error: too much coords",
    }
}

pub fn figure() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("figure"))
        .and(warp::query::<MyPoint>())
        .map(|p: MyPoint| {
            let x: String = p.x.to_string();
            let y: String = p.y.to_string();
            let line = x + " " + &y;
            let res = set_point_location(line);
            let result = format_result(res);
            Response::builder().body(result.to_string())
        })
}

#[tokio::main]
async fn main() {
    let ans = figure();
    warp::serve(ans).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::*;

    #[tokio::test]
    async fn test_get() {
        let resp = request()
            .method("GET")
            .path("/figure?x=10&y=10")
            .reply(&figure())
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(), "border");
    }

    #[tokio::test]
    async fn test_too_much_coord_error() {
        let resp = request()
            .method("GET")
            .path("/figure?x=10&y=10&z=23")
            .reply(&figure())
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}

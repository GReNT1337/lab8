use std::io;
use serde_derive::{
    Deserialize,
    Serialize,
};
use std::io::BufRead;
use warp::{
    http::{Response, StatusCode},
    Filter,
};

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
#[derive(Deserialize, Serialize)]
struct MyPoint {
    x: i32,
    y: i32,
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

#[tokio::main]
async fn main() {    
    
    let ans = warp::get()
        .and(warp::path("figure"))
        .and(warp::query::<MyPoint>())
        .map(|p: MyPoint| {
            Response::builder().body(format!("x={}, y={}", p.x, p.y));
            let x: String = p.x.to_string();
            let y: String = p.y.to_string();
            let line = x + " " + &y;
            let res = set_point_location(line);
            let result = format_result(res);
            Response::builder().body(format!("{}", result))
        });

    warp::serve(ans)
    .run(([127,0,0,1], 3030))
    .await;
    
     
    
}
// async fn main() {
//     for line in io::stdin().lock().lines() {
//         let line = line.expect("Can't read line from STDIN");
//         let res = set_point_location(line);
//         let result = warp::path("Figure").param(format_result(res));
//         //println!("{}", format_result(res));
//     }
//     warp::serve(result)
//         .run(([127, 0, 0, 1], 3030))
//         .await;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_radii() {
        assert_eq!(radii_calc(-2, 4, 5), Inside);
        assert_eq!(radii_calc(-3, -4, 5), Border);
        assert_eq!(radii_calc(30, 40, 20), Outside);
    }
    #[test]
    fn check_box() {
        assert_eq!(box_calc(-2, 4, 5), Inside);
        assert_eq!(box_calc(-3, -5, 5), Border);
        assert_eq!(box_calc(30, 40, 20), Outside);
    }
    #[test]
    fn check_figure() {
        assert_eq!(point_location(-2, 4), Outside);
    }
}

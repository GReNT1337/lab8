use warp::{http::Response, Filter};

mod figure;

fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let figure1 = warp::path("figure-1")
        .and(warp::query::<figure::MyPoint>())
        .map(|p: figure::MyPoint| {
            let relation = figure::point_location1(p.x, p.y);
            Response::builder().body(relation.to_string())
        });

    let figure2 = warp::path("figure-2")
        .and(warp::query::<figure::MyPoint>())
        .map(|p: figure::MyPoint| {
            //TODO: implement as new logic for figure-2
            let relation = figure::point_location2(p.x, p.y);
            Response::builder().body(relation.to_string())
        });

    warp::get().and(figure1.or(figure2))
}

#[tokio::main]
async fn main() {
    warp::serve(routes()).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests;

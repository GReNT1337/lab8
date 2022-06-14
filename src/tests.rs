use super::*;
use warp::http::StatusCode;
use warp::test::request;

//Figure-1 tests
#[tokio::test]
async fn figure1_test_get_border() {
    let resp = request()
        .method("GET")
        .path("/figure-1?x=10&y=10")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "border");
}
#[tokio::test]
async fn figure1_test_get_outside() {
    let resp = request()
        .method("GET")
        .path("/figure-1?x=5&y=5")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "outside");
}
#[tokio::test]
async fn figure1_test_get_outside1() {
    let resp = request()
        .method("GET")
        .path("/figure-1?x=20&y=15")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "outside");
}
#[tokio::test]
async fn figure1_test_get_inside() {
    let resp = request()
        .method("GET")
        .path("/figure-1?x=10&y=15")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "inside");
}
#[tokio::test]
async fn figure1_test_too_much_coord_error() {
    let resp = request()
        .method("GET")
        .path("/figure-1?x=10&y=10&z=23")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
#[tokio::test]
async fn figure1_test_variable_not_defined_error() {
    let resp = request()
        .method("GET")
        .path("/figure-1?")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

//Figure-2 tests
#[tokio::test]
async fn figure2_test_get_border() {
    let resp = request()
        .method("GET")
        .path("/figure-2?x=20&y=20")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "border");
}
#[tokio::test]
async fn figure2_test_get_outside() {
    let resp = request()
        .method("GET")
        .path("/figure-2?x=11&y=10")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "outside");
}
#[tokio::test]
async fn figure2_test_get_outside1() {
    let resp = request()
        .method("GET")
        .path("/figure-2?x=43&y=41")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "outside");
}
#[tokio::test]
async fn figure2_test_get_inside() {
    let resp = request()
        .method("GET")
        .path("/figure-2?x=-25&y=15")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "inside");
}
#[tokio::test]
async fn figure2_test_too_much_coord_error() {
    let resp = request()
        .method("GET")
        .path("/figure-2?x=10&y=10&z=23")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
#[tokio::test]
async fn figure2_test_variable_not_defined_error() {
    let resp = request()
        .method("GET")
        .path("/figure-2?")
        .reply(&routes())
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

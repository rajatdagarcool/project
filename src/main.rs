use warp::Filter;
//using custom headers
#[tokio::main]
async fn main(){
    let html_route = warp::path("complex").map(|| {
        warp::reply::with_header(
            warp::reply::with_status("Complex Response", warp::http::StatusCode::CREATED),
            "X-Complex-Header", "ComplexValue")
    });

    warp::serve(html_route).run(([127,0,0,1],8080)).await;
}


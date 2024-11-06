use warp::Filter;
use std::path::Path;

#[tokio::main]
async fn main(){
    //Define the route for serving the HTML file

    let html_route = warp::path::end()
        .and(warp::fs::file("index.html"));

    warp::serve(html_route).run(([127,0,0,1],8080)).await;
}

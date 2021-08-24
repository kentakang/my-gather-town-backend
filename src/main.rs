use futures::{FutureExt, StreamExt};
use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(|| "Hello, warp!");
    let websocket_hello = warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();

                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });
    let routes = warp::get()
        .and(
            index.or(websocket_hello)
        );
    
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
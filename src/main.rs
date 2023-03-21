use warp::Filter;

#[tokio::main]
async fn main() {
    // GETリクエストのルートパスに対してレスポンスを返す
    let hello = warp::path::end()
        .map(|| "こんにちは、Rustで作られたWebアプリケーションです！");

    // ルートを指定し、サーバーを起動します。
    println!("サーバーが起動しました。http://localhost:3030 にアクセスしてください。");
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
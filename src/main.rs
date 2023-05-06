// use axum::{response::IntoResponse, routing::get, Json, Router};
// use serde::Serialize;

// #[tokio::main]
// async fn main() {
//     let app_router = Router::new()
//         .route("/health1", get(|| async { "Works!" }))
//         .route("/health2", get(health_2_handler));

//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app_router.into_make_service())
//         .await
//         .unwrap();
// }

// #[derive(Serialize)]
// struct Health2Response {
//     message: String,
// }

// async fn health_2_handler() -> impl IntoResponse {
//     let result = Health2Response {
//         message: "It Works!!".to_string(),
//     };

//     return Json(result);
// }

fn main() {
    let new_vec = vec![1, 2, 3].iter().map(|x| x + 1).collect::<Vec<_>>();
    println!("{:?}", new_vec);

    let new_vec_2: Vec<i32> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
    println!("{:?}", new_vec_2);

    let new_vec_3: Vec<i32> = vec![1, 2, 5, 9, 4, 7]
        .into_iter()
        .skip(2)
        .take_while(|&x| x > 4)
        .collect();

    println!("{:?}", new_vec_3);
}

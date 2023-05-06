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

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn is_green(&self) -> bool {
        // if let Color::Green = self {
        //     return true;
        // }

        // return false;

        return matches!(&self, Color::Green);
    }
}

struct Custom {
    age: usize,
    name: String,
}

enum Entry {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(entries: &mut Vec<Entry>) {
    entries.push(Entry::String(String::from("test")));
    entries.push(Entry::Number(2));
    entries.push(Entry::MyCustom(Custom {
        age: 25,
        name: String::from("John"),
    }))
}

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

    let car_1_color = Color::Green;
    let car_2_color = Color::Red;
    println!("Is car 1 green? {}", car_1_color.is_green());
    println!("Is car 2 green? {}", car_2_color.is_green());
}

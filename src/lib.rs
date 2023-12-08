use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Json, Params, Request, Router};
use spin_sdk::http_component;
use std::cmp;

#[derive(Debug, Deserialize, Clone)]
struct Input {
    kids: Vec<i8>,
    weight: Vec<i8>,
    capacity: i8,
}

#[derive(Debug, Deserialize, Clone)]
struct Item {
    kids: i8,
    weight: i8,
}

#[derive(Debug, Clone, Serialize)]
struct Output {
    kids: i8,
}

fn knapsack(items: Vec<Item>, max_weight: i8) -> Vec<Item> {
    let mut best_value = vec![vec![0; max_weight as usize + 1]; items.len() + 1];
    for (i, it) in items.iter().enumerate() {
        for w in 1..max_weight + 1 {
            best_value[i + 1][w as usize] = if it.weight > w {
                best_value[i][w as usize]
            } else {
                cmp::max(
                    best_value[i][w as usize],
                    best_value[i][(w - it.weight) as usize] + it.kids,
                )
            }
        }
    }

    let mut res = Vec::with_capacity(items.len());
    let mut left_weight = max_weight as usize;

    for (i, it) in items.iter().enumerate().rev() {
        if best_value[i + 1][left_weight] != best_value[i][left_weight] {
            res.push(it.clone());
            left_weight -= it.weight as usize;
        }
    }

    res
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_advent_of_spin_challenge_two(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let mut router = Router::new();
    router.get("/", get_handler);
    router.post("/", post_handler);

    Ok(router.handle(req))
}

fn get_handler(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(http::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body("{}".to_string())?)
}

fn post_handler(
    req: http::Request<Json<Input>>,
    _params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let input_json = req.body();
    let kids = &input_json.kids;
    let weights = &input_json.weight;
    let capacity = input_json.capacity;

    println!("This is an Input struct {input_json:?}");

    let zipped = kids.iter().zip(weights.iter());
    let mut items = Vec::new();

    for (k, w) in zipped {
        let item = Item {
            kids: *k,
            weight: *w,
        };

        items.push(item);
    }

    let table = knapsack(items, capacity);
    let total_weight = table.iter().map(|x| x.weight).sum::<i8>();
    let total_kids = table.iter().map(|x| x.kids).sum::<i8>();

    println!("Total weight: {total_weight}");
    println!("Total kids: {total_kids}");

    let response_body =
        serde_json::to_string(&Output { kids: total_kids }).unwrap_or("".to_string());

    Ok(http::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(response_body)?)
}

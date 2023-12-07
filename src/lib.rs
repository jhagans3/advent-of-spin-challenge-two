use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;
use std::cmp;

#[derive(Debug, Deserialize)]
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

    let body = req.body();
    let json_result = std::str::from_utf8(body);

    let response = if let Ok(json_str) = json_result {
        let input: Input = serde_json::from_str(json_str).unwrap();
        println!("This is a JSON str {json_str:?}");
        println!("This is an Input struct {input:?}");

        let zipped = input.kids.iter().zip(input.weight.iter());
        let mut items = Vec::new();

        for (k, w) in zipped {
            let item = Item {
                kids: *k,
                weight: *w,
            };

            items.push(item);
        }

        let table = knapsack(items, input.capacity);
        let total_weight = table.iter().map(|x| x.weight).sum::<i8>();
        let total_kids = table.iter().map(|x| x.kids).sum::<i8>();

        println!("Total weight: {total_weight}");
        println!("Total kids: {total_kids}");

        let response_body =
            serde_json::to_string(&Output { kids: total_kids }).unwrap_or("".to_string());

        http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(response_body)?
    } else {
        http::Response::builder().status(400).body("".to_string())?
    };

    Ok(response)
}

use pest::iterators::Pair;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    product_name: String,
    skin_type: String,
    ingredients: String,
    rating: f64,
    price: f64,
    user_ratings: Vec<f64>,
    recommendations: String,
    reviews: Vec<String>,
    availability: bool,
}

impl Product {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let mut product_name = String::new();
        let mut skin_type = String::new();
        let mut ingredients = String::new();
        let mut rating = 0.0;
        let mut price = 0.0;
        let mut user_ratings = Vec::new();
        let mut recommendations = String::new();
        let mut reviews = Vec::new();
        let mut availability = false;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::product_name => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    product_name = inner.next().map_or(String::new(), |p: Pair<'_, Rule>| {
                        p.as_str().trim().to_string()
                    })
                }
                Rule::skin_type => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    skin_type = inner.next().map_or(String::new(), |p: Pair<'_, Rule>| {
                        p.as_str().trim().to_string()
                    })
                }
                Rule::ingredients => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    ingredients = inner.next().map_or(String::new(), |p: Pair<'_, Rule>| {
                        p.as_str().trim().to_string()
                    })
                }
                Rule::rating => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    rating = inner.next().map_or(0.0, |p: Pair<'_, Rule>| {
                        p.as_str().trim().parse::<f64>().unwrap_or(0.0)
                    });
                }
                Rule::price => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    price = inner.next().map_or(0.0, |p: Pair<'_, Rule>| {
                        p.as_str().trim().parse::<f64>().unwrap_or(0.0)
                    });
                }
                Rule::user_ratings => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    user_ratings = inner
                        .next()
                        .map_or(String::new(), |p: Pair<'_, Rule>| {
                            p.as_str().trim().to_string()
                        })
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                        .split(',')
                        .filter_map(|s| s.trim().parse::<f64>().ok())
                        .collect();
                }
                Rule::recommendations => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    recommendations = inner.next().map_or(String::new(), |p: Pair<'_, Rule>| {
                        p.as_str().trim().to_string()
                    })
                }
                Rule::reviews => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    inner.next();
                    reviews = inner
                        .map(|r: Pair<'_, Rule>| r.as_str().trim().to_string())
                        .collect()
                }
                Rule::availability => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    availability = inner
                        .next()
                        .map_or(false, |p: Pair<'_, Rule>| p.as_str() == "true")
                }
                _ => {}
            }
        }
        Product {
            product_name,
            skin_type,
            ingredients,
            rating,
            price,
            user_ratings,
            recommendations,
            reviews,
            availability,
        }
    }
}

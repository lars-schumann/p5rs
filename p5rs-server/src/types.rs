use serde::Serialize;
use typeshare::*;

#[derive(Serialize)]
#[typeshare]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub favourite_food: Option<String>,
}


//typeshare . --lang=typescript --output-file=types.ts


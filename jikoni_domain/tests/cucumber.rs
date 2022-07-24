// use std::convert::Infallible;

// use async_trait::async_trait;
// use bdd::Cat;
// use cucumber::{WorldInit, World};
// use steps::operations::Operations;

// mod steps;

// #[derive(Debug, WorldInit)]
// pub struct MyWorld {
//     pub cat: Cat,
//     pub ops: Operations,
// }

// #[async_trait(?Send)]
// impl World for MyWorld {
//     type Error = Infallible;

//     async fn new() -> Result<Self, Infallible> {
//         Ok(Self {
//             cat: Cat { hungry: false },
//             ops: Operations::Init,
//         })
//     }
// }

#[tokio::main]
async fn main() {
    // MyWorld::run("./features").await;
}

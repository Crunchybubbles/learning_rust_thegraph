// // use std::fs;
// // use serde::{Deserialize, Serialize};
// // use serde_json::Result;
// //
// // // // use std::collections::Hashmap;
// // //
// // //
// // #[derive(Serialize, Deserialize, Debug)]
// // struct Pooldata {
// //     pools: Vec<Univ3pool>
// // }
// //
// //
// // #[derive(Serialize, Deserialize, Debug)]
// // struct Univ3pool {
// //     id: String,
// //     feeTier: u32,
// //     token0: String,
// //     token1: String,
// //
// // }
// //
// // fn load_pooldata_from_file() -> Result<()> {
// //     let file = "/home/jasper/learnrust/thegraph/src/univ3pools.json";
// //     let contents = fs::read_to_string(file).expect("somethin went wrog");
// //     let u: Pooldata = serde_json::from_str(&contents)?;
// //     println!("{:#?}", u);
// //     Ok(())
// //
// // }
//
//
//
//
// // fn pars(data: String) {
// //     let mut data_entry = String::new();
// //     let mut collectable = false;
// //     let mut qcount = 0;
// //     let mut idcount = 0;
// //     let mut indatablock = false;
// //     // let mut lines_after = 0;
// //     for line in data.lines() {
// //         if line.contains("id") {
// //             for cha in line.chars() {
// //                 if cha == ':' {
// //                     collectable = true;
// //                 }
// //
// //                 if cha != '"' && cha != ' ' && cha != ':'  && collectable {
// //                     data_entry.push(cha);
// //                 }
// //
// //                 if cha == '"' && collectable {
// //                     qcount = qcount + 1;
// //                 }
// //
// //                 if qcount == 2 && collectable {
// //                     collectable = false;
// //                 }
// //
// //             }
// //             println!("{}", line);
// //             println!("{}", data_entry);
// //
// //             break;
// //
// //             }
// //         }
// //     }
// //     // let mut count = 0;
// //     // for c in data.chars() {
// //     //     println!("{}",c);
// //     //     count = count + 1;
// //     //     if count == 10 {
// //     //         break;
// //     //     }
// //     // }
// //
// // // }
// //
// // fn main() {
// //     load_pooldata_from_file().unwrap();
// // }
//     // println!("{}",load_pooldata_from_file())
//     // println!("{}",load_pooldata_from_file());
//
//
// use serde::Deserialize;
// use gql_client::Client;
//
//
// #[derive(Deserialize, Debug)]
// pub struct Univ3pool {
//     feeTier: String,
//     id: String,
//     token0: String,
//     token1: String
// }
//
// #[derive(Deserialize, Debug)]
// struct Data {
//     pool: Poollist<Univ3pool>
// }
//
// #[derive(Deserialize, Debug)]
// struct Poollist<T> {
//     data: Vec<T>
// }
//
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let q = r#"
//     query pooldata {
//   pools(first: 1000, orderBy: createdAtTimestamp, orderDirection: asc) {
//     token0 {
//       id
//     }
//     token1 {
//       id
//     }
//     id
//     feeTier
//   }
// }
//
//     "#;
//
//     let endpoint = "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3";
//     let client = Client::new(endpoint);
//     let response = client.query::<Data>(q).await.unwrap();
//
//
//     for pool in &response.pool.data {
//         println!("{} | {} | {} | {} ", pool.feeTier, pool.id, pool.token0, pool.token1);
//     }
//
//     Ok(())
// }

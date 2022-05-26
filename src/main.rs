use anyhow::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;
use serde_json::Result as OtherResult;
use ethers::{prelude::*};

#[derive(Debug, Deserialize)]
struct Token {
    id: String
}

#[derive(Debug, Deserialize)]
struct QueryUniV3Pool {
    id: String,
    token0: Token,
    token1: Token,
    feeTier: String,
    createdAtBlockNumber: String,
    createdAtTimestamp: String,
}

#[derive(Debug, Deserialize)]
struct Pools {
    pools: Vec<QueryUniV3Pool>
}

#[derive(Debug, Deserialize)]
struct Query {
    data: Pools
}

#[derive(Debug, Deserialize)]
struct UniV3Pool {
    address: Address,
    token0: Address,
    token1: Address,
    fee: U256,
}

#[tokio::main]
async fn main() -> Result<()> {
    let query1 = "{pools(first:1000,orderBy:createdAtTimestamp,orderDirection:asc){id,token0{id},token1{id},feeTier,createdAtBlockNumber,createdAtTimestamp}}".to_string();
    let urlbase = "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3".to_string();
    let client = reqwest::Client::new();
    let mut responses: Vec<Query> = Vec::new();
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("query".to_string(), query1);
    let response1 = client.post(&urlbase).json(&data).send().await?.text().await?;
    let uni3: Query = serde_json::from_str(&response1)?;
    let mut length = uni3.data.pools.len() - 1;
    let mut last_pool_time_stamp: String = uni3.data.pools[length].createdAtTimestamp.clone();
    responses.push(uni3);
    loop {
	let q1 = "{pools(first:1000,orderBy:createdAtTimestamp,orderDirection:asc where:{createdAtTimestamp_gt:".to_string();
	let q2 = "}){id,token0{id},token1{id},feeTier,createdAtBlockNumber,createdAtTimestamp}}".to_string();
	let mut query =  q1 + &last_pool_time_stamp;
	query += &q2;
	data.insert("query".to_string(), query);
	let response = client.post(&urlbase).json(&data).send().await?.text().await?;
	let uni3_response: Query = serde_json::from_str(&response)?;
	if uni3_response.data.pools.len() == 0 {
	    break;
	} else {
	    length = uni3_response.data.pools.len() -1;
	    last_pool_time_stamp = uni3_response.data.pools[length].createdAtTimestamp.clone();
	    responses.push(uni3_response);
	}
	
    }
    let mut uni3_pools: Vec<UniV3Pool> = Vec::new();
    for response in responses.into_iter() {
	for pool in response.data.pools.into_iter() {
	    let a = pool.id.parse::<Address>()?;
	    let t0 = pool.token0.id.parse::<Address>()?;
	    let t1 = pool.token1.id.parse::<Address>()?;
	    // feetier did this
	    // 256 = 100 = 2**8
	    // 12288 = 3000 = 
	    // 65536 = 10000 = 2**16
	    let f = pool.feeTier.parse::<U256>()?;
	    let p: UniV3Pool = UniV3Pool {address: a, token0: t0, token1: t1, fee: f};
	    uni3_pools.push(p);
	}
    }
    println!("{:#?}, {:#?}", uni3_pools, uni3_pools.len());
    Ok(())
}
	

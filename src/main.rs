use anyhow::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;
use serde_json::{Result as OtherResult, Value};
use ethers::{prelude::*};

#[derive(Debug, Deserialize)]
struct QueryToken {
    id: String,
    symbol: String,
    decimals: String,
}

#[derive(Debug, Deserialize)]
struct Token {
    id: Address,
    symbol: String,
    decimals: u64,
}


#[derive(Debug, Deserialize)]
struct QueryTick {
    id: String,
    tickIdx: String,
    liquidityNet: String,
    liquidityGross: String,
    price0: String,
    price1: String,
	
}

#[derive(Debug, Deserialize)]
struct QueryUniV3Pool {
    id: String,
    token0: QueryToken,
    token1: QueryToken,
    feeTier: String,
    createdAtBlockNumber: String,
    createdAtTimestamp: String,
    ticks: Vec<QueryTick>,
    tick: Value,
}

#[derive(Debug, Deserialize)]
struct Pools {
    pools: Vec<QueryUniV3Pool>
}

#[derive(Debug, Deserialize)]
struct Query {
    data: Pools
}

#[derive(Debug)]
struct Tick {
    index: i64,
    liquidity_net: U256,
    liquidity_gross: U256,
    price0: f64,
    price1: f64,
}

#[derive(Debug)]
struct UniV3Pool {
    address: Address,
    token0: Token,
    token1: Token,
    fee: u64,
    current_tick: i64,
    ticks: Vec<Tick>,
}

impl UniV3Pool {

    fn from_query(query: QueryUniV3Pool) -> UniV3Pool {
	let address: Address = query.id.parse::<Address>().unwrap();
	let token0_id: Address = query.token0.id.parse::<Address>().unwrap();
	let token0_symbol: String = query.token0.symbol;
	let token0_decimals: u64 = query.token0.decimals.clone().parse::<u64>().unwrap();
	let token1_id: Address = query.token1.id.parse::<Address>().unwrap();
	let token1_symbol: String = query.token1.symbol;
	let token1_decimals: u64 = query.token1.decimals.clone().parse::<u64>().unwrap();

	let token0: Token = Token {id: token0_id, symbol: token0_symbol, decimals: token0_decimals};
	let token1: Token = Token {id: token1_id, symbol: token1_symbol, decimals: token1_decimals};
	let fee: u64 = query.feeTier.parse::<u64>().unwrap();

	let tick_index: i64 = query.tick.to_string().parse::<i64>().unwrap();
	let mut ticks: Vec<Tick> = Vec::new();
	for t in query.ticks.into_iter() {
	    
	}
	
	return UniV3Pool {address, token0, token1, fee};
	
    }
    
}

      

#[tokio::main]
async fn main() -> Result<()> {
    let query1 = "{pools(first:1000,orderBy:createdAtTimestamp,orderDirection:asc){id,token0{id,symbol,decimals},token1{id,symbol,decimals},feeTier,createdAtBlockNumber,createdAtTimestamp,ticks{id,tickIdx,liquidityNet,liquidityGross,price0,price1},tick}}".to_string();
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
	let q2 = "}){id,token0{id,symbol,decimals},token1{id,symbol,decimals},feeTier,createdAtBlockNumber,createdAtTimestamp,ticks{id,tickIdx,liquidityNet,liquidityGross,price0,price1},tick}}".to_string();
	let mut query =  q1 + &last_pool_time_stamp;
	query += &q2;
	data.insert("query".to_string(), query);
	let response = client.post(&urlbase).json(&data).send().await?.text().await?;
	//println!("{}", response.len());
	//length of empty response is 21
	if response.len() == 21 {
	    //println!("{}", response);
	    break;
	} else {
	    let uni3_response: Query = serde_json::from_str(&response)?;
	    length = uni3_response.data.pools.len() - 1;
	    last_pool_time_stamp = uni3_response.data.pools[length].createdAtTimestamp.clone();
	    responses.push(uni3_response);
	}
	
    }
    //println!("{:#?}", responses.len());
    let mut uni3_pools: Vec<UniV3Pool> = Vec::new();
    for response in responses.into_iter() {
	for pool in response.data.pools.into_iter() {
	    let v3: UniV3Pool = UniV3Pool::from_query(pool);
	    uni3_pools.push(v3);
	}
    }
    println!("{:#?} {:#?}", uni3_pools, uni3_pools.len());
    Ok(())
}
	

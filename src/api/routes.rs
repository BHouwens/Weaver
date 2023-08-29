use crate::constants::DB_KEY;
use crate::api::utils::{
    post_cors,
    handle_rejection,
    with_node_component,
    sig_verify_middleware,
    map_api_res
};
use crate::api::handlers::{ get_data_handler, set_data_handler };
use crate::api::interfaces::{
    CFilterConnection,
    CacheConnection,
    DbConnection
};
use warp::{ Filter, Rejection, Reply };

/// ========== BASE ROUTES ========== ///

pub fn get_data(
    db: DbConnection,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    println!("Getting data...");
    warp::path("get_data")
        .and(sig_verify_middleware())
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |_, data, cache, db, cf| {
            map_api_res(get_data_handler(db, cache, data, cf))
        })
        .recover(handle_rejection)
        .with(post_cors())
}

pub fn set_data(
    db: DbConnection,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    println!("Setting data...");

    warp::path("set_data")
        .and(sig_verify_middleware())
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |_, info, cache, db, cf| {
            map_api_res(set_data_handler(info, db, DB_KEY.to_string(), cache, cf))
        })
        .recover(handle_rejection)
        .with(post_cors())
}

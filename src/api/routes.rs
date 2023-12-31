use crate::api::handlers::{get_data_handler, set_data_handler};
use weaver_core::api::interfaces::{CFilterConnection, CacheConnection, DbConnection};
use weaver_core::api::utils::{
    map_api_res, post_cors, sig_verify_middleware, with_node_component,
};
use warp::{Filter, Rejection, Reply};

/// ========== BASE ROUTES ========== ///

pub fn get_data(
    db: DbConnection,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection,
    body_limit: u64,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("get_data")
        .and(warp::post())
        .and(sig_verify_middleware())
        .and(warp::body::content_length_limit(body_limit))
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |_, info, cache, db, cf| map_api_res(get_data_handler(info, db, cache, cf)))
        .with(post_cors())
}

pub fn set_data(
    db: DbConnection,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection,
    body_limit: u64,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("set_data")
        .and(warp::post())
        .and(sig_verify_middleware())
        .and(warp::body::content_length_limit(body_limit))
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |_, info, cache, db, cf| map_api_res(set_data_handler(info, db, cache, cf)))
        .with(post_cors())
}

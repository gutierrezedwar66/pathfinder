use crate::jsonrpc::{RpcRouter, RpcRouterBuilder};

pub(crate) mod methods;

#[rustfmt::skip]
pub fn register_routes() -> RpcRouterBuilder {
    RpcRouter::builder("v0.1")
        .register("pathfinder_version",              || { pathfinder_common::consts::VERGEN_GIT_DESCRIBE })
        .register("pathfinder_getProof",             methods::get_proof)
        .register("pathfinder_getTransactionStatus", methods::get_transaction_status)
}

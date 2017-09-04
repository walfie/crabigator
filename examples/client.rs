#[macro_use]
extern crate error_chain;

extern crate crabigator;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate hyper_tls;

use crabigator::Client;
use crabigator::error::*;
use futures::Future;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

quick_main!(|| -> Result<()> {
    let api_key = ::std::env::var("WANIKANI_API_KEY").chain_err(
        || "invalid WANIKANI_API_KEY environment variable",
    )?;

    let mut core = Core::new().chain_err(|| "could not create Core")?;
    let handle = core.handle();

    let connector = HttpsConnector::new(1, &handle).chain_err(
        || "could not create HttpsConnector",
    )?;

    let hyper_client = hyper::Client::configure().connector(connector).build(
        &handle,
    );

    let client = Client::new(&hyper_client, api_key);

    let request = client.study_queue().map(|info| {
        println!("{:#?}", info);
    });

    core.run(request).chain_err(
        || "received error from timeline",
    )
});

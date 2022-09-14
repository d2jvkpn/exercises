use rayon::prelude::*;
use reqwest::{blocking, redirect};

use std::{env, time::Duration};

mod error;
// pub use error::Error; // crate level
mod model;
use model::Subdomain;
mod common_ports;
mod ports;
mod subdomains;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(error::Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_client = blocking::Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(Duration::from_secs(5))
        .build()?;

    // we use a custom threadpool to improve speed
    let pool = rayon::ThreadPoolBuilder::new().num_threads(256).build().unwrap();

    // pool.install is required to use our custom threadpool, instead of rayon's default one
    pool.install(|| {
        let result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .filter_map(|v| ports::scan_ports(v).ok())
            .collect();

        for subdomain in result {
            let ports = subdomain
                .open_ports
                .iter()
                .map(|v| v.port.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            println!("{}:\n    {}\n    {}\n", &subdomain.domain, &subdomain.ip, ports);
        }
    });

    Ok(())
}

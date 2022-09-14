use reqwest::blocking::Client;
// use trust_dns_proto::rr::Record;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

use std::{collections::HashSet, time::Duration};

use crate::error::Error;
use crate::model::{CrtShEntry, Subdomain};

pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    let entries: Vec<CrtShEntry> =
        http_client.get(&format!("https://crt.sh/?q=%25.{}&output=json", target)).send()?.json()?;

    // clean and dedup results
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain: &String| !subdomain.contains('*'))
        .collect();

    subdomains.insert(target.to_string());

    let subdomains: Vec<Subdomain> = subdomains.into_iter().filter_map(new_domain).collect();

    Ok(subdomains)
}

pub fn new_domain(domain: String) -> Option<Subdomain> {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    //    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
    //        .expect("subdomain resolver: building DNS client");

    // convert result to option (.ok()), and return early if is None
    let dns_resolver = Resolver::new(ResolverConfig::default(), opts).ok()?;

    let lookup_ip = dns_resolver.lookup_ip(domain.as_str()).ok()?;

    let ip = lookup_ip
        .as_lookup()
        .record_iter()
        .filter_map(|v| v.data())
        .map(|v| format!("{:?}", v))
        .collect::<Vec<String>>()
        .join(", ");

    Some(Subdomain { ip, domain, open_ports: Vec::new() })
}

use ssl_exp::SslExpiration;
use std::env;
use std::io::{stderr, Write};
use std::process::exit;

fn main() {
    let mut exit_code = 0;
    for domain in env::args().skip(1) {
        match SslExpiration::from_domain_name(&domain) {
            Ok(expiration) => {
                let days = expiration.days();
                println!("Date: {}", expiration.date());
                if expiration.is_expired() {
                    let _ = writeln!(
                        stderr(),
                        "{} SSL certificate expired {} days ago",
                        domain,
                        !days
                    );
                    exit_code = 1;
                } else {
                    println!("{} SSL certificate will expire in {} days", domain, days);
                }
            }
            Err(e) => {
                let _ = writeln!(stderr(), "An error occured when checking {}: {}", domain, e);
            }
        }
    }
    exit(exit_code);
}

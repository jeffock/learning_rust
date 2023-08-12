#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde_derive;

use log::{error, info, LevelFilter};
use rand::{thread_rng, Rng};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
use tokio_native_tls::TlsAcceptor;

use crate::configuration::{ProxyConfiguration, ProxyMode};
use crate::http_tunnel_codec::{HttpTunnelCodec, HttpTunnelCodecBuilder, HttpTunnelTarget};
use crate::proxy_target::{SimpleCachingDnsResolver, SimpleTcpConnector, TargetConnector};
use crate::tunnel::{
    relay_connections, ConnectionTunnel, TunnelCtx, TunnelCtxBuilder, TunnelStats,
};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncRead, AsyncWrite};

mod configuration;
mod http_tunnel_codec;
mod proxy_target;
mod relay;
mod tunnel;

type DnsResolver = SimpleCachingDnsResolver;


/// ^ dependencies


#[tokio::main]
async fn main() -> io::Result<()> {
    init_logger();
    
    /// tcp acceptor
    let mut tcp_listener = TcpListener::bind(&proxy_configuration.bind_address)
        .await
        .map_err(|e| {
            error!(
                "Error binding address {}: {}",
                &proxy_configuration.bind_address, e
            );
            e
        })?;
    
    /// general loop for both tls and tcp
    loop {
        let socket = tcp_listener.accept().await;

        let dns_resolver_ref = dns_resolver.clone();

        match socket {
            OK((stream, _)) => {
                let config = config.clone();
                tokio::spawn(async move { tunnel_stream(&config, stream, dns_resolver_ref).await });
            }
            Err(e) => error!("Failed TCP handshake {}", e),
        }
    }
    
    async fn tunnel_stream<C: AsyncRead + AsyncWrite + Send + Unpin + 'static>(
        ) -> io::Result<()> {
        ///
    }

    async fn serve_tls(
        config: ProxyConfiguration,
        listener: &mut TcpListener,
        tls_acceptor: TlsAcceptor,
        dns_resolver: DnsResolver,
        ) -> io::Result<()> {

        info!("Serving requests on: {}", config.bind_address);

        loop {
            let socket = tcp_listener.accept().await;

            let dns_resolver_ref = dns_resolver.clone();

            match socket {
                OK((stream, _)) => {
                    let config = config.clone();
                    tokio::spawn(async move { tunnel_stream(&config, stream, dns_resolver_ref).await });
                }
                Err(e) => error!("Failed TCP handshake {}", e),
            }
        }
    }


    /// initialize logger
    fn init_logger() {
        let logger_configuration = "./config/log4rs.yaml";
        if let Err(e) = log4rs::init_file(logger_configuration, Default::default()) {
            println!(
                "Cannot initialize logger from {logger_configuration}, error=[{e}]. Logging to the console.");
            let config = Config::builder()
                .appender(
                    Appender::builder()
                    .build("application", Box::new(ConsoleAppender::builder().build())),
                    )
                .build(
                    Root::builder()
                    .appender("application")
                    .build(LevelFilter::Info),
                    )
                .unwrap();
            log4rs::init_config(config).expect("Bug: bad default config");
        }
    }
}

async fn serve_tls(
        config: ProxyConfiguration,
        listener: &mut TcpListener,
        tls_acceptor: TlsAcceptor,
        dns_resolver: DnsResolver,
    ) -> io::Result<()> {
    
    info!("Serving requests on: {}", config.bind_address);

    loop {
        let socket = tcp_listener.accept().await;

        let dns_resolver_ref = dns_resolver.clone();

        match socket {
            OK((stream, _)) => {
                let config = config.clone();
                tokio::spawn(async move { tunnel_stream(&config, stream, dns_resolver_ref).await });
            }
            Err(e) => error!("Failed TCP handshake {}", e),
        }
    }
}

use std::time::Duration;

use anyhow::anyhow;
use url::{Host, Url};

use crate::protocols;
use crate::protocols::dns::DnsResolver;
use crate::protocols::udp::WsUdpSocket;
use crate::tunnel::connectors::TunnelConnector;
use crate::tunnel::RemoteAddr;

pub struct UdpTunnelConnector<'a> {
    host: &'a Host,
    port: u16,
    so_mark: Option<u32>,
    connect_timeout: Duration,
    dns_resolver: &'a DnsResolver,
}

impl<'a> UdpTunnelConnector<'a> {
    pub fn new(
        host: &'a Host,
        port: u16,
        so_mark: Option<u32>,
        connect_timeout: Duration,
        dns_resolver: &'a DnsResolver,
    ) -> UdpTunnelConnector<'a> {
        UdpTunnelConnector {
            host,
            port,
            so_mark,
            connect_timeout,
            dns_resolver,
        }
    }
}

impl TunnelConnector for UdpTunnelConnector<'_> {
    type Reader = WsUdpSocket;
    type Writer = WsUdpSocket;

    async fn connect(&self, _: &Option<RemoteAddr>) -> anyhow::Result<(Self::Reader, Self::Writer)> {
        let stream =
            protocols::udp::connect(self.host, self.port, self.connect_timeout, self.so_mark, self.dns_resolver)
                .await?;

        Ok((stream.clone(), stream))
    }

    async fn connect_with_http_proxy(
        &self,
        _proxy: &Url,
        _remote: &Option<RemoteAddr>,
    ) -> anyhow::Result<(Self::Reader, Self::Writer)> {
        Err(anyhow!("UDP tunneling is not supported with HTTP proxy"))
    }
}

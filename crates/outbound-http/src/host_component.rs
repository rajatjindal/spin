use anyhow::Result;

use spin_app::DynamicHostComponent;
use spin_core::{Data, HostComponent, Linker};
use spin_outbound_networking::{AllowedHosts, ALLOWED_HOSTS_KEY};
use spin_world::v1::http;

use crate::{allowed_http_hosts::parse_allowed_http_hosts, host_impl::OutboundHttp};

pub struct OutboundHttpComponent;

impl HostComponent for OutboundHttpComponent {
    type Data = OutboundHttp;

    fn add_to_linker<T: Send>(
        linker: &mut Linker<T>,
        get: impl Fn(&mut Data<T>) -> &mut Self::Data + Send + Sync + Copy + 'static,
    ) -> Result<()> {
        http::add_to_linker(linker, get)
    }

    fn build_data(&self) -> Self::Data {
        Default::default()
    }
}

impl DynamicHostComponent for OutboundHttpComponent {
    fn update_data(
        &self,
        data: &mut Self::Data,
        component: &spin_app::AppComponent,
    ) -> anyhow::Result<()> {
        let http_hosts = component
            .get_metadata(crate::ALLOWED_HTTP_HOSTS_KEY)?
            .unwrap_or_default();
        let hosts = component
            .get_metadata(ALLOWED_HOSTS_KEY)?
            .unwrap_or_default()
            .unwrap_or_default();
        data.allowed_http_hosts = parse_allowed_http_hosts(&http_hosts)?;
        data.allowed_hosts = AllowedHosts::parse(&hosts)?;
        Ok(())
    }
}

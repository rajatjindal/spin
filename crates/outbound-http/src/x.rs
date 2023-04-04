#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod allowed_http_hosts {










    // Allow reuse of Client's internal connection pool for multiple requests
    // in a single component execution










    use anyhow::{anyhow, Result};
    use reqwest::Url;
    const ALLOW_ALL_HOSTS: &str = "insecure:allow-all";
    /// An HTTP host allow-list.
    pub enum AllowedHttpHosts {

        /// All HTTP hosts are allowed (the "insecure:allow-all" value was present in the list)
        AllowAll,

        /// Only the specified hosts are allowed.
        AllowSpecific(Vec<AllowedHttpHost>),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AllowedHttpHosts {
        #[inline]
        fn clone(&self) -> AllowedHttpHosts {
            match self {
                AllowedHttpHosts::AllowAll => AllowedHttpHosts::AllowAll,
                AllowedHttpHosts::AllowSpecific(__self_0) =>
                    AllowedHttpHosts::AllowSpecific(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AllowedHttpHosts {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AllowedHttpHosts::AllowAll =>
                    ::core::fmt::Formatter::write_str(f, "AllowAll"),
                AllowedHttpHosts::AllowSpecific(__self_0) =>
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f,
                        "AllowSpecific", &__self_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for AllowedHttpHosts { }
    #[automatically_derived]
    impl ::core::cmp::Eq for AllowedHttpHosts {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Vec<AllowedHttpHost>>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AllowedHttpHosts { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AllowedHttpHosts {
        #[inline]
        fn eq(&self, other: &AllowedHttpHosts) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag &&
                match (self, other) {
                    (AllowedHttpHosts::AllowSpecific(__self_0),
                        AllowedHttpHosts::AllowSpecific(__arg1_0)) =>
                        *__self_0 == *__arg1_0,
                    _ => true,
                }
        }
    }
    impl Default for AllowedHttpHosts {
        fn default() -> Self { Self::AllowSpecific(::alloc::vec::Vec::new()) }
    }
    impl AllowedHttpHosts {
        /// Tests whether the given URL is allowed according to the allow-list.
        pub fn allow(&self, url: &url::Url) -> bool {
            match self {
                Self::AllowAll => true,
                Self::AllowSpecific(hosts) =>
                    hosts.iter().any(|h| h.allow(url)),
            }
        }
    }
    /// An HTTP host allow-list entry.
    pub struct AllowedHttpHost {
        domain: String,
        port: Option<u16>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AllowedHttpHost {
        #[inline]
        fn clone(&self) -> AllowedHttpHost {
            AllowedHttpHost {
                domain: ::core::clone::Clone::clone(&self.domain),
                port: ::core::clone::Clone::clone(&self.port),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AllowedHttpHost {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(f,
                "AllowedHttpHost", "domain", &self.domain, "port",
                &&self.port)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for AllowedHttpHost {
        #[inline]
        fn default() -> AllowedHttpHost {
            AllowedHttpHost {
                domain: ::core::default::Default::default(),
                port: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for AllowedHttpHost { }
    #[automatically_derived]
    impl ::core::cmp::Eq for AllowedHttpHost {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Option<u16>>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AllowedHttpHost { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AllowedHttpHost {
        #[inline]
        fn eq(&self, other: &AllowedHttpHost) -> bool {
            self.domain == other.domain && self.port == other.port
        }
    }
    impl AllowedHttpHost {
        /// Creates a new allow-list entry.
        pub fn new(name: impl Into<String>, port: Option<u16>) -> Self {
            Self { domain: name.into(), port }
        }
        /// An allow-list entry that specifies a host and allows the default port.
        pub fn host(name: impl Into<String>) -> Self {
            Self { domain: name.into(), port: None }
        }
        /// An allow-list entry that specifies a host and port.
        pub fn host_and_port(name: impl Into<String>, port: u16) -> Self {
            Self { domain: name.into(), port: Some(port) }
        }
        fn allow(&self, url: &url::Url) -> bool {
            (url.scheme() == "http" || url.scheme() == "https") &&
                    self.domain == url.host_str().unwrap_or_default() &&
                self.port == url.port()
        }
    }
    pub fn validate_allowed_http_hosts(http_hosts: &Option<Vec<String>>)
        -> Result<()> {
        parse_allowed_http_hosts(http_hosts).map(|_| ())
    }
    pub fn parse_allowed_http_hosts(raw: &Option<Vec<String>>)
        -> Result<AllowedHttpHosts> {
        match raw {
            None =>
                Ok(AllowedHttpHosts::AllowSpecific(::alloc::vec::Vec::new())),
            Some(list) => {
                if list.iter().any(|domain| domain == ALLOW_ALL_HOSTS) {
                        Ok(AllowedHttpHosts::AllowAll)
                    } else {
                       let parse_results =
                           list.iter().map(|h|
                                       parse_allowed_http_host(h)).collect::<Vec<_>>();
                       let (hosts, errors) = partition_results(parse_results);
                       if errors.is_empty() {
                               Ok(AllowedHttpHosts::AllowSpecific(hosts))
                           } else {
                              Err(::anyhow::Error::msg({
                                          let res =
                                              ::alloc::fmt::format(format_args!("One or more allowed_http_hosts entries was invalid:\n{0}",
                                                      errors.join("\n")));
                                          res
                                      }))
                          }
                   }
            }
        }
    }
    fn parse_allowed_http_host(text: &str)
        -> Result<AllowedHttpHost, String> {
        if text.contains("//") {
                parse_allowed_http_host_from_schemed(text)
            } else { parse_allowed_http_host_from_unschemed(text) }
    }
    fn parse_allowed_http_host_from_unschemed(text: &str)
        -> Result<AllowedHttpHost, String> {
        let urlised =
            {
                let res =
                    ::alloc::fmt::format(format_args!("http://{0}", text));
                res
            };
        let fake_url =
            Url::parse(&urlised).map_err(|_|
                        {
                            let res =
                                ::alloc::fmt::format(format_args!("{0} isn\'t a valid host or host:port string",
                                        text));
                            res
                        })?;
        parse_allowed_http_host_from_http_url(&fake_url, text)
    }
    fn parse_allowed_http_host_from_schemed(text: &str)
        -> Result<AllowedHttpHost, String> {
        let url =
            Url::parse(text).map_err(|e|
                        {
                            let res =
                                ::alloc::fmt::format(format_args!("{0} isn\'t a valid HTTP host URL: {1}",
                                        text, e));
                            res
                        })?;
        if !match url.scheme() { "http" | "https" => true, _ => false, } {
                return Err({
                            let res =
                                ::alloc::fmt::format(format_args!("{0} isn\'t a valid host or host:port string",
                                        text));
                            res
                        });
            }
        parse_allowed_http_host_from_http_url(&url, text)
    }
    fn parse_allowed_http_host_from_http_url(url: &Url, text: &str)
        -> Result<AllowedHttpHost, String> {
        let host =
            url.host_str().ok_or_else(||
                        {
                            let res =
                                ::alloc::fmt::format(format_args!("{0} doesn\'t contain a host name",
                                        text));
                            res
                        })?;
        let has_path = url.path().len() > 1;
        if has_path {
                return Err({
                            let res =
                                ::alloc::fmt::format(format_args!("{0} contains a path, should be host and optional port only",
                                        text));
                            res
                        });
            }
        Ok(AllowedHttpHost::new(host, url.port()))
    }
    fn partition_results<T, E>(results: Vec<Result<T, E>>)
        -> (Vec<T>, Vec<E>) {
        let mut oks = Vec::with_capacity(results.len());
        let mut errs = ::alloc::vec::Vec::new();
        for result in results {
            match result { Ok(t) => oks.push(t), Err(e) => errs.push(e), }
        }
        (oks, errs)
    }
}
mod host_component {
    use anyhow::Result;
    use spin_app::DynamicHostComponent;
    use spin_core::{http, Data, HostComponent, Linker};
    use crate::{allowed_http_hosts::parse_allowed_http_hosts, OutboundHttp};
    pub struct OutboundHttpComponent;
    impl HostComponent for OutboundHttpComponent {
        type Data = OutboundHttp;
        fn add_to_linker<T: Send>(linker: &mut Linker<T>,
            get:
                impl Fn(&mut Data<T>) -> &mut Self::Data + Send + Sync +
                Copy + 'static) -> Result<()> {
            http::add_to_linker(linker, get)
        }
        fn build_data(&self) -> Self::Data { Default::default() }
    }
    impl DynamicHostComponent for OutboundHttpComponent {
        fn update_data(&self, data: &mut Self::Data,
            component: &spin_app::AppComponent) -> anyhow::Result<()> {
            let hosts =
                component.get_metadata(crate::ALLOWED_HTTP_HOSTS_KEY)?;
            data.allowed_hosts = parse_allowed_http_hosts(&hosts)?;
            Ok(())
        }
    }
}
use std::str::FromStr;
use anyhow::Result;
use http::HeaderMap;
use reqwest::{Client, Url};
use spin_app::MetadataKey;
use spin_core::{
    async_trait, http as outbound_http,
    http_types::{HeadersParam, HttpError, Method, RequestResult, Response},
};
use allowed_http_hosts::AllowedHttpHosts;
pub use host_component::OutboundHttpComponent;
pub const ALLOWED_HTTP_HOSTS_KEY: MetadataKey<Vec<String>> =
    MetadataKey::new("allowed_http_hosts");
/// A very simple implementation for outbound HTTP requests.
pub struct OutboundHttp {
    /// List of hosts guest modules are allowed to make requests to.
    pub allowed_hosts: AllowedHttpHosts,
    client: Option<Client>,
}
#[automatically_derived]
impl ::core::default::Default for OutboundHttp {
    #[inline]
    fn default() -> OutboundHttp {
        OutboundHttp {
            allowed_hosts: ::core::default::Default::default(),
            client: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for OutboundHttp {
    #[inline]
    fn clone(&self) -> OutboundHttp {
        OutboundHttp {
            allowed_hosts: ::core::clone::Clone::clone(&self.allowed_hosts),
            client: ::core::clone::Clone::clone(&self.client),
        }
    }
}
impl OutboundHttp {
    /// Check if guest module is allowed to send request to URL, based on the list of
    /// allowed hosts defined by the runtime. If the list of allowed hosts contains
    /// `insecure:allow-all`, then all hosts are allowed.
    /// If `None` is passed, the guest module is not allowed to send the request.
    fn is_allowed(&self, url: &str) -> Result<bool, HttpError> {
        let url = Url::parse(url).map_err(|_| HttpError::InvalidUrl)?;
        Ok(self.allowed_hosts.allow(&url))
    }
}
impl outbound_http::Host for OutboundHttp {
    #[allow(clippy :: async_yields_async, clippy :: let_unit_value, clippy ::
    no_effect_underscore_binding, clippy :: shadow_same, clippy ::
    type_complexity, clippy :: type_repetition_in_bounds, clippy ::
    used_underscore_binding)]
    fn send_request<'life0,
        'async_trait>(&'life0 mut self, req: RequestResult)
        ->
            ::core::pin::Pin<Box<dyn ::core::future::Future<Output =
            Result<Result<Response, HttpError>>> + ::core::marker::Send +
            'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        Box::pin(async move
                {
                if let ::core::option::Option::Some(__ret) =
                            ::core::option::Option::None::<Result<Result<Response,
                                HttpError>>> {
                        return __ret;
                    }
                let mut __self = self;
                let req = req;
                let __ret: Result<Result<Response, HttpError>> =
                    {
                        Ok(async {
                                    {
                                        let lvl = ::log::Level::Trace;
                                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                                                    lvl <= ::log::max_level() {
                                                ::log::__private_api_log(format_args!("Attempting to send outbound HTTP request to {0}",
                                                        req.uri), lvl,
                                                    &("outbound_http", "outbound_http",
                                                            "crates/outbound-http/src/lib.rs", 43u32),
                                                    ::log::__private_api::Option::None);
                                            }
                                    };
                                    if !__self.is_allowed(&req.uri).map_err(|_|
                                                            HttpError::RuntimeError)? {
                                            {
                                                let lvl = ::log::Level::Info;
                                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                                            lvl <= ::log::max_level() {
                                                        ::log::__private_api_log(format_args!("Destination not allowed: {0}",
                                                                req.uri), lvl,
                                                            &("outbound_http", "outbound_http",
                                                                    "crates/outbound-http/src/lib.rs", 48u32),
                                                            ::log::__private_api::Option::None);
                                                    }
                                            };
                                            return Err(HttpError::DestinationNotAllowed);
                                        }
                                    let method = method_from(req.method);
                                    let url =
                                        Url::parse(&req.uri).map_err(|_| HttpError::InvalidUrl)?;
                                    let headers =
                                        request_headers(&req.headers.iter().map(|(k, v)|
                                                                    (k.as_str(),
                                                                        v.as_str())).collect::<Vec<_>>()).map_err(|_|
                                                    HttpError::RuntimeError)?;
                                    let body = req.body.unwrap_or_default().to_vec();
                                    if !req.params.is_empty() {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                                            lvl <= ::log::max_level() {
                                                        ::log::__private_api_log(format_args!("HTTP params field is deprecated"),
                                                            lvl,
                                                            &("outbound_http", "outbound_http",
                                                                    "crates/outbound-http/src/lib.rs", 64u32),
                                                            ::log::__private_api::Option::None);
                                                    }
                                            };
                                        }
                                    let client =
                                        __self.client.get_or_insert_with(Default::default);
                                    let resp =
                                        client.request(method,
                                                                    url).headers(headers).body(body).send().await.map_err(log_reqwest_error)?;
                                    {
                                        let lvl = ::log::Level::Trace;
                                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                                                    lvl <= ::log::max_level() {
                                                ::log::__private_api_log(format_args!("Returning response from outbound request to {0}",
                                                        req.uri), lvl,
                                                    &("outbound_http", "outbound_http",
                                                            "crates/outbound-http/src/lib.rs", 78u32),
                                                    ::log::__private_api::Option::None);
                                            }
                                    };
                                    response_from_reqwest(resp).await
                                }.await)
                    };

                #[allow(unreachable_code)]
                __ret
            })
    }
}
fn log_reqwest_error(err: reqwest::Error) -> HttpError {
    let error_desc =
        if err.is_timeout() {
                "timeout error"
            } else if err.is_connect() {
               "connection error"
           } else if err.is_body() || err.is_decode() {
               "message body error"
           } else if err.is_request() { "request error" } else { "error" };
    {
        use ::tracing::__macro_support::Callsite as _;
        static CALLSITE: ::tracing::callsite::DefaultCallsite =
            {
                static META: ::tracing::Metadata<'static> =
                    {
                        ::tracing_core::metadata::Metadata::new("event crates/outbound-http/src/lib.rs:97",
                            "outbound_http", ::tracing::Level::WARN,
                            Some("crates/outbound-http/src/lib.rs"), Some(97u32),
                            Some("outbound_http"),
                            ::tracing_core::field::FieldSet::new(&["message"],
                                ::tracing_core::callsite::Identifier(&CALLSITE)),
                            ::tracing::metadata::Kind::EVENT)
                    };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
        let enabled =
            ::tracing::Level::WARN <=
                        ::tracing::level_filters::STATIC_MAX_LEVEL &&
                    ::tracing::Level::WARN <=
                        ::tracing::level_filters::LevelFilter::current() &&
                {
                    let interest = CALLSITE.interest();
                    !interest.is_never() &&
                        ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(),
                            interest)
                };
        if enabled {
                (|value_set: ::tracing::field::ValueSet|
                            {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                                if match ::tracing::Level::WARN {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL {
                                        if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level =
                                                        match ::tracing::Level::WARN {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                    if level <= log::max_level() {
                                                            let meta = CALLSITE.metadata();
                                                            let log_meta =
                                                                log::Metadata::builder().level(level).target(meta.target()).build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(meta, logger,
                                                                        log_meta, &value_set)
                                                                }
                                                        }
                                                }
                                            } else { {} }
                                    } else { {} };
                            })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = CALLSITE.metadata().fields().iter();
                        CALLSITE.metadata().fields().value_set(&[(&iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("Outbound HTTP {0}: URL {1}, error detail {2:?}",
                                                            error_desc,
                                                            err.url().map(|u|
                                                                        u.to_string()).unwrap_or_else(|| "<unknown>".to_owned()),
                                                            err) as &dyn Value))])
                    });
            } else {
               if match ::tracing::Level::WARN {
                               ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                               ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                               ::tracing::Level::INFO => ::tracing::log::Level::Info,
                               ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                               _ => ::tracing::log::Level::Trace,
                           } <= ::tracing::log::STATIC_MAX_LEVEL {
                       if !::tracing::dispatcher::has_been_set() {
                               {
                                   use ::tracing::log;
                                   let level =
                                       match ::tracing::Level::WARN {
                                           ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                           ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                           ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                           ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                           _ => ::tracing::log::Level::Trace,
                                       };
                                   if level <= log::max_level() {
                                           let meta = CALLSITE.metadata();
                                           let log_meta =
                                               log::Metadata::builder().level(level).target(meta.target()).build();
                                           let logger = log::logger();
                                           if logger.enabled(&log_meta) {
                                                   ::tracing::__macro_support::__tracing_log(meta, logger,
                                                       log_meta,
                                                       &{
                                                               #[allow(unused_imports)]
                                                               use ::tracing::field::{debug, display, Value};
                                                               let mut iter = CALLSITE.metadata().fields().iter();
                                                               CALLSITE.metadata().fields().value_set(&[(&iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                                                   Some(&format_args!("Outbound HTTP {0}: URL {1}, error detail {2:?}",
                                                                                                   error_desc,
                                                                                                   err.url().map(|u|
                                                                                                               u.to_string()).unwrap_or_else(|| "<unknown>".to_owned()),
                                                                                                   err) as &dyn Value))])
                                                           })
                                               }
                                       }
                               }
                           } else { {} }
                   } else { {} };
           }
    };
    HttpError::RuntimeError
}
fn method_from(m: Method) -> http::Method {
    match m {
        Method::Get => http::Method::GET,
        Method::Post => http::Method::POST,
        Method::Put => http::Method::PUT,
        Method::Delete => http::Method::DELETE,
        Method::Patch => http::Method::PATCH,
        Method::Head => http::Method::HEAD,
        Method::Options => http::Method::OPTIONS,
    }
}
async fn response_from_reqwest(res: reqwest::Response)
    -> Result<Response, HttpError> {
    let status = res.status().as_u16();
    let headers =
        response_headers(res.headers()).map_err(|_| HttpError::RuntimeError)?;
    let body =
        Some(res.bytes().await.map_err(|_|
                            HttpError::RuntimeError)?.to_vec());
    Ok(Response { status, headers, body })
}
fn request_headers(h: HeadersParam) -> anyhow::Result<HeaderMap> {
    let mut res = HeaderMap::new();
    for (k, v) in h {
        res.insert(http::header::HeaderName::from_str(k)?,
            http::header::HeaderValue::from_str(v)?);
    }
    Ok(res)
}
fn response_headers(h: &HeaderMap)
    -> anyhow::Result<Option<Vec<(String, String)>>> {
    let mut res: Vec<(String, String)> = ::alloc::vec::Vec::new();
    for (k, v) in h {
        res.push((k.to_string(),
                std::str::from_utf8(v.as_bytes())?.to_string()));
    }
    Ok(Some(res))
}

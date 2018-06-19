use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};
use futures;
use futures::future::Future;

use hyper::{self, Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};

use {MetricCollector, PrometheusMetricCollector, PrometheusMetric};

pub struct PrometheusExporter<T: MetricCollector> {
    collector: Box<T>,
}

impl<T: MetricCollector> Clone for PrometheusExporter<T>{
    fn clone(&self) -> Self {
        PrometheusExporter {
            collector: self.collector.clone()
        }
    }
}

impl<T:'static +  MetricCollector> PrometheusExporter<T> {
    pub fn with_collector(collector: impl MetricCollector) -> PrometheusExporter<impl MetricCollector> {
        PrometheusExporter {
            collector: Box::new(collector),
        }
    }

    pub fn bind(self, port: u16) {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port).into();
        let closure_exporter = self.clone();
        let closure = move || Ok(closure_exporter.clone());
        let server = Http::new().bind(&addr, closure).unwrap();
        server.run().unwrap()
    }
}

impl<T: MetricCollector> Service for PrometheusExporter<T> {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => response.set_body(r#"<a href="/metrics">Metrics</a>"#),
            (&Method::Get, "/metrics") => {
                response.set_body(
                        self.collector.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                );
            }
            _ => {
                response.set_status(StatusCode::NotFound);
            }
        };

        Box::new(futures::future::ok(response))
    }
}

pub struct PrometheusExporterBuilder {
    metrics: Vec<PrometheusMetric>,
}

impl PrometheusExporterBuilder {
    pub fn new() -> PrometheusExporterBuilder {
        PrometheusExporterBuilder {
            metrics: Vec::new(),
        }
    }

    pub fn metric(mut self, metric: PrometheusMetric) -> Self {
        self.metrics.push(metric);
        self
    }

    pub fn bind(self, port: u16) {
        self.into_exporter().bind(port)
    }

    fn into_exporter(self) -> PrometheusExporter<PrometheusMetricCollector> {
        PrometheusExporter {
            collector: Box::new(
                PrometheusMetricCollector { metrics: Arc::new(self.metrics) }
            )
        }
    }
}

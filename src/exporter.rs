use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use futures;
use futures::future::Future;

use hyper::{self, Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};

use PrometheusMetric;

#[derive(Clone, Debug)]
pub struct PrometheusExporter {
    metrics: Vec<PrometheusMetric>,
}

impl PrometheusExporter {
    pub fn new() -> PrometheusExporterBuilder {
        PrometheusExporterBuilder {
            metrics: Vec::new(),
        }
    }
}

impl Service for PrometheusExporter {
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
                    self.metrics
                        .iter()
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
    pub fn metric(mut self, metric: PrometheusMetric) -> Self {
        self.metrics.push(metric);
        self
    }

    pub fn bind(self, port: u16) {
        let exporter = self.into_exporter();

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port).into();
        let closure_exporter = exporter.clone();
        let closure = move || Ok(closure_exporter.clone());
        let server = Http::new().bind(&addr, closure).unwrap();
        server.run().unwrap()
    }

    fn into_exporter(self) -> PrometheusExporter {
        PrometheusExporter {
            metrics: self.metrics,
        }
    }
}

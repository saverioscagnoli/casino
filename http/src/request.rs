use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

pub struct Request {
    inner: hyper::Request<Incoming>,
    segments: Vec<String>,
}

impl From<hyper::Request<Incoming>> for Request {
    fn from(inner: hyper::Request<Incoming>) -> Self {
        let path = inner.uri().path();
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Self { inner, segments }
    }
}

impl Deref for Request {
    type Target = hyper::Request<Incoming>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Request {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Request {
    pub fn segments(&self) -> Vec<&str> {
        self.segments.iter().map(|s| s.as_str()).collect::<Vec<_>>()
    }

    pub async fn json<B: for<'a> Deserialize<'a>>(self) -> serde_json::Result<B> {
        let body = self.inner.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();

        serde_json::from_slice(&bytes)
    }
}

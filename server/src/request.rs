use std::ops::{Deref, DerefMut};

use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::{Deserialize, Serialize};

pub struct Request {
    inner: hyper::Request<Incoming>,
    path_segments: Vec<String>,
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
    pub fn path(&self) -> Vec<&str> {
        self.path_segments.iter().map(|s| s.as_str()).collect()
    }

    pub async fn json<T: for<'a> Deserialize<'a>>(self) -> serde_json::Result<T> {
        let body = self.inner.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();

        serde_json::from_slice(&bytes)
    }
}

impl Into<Request> for hyper::Request<Incoming> {
    fn into(self) -> Request {
        let path_segments = self
            .uri()
            .path()
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Request {
            inner: self,
            path_segments,
        }
    }
}

use std::ops::{Deref, DerefMut};

use bytes::Bytes;
use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::{
    Response as HyperResponse, StatusCode,
    header::{HeaderValue, IntoHeaderName},
};
use serde::Serialize;

pub struct Response(HyperResponse<BoxBody<Bytes, hyper::Error>>);

impl Deref for Response {
    type Target = HyperResponse<BoxBody<Bytes, hyper::Error>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<HyperResponse<BoxBody<Bytes, hyper::Error>>> for Response {
    fn into(self) -> HyperResponse<BoxBody<Bytes, hyper::Error>> {
        self.0
    }
}

impl Response {
    pub fn empty() -> Self {
        Self(HyperResponse::new(
            Empty::<Bytes>::new()
                .map_err(|never| match never {})
                .boxed(),
        ))
        .cors()
    }

    pub fn header<K: IntoHeaderName, V: Into<HeaderValue>>(mut self, k: K, v: V) -> Self {
        self.headers_mut().insert(k, v.into());
        self
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        *self.status_mut() = status;
        self
    }

    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        *self.body_mut() = Full::new(Bytes::from(text.into()))
            .map_err(|never| match never {})
            .boxed();

        self
    }

    pub fn body<B: Serialize>(mut self, payload: B) -> serde_json::Result<Self> {
        let json = serde_json::to_string(&payload)?;
        *self.body_mut() = Full::new(Bytes::from(json))
            .map_err(|never| match never {})
            .boxed();

        Ok(self)
    }

    fn cors(self) -> Self {
        self.header("Access-Control-Allow-Origin", HeaderValue::from_static("*"))
        .header(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
        )
        .header(
            "Access-Control-Allow-Headers",
            HeaderValue::from_static(
                "Accept, Accept-Language, Content-Language,  Origin, Content-Type, Authorization",
            ),
        )
    }
}

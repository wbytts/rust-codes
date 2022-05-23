use super::{FromRequest, RequestParts};
use crate::response::IntoResponse;
use async_trait::async_trait;
use bytes::Bytes;
use headers::HeaderMapExt;
use http_body::Full;
use std::{convert::Infallible, ops::Deref};

#[cfg(feature = "headers")]
#[cfg_attr(docsrs, doc(cfg(feature = "headers")))]
#[derive(Debug, Clone, Copy)]
pub struct TypedHeader<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for TypedHeader<T>
where
    T: headers::Header,
    B: Send,
{
    type Rejection = TypedHeaderRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = if let Some(headers) = req.headers() {
            headers
        } else {
            return Err(TypedHeaderRejection {
                name: T::name(),
                reason: Reason::Missing,
            });
        };

        match headers.typed_try_get::<T>() {
            Ok(Some(value)) => Ok(Self(value)),
            Ok(None) => Err(TypedHeaderRejection {
                name: T::name(),
                reason: Reason::Missing,
            }),
            Err(err) => Err(TypedHeaderRejection {
                name: T::name(),
                reason: Reason::Error(err),
            }),
        }
    }
}

impl<T> Deref for TypedHeader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Rejection used for [`TypedHeader`](super::TypedHeader).
#[cfg(feature = "headers")]
#[cfg_attr(docsrs, doc(cfg(feature = "headers")))]
#[derive(Debug)]
pub struct TypedHeaderRejection {
    name: &'static http::header::HeaderName,
    reason: Reason,
}

#[derive(Debug)]
enum Reason {
    Missing,
    Error(headers::Error),
}

impl IntoResponse for TypedHeaderRejection {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> http::Response<Self::Body> {
        let mut res = self.to_string().into_response();
        *res.status_mut() = http::StatusCode::BAD_REQUEST;
        res
    }
}

impl std::fmt::Display for TypedHeaderRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            Reason::Missing => {
                write!(f, "Header of type `{}` was missing", self.name)
            }
            Reason::Error(err) => {
                write!(f, "{} ({})", err, self.name)
            }
        }
    }
}

impl std::error::Error for TypedHeaderRejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.reason {
            Reason::Error(err) => Some(err),
            Reason::Missing => None,
        }
    }
}

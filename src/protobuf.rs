use axum::{
    body::{Body, Bytes},
    extract::FromRequest,
    http::{StatusCode, header},
    response::IntoResponse,
};
use prost::Message;

pub struct Protobuf<T>(pub T);

impl<T, S> FromRequest<S> for Protobuf<T>
where
    T: Message + Default,
    S: Sync + Send + 'static,
{
    type Rejection = StatusCode;

    async fn from_request(
        req: axum::extract::Request<Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        T::decode(bytes)
            .map(Protobuf)
            .map_err(|_| StatusCode::BAD_REQUEST)
    }
}

impl<T> IntoResponse for Protobuf<T>
where
    T: Message,
{
    fn into_response(self) -> axum::response::Response {
        let buf = self.0.encode_to_vec();
        ([(header::CONTENT_TYPE, "application/x-protobuf")], buf).into_response()
    }
}

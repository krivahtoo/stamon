use axum::{
    RequestPartsExt,
    extract::{FromRequest, MatchedPath, Request, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};

// We define our own `Json` extractor that customizes the error from `axum::Json`
pub struct Json<T>(pub T);

impl IntoResponse for Json<Value> {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        // Have to run that first since `Json` extraction consumes the request.
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                let payload = json!({
                    "message": rejection.body_text(),
                    "origin": "json_extractor",
                    "path": path,
                });

                Err((rejection.status(), axum::Json(payload)))
            }
        }
    }
}

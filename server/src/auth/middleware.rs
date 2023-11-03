use super::Claims;
use axum::{extract::FromRequest, http::Request, middleware::Next, response::IntoResponse};

pub async fn auth_bearer_middleware<B, S>(
    req: Request<B>,
    next: Next<B>,
    state: S,
) -> impl IntoResponse
where
    B: Send + 'static,
    S: Send + Sync,
{
    // let mut request_parts = Request::new(req);
    let result = Claims::from_request(req, &state).await;

    match result {
        Ok(claims) => {
            format!("Found claims: {:?}", claims);

            // let mut req = request_parts.try_into_request().expect("body extracted");

            req.extensions_mut().insert(claims);

            Ok(next.run(req).await)
        }
        Err(web_error) => Err(web_error),
    }
}

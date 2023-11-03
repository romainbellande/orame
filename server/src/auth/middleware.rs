use super::Claims;
use axum::{extract::FromRequestParts, http::Request, middleware::Next, response::IntoResponse};

pub async fn auth_bearer_middleware<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse
where
    B: Send + Sync + 'static,
{
    let (mut request_parts, body) = req.into_parts();
    let result = Claims::from_request_parts(&mut request_parts, &body).await;

    match result {
        Ok(claims) => {
            let mut req = Request::from_parts(request_parts, body);

            req.extensions_mut().insert(claims);

            Ok(next.run(req).await)
        }
        Err(web_error) => Err(web_error),
    }
}

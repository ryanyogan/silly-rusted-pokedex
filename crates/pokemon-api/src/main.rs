use aws_lambda_events::{
    apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
    encodings::Body,
    http::HeaderMap,
};
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("cold start");
    let processor = service_fn(handler);
    lambda_runtime::run(processor).await?;
    Ok(())
}

async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    println!("handler");
    dbg!(event);

    let response = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text("Boop".to_string())),
        is_base64_encoded: false,
    };

    Ok(response)
}

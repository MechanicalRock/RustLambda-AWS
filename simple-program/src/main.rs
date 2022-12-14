use lambda_runtime::{Context, Error, service_fn, run, LambdaEvent};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main () ->Result<(), Error>{
    let handler = service_fn(handler);  // This lets you build a Service from an async function that returns a Result.
   run(handler).await?; //Starts the Lambda Rust runtime and begins polling for events on the Lambda Runtime APIs
    Ok(())
}

#[derive(Deserialize)]
struct Event {
    first_name: String,
    last_name: String

}

#[derive(Serialize)]
struct Output {
    message: String,
    request_id:String
}

async fn handler (event: LambdaEvent<Event>) -> Result<Output, Error> {
    let message:String = format!("Hi {} {} , welcome to rust in the cloud!", event.payload.first_name, event.payload.last_name);
    Ok(Output { message, request_id: event.context.request_id })
}
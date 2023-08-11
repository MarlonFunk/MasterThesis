use ow_evaluation::{execute_requests, generate_requests, init};
use futures::{stream::FuturesUnordered, Future, StreamExt};
use rand::prelude::SliceRandom;
use serde_json::{json, Value};

pub const SERVER_URI: &'static str = "http://172.17.0.1:3233"; //TODO: Why not possible to import?
const SAMPLE_SIZE: usize = 50;

// Function to create and send a request to the server
pub fn make_request(
    uri: &str,
    auth: &str,
    body: surf::Body,
) -> impl Future<Output = Result<Value, surf::Error>> {
    // Create a POST request with the provided URI, body, and authorization header
    let request = surf::post(uri)
        .body(body)
        .header("Authorization", format!("Basic {}", auth))
        .recv_json::<serde_json::Value>(); // Expect the response body to be deserialized as JSON

    request // Return the future representing the request
}

// Function to make multiple concurrent requests
pub fn make_concurrent_requests<F>(
    count: usize,
    action_name: &str,
    auth: &str,
    param: &F,
) -> FuturesUnordered<impl Future<Output = Result<Value, surf::Error>>>
where
    F: Fn() -> Value, // The param function returns a JSON Value
{
    let mut futures = Vec::with_capacity(count);

    // Create `count` concurrent requests by appending a unique index to the action name
    for i in 0..count {
        let mut action = action_name.to_owned();
        action.push_str(&(i + 1).to_string());
        std::eprintln!("Executing action: {}", action);

        // Construct the path for the request
        let path = format!(
            "{}/api/v1/namespaces/_/actions/{}?blocking=true&result=false",
            SERVER_URI, action
        );

        // Obtain the parameters for the request using the provided param function
        let parameters = param();
        let body = surf::Body::from_json(&parameters).unwrap(); // Create the request body

        // Push the request future into the futures vector
        futures.push(make_request(&path, auth, body));
    }

    futures.into_iter().collect::<FuturesUnordered<_>>() // Convert the futures vector into a FuturesUnordered stream
}


#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // let net_file = "/home/m/Documents/MasterTGN/MasterThesis/wasm/net-wasmtime.zip";
    // let prime_file = "/home/m/Documents/MasterTGN/MasterThesis/wasm/prime-wasmtime.zip";


    // init(
    //     "net",
    //     net_file.to_owned(),
    //     ow_common::ActionCapabilities {
    //         net_access: Some(true),
    //         ..Default::default()
    //     },
    // )
    // .await?;

    // init("prime", prime_file.to_owned(), Default::default()).await?;

    let num_requests = num_cpus::get() * 4;
    let mut summary = Vec::with_capacity(num_requests);
    let mut exec_times = Vec::with_capacity(SAMPLE_SIZE);

    for _ in 0..SAMPLE_SIZE {
        let (exec_time, sample) = take_sample(num_requests).await;
        exec_times.push(exec_time);
        summary.append(&mut summarize(sample));
        async_std::task::sleep(std::time::Duration::new(0, 50_000_000u32)).await;
    }

    let result = json!({
        "times": exec_times,
        "requests": summary,
    });

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}

async fn take_sample(num_requests: usize) -> (u64, Vec<Value>) {
    let auth = base64::encode(std::env::var("WHISK_AUTH").unwrap());
    let mut param = json!({});
    let mut requests = make_concurrent_requests(num_requests / 2, "prime", &auth, &param);
    requests.append(&mut make_concurrent_requests(num_requests / 2, "net", &auth, &param));

    //TODO: Here set different percentages of requests
    let mut rng = rand::thread_rng();
    requests.shuffle(&mut rng);

    execute_requests(requests).await
}

fn summarize(responses: Vec<Value>) -> Vec<Value> {
    let mut summary = vec![];

    for response in responses {
        let result = response.get("result").unwrap();
        let duration = result.get("exit_at").unwrap().as_f64().unwrap()
            - result.get("entry_at").unwrap().as_f64().unwrap();
        if let Some(_) = result.get("prime") {
            summary.push(json!({
                "duration": duration,
                "type": "prime"
            }));
        } else {
            summary.push(json!({
                "duration": duration,
                "type": "net"
            }));
        }
    }

    summary
}

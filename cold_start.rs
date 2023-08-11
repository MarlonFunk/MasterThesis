// Import necessary modules and items from the crate
use crate::SERVER_URI;

use async_std::eprintln;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use serde::Serialize;
use serde_json::Value;

// A constant that defines the maximum number of concurrent requests to test
const ABORT_AFTER: usize = 11;

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
    param: &mut F,
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

// Asynchronous function to collect the details of an activation using its ID
async fn collect_activation(activation_id: &str, auth: &str) -> Value {
    // Construct the path for fetching activation details
    let path = format!(
        "{}/api/v1/namespaces/_/activations/{}",
        SERVER_URI, activation_id
    );

    // Send a GET request to the path with the provided authorization header
    surf::get(&path)
        .header("Authorization", format!("Basic {}", auth))
        .recv_json::<serde_json::Value>()
        .await
        .unwrap() // Expect the response body to be deserialized as JSON
}

// Structure to hold the result of a cold start test
#[derive(Serialize, Debug)]
pub struct ColdStartResult {
    pub no_concurrent_requests: usize,
    pub responses: Vec<Value>,
}

// Asynchronous function to perform a cold start test with varying concurrent requests
pub async fn cold_start_test<F>(action_name: &str, mut param: F) -> Value
where
    F: Fn() -> Value, // The param function returns a JSON Value
{
    // Encode the OpenWhisk authorization into Base64
    let auth = base64::encode(std::env::var("WHISK_AUTH").unwrap());

    let mut cold_start_results = Vec::with_capacity(ABORT_AFTER);

    // Loop through different numbers of concurrent requests
    for i in 1..ABORT_AFTER {
        let mut futures = make_concurrent_requests(i, action_name, &auth, &mut param); // Create concurrent requests

        let mut responses = Vec::with_capacity(ABORT_AFTER * 3 * ABORT_AFTER);

        eprintln!("Sending {} concurrent requests", i).await;

        let mut failed = false;

        let mut activation_ids = vec![];

        loop {
            // Check for responses from concurrent requests
            match futures.next().await {
                Some(response) => match response {
                    Ok(res) => {
                        if let Some(_) = res.get("response") {
                            eprintln!("Response: {:?}", res).await;
                            responses.push(res); // Store successful responses
                        } else {
                            let activation_id = res.get("activationId").unwrap().as_str().unwrap();
                            activation_ids.push(activation_id.to_owned()); // Store activation IDs for non-blocking responses
                        }
                    }
                    Err(err) => {
                        eprintln!("Err: {:?}", err).await;
                        failed = true; // Mark if any request fails
                    }
                },
                None => break, // No more responses, break the loop
            }
        }

        // Check if there are any activation IDs for non-blocking responses
        if !activation_ids.is_empty() {
            eprintln!("Collect {num} activations?", num = activation_ids.len()).await;
            let stdin = async_std::io::stdin();
            let mut line = String::new();
            stdin.read_line(&mut line).await.unwrap();

            if line.starts_with("y") {
                for activation_id in activation_ids {
                    eprintln!("Collecting {activation_id}", activation_id = activation_id).await;
                    let response = collect_activation(&activation_id, &auth).await;
                    responses.push(response); // Fetch and store the activation details for non-blocking responses
                }
            }
        }

        // Store the cold start result for this set of concurrent requests
        cold_start_results.push(ColdStartResult {
            no_concurrent_requests: i,
            responses,
        });

        if failed {
            eprintln!("aborting").await;
            break; // Abort the test if any request fails
        }

        // Prompt the user to continue or abort the test
        // eprintln!("Continue?").await;
        // let stdin = async_std::io::stdin();
        // let mut line = String::new();
        // stdin.read_line(&mut line).await.unwrap();

        // if !line.starts_with("y") {
        //     break; // Stop the test if the user does not want to continue
        // }

        // After 10 concurrent requests the test ends, there is no reason to wait again.
        if  i < ABORT_AFTER -1 {
            eprintln!("Waiting for 25s to ensure deallocation").await;
            let _req = std::thread::sleep(std::time::Duration::new(25, 0)); //unsafe { get() };
            eprintln!("Continue with test").await;
        }

        

        // Wait for containers to deallocate (optional)
        // async_std::task::sleep(std::time::Duration::new(40, 0)).await;
    }

    // Serialize and return the cold start test results
    serde_json::to_value(cold_start_results).unwrap()
}

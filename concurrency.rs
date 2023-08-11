// Import necessary modules and items from the crate
use crate::SERVER_URI;
use async_std::eprintln;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use serde_json::Value;

// Define static mutable variables to hold the server URI and OpenWhisk authorization
// static mut SERVER_ACTION_URI: String = String::new();
// static mut WHISK_AUTH: String = String::new();

// Constants for the test conditions
const ABORT_AFTER: usize = 3;
const INCREASE_AFTER: u128 = 5000;

// Function to make a request to the server
// pub fn make_request(body: surf::Body) -> impl Future<Output = Result<Value, surf::Error>> {
//     // Access the server URI and OpenWhisk authorization using unsafe, as they are static mutable variables
//     let uri = unsafe { &SERVER_ACTION_URI };
//     let auth = unsafe { &WHISK_AUTH };

//     // Create a POST request with the provided URI, body, and authorization header
//     let request = surf::post(uri)
//         .body(body)
//         .header("Authorization", format!("Basic {}", auth))
//         .recv_json::<serde_json::Value>();

//     request // Return the future representing the request
// }

// From cold_start.rs
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

// Structure to hold the results of a concurrency test
use serde::Serialize;
#[derive(Serialize, Debug)]
pub struct ConcurrencyResult {
    pub no_concurrent_requests: usize,
    pub responses: Vec<Value>,
}

// Asynchronous function to perform a concurrency test with varying numbers of concurrent requests
pub async fn concurrency_test<F>(action_name: &str, param: F) -> Value
where
    F: Fn() -> Value,
{
    // Create the path for the request
    let path = format!(
        "{}/api/v1/namespaces/_/actions/{}?blocking=true&result=false",
        SERVER_URI, action_name
    );

    // Set the server URI using unsafe, as it is a static mutable variable
    // unsafe {
    //     SERVER_ACTION_URI = path;
    // }

    // Encode the OpenWhisk authorization into Base64
    let auth = base64::encode(std::env::var("WHISK_AUTH").unwrap());

    // Set the OpenWhisk authorization using unsafe, as it is a static mutable variable
    // unsafe {
    //     WHISK_AUTH = auth;
    // }

    // Get the parameters for the request using the provided param function
    let parameters = param();
    let body = surf::Body::from_json(&parameters).unwrap(); // Create the request body

    // Create a vector containing a single request (to be expanded later with more concurrent requests)
    // let requests = vec![make_request(body)];

    let requests = vec![make_request(&path, &auth, body)];

    // Create a FuturesUnordered stream to hold the future requests
    let mut futures = requests.into_iter().collect::<FuturesUnordered<_>>();

    // Create vectors to hold the responses and concurrency test results
    let mut responses = Vec::with_capacity(ABORT_AFTER * 3 * ABORT_AFTER);
    let mut concurrency_results = Vec::with_capacity(ABORT_AFTER);

    // Variables to track the number of added requests and the last added time
    let mut num_added = 1;
    let mut last_added = std::time::Instant::now();

    // Loop until there are no more responses from the FuturesUnordered stream
    while let Some(res) = futures.next().await {
        match res {
            Ok(response) => responses.push(response), // Store successful responses
            Err(err) => eprintln!("Err: {:?}", err).await, // Print errors for failed requests
        }

        // Check if the maximum number of added requests is reached
        if num_added > ABORT_AFTER {
            eprintln!("Aborting after {} additions", num_added).await;
            break; // Stop the test if the maximum number of requests is reached
        } else if last_added.elapsed().as_millis() > INCREASE_AFTER {
            // Check if the time since the last addition exceeds the threshold to increase concurrent requests
            // Store the concurrency test result for the current number of concurrent requests
            concurrency_results.push(ConcurrencyResult {
                no_concurrent_requests: num_added,
                responses,
            });

            responses = Vec::with_capacity(ABORT_AFTER); // Clear the responses for the next batch

            // Prepare and add a new concurrent request
            let parameters = param();
            let body = surf::Body::from_json(&parameters).unwrap();
            futures.push(make_request(&path, &auth, body));

            last_added = std::time::Instant::now(); // Update the last added time
            num_added += 1; // Increment the number of added requests

            eprintln!("Issuing {} concurrent requests", num_added).await;
        }

        // Prepare and add a new concurrent request
        let parameters = param();
        let body = surf::Body::from_json(&parameters).unwrap();
        futures.push(make_request(&path, &auth, body));
    }

    // Serialize and return the concurrency test results
    serde_json::to_value(concurrency_results).unwrap()
}

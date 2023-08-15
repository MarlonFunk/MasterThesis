// Import necessary modules and items from the crate
use crate::SERVER_URI;
use async_std::eprintln;
use async_std::sync::channel; // Import channel from async_std
use futures::{stream::FuturesUnordered, Future, StreamExt};
use futures::future::join_all;
use serde_json::Value;

// Define static mutable variables to hold the server URI and OpenWhisk authorization
// static mut SERVER_ACTION_URI: String = String::new();
// static mut WHISK_AUTH: String = String::new();

// Constants for the test conditions
const START_SIZE: usize = 3;
const INCREASE_SIZE: usize = 10;
const ABORT_AFTER: usize = 80;
const INCREASE_AFTER: u128 = 1000;

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

// Asynchronous function to perform a concurrency test with varying numbers of concurrent requests
pub async fn capacity_test<F>(action_name: &str, param: F) -> Value
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


    // Create vectors to hold the responses and concurrency test results
    let mut concurrency_results = Vec::with_capacity(ABORT_AFTER);

    let mut failed = false;
    

    // Variables to track the number of added requests and the last added time
    let mut num_added: usize = START_SIZE;
    let mut last_added = std::time::Instant::now();
    let mut futures = requests.into_iter().collect::<FuturesUnordered<_>>();

    for _ in 1..=START_SIZE-1 {
        let parameters = param();
        let body = surf::Body::from_json(&parameters).unwrap();
        futures.push(make_request(&path, &auth, body));
    }

    let mut activation_ids = vec![];
    let mut responses = Vec::with_capacity(ABORT_AFTER * 3 * ABORT_AFTER);


    eprintln!("Sending {} concurrent requests", num_added).await;

    loop {

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
            // None => break, // No more responses, break the loop
            None =>  break,
        } // all futures send and results received
    }
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

    concurrency_results.push(ConcurrencyResult {
        no_concurrent_requests: num_added,
        responses,
    });
    
    if failed {
        eprintln!("aborting").await;
        std::process::exit(-1); // Abort the test if any request fails
    }
    
    if last_added.elapsed().as_millis() > INCREASE_AFTER {
        // Check if the time since the last addition exceeds the threshold to increase concurrent requests
        // Store the concurrency test result for the current number of concurrent requests
        responses = Vec::with_capacity(ABORT_AFTER); // Clear the responses for the next batch

        // Prepare and add a NEW concurrent request
        let parameters = param();
        let body = surf::Body::from_json(&parameters).unwrap();
        futures.push(make_request(&path, &auth, body));

        last_added = std::time::Instant::now(); // Update the last added time
        num_added += INCREASE_SIZE; // Increment the number of added requests

        eprintln!("Issuing {} concurrent requests", num_added).await;
    }
        // last_added = std::time::Instant::now(); // Update the last added time
        // num_added += 1; // Increment the number of added requests

        // eprintln!("Issuing {} concurrent requests", num_added).await;
    //}

    // Prepare and add a new concurrent request
    for _ in 1..=num_added {
        let parameters = param();
        let body = surf::Body::from_json(&parameters).unwrap();
        futures.push(make_request(&path, &auth, body));
    }

    // Serialize and return the concurrency test results
    serde_json::to_value(concurrency_results).unwrap()
}

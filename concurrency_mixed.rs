// Import necessary modules and items from the crate
use crate::SERVER_URI;
use async_std::eprintln;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use serde_json::Value;
use serde_json::json;

use rand::prelude::*;
use rand::distributions::WeightedIndex;

// Define static mutable variables to hold the server URI and OpenWhisk authorization
// static mut SERVER_ACTION_URI: String = String::new();
// static mut WHISK_AUTH: String = String::new();

// Constants for the test conditions
const ABORT_AFTER: usize = 3;
const INCREASE_AFTER: u128 = 5000;
const ABORT_SCRIPT: u128 = 60000; //600000;

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
) -> impl Future<Output = Result<Value, surf::Error>> {
    // Create a POST request with the provided URI, body, and authorization header
    let request = surf::post(uri)
        .body(json!({}))
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
pub async fn concurrency_test<F>(first_action_name: &str, second_action_name: &str, param: F) -> Value
where
    F: Fn() -> Value,
{
    // Create the path for the request
    let first_path = format!(
        "{}/api/v1/namespaces/_/actions/{}?blocking=true&result=false",
        SERVER_URI, first_action_name
    );
    let second_path = format!(
        "{}/api/v1/namespaces/_/actions/{}?blocking=true&result=false",
        SERVER_URI, second_action_name
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

    
    // Create a vector containing a single request (to be expanded later with more concurrent requests)
    // let requests = vec![make_request(body)];

    
    //--------------------------------------------------------------------------------

    // let first_requests = vec![make_request(&first_path, &auth, body)];
    // let second_requests = vec![make_request(&second_path, &auth, body)];

    // // Create a FuturesUnordered stream to hold the future requests
    // let mut first_futures = first_requests.into_iter().collect::<FuturesUnordered<_>>();
    // let mut second_futures = second_requests.into_iter().collect::<FuturesUnordered<_>>();

    // // Create vectors to hold the responses and concurrency test results
    // let mut first_responses = Vec::with_capacity(ABORT_AFTER * 3 * ABORT_AFTER);
    // let mut second_responses = Vec::with_capacity(ABORT_AFTER * 3 * ABORT_AFTER);

    // let mut first_concurrency_results = Vec::with_capacity(ABORT_AFTER);
    // let mut second_concurrency_results = Vec::with_capacity(ABORT_AFTER);
    
    //--------------------------------------------------------------------------------

    // Get the parameters for the request using the provided param function
    let parameters = param();
    let first_body = surf::Body::from_json(&parameters).unwrap(); // Create the request body
    let second_body = surf::Body::from_json(&parameters).unwrap(); // Create the request body

    
    let first_requests = vec![make_request(&first_path, &auth)];//, first_body)];
    let second_requests = vec![make_request(&second_path, &auth)];//, second_body)];

    // let amount_first = 10-5; //TODO: here parameter
    // let amount_second = 10 - amount_first;

    // let mut futures: FuturesUnordered<()> = FuturesUnordered::new(); // Why not working?

    //let mut futures = first_requests.into_iter().collect::<FuturesUnordered<_>>();
    // for i in 1..=10 {
    //     if i < amount_first-1 { //first one in initialization
    //         futures.push(make_request(&first_path, &auth));
    //     }
    //     else {
    //         futures.push(make_request(&second_path, &auth));
    //     }
    // }


    // Create a FuturesUnordered stream to hold the future requests
    // let mut futures = first_requests.into_iter().collect::<FuturesUnordered<_>>();
    // futures.push(make_request(&second_path, &auth));//, first_body));

    let mut futures = FuturesUnordered::new();

    let probabilities = [5, 5];
    let mut rng = thread_rng();
    let distribution = WeightedIndex::new(&probabilities).unwrap();
    let mut random_index = distribution.sample(&mut rng);
    match random_index {
        0 => {
            futures.push(make_request(&first_path, &auth));    

        }
        1 => {
            futures.push(make_request(&second_path, &auth));    

        }
        _ => {
            panic!("Unexpected value for var: {}", random_index);
        }
    }
    // Create vectors to hold the responses and concurrency test results
    let mut responses = Vec::with_capacity(10 * 3 * 10);
    let mut concurrency_results = Vec::with_capacity(10);

    let start_time = std::time::Instant::now();

    // Loop until there are no more responses from the FuturesUnordered stream
    while let Some(res) = futures.next().await {
        match res {
            Ok(response) => responses.push(response), // Store successful responses
            Err(err) => eprintln!("Err: {:?}", err).await, // Print errors for failed requests
        }

        if start_time.elapsed().as_millis() > ABORT_SCRIPT {
            // Check if the time since the last addition exceeds the threshold to increase concurrent requests
            // Store the concurrency test result for the current number of concurrent requests
            concurrency_results.push(ConcurrencyResult {
                no_concurrent_requests: 10,
                responses,
            });
            eprintln!("Aborting after {} miliseconds", ABORT_SCRIPT).await;
            break;          
        }
        eprintln!("{}", serde_json::to_value(responses.to_vec()).unwrap());

        random_index = distribution.sample(&mut rng);
        match random_index {
            0 => {
                futures.push(make_request(&first_path, &auth));    
            }
            1 => {
                futures.push(make_request(&second_path, &auth));    
            }
            _ => {
                panic!("Unexpected value for var: {}", random_index);
            }
        }           
        
    }
    // Serialize and return the concurrency test results
    serde_json::to_value(concurrency_results).unwrap()
}

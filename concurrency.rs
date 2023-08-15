// // Import necessary modules and items from the crate
// use crate::SERVER_URI;
// use async_std::eprintln;
// use futures::{stream::FuturesUnordered, Future, StreamExt};
// use serde_json::Value;

// // Define static mutable variables to hold the server URI and OpenWhisk authorization
// // static mut SERVER_ACTION_URI: String = String::new();
// // static mut WHISK_AUTH: String = String::new();

// // Constants for the test conditions
// const ABORT_AFTER: usize = 12;
// const INCREASE_AFTER: u128 = 5000;

// // Function to make a request to the server
// // pub fn make_request(body: surf::Body) -> impl Future<Output = Result<Value, surf::Error>> {
// //     // Access the server URI and OpenWhisk authorization using unsafe, as they are static mutable variables
// //     let uri = unsafe { &SERVER_ACTION_URI };
// //     let auth = unsafe { &WHISK_AUTH };

// //     // Create a POST request with the provided URI, body, and authorization header
// //     let request = surf::post(uri)
// //         .body(body)
// //         .header("Authorization", format!("Basic {}", auth))
// //         .recv_json::<serde_json::Value>();

// //     request // Return the future representing the request
// // }

// // From cold_start.rs
// pub fn make_request(
//     uri: &str,
//     auth: &str,
//     body: surf::Body,
// ) -> impl Future<Output = Result<Value, surf::Error>> {
//     // Create a POST request with the provided URI, body, and authorization header
//     let request = surf::post(uri)
//         .body(body)
//         .header("Authorization", format!("Basic {}", auth))
//         .recv_json::<serde_json::Value>(); // Expect the response body to be deserialized as JSON

//     request // Return the future representing the request
// }

// pub fn make_concurrent_requests<F>(
//     count: usize,
//     action_name: &str,
//     auth: &str,
//     param: &mut F,
// ) -> FuturesUnordered<impl Future<Output = Result<Value, surf::Error>>>
// where
//     F: Fn() -> Value, // The param function returns a JSON Value
// {
//     let mut futures = Vec::with_capacity(count);

//     // Create `count` concurrent requests by appending a unique index to the action name
//     for i in 0..count {
//         let mut action = action_name.to_owned();
//         action.push_str(&(i + 1).to_string());
//         std::eprintln!("Executing action: {}", action);

//         // Construct the path for the request
//         let path = format!(
//             "{}/api/v1/namespaces/_/actions/{}?blocking=true&result=false",
//             SERVER_URI, action
//         );

//         // Obtain the parameters for the request using the provided param function
//         let parameters = param();
//         let body = surf::Body::from_json(&parameters).unwrap(); // Create the request body

//         // Push the request future into the futures vector
//         futures.push(make_request(&path, auth, body));
//     }

//     futures.into_iter().collect::<FuturesUnordered<_>>() // Convert the futures vector into a FuturesUnordered stream
// }

// // Structure to hold the results of a concurrency test
// use serde::Serialize;
// #[derive(Serialize, Debug)]
// pub struct ConcurrencyResult {
//     pub no_concurrent_requests: usize,
//     pub responses: Vec<Value>,
// }


// // Asynchronous function to collect the details of an activation using its ID
// async fn collect_activation(activation_id: &str, auth: &str) -> Value {
//     // Construct the path for fetching activation details
//     let path = format!(
//         "{}/api/v1/namespaces/_/activations/{}",
//         SERVER_URI, activation_id
//     );

//     // Send a GET request to the path with the provided authorization header
//     surf::get(&path)
//         .header("Authorization", format!("Basic {}", auth))
//         .recv_json::<serde_json::Value>()
//         .await
//         .unwrap() // Expect the response body to be deserialized as JSON
// }

// // Asynchronous function to perform a concurrency test with varying numbers of concurrent requests
// pub async fn concurrency_test<F>(action_name: &str, param: F) -> Value
// where
//     F: Fn() -> Value,
// {
//     // Create the path for the request
//     let path = format!(
//         "{}/api/v1/namespaces/_/actions/{}?blocking=true&result=false",
//         SERVER_URI, action_name
//     );

//     // Set the server URI using unsafe, as it is a static mutable variable
//     // unsafe {
//     //     SERVER_ACTION_URI = path;
//     // }

//     // Encode the OpenWhisk authorization into Base64
//     let auth = base64::encode(std::env::var("WHISK_AUTH").unwrap());

//     // Set the OpenWhisk authorization using unsafe, as it is a static mutable variable
//     // unsafe {
//     //     WHISK_AUTH = auth;
//     // }

//     // Get the parameters for the request using the provided param function
//     let parameters = param();
//     let body = surf::Body::from_json(&parameters).unwrap(); // Create the request body

//     // Create a vector containing a single request (to be expanded later with more concurrent requests)
//     // let requests = vec![make_request(body)];

//     let requests = vec![make_request(&path, &auth, body)];

//     // Create a FuturesUnordered stream to hold the future requests
//     let mut futures = requests.into_iter().collect::<FuturesUnordered<_>>();

//     // Create vectors to hold the responses and concurrency test results
//     // let mut responses = Vec::with_capacity(ABORT_AFTER * 3 * ABORT_AFTER);
//     let mut concurrency_results = Vec::with_capacity(ABORT_AFTER);

//     let mut failed = false;
    

//     // Variables to track the number of added requests and the last added time
//     let mut num_added = 1;
//     let mut last_added = std::time::Instant::now();


//     // loop {
//         let mut responses = Vec::with_capacity(ABORT_AFTER * 3 * ABORT_AFTER);
//         let mut activation_ids = vec![];

//     // Loop until there are no more responses from the FuturesUnordered stream
//     // while let Some(res) = futures.next().await {
//         // match res {
//         //     Ok(response) => responses.push(response), // Store successful responses
//         //     Err(err) => {
//         //         eprintln!("Err: {:?}", err).await; // Print errors for failed requests
//         //         concurrency_results.push(ConcurrencyResult {
//         //                     no_concurrent_requests: num_added,
//         //                     responses,
//         //                 });
//         //         break;
//         //     },
            
            
//         // }
        
//         // Check if the maximum number of added requests is reached
//         // if num_added > ABORT_AFTER {
//         //     eprintln!("Aborting after {} additions", num_added).await;
//         //     concurrency_results.push(ConcurrencyResult {
//         //         no_concurrent_requests: num_added,
//         //         responses,
//         //     });
//         //     break; // Stop the test if the maximum number of requests is reached
//         // } else if last_added.elapsed().as_millis() > INCREASE_AFTER {


//         match futures.next().await {
//             Some(response) => match response {
//                 Ok(res) => {
//                     if let Some(_) = res.get("response") {
//                         eprintln!("Response: {:?}", res).await;
//                         responses.push(res); // Store successful responses
//                     } else {
//                         let activation_id = res.get("activationId").unwrap().as_str().unwrap();
//                         activation_ids.push(activation_id.to_owned()); // Store activation IDs for non-blocking responses
//                     }
//                 }
//                 Err(err) => {
//                     eprintln!("Err: {:?}", err).await;
//                     failed = true; // Mark if any request fails
//                 }
//             },
//             // None => break, // No more responses, break the loop
//             None =>  eprintln!("NONE!").await,
//         } // all futures send and results received

//         if !activation_ids.is_empty() {
//             eprintln!("Collect {num} activations?", num = activation_ids.len()).await;
//             let stdin = async_std::io::stdin();
//             let mut line = String::new();
//             stdin.read_line(&mut line).await.unwrap();

//             if line.starts_with("y") {
//                 for activation_id in activation_ids {
//                     eprintln!("Collecting {activation_id}", activation_id = activation_id).await;
//                     let response = collect_activation(&activation_id, &auth).await;
//                     responses.push(response); // Fetch and store the activation details for non-blocking responses
//                 }
//             }
//         }

//         concurrency_results.push(ConcurrencyResult {
//             no_concurrent_requests: num_added,
//             responses,
//         });

//         // if last_added.elapsed().as_millis() > INCREASE_AFTER {

//             // Check if the time since the last addition exceeds the threshold to increase concurrent requests
//             // Store the concurrency test result for the current number of concurrent requests
            

//             // responses = Vec::with_capacity(ABORT_AFTER); // Clear the responses for the next batch

//             // Prepare and add a new concurrent request
//             // let parameters = param();
//             // let body = surf::Body::from_json(&parameters).unwrap();
//             // futures.push(make_request(&path, &auth, body));

//             // last_added = std::time::Instant::now(); // Update the last added time
//             // num_added += 1; // Increment the number of added requests

//             // eprintln!("Issuing {} concurrent requests", num_added).await;
//         //}

//         // Prepare and add a new concurrent request
//         for _ in 1..=num_added {
//             let parameters = param();
//             let body = surf::Body::from_json(&parameters).unwrap();
//             futures.push(make_request(&path, &auth, body));
//         }

        
//         num_added += 1; // Increment the number of added requests
//         eprintln!("Issuing {} concurrent requests", num_added).await;
//     //}

//         if failed {
//             eprintln!("aborting").await;
//             // break; // Abort the test if any request fails
//         }
//     // }
//     // Serialize and return the concurrency test results
//     serde_json::to_value(concurrency_results).unwrap()
// }







// ###################################################################################################











// Import necessary modules and items from the crate
use crate::SERVER_URI;
use async_std::eprintln;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use serde_json::Value;

// Define static mutable variables to hold the server URI and OpenWhisk authorization
// static mut SERVER_ACTION_URI: String = String::new();
// static mut WHISK_AUTH: String = String::new();

// Constants for the test conditions
const ABORT_AFTER: usize = 15;
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

            // Prepare and add a NEW concurrent request
            let parameters = param();
            let body = surf::Body::from_json(&parameters).unwrap();
            futures.push(make_request(&path, &auth, body));

            last_added = std::time::Instant::now(); // Update the last added time
            num_added += 1; // Increment the number of added requests

            eprintln!("Issuing {} concurrent requests", num_added).await;
        }

        // Prepare and add request
        let parameters = param();
        let body = surf::Body::from_json(&parameters).unwrap();
        futures.push(make_request(&path, &auth, body));
    }

    // Serialize and return the concurrency test results
    serde_json::to_value(concurrency_results).unwrap()
}
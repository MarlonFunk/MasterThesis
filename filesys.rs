use ow_common::ActionCapabilities;
use ow_evaluation::{benchmark, get_first_arg, init};
use serde_json::json;

//The main function is marked with the async_std::main attribute, indicating it is the entry point of the program and will run asynchronously.
#[async_std::main]
async fn main() -> anyhow::Result<()> {
    //The file_name variable is assigned the value of the first command-line argument obtained using the get_first_arg function from the ow_evaluation crate.
    let file_name = get_first_arg()?;

    //An ActionCapabilities struct is created, representing the capabilities of the action. In this case, it specifies a 
    //directory ("/tmp/filesys") for the action to operate on. Other capabilities may be present, depending on the specific implementation.
    let capabilities = ActionCapabilities {
        dir: Some("/tmp/filesys".into()),
        ..Default::default()
    };

    //The init function from the ow_evaluation crate is called to initialize the action. It takes the name of the action ("filesys"), 
    //the file_name, and the capabilities as arguments. This function likely sets up the necessary environment and prepares the action for execution.
    init("filesys", file_name, capabilities).await?;

    //The generate_test_file function is called asynchronously to create a test file of size 30,000,000 bytes (NUM_BYTES). 
    //It generates a vector random_bytes containing random data and writes it to the file "/tmp/filesys/test.txt" using async_std::fs::write.
    generate_test_file().await;

    //The variable num_requests is set to twice the number of CPUs available on the system, obtained through the num_cpus::get() function.
    let num_requests = num_cpus::get() * 2;

    /*
    The function initializes an empty vector called requests with a capacity of num_requests using Vec::with_capacity.

    An ActivationContext struct is created with various fields set. The parameters are assigned to the value field, while other fields like namespace, 
    action_name, and api_host are given default or empty values.

    The activation_ctx is serialized into a JSON value using serde_json::to_value. If the resulting JSON value is an object, the "deadline" field is 
    modified from a string value to the string "0" to match the expected type.

    A loop iterates num_requests times to generate requests. Inside the loop, an activation_ctx_json JSON value is used to construct a request body 
    using surf::Body::from_json. The request is then created with surf::post to a server URI followed by "/run", and the request body is set using 
    body(body).

    Each request is asynchronously sent using recv_json::<serde_json::Value>(), which expects the response body to be deserialized into a JSON value.

    Each request is added to the requests vector using requests.push(request).

    The requests vector is transformed into a FuturesUnordered iterator using into_iter().collect::<FuturesUnordered<_>>(). This iterator allows us 
    to asynchronously process the requests.

    A new vector called responses is initialized with a capacity of num_requests.

    A timestamp is recorded using std::time::Instant::now() to measure the elapsed time for request processing.

    A while loop is executed, awaiting the next request result from the futures iterator using futures.next().await.

    Inside the loop, the response is matched against Ok(json) and Err(err). If the response is successful, it is printed as "Request completed" and 
    added to the responses vector. If there is an error, it is printed as "Recv Error".

    Once all the requests are processed, the elapsed time is calculated by subtracting the start time recorded earlier using 
    before.elapsed().as_millis().

    The function prints the total time taken for request processing.

    The responses vector containing the response JSON values is returned.

    In summary, the benchmark function generates and asynchronously executes multiple requests, collects the responses, and returns them in a vector. 
    It also measures the elapsed time for the request processing and prints it to the console.
    */
    let responses = benchmark(num_requests, json!({})).await;

    //A loop iterates over each response in the responses vector. It retrieves the "result" field from each response, assuming it exists. 
    //The execution time for write and read operations (write_read_time) is printed for each response.
 
    for response in responses.iter() {
        let res = response.get("result").unwrap();
        println!(
            "filesys write/read time: {}ms",
            res.get("write_read_time").unwrap(),
        );
    }

    Ok(())
}

async fn generate_test_file() {
    const NUM_BYTES: usize = 30_000_000;

    let mut random_bytes: Vec<u8> = Vec::with_capacity(NUM_BYTES);

    for i in 0..NUM_BYTES {
        random_bytes.push((i % 255) as u8);
    }

    //The generate_test_file function generates a test file with random data. It initializes an empty vector random_bytes with capacity NUM_BYTES. 
    //Then, it populates the vector with bytes ranging from 0 to 254, looping through i from 0 to NUM_BYTES - 1 and pushing (i % 255) as u8 into 
    //random_bytes. Finally, it uses async_std::fs::write to write random_bytes to the file "/tmp/filesys/test.txt" asynchronously.
    async_std::fs::write("/tmp/filesys/test.txt", &random_bytes)
        .await
        .unwrap();
}

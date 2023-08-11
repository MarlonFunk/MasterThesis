// If the "wasm" feature is enabled, the following macro will be applied.
// This macro is used for WebAssembly builds to pass JSON data to the `func` function.
#[cfg(feature = "wasm")]
ow_wasm_action::pass_json!(func);

// If the "wasm" feature is not enabled, the following macro will be applied instead.
// This macro is used for non-WebAssembly builds to handle JSON arguments for the `func` function.
#[cfg(not(feature = "wasm"))]
ow_wasm_action::json_args!(func);

// The `func` function takes a `serde_json::Value` as input and returns a `Result<serde_json::Value, anyhow::Error>`.
pub fn func(json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    // Extract the value associated with the key "iterations" from the JSON object.
    // This value is expected to be an integer (i64), and it will be converted to a usize.
    let iterations = json.get("iterations").unwrap().as_i64().unwrap() as usize;

    // Extract the value associated with the key "input" from the JSON object.
    // This value is expected to be a string, and it will be stored in the `input` variable.
    let input = json.get("input").unwrap().as_str().unwrap();

    // If the "hash" feature is enabled, the following code block will be executed.
    // This block performs a hash computation using the BLAKE3 hash function.
    #[cfg(feature = "hash")]
    let hash = {
        let mut prev_output;
        let mut hash = input.as_bytes();

        // Perform the hash computation for a given number of iterations.
        for _ in 0..iterations {
            prev_output = blake3::hash(hash);
            hash = prev_output.as_bytes();
        }

        // Convert the final hash to a Vec<u8>.
        hash.to_vec()
    };

    // Create a JSON object containing the result of the computation.
    // The result includes the "hash" key with the value being the hexadecimal representation of the computed hash.
    Ok(serde_json::json!({ "hash": format!("{:x?}", hash) }))
}

// Conditional compilation based on the feature "wasm".
// If the "wasm" feature is enabled, this line uses a macro from the "ow_wasm_action" crate
// to pass the JSON input to the `func` function.
#[cfg(feature = "wasm")]
ow_wasm_action::pass_json!(func);

// If the "wasm" feature is not enabled, this line uses a macro from the "ow_wasm_action" crate
// to deserialize JSON input into arguments for the `func` function.
#[cfg(not(feature = "wasm"))]
ow_wasm_action::json_args!(func);

// Import the standard library's "Instant" type to measure elapsed time.
use std::time::Instant;

// The `func` function takes a JSON input (`_json`) and returns a `Result` containing either
// a JSON value or an error (`anyhow::Error`).
pub fn func(_json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    // Create a new `Instant` instance to record the current time.
    let now = Instant::now();

    // Calculate the elapsed time since the creation of the `Instant` instance,
    // and convert it to nanoseconds (as usize).
    let elapsed = now.elapsed().as_nanos() as usize;

    // Create a JSON object containing the elapsed time and return it as a successful result.
    Ok(serde_json::json!({ "elapsed": elapsed }))
}

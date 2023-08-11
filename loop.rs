#[cfg(feature = "wasm")]
ow_wasm_action::pass_json!(func);

#[cfg(not(feature = "wasm"))]
ow_wasm_action::json_args!(func);

// fn func(_json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
//     let mut t = 0;
//     for i in 1..=1000000000 {
//         t = i;
//     }
//     Ok(serde_json::json!({ "result": t}))
// }


fn func(_json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {

    let mut result: f64 = 0.0;
    loop {
        result = (result + 1.0) / 2.0;
    }
    Ok(serde_json::json!({ "result": result}))
}
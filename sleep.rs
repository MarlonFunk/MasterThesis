#[cfg(feature = "wasm")]
ow_wasm_action::pass_json!(func);

#[cfg(not(feature = "wasm"))]
ow_wasm_action::json_args!(func);

pub fn func(json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    let input = json
        .get("input")
        .ok_or_else(|| anyhow::anyhow!("Expected input to be present"))?;

        let request_time = std::time::Instant::now();

        // 10 000 000 = 10s //no spaces in actual input!

        if let Some(timeout) = input.as_u64() {
            let u32_timeout = timeout as u32;
            let _req = std::thread::sleep(std::time::Duration::new(0, u32_timeout*1000000)); //unsafe { get() };

        }

        
       
    
        Ok(serde_json::json!({
            "request_time": request_time.elapsed().as_millis() as u64,
        }))
}
use ow_evaluation::cold_start::cold_start_test;
use serde_json::json;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // let responses = cold_start_test("hello", || {
    //     json!({
    //         "input": "there!",
    //     })
    // })
    // .await;
    let responses = cold_start_test("hash", || {
        let random = rand::random::<i64>();
        json!({
            "iterations": 100000,
            "input": random.to_string(),
        })
    })
    .await;

    println!("{}", serde_json::to_string(&responses).unwrap());

    Ok(())
}


/*

Response: 
Object({"activationId": String("00a4cb2a10604498a4cb2a10605498b8"), 
"annotations": Array([Object({"key": String("path"), "value": String("guest/clock")}), 
Object({"key": String("waitTime"), "value": Number(21)}), 
Object({"key": String("kind"), "value": String("wasm:0.1")}), 
Object({"key": String("timeout"), "value": Bool(false)}), 
Object({"key": String("limits"), "value": 
    Object({"concurrency": Number(1), "logs": Number(10), "memory": Number(256), "timeout": Number(60000)})}), 
Object({"key": String("initTime"), "value": Number(5)})]), "duration": Number(6), "end": Number(1690639553680), 
    "logs": Array([]), "name": String("clock"), "namespace": String("guest"), "publish": Bool(false), 
    "response": Object({"result": Object({"result": Object({"elapsed": Number(534)}), 
    "status": String("success"), "status_code": Number(0), "success": Bool(true)}), "size": Number(76), "status": String("success"), "success": Bool(true)}), 
    "start": Number(1690639553674), "subject": String("guest"), "version": String("0.0.1")})


*/
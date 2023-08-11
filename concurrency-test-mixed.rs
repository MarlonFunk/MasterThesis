use ow_evaluation::concurrency_mixed::concurrency_test;
use serde_json::json;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // let responses = concurrency_test("hello", || {
    //     json!({
    //         "input": "there!",
    //     })
    // })
    // .await;

    // let responses = concurrency_test("sleep", || {
    //     json!({
    //         "input": "1500000000!",
    //     })
    // })
    // .await;
    let responses = concurrency_test("net", "prime",|| {
        json!({})
    })
    .await;

    println!("{}", serde_json::to_string(&responses).unwrap());

    Ok(())
}

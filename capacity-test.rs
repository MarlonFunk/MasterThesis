use ow_evaluation::capacity::capacity_test;
use serde_json::json;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let responses = capacity_test("sleep", || {
        json!({
            "input": 15,
        })
    })
    .await;



    
    println!("{}", serde_json::to_string(&responses).unwrap());

    Ok(())
}

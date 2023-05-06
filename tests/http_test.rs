use anyhow::Result;

#[tokio::test]
async fn test1() -> Result<()> {
    let httpClient = httpc_test::new_client("http://localhost:3000");

    httpClient.do_get("/health").await?.print().await?;

    Ok(())
}

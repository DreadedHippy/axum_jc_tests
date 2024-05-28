#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:3000")?;

	// hc.do_get("/hello2/Mike").await?.print().await?;

	hc.do_post("/api/login", json!({
		"username": "demo1",
		"pwd": "welcome"
	})).await?.print().await?;

	let req_ceate_ticket = hc.do_post("/api/tickets", json!({
		"title": "AAA"
	}));

	req_ceate_ticket.await?.print().await?;
	hc.do_delete("/api/tickets/0").await?.print().await?;

	hc.do_get("/api/tickets").await?.print().await?;
	// hc.do_get("/src/main.rs").await?.print().await?;
	Ok(())
}
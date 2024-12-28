use tokio;

#[tokio::test]
async fn test_get_root() {

    // Set up mock upstream server for testing that 
    // requests get forwarder 

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("backend")
        )
        .expect(1)
        .mount(&mock_server)
        .await;
    
    let client = Client::new();

    // The load balancer being tested. 
    let server = LoadBalancer::new(8080, vec![mock_server.uri()]);

    let server_uri = server.uri();
    
    tokio::spawn(async move { server.run().await });

    // Wait for server to spin up 

    tokio::time::sleep(std::time::Duration::from_secs(3));
    // Check that response is received from mock server 
    let response = client.get(server_uri).send().await.unwrap();
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(StatusCode::OK, response.text().await.unwrap());
}

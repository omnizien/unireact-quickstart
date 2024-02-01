use warp::{Filter, Reply, Rejection, http::Response as HttpResponse};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio;

// If you're using any additional features from `reqwest`, `warp`, or `serde`,
// make sure to include the relevant imports here as well.



#[derive(Serialize, Deserialize)]
struct ResponseBody {
    status: String,
}
async fn forward_to_unity(message: String) -> Result<impl Reply, Rejection> {
    println!("Forwarding message to Unity: {}", message); // Log the incoming message

    let unity_url = "http://localhost:8080/unity-endpoint"; // URL where Unity listens for messages
    println!("Unity URL: {}", unity_url); // Log the Unity URL

    let client = reqwest::Client::new();

    // For testing, use a hardcoded test message
    let test_message = serde_json::json!({ "message": "Test from Warp" }).to_string();
    println!("Sending test message to Unity: {}", message); // Log the test message

    // Attempt to send the message
    let res = client.post(unity_url)
        .header("Content-Type", "application/json")
        .body(message)
        .send()
        .await;

    // Log the result of the attempt
    let response_body = match &res {
        Ok(response) => {
            println!("Message successfully forwarded to Unity, Response Status: {}", response.status()); // Log success and response status
            ResponseBody { status: "Message forwarded to Unity".to_string() }
        },
        Err(e) => {
            println!("Failed to forward message to Unity: {}", e); // Log the error
            ResponseBody { status: format!("Failed to forward message to Unity: {}", e) }
        },
    };

    // Return the response
    Ok(warp::reply::json(&response_body))
}


async fn send_to_unity_handler(body: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    // Extract the message or use a default value if it's not provided
    
    let message = body.get("message").cloned().unwrap_or_else(|| "Default message".to_string());

    println!("got");
    // Now, since we always have a message, we can directly call forward_to_unity
    // and don't need to worry about handling None separately.
    forward_to_unity(message).await
}


#[tokio::main]
async fn main() {
    let send_to_unity_route = warp::post()
        .and(warp::path("send-to-unity"))
        .and(warp::body::json())
        .and_then(send_to_unity_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["POST"]);

    let routes = send_to_unity_route.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3333))
        .await;
}
 


/* test code - remember to turn off socks5
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
 
    let res = client.post("http://localhost:8080/unity-endpoint")
        .body("test message") // Send plain text body similar to the curl command
        .header("Content-Type", "text/plain") // Set Content-Type header to text/plain
        .send()
        .await?;

    println!("Status: {}", res.status());
    Ok(())
}
*/
 
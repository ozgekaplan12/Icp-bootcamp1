
use reqwest::Client;

fn main() {
    // ICP endpoint URL
    let icp_endpoint = "https://example.com/icp/api";

    // Örnek bir ICP isteği oluşturun (Bu sadece bir örnek, gerçek ICP isteğine uygun olarak değiştirilmelidir)
    let icp_request_body = r#"{
        "command": "icp_info",
        "parameters": {
            "param1": "value1",
            "param2": "value2"
        }
    }"#;

    // ICP ile iletişim için bir HTTP isteği gönderin
    let response = match send_icp_request(icp_endpoint, icp_request_body) {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Error sending ICP request: {}", error);
            return;
        }
    };

    // İsteğin sonucunu yazdırın
    println!("ICP Response: {}", response);
}

// ICP isteği gönderen fonksiyon
fn send_icp_request(endpoint: &str, body: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let response = client.post(endpoint)
        .body(body.to_owned())
        .send()?;

    // Response'un içeriğini alın
    let response_body = response.text()?;

    Ok(response_body)
}

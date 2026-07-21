use mail_send::{
    Credentials, SmtpClientBuilder, mail_builder::MessageBuilder
};
use std::error::Error;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
#[tokio::main]
async fn main() {
    let username = "";
    let password = "";
    let url = "yoursite@domain";

    let interval = Duration::from_mins(5);
    

    loop {
        println!("Starting");
        match ping(&url).await {
        Ok(Some(body)) => {
            println!("Request was successful \n {}", body);
        }
        Ok(None) => {

            if let Err(err) = smtp_client(&username, &password).await {
                eprintln!("SMTP error {err}");
            }
            println!("Time to debug");
            break;
        }
        Err(err) => {
            eprintln!("Network or request error {}", err)
        }
    }

        sleep(interval).await;
    }   
}

async fn smtp_client(username: &str, password: &str) -> Result<(), Box<dyn Error>>{
    let mut client = SmtpClientBuilder::new("smtp.gmail.com", 465)?
    .credentials(Credentials::new(username, password))
    .connect()
    .await?;

    let from_email_address = "fromemail@example.com";
    let site_name = "";
    let recipients: Vec<String> = vec![
        "email1@example.com".to_string()
    ];

    
    let body = format!(r#"
    <html>
    <body>
    <h1>Alert Alert!</h1>
    <p>Your system site {} is currently down kindly check for resolve</p>
    <p>I'm just an assistant to help you track your system current status i.e 
    just notify you when the site is currently down!
    </p>
    <h3>Happy time debugging kiddie or get a lifetime server if you don't care about bills</h3>
    <p>Well, Thank you.</p>
    <body>
    </html>
    "#, site_name);

    let message_html = MessageBuilder::new()
    .from(("System Health checker", from_email_address))
    .subject("System status alert")
    .to(recipients)
    .html_body(&body);

    client.send(message_html).await?;

    print!("Email send successfully");
    Ok(())
}

async fn ping(url: &str) -> Result<Option<String>, Box<dyn Error>>{
    let client = Client::new();
    let response = client.get(url)
    .send()
    .await?;

    if response.status().is_success(){
        let text = response.text().await?;
        Ok(Some(text))
    }else {
        println!("Request failed with status: {}", response.status());
        Ok(None)
    }
}
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};

use crate::{ db, auth };


pub async fn send_verification_email(
    username: &str,
    user_id: i32,
    email_address: &str
) -> bool {
    // first get the verification code
    let new_verification_code: auth::NewVerificationCode = auth::NewVerificationCode::new(user_id);

    // TODO: create email template (askama). Use inline CSS.
    let email_body: String =
    format!(
        "<h3>Welcome to Crankade, {}!<br>
            <p>Your verification code is {}. 
            It expires in five minutes.",
        username,
        new_verification_code.raw_code
    );

    // PROCESS:
    // 1. put verification code in database
    // 2. send verification code via email to actual address
    // 3. create verification route
    // 3. verification check updates DB
    // 4. Use askama template

    // put "create and send verification" in a function
    // user can send NEW verification
    // once per minute

    // non-verified accounts cannot create a new game
    //
    // "reset password" is really just "login through email"
    // verification link can STILL be used here to log user in
    let resend_api: String = match std::env::var("RESEND_API") {
        Ok(api) => api,
        Err(e) => {
            eprintln!("Email Error: Failed to retrieve API key: {:?}", e);
            return false
        }
    };

    let resend: Resend = Resend::new(&resend_api);

    let from: &str = "noreply@mail.crankade.com";
    let to: [&str; 1] = [email_address];
    let subject: &str = "Welcome to Crankade!";

    let email_object: CreateEmailBaseOptions =
        CreateEmailBaseOptions::new(from, to, subject)
        .with_html(&email_body);

    match resend.emails.send(email_object).await {
        Ok(email_response) => println!("{:?}", email_response),
        Err(e) => eprintln!("Email Error: {:?}", e)
    };

    true
}
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};


pub async fn send_verification_email(email_address: &str) -> bool {
    // test resend email
    // TODO: put API key in .env
    // TODO: create email template
    // TODO: create a function for this
    // ACTUALLY probably create a whole module

    // PROCESS:
    // 1. put verification code in database
    // 2. send verification code via email to actual address
    // 3. create verification route
    // 3. verification check updates DB

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
            eprintln!("ERROR: failed to fetch API key");
            return false
        }
    };

    let resend: Resend = Resend::new(&resend_api);

    let from: &str = "noreply@mail.crankade.com";
    let to: [&str; 1] = [email_address];
    let subject: &str = "Hello World";

    let email_object: CreateEmailBaseOptions =
        CreateEmailBaseOptions::new(from, to, subject)
        .with_html("<p>Congrats on sending your <strong>first email</strong>!</p>");

    match resend.emails.send(email_object).await {
        Ok(email_response) => println!("{:?}", email_response),
        Err(e) => println!("Email Error: {:?}", e)
    };

    true
    // END OF resend email test
}
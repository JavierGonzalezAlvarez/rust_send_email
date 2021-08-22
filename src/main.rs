// rust_send_email/main.rs

//https://crates.io/crates/lettre
extern crate lettre;
extern crate lettre_email;

//use lettre::{EmailTransport, SmtpTransport};
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

//https://docs.rs/lettre/0.9.2/lettre/all.html
use lettre::smtp::authentication::Credentials;
//para fichero adjuntos
//use std::path::Path;

fn main() {
    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        //.to(("user@example.org", "Firstname Lastname"))
        .to("javier@gmail.com")
        //copia oculta
        .bcc("jcopia@gmail.com")
        // ... or by an address only
        .from("info@gmail.com")
        .subject("Confirmación de email enviado")
        //.text("Hello world. Rust_1")
        .html(" 
            <div>               
                Hola,<br><br>
                Prueba de email.<br>
                <a>Saludos.</a><br><br>
                <a>Recibe un cordial saludos</a><br>
                <a href='www.prueba.com'>www.prueba.com</a>
            </div>               
            ")
        .build()
        .unwrap();
    //configurar servidor de email    
    let mut mailer = SmtpClient::new_simple("ssl0.ovh.net")
        .unwrap()
        .credentials(Credentials::new("info@gmail.com".into(), "password_aqui".into()))
        //.credentials(Credentials::new("username".into(), "password".into()))
        .transport();
    // Send the email
    let result = mailer.send(email.into());
    if result.is_ok() {
        println!("Email enviado");
    } else {
        println!("No se puede enviar email: {:?}", result);
    }
    assert!(result.is_ok());    
}
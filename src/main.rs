extern crate reqwest;
extern crate scraper;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;

use std::io;
use std::time::Duration;
use std::thread;
use scraper::{Html, Selector};

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::SmtpTransportBuilder;
use lettre::{ClientSecurity, ClientTlsParameters, EmailTransport};
use lettre_email::EmailBuilder;
use native_tls::Protocol;
use native_tls::TlsConnector;


fn send_email(ip: &str, email_client: &str) {
     let email = EmailBuilder::new()
        .to(email_client)
        .from("rust.sender.services@gmail.com")
        .subject("Новый IP ".to_owned() + ip)
        .text("Новый IP ".to_owned() + ip)
        .build()
        .unwrap();

    pub const DEFAULT_TLS_PROT: &[Protocol] = &[Protocol::Tlsv10];

    let mut tls_builder = TlsConnector::builder().unwrap();
    tls_builder.supported_protocols(DEFAULT_TLS_PROT).unwrap();

    let tls_parameters =
        ClientTlsParameters::new("smtp.gmail.com".to_string(), tls_builder.build().unwrap());

    pub const SUBMISSION_PORT: u16 = 465;

    let mut mailer = SmtpTransportBuilder::new(
        ("smtp.gmail.com", SUBMISSION_PORT),
        ClientSecurity::Wrapper(tls_parameters),
    ).expect("Failed to create transport")
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(
            "rust.sender.services@gmail.com".to_string(),
            "aasl2302Fjsur4723Kadswl".to_string(),
        ))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .build();
    mailer.send(&email).unwrap();
    mailer.close();
}

fn get_ip() -> String {
    let body = reqwest::get("https://pr-cy.ru/browser-details/").unwrap().text().unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".ip").unwrap();
    let ip_new = document.select(&selector).next().unwrap().inner_html();
    ip_new
}

fn main() {
    println!("Введите почту, на которую будут приходить оповещения об изменении IP:");
    let mut email_client = String::new();
    io::stdin().read_line(&mut email_client).expect("Не удалось прочитать строку");
    println!("Программа работает, текущий IP был отправлен на указанную почту");
    email_client.trim();

    let mut ip_old = "".to_string();
    loop {
        let ip_new = get_ip();
        if ip_old != ip_new {
            ip_old = ip_new.clone();
            println!("{}", ip_old);
            send_email(&ip_new, &email_client.trim());
        }
        thread::sleep(Duration::new(60, 0));
    }
}

use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;

pub struct Email {
    smtp: String,
    receiver: String,
    user: String,
    password: String
}

impl Email {
    pub fn new(smtp: &str, receiver: &str, user: &str, password: &str) -> Email {
        Email {
            smtp: String::from(smtp),
            receiver: String::from(receiver),
            user: String::from(user),
            password: String::from(password)
        }
    }

    pub fn send(self, subject: &str, message: &str){
        let email = Message::builder()
            .from(self.user.parse().unwrap())
            .reply_to(self.user.parse().unwrap())
            .to(self.receiver.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(message))
            .unwrap();

        let credentials = Credentials::new(self.user.to_owned(), self.password.to_owned());

        let mailer = SmtpTransport::relay(self.smtp.as_str())
            .unwrap()
            .credentials(credentials)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => eprint!("Failed to send email: {e:?}")
        }
    }
}

use crate::config::AppConfig;
use aws_config::BehaviorVersion;
use aws_sdk_ses::types::{Body, Content, Destination, Message};
use aws_sdk_ses::{Client, Error};

pub struct EmailService {
    client: Client,
    from_email: String,
}

impl EmailService {
    pub async fn new(config: &AppConfig) -> Self {
        let aws_config = aws_config::defaults(BehaviorVersion::latest())
            .region(aws_sdk_ses::config::Region::new(config.aws_region.clone()))
            .load()
            .await;

        let client = Client::new(&aws_config);

        Self {
            client,
            from_email: config.from_email.clone(),
        }
    }

    #[allow(dead_code)]
    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), Error> {
        let dest = Destination::builder().to_addresses(to).build();

        let subject_content = Content::builder().data(subject).charset("UTF-8").build();
        let body_content = Content::builder().data(body).charset("UTF-8").build();
        let body = Body::builder().text(body_content.unwrap()).build();

        let msg = Message::builder()
            .subject(subject_content.unwrap())
            .body(body)
            .build();

        self.client
            .send_email()
            .source(&self.from_email)
            .destination(dest)
            .message(msg)
            .send()
            .await?;

        Ok(())
    }

    pub async fn send_html_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<(), Error> {
        let dest = Destination::builder().to_addresses(to).build();

        let subject_content = Content::builder().data(subject).charset("UTF-8").build();
        let html_content = Content::builder().data(html_body).charset("UTF-8").build();
        let body = Body::builder().html(html_content.unwrap()).build();

        let msg = Message::builder()
            .subject(subject_content.unwrap())
            .body(body)
            .build();

        self.client
            .send_email()
            .source(&self.from_email)
            .destination(dest)
            .message(msg)
            .send()
            .await?;

        Ok(())
    }
}

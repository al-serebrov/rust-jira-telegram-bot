use std::env;

use futures::StreamExt;
use telegram_bot::*;
extern crate env_logger;
extern crate goji;

use goji::{Credentials, Jira};

#[tokio::main]
async fn main() -> Result<(), Error> {
    drop(env_logger::init());
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    if let (Ok(host), Ok(user), Ok(pass)) =
        (
            env::var("JIRA_HOST"),
            env::var("JIRA_USER"),
            env::var("JIRA_PASS"),
        )
    {

        let jira = Jira::new(host, Credentials::Basic(user, pass)).unwrap();

        //Fetch new updates via long poll method
        let mut stream = api.stream();
        while let Some(update) = stream.next().await {
             //If the received update contains a new message...
            let update = update?;
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    println!("<{}>: {}", &message.from.first_name, data);
                    let query = env::var("JIRA_QUERY").unwrap_or("assignee=doug".to_owned());
                    match jira.search().iter(query, &Default::default()) {
                        Ok(results) => {
                            for issue in results {
                                let base_url = "https://scrapinghub.atlassian.net/browse/";
                                api.send(message.text_reply(format!(
                                    "[{}] [{}]({}{}) {} ({})",
                                    issue.issue_type().map(|value| value.name).unwrap_or("???".to_owned()),
                                    issue.key,
                                    base_url,
                                    issue.key,
                                    issue.summary().unwrap_or("???".to_owned()),
                                    issue
                                        .status()
                                        .map(|value| value.name,)
                                        .unwrap_or("???".to_owned(),),
                                    // always ??? - why?
                                    //issue.priority().map(|value| value.name).unwrap_or("???".to_owned()),
                                )).parse_mode(ParseMode::Markdown).disable_preview()).await?;
                                //debugging
                                //println!("{:#?}", issue);
                            }
                        }
                        Err(err) => panic!("{:#?}", err),
                    }
                }
            }
        }
    }
    Ok(())
}

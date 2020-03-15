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
                    let query = env::args().nth(1).unwrap_or("project = BV AND summary ~ amazon AND status = New AND assignee in (EMPTY) ORDER BY priority ASC".to_owned());
                    match jira.search().iter(query, &Default::default()) {
                        Ok(results) => {
                            for issue in results {
                                api.send(message.text_reply(format!(
                                    "{} {} ({}): priority {}",
                                    issue.key,
                                    issue.summary().unwrap_or("???".to_owned()),
                                    issue
                                        .status()
                                        .map(|value| value.name,)
                                        .unwrap_or("???".to_owned(),),
                                    issue
                                        .priority()
                                        .map(|value| value.id,)
                                        .unwrap_or("???".to_owned(),),
                                ))).await?;
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
                //api.send(message.text_reply(format!(
                    //"Hi, {}! You just wrote '{}'",
                    //&message.from.first_name, data
                //)))
                //.await?;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
    sync::mpsc::{channel, Sender}, time::Instant, fmt::Display, process::exit,
};

mod types;

use hacker_news::client::json_client::JsonClient;
use reqwest::Client;
use types::{Item, Story};

async fn get_story(id: u32, c: reqwest::Client, send: Sender<Story>) {
    let url = format!(
        "https://hacker-news.firebaseio.com/v0/item/{id}.json",
        id = id
    );
    let r = c.get(url).send().await.unwrap().bytes().await.unwrap();
    let item: Item = serde_json::from_slice(r.as_ref()).unwrap();
    match item {
        Item::Story(item) => send.send(item).unwrap(),
        _ => {},
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Pair(u32, u32);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

fn measure(t: &mut Instant, m: impl Display){
    println!("{m}: {:#?}", t.elapsed());
    *t = Instant::now();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let mut t = Instant::now();
    measure(&mut t, "t0");

    let mut read = HashSet::<u32>::new();
    // Stories with the same score will not be shown but that's OK
    let mut sorted_stories = BinaryHeap::<Pair>::new();
    let mut stories = HashMap::<u32, Story>::new();

    let json_client = JsonClient::new();
    measure(&mut t, "begin");
    let last_stories = json_client.top_stories()?;
    measure(&mut t, "get top stories");

    let c = Client::new();

    let (tx, rx) = channel::<Story>();

    measure(&mut t, "setup");
    for s in last_stories {
        if read.contains(&s) {
            continue;
        }
        let (c, tx) = (c.clone(), tx.clone());
        tokio::spawn(async move { get_story(s, c, tx).await });
    }
    drop(tx);
    measure(&mut t, "sent all futures");

    while let Ok(story) = rx.recv()  {
        sorted_stories.push(Pair(story.score.unwrap_or_default(), story.id));
        stories.insert(story.id, story);

    }
    drop(rx);
    measure(&mut t, "done");
    println!("{}", stories.len());
    //exit(0);

    Ok(())
}

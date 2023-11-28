use serde::Deserialize;
use serde::Serialize;

pub type Score = u32;
pub type Id = u32;

#[derive(Debug)]
pub struct Date(pub u16, pub u8, pub u8);
#[derive(Debug, Serialize, Deserialize)]

pub struct User {
    /// The user's unique username. Case-sensitive.
    id: Option<String>,
    /// Delay in minutes between a comment's creation and its visibility to other users.
    delay: Option<u32>,
    /// Creation date of the user, in Unix Time.
    created: u32,
    /// The user's karma.
    karma: Score,
    /// The user's optional self-description. HTML.
    about: Option<String>,
    /// List of the user's stories, polls and comments.
    submitted: Option<Vec<Id>>,
}

// TODO: This is essentially a Listing, at least with respect to what it represents in the data
// model. There should be some sort of unification in the API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    pub deleted: Option<bool>,
    /// The username of the item's author.
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
    /// The URL of the story.
    pub url: Option<String>,
    /// The title of the story, poll or job.
    pub title: String,
}

// TODO: This is essentially a Listing, at least with respect to what it represents in the data
// model. There should be some sort of unification in the API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    /// The item's unique id.
    pub id: Id,
    /// True if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// True if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: Option<u32>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<Score>,
    /// The title of the story, poll or job.
    pub title: Option<String>,
    /// The URL of the story.
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    // Fields directly obtained from the response payload
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: Option<Id>,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// A list of related pollopts, in display order.
    pub parts: Option<Vec<Id>>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: Option<u32>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<Score>,
    /// The title of the story, poll or job.
    pub title: Option<String>,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollOption {
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: Option<Id>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<Score>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "job")]
    Job(Job),
    #[serde(rename = "story")]
    Story(Story),
    #[serde(rename = "comment")]
    Comment(Comment),
    #[serde(rename = "poll")]
    Poll(Poll),
    #[serde(rename = "pollopt")]
    PollOption(PollOption),
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemsAndProfiles {
    pub items: Vec<Id>,
    pub profiles: Vec<String>,
}

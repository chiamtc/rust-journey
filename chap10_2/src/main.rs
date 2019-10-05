

pub trait Summary{
    fn summarize_author(&self) -> String;

    //fn summarize(&self) -> String;

    //for default implementations
     //default implementation calls another methods in the same trait
    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author())
    }

}

pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String
}

/*
to use default implementation aka empty impl block
impl Summary for NewsArticle{}
*/

impl Summary for NewsArticle{
   /* fn summarize(&self) -> String{
        format!("{}, {} by {}", self.headline, self.author, self.location)
    }*/
    fn summarize_author(&self) -> String {
        format!("From NewsArticle - {}", self.author)
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool
}

impl Summary for Tweet{
    /*fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }*/
    fn summarize_author(&self) -> String {
        format!("From tweet @{}", self.username)
    }
}
//traits as parameter
pub fn notify(item: impl Summary){
    println!("Breaking news! {}", item.summarize());
}
//trait bound syntax
pub fn notify2<T:Summary>(item: T){
    println!("Breaking news! {}", item.summarize());
}

//specifying multiple trait bounds with + syntax

/*

option 1: pub fn notify (item: impl Summary + Display) {}
or
option 2: pub fn notify<T: Summary + Display>(item: T) {}

*/

fn main() {
    let tweet = Tweet{
        username:String::from("horse_ebooks"),
        content:String::from("some content for this tweet"),
        reply:false,
        retweet:false
    };
    println!("Summary? {} ", tweet.summarize());

    notify(tweet);

    let tweet2 = Tweet{
        username:String::from("2horse_ebooks"),
        content:String::from("2some content for this tweet"),
        reply:false,
        retweet:false
    };
    notify2(tweet2);
}

//using where clause for trait bounds

/*
Instead of this
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

we use

fn some_function<T,U> ( t: T, u:U) -> 32
    where T: Display + Clone,
           U: Clone + Debug
{...}
*/
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}



pub trait Summary{
    fn summarize_author(&self)->String;

    fn summarize(&self)->String{
        format!("(Read more from {} ...)",self.summarize_author())
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub fn notify(item:&impl Summary){
    println!("Breaking news! {}",item.summarize());
}

impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{},by {} ({})",self.headline,self.author,self.location)
    }
    fn summarize_author(&self)->String{
        String::from("")
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet{
    fn summarize_author(&self)->String{
        format!("@{}",self.username)
    }
}


fn benefits_of_logical_entity(){
        
    pub trait Summary { // Trait should be public if we want to allow others to implement it
        fn summarize(&self) -> String; // no body just declaration like interface
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle { 
        fn summarize(&self) -> String { 
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    let tw = Tweet {
             username: String::from("Elon"),
             content: String::from("to the Moon"),
             reply: false,
             retweet: false,
         };
    println!("{}",tw.summarize());
        
    let article = NewsArticle {
             headline: String::from("Elon sells Bitcoin"),
             location: String::from("Menlo Park, CA, USA"),
             author: String::from("CNN"),
             content: String::from("FBI investigates"),
         };
    
    println!("{}", article.summarize());

    // Real reason we need to use traits or interfaces
    // Change you coding paradigm, start to programm to behavior!

    pub fn notify_sugar_syntax(item: impl Summary) { // your function will accept any parameter that implements Summary trait. (so all parameters will have the common behavior)
        println!("Breaking news! {}", item.summarize());
    }

    // Same logic as above but explicit, this is refereed to the idea TRAIT BOUNDS
    // or in simple language, sometimes you want to accept parameters, which implement more than one trait(have more than one common method to call on it)
    
    pub fn notify_real_syntax<T: Summary>(item: T){ // please notice generics you are saying. My function will accept as a parameter a generic type T which implements Summary trait, because I just want to make sure that I can call the methods safely.
        
        println!("Breaking news! {}", item.summarize());
    }


    notify_real_syntax(tw);
    notify_sugar_syntax(article);

}
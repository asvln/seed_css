#![allow(dead_code)]

// pub mod media;
pub mod units;
pub mod values;

#[macro_use]
mod media;

#[cfg(test)]
mod tests {
    #[test]
    fn media_test() {
        println!("at_media!(Screen, \"color: green\")");
        // at_media!(Screen, "color: green");
        println!();
        println!("at_media!(OnlyScreen \"and\" Hover , \"color: orange\")");
        // at_media!(OnlyScreen "and" Hover(Hover) , "color: orange");
    }
}

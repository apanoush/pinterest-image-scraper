use std::fs::File;
use std::io::Write;
use std::env;
use reqwest;
use scraper::{Html, Selector};
use std::fs;

/// saves imave from url to path
fn save_image(url: &String, path: &String) -> () {
    let img_bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let mut file = File::create(path).expect("file note created");
    file.write(&img_bytes).expect("not written");

}


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        return;
    }
    let url: &String = &args[1];

    fs::create_dir_all("images").expect("cannot create directory");

    let response = reqwest::blocking::get(url).unwrap().text().unwrap();

    let page = Html::parse_document(&response);

    let selector = Selector::parse("img").unwrap();
    let selector1 = Selector::parse("srcset").unwrap();
    let mut c = 1;

    for element in page.select(&selector) {
        match element.value().attr("src") {
            Some(e) => {//println!("{}", e)
                let path = "images/image".to_owned() + &c.to_string() + ".jpg";
                save_image(&e.to_owned(), &path);
                c += 1;
            },
            None => println!("error")
        }
        //println!("{}", element.html());
    }


    //let document = scraper::Html::parse_document(&response);

    //let mut file = File::create("data.html").expect("file note created");
    //file.write(response.as_bytes()).expect("not written");
}

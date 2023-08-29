use std::fs::File;
use std::io::Write;
use std::env;
use reqwest;
use scraper::{Html, Selector};
use std::fs;
use headless_chrome::{Browser};
use headless_chrome::protocol::cdp::Page;

/// gets the pinterest html with a headless browser
fn get_pinterest_html(url: &String) -> String 
{
    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to(url).unwrap();
    tab.wait_until_navigated().unwrap();
    //tab.wait_for_element("srcset").unwrap_or(println!("wait pas marché"));
    //let content = tab.get_content().unwrap();
    let result = tab.evaluate("document.documentElement.outerHTML", false).unwrap();
    let html: String = result.value.unwrap().to_string();
    println!("{}", html);

    return html;
}


/// saves imave from url to path
fn save_image(url: &String, path: &String) -> () 
{
    let img_bytes = reqwest::blocking::get(url).expect("cannot load pin image").bytes().expect("cannot convert pin image to bytes");
    let mut file: File = File::create(path).expect("file note created");
    file.write(&img_bytes).expect("not written");
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        return;
    }
    let url: &String = &args[1];

    let path_directory: &str = if args.len() == 3 { &args[2] } else {"images"};


    // creates the image directory
    fs::create_dir_all(path_directory).expect("cannot create directory");

    let content: String = get_pinterest_html(url);
    let mut file: File = File::create("data.html").expect("file note created");
    file.write(content.as_bytes()).expect("not written");

    // loads (gets) pinterest url
    //let response: String = reqwest::blocking::get(url).expect("cannot load pinterest url").text().unwrap();
    // parses the result
    let page: Html = Html::parse_document(&content);

    let selector: Selector = Selector::parse("img").unwrap();
    //let selector1 = Selector::parse("srcset").unwrap();
    let mut c = 1;

    for element in page.select(&selector) {
        match element.value().attr("src") {
            Some(e) => {//println!("{}", e)
                // creates the image name
                let path: String = path_directory.to_owned() + "/image" + &c.to_string() + ".jpg";
                save_image(&e.to_owned(), &path);
                c += 1;
                print!("\r{} pins'image scrapped", c);
                std::io::stdout().flush().expect("flush error");
            },
            None => println!("error")
        }
        //println!("{}", element.html());
    }
    println!("");


    //let document = scraper::Html::parse_document(&response);

    //let mut file = File::create("data.html").expect("file note created");
    //file.write(response.as_bytes()).expect("not written");
}

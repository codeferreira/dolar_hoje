use reqwest;
use scraper::{Html, Selector};

fn main() {
    let response = reqwest::blocking::get("https://dolarhoje.com")
        .unwrap()
        .text()
        .unwrap();

    let document = Html::parse_document(&response);

    let dolar_value_selector = Selector::parse("#estrangeiro").unwrap();
    let real_value_selector = Selector::parse("#nacional").unwrap();

    let dolar = document
        .select(&dolar_value_selector)
        .next()
        .unwrap()
        .value()
        .attr("value")
        .unwrap();

    let real = document
        .select(&real_value_selector)
        .next()
        .unwrap()
        .value()
        .attr("value")
        .unwrap();

    println!("$ {} = R$ {}", dolar, real);
}

use select::document::Document;
use select::predicate::*;
use reqwest::blocking;
use serde::Serialize;
use percent_encoding::percent_decode_str;

use std::fmt;
use std::error;
use std::collections::HashMap;

const VALID_CHARS: [char;26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const INVALID_WORDS: [&str;10] = ["mw", "cs", "org", "svg","em","output","parser","wikipedia","wikimedia","url"];

#[derive(Debug)]
pub enum CustomError {
    UrlError(String, u16),
    PageNameError(String),
    NoBodyError,
}
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::UrlError(url, code)    => write!(f, "UrlError: {} returned status code {}",url,code),
            CustomError::PageNameError(url)     => write!(f, "PageNameError: failed to get page name for {}", url),
            CustomError::NoBodyError            => write!(f, "NoBodyError: html response did not contain body"),
        }
    }
}
impl error::Error for CustomError {}



pub struct Page {
    pub name: String,
    pub data: Document
}

///get html of url
pub fn get_html(url: &str) -> Result <Page, Box<dyn error::Error>>{
    let resp = blocking::get(url)?;
    if !resp.status().is_success() {
        return Err(Box::new(CustomError::UrlError(
            url.to_string(),
            resp.status().as_u16()))
        )
    };
    //decode url and get page name
    let page_name = percent_decode_str(resp.url().as_str()).decode_utf8_lossy().split("/").last()
        .ok_or(CustomError::PageNameError(resp.url().to_string()))?
        .to_string().replace("_", " ");
    Ok(Page{data: Document::from_read(resp)?, name: page_name})
}



pub struct PageData {
    name: String,
    data: Vec<String>,
}

//seems to return more instances of words that a ctrl+f on page
///get list of words in url
pub fn get_page_data(url: String) -> Result<PageData, Box<dyn error::Error>> {
    let page = get_html(&url)?;
    let mut text = Vec::new();
    //for each node in body
    for node in page.data.find(Name("body")).next()
        .ok_or(CustomError::NoBodyError)?.descendants() {
        //if node contains text
        if let Some(words) = node.as_text() {
            //replace invalid chars with space
            let words = words.to_lowercase().chars().map(|c| {
                if VALID_CHARS.contains(&c) {c}
                else {' '}
            }).collect::<String>();
            //process words then append to text vec
            text.append(&mut words.split(" ")
                .map(|word| word.trim().to_string())
                .filter(|word| word.len()!=0 && !INVALID_WORDS.iter().any(|w| word==w))
                .collect::<Vec<String>>()
            );
        }
    }
    Ok(PageData{name: page.name, data: text})
}




#[derive(Serialize)]
pub struct ApiResp {
    name: Option<String>,
    data: Option<Vec<WordData>>,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct WordData {
    pub word: String,
    pub count: u32,
}

//gets hashmap of how many times a word appears on a page
pub fn get_analytics(page: String) -> ApiResp {
    let mut page_analytics = ApiResp{name: None, data: None, error: None};
    match get_page_data(format!("https://en.wikipedia.org/wiki/{}", page)) {
        Ok(page_data) => {
            let mut count_list = HashMap::new();
            //count how many times each word appears
            for word in page_data.data {*count_list.entry(word).or_insert(0)+=1}
            //translate to Vec<WordData> and sort
            let mut analytics = count_list.into_iter().map(|(w,c)|WordData{word: w, count: c}).collect::<Vec<WordData>>();
            analytics.sort_by(|a,b|{a.count.cmp(&b.count)});
            analytics.reverse(); //remove this shit
            //update values in ApiResp
            page_analytics.name = Some(page_data.name);
            page_analytics.data = Some(analytics);
        }
        //if error then update error in ApiResp
        Err(e) => page_analytics.error = Some(format!("{}",e)),
    }
    page_analytics
}
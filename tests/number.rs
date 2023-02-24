use std::time::Duration;
use cucumber::{given, then, when, World as _};
use tokio::time::sleep;
use webpage::{Webpage, WebpageOptions};
use std::process::Command;
use thirtyfour::prelude::*;

//Making the Driver global to be used in enter_number function
static mut DRIVER: Option<WebDriver> = None;

pub async fn url_navigation() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://qainterview.pythonanywhere.com/").await?;
    sleep(Duration::from_secs(5)).await;
    unsafe { DRIVER = Some(driver) };
    Ok(())
}

#[derive(cucumber::World, Debug, Default)]
struct World {
    pub number: u32,
    pub answer: u32,
}

#[given(regex = r"^I enter (\S+)$ in the input box")]
async fn enter_number(w: &mut World, number: u32) {
    let driver = unsafe { DRIVER.as_ref().unwrap() };
    sleep(Duration::from_secs(2)).await;
    let elem_text = driver.find(By::Name("number")).await.unwrap();
    //Tried this line to check the html as the elem_text is priniting [Summary]
    let inner_html = elem_text.inner_html().await.unwrap();
    //Prints the same -> [Summary]
    println!("{:?}", inner_html);
    elem_text.send_keys(number.to_string()).await.unwrap();
}

#[tokio::main]
async fn main() {
    url_navigation().await.unwrap();
    World::run("tests/features/book/number.feature").await;
}


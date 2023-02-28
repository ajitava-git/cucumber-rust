use std::time::Duration;
use cucumber::{given, then, when, World as _};
use tokio::time::sleep;
use webpage::{Webpage, WebpageOptions};
use std::process::Command;
use thirtyfour::prelude::*;

static mut DRIVER: Option<WebDriver> = None;

pub async fn url_navigation() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://qainterview.pythonanywhere.com/").await?;
    driver.maximize_window().await?;
    sleep(Duration::from_secs(5)).await;
    unsafe { DRIVER = Some(driver) };
    Ok(())
}

#[derive(cucumber::World, Debug, Default)]
struct World {
    pub number: u32,
    pub answer: u32,
}

#[given(expr = "I enter {word} in the input box")]
async fn enter_number(w: &mut World, number_str: String) -> WebDriverResult<()> {
    let number = number_str.parse::<u32>().unwrap();
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://qainterview.pythonanywhere.com/").await?;
    println!("Did unwrap works? {}", number);
    sleep(Duration::from_secs(2)).await;
    let elem_form = driver.find(By::ClassName("input-group")).await?;
    let elem_text = elem_form.find(By::Id("number")).await?;
    elem_text.send_keys(number.to_string()).await?;
    w.number=number;
    Ok(())
}

#[when("I click calculate")]
async fn click_caclculate(w: &mut World) -> WebDriverResult<()> {
    let driver = unsafe { DRIVER.as_ref().unwrap() };
    sleep(Duration::from_secs(2)).await;
    let calc_button = driver.find(By::Id("getFactorial")).await?;
    calc_button.click().await?;
    Ok(())
}

#[then(regex = r#"^I check the "(.+)""#)]
async fn check_factorial(w: &mut World, answer_str: String) -> WebDriverResult<()> {
    let driver = unsafe { DRIVER.as_ref().unwrap() };
    sleep(Duration::from_secs(2)).await;  
    let actual_answer = answer_str.parse::<u32>().unwrap();
    let factorial_res = driver.find(By::Id("resultDiv")).await?;
    let prefix = factorial_res.text().await?;
    let expected_answer: Vec<&str> = prefix.split_whitespace().collect();
    assert_eq!(expected_answer[5], actual_answer.to_string());
    w.answer = actual_answer;
    driver.refresh().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    World::run("tests/features/book/number.feature").await;
}




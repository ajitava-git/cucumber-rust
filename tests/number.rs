use std::time::Duration;
use cucumber::{given, then, when, World as _};
use tokio::time::sleep;
use thirtyfour::prelude::*;
use std::fmt;

//Define a struct with driver
#[derive(cucumber::World, Default)]
struct World {
    pub driver: Option<WebDriver>,
}

//Implementing Debug for World Struct
impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("World")
            .field("driver", &self.driver.as_ref().map(|_| "<WebDriver>"))
            .finish()
    }
}

//Initializing the driver
#[given(expr = "I enter {word} in the input box")]
async fn enter_number(world: &mut World, number_str: String) -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let number = number_str.parse::<u32>().unwrap();
    driver.goto("https://qainterview.pythonanywhere.com/").await?;
    driver.maximize_window().await?;
    sleep(Duration::from_secs(5)).await;   
    let elem_form = driver.find(By::ClassName("input-group")).await?;
    let elem_text = elem_form.find(By::Id("number")).await?;
    elem_text.send_keys(number.to_string()).await?;
    world.driver=Some(driver);
    Ok(())
}

//Using the driver to click calculate for factorial
#[when("I click calculate")]
async fn click_caclculate(world: &mut World) -> WebDriverResult<()> {
    let driver = world.driver.as_ref().unwrap();
    sleep(Duration::from_secs(2)).await;
    let calc_button = driver.find(By::Id("getFactorial")).await?;
    calc_button.click().await?;
    Ok(())
}

//Assert the factorial result with values from example
#[then(expr = "I check the {word}")]
async fn check_factorial(world: &mut World, answer_str: String) -> WebDriverResult<()> {
    let driver = world.driver.as_ref().unwrap();
    sleep(Duration::from_secs(2)).await;  
    let actual_answer = answer_str.parse::<u32>().unwrap(); 
    let factorial_res = driver.find(By::Id("resultDiv")).await?;
    let prefix = factorial_res.text().await?;
    let expected_answer: Vec<&str> = prefix.split_whitespace().collect();
    assert_eq!(expected_answer[5], actual_answer.to_string());
    Ok(())
}

#[tokio::main]
async fn main() {
    World::run("tests/features/book/number.feature").await;
}
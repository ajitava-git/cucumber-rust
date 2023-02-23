use std::time::Duration;
use cucumber::{given, then, when, World as _};
use tokio::time::sleep;
use webpage::{Webpage, WebpageOptions};
use std::process::Command;

#[derive(cucumber::World, Debug, Default)]
struct World {
    Factorial: Option<String>,
}

#[given(expr = "{word} opens the factorial website in the browser")] 
async fn open_website(w: &mut World) {
    sleep(Duration::from_secs(2)).await;    
    open::with("https://qainterview.pythonanywhere.com/", "chrome").unwrap();
}

#[when(expr = "{word} checks the page title as Factorial")]
async fn check_page_title(w: &mut World, Factorial: String) {
    sleep(Duration::from_secs(2)).await;

let info = Webpage::from_url("https://qainterview.pythonanywhere.com/", WebpageOptions::default())
    .expect("Could not read from URL");

let html = info.html;

w.Factorial = Some("Factorial".to_string());

assert_eq!(html.title.as_ref().unwrap(), w.Factorial.as_ref().unwrap());

}

#[then("user close the browser")]
async fn close_browser(w: &mut World) {
    if cfg!(target_os = "windows") {
        Command::new("taskkill")
            .args(&["/F", "/IM", "chrome.exe"])
            .output()
            .expect("failed to execute process");
    } else {
        Command::new("pkill")
            .arg("chrome")
            .output()
            .expect("failed to execute process");
    }
}


#[tokio::main]
async fn main() {
    World::run("tests/features/book").await;
}
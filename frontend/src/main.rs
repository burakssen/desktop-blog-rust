use std::borrow::Borrow;
use std::collections::HashMap;
use std::future::poll_fn;
use std::io::Read;
use adw::prelude::*;

use adw::gtk::{Box, Button, Label, ListBox, Orientation, SelectionMode, InputPurpose::Email, InputPurpose};
use adw::{ActionRow, Application, ApplicationWindow, EntryRow, HeaderBar, PasswordEntryRow};

use serde_json;
use tokio::fs::read_to_string;

fn main() {
    let application = Application::builder()
        .application_id("com.burakssen.linux_blog")
        .build();

    application.connect_activate(build_ui);

    application.run();
}


fn build_ui(app: &Application) {
    let user_name = EntryRow::builder().title("Username").build();
    let password = PasswordEntryRow::builder().title("Password").build();
    let mail = EntryRow::builder().title("Email").input_purpose(Email).build();
    // ActionRows are only available in Adwaita
    let row = ActionRow::builder()
        .title("Submit")
        .activatable(true)
        .build();

    let list = ListBox::builder()
        .margin_top(32)
        .margin_end(100)
        .margin_bottom(32)
        .margin_start(100)
        .selection_mode(SelectionMode::None)
        .css_classes(vec![String::from("boxed-list")])
        .build();

    list.append(&user_name);
    list.append(&password);
    list.append(&mail);

    let list2 = ListBox::builder()
        .margin_end(100)
        .margin_bottom(32)
        .margin_start(100)
        .css_classes(vec![String::from("boxed-list")])
        .build();

    list2.append(&row);

    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    content.append(&list);
    content.append(&list2);
    row.connect_activated(move |_| submit(&user_name, &password, &mail));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Linux Blog")
        .default_width(512)
        .default_height(360)
        // add content to window
        .content(&content)
        .build();
    window.show();
}

#[tokio::main]
async fn submit(user_name: &EntryRow, password: &PasswordEntryRow, mail: &EntryRow) {
    create_user(user_name.text().to_string(), password.text().to_string(), mail.text().to_string()).await.expect("User creation error");
}

async fn get_all_users() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let body = client.get("http://127.0.0.1:8080/get_all_users").send()
        .await?
        .text()
        .await?;

    Ok(body)
}

async fn create_user(user_name: String, password: String, mail: String) -> Result<String, reqwest::Error>{
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("user_name", user_name);
    map.insert("password", password);
    map.insert("mail", mail);
    let res = client.post("http://127.0.0.1:8080/create_user")
        .json(&map)
        .send()
        .await?;

    Ok(res.status().to_string())
}
use axum::{
    extract::{Form, Path},
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use crate::storage::{load_data, save_data};

#[derive(Deserialize)]
pub struct AddEntry {
    name: String,
    value: String,
}

pub async fn start_web_server() {
    let app = Router::new()
        .route("/", get(index))
        .route("/add", post(add_entry))
        .route("/list", get(list_entries))
        .route("/delete/:name", get(delete_entry))
        .route("/edit/:name", get(edit_entry))
        .route("/update", post(update_entry));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🌐 http://{}", addr);
    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html(r#"
        <h1>🔐 HashPass Web Interface</h1>
        <form action="/add" method="post">
            İsim: <input type="text" name="name" />
            Parola: <input type="text" name="value" />
            <button type="submit">Ekle</button>
        </form>
        <a href="/list">📋 Listele</a>
    "#)
}

async fn add_entry(Form(entry): Form<AddEntry>) -> Html<String> {
    let mut data = load_data();
    data.insert(entry.name.clone(), entry.value.clone());
    save_data(&data);
    Html(format!("✅ '{}' Add. <a href=\"/\">Back</a>", entry.name))
}

async fn list_entries() -> Html<String> {
    let data = load_data();
    let mut html = String::from("<h1>📋 Listed Passwords</h1><ul>");
    for (key, _) in data.iter() {
        html.push_str(&format!(
            "<li>{} – <a href=\"/edit/{}\">✏️ Update</a> | <a href=\"/delete/{}\">❌ Sil</a></li>",
            key, key, key
        ));
    }
    html.push_str("</ul><a href=\"/\">↩️ Back</a>");
    Html(html)
}

async fn delete_entry(Path(name): Path<String>) -> Html<String> {
    let mut data = load_data();
    if data.remove(&name).is_some() {
        save_data(&data);
        Html(format!("🗑️ '{}' deleted. <a href=\"/list\">List</a>", name))
    } else {
        Html(format!("❌ '{}' not found. <a href=\"/list\">List</a>", name))
    }
}

async fn edit_entry(Path(name): Path<String>) -> Html<String> {
    let data = load_data();
    let value = data.get(&name).unwrap_or(&"".to_string()).clone();
    Html(format!(
        r#"
        <h1> updated for '{}'  </h1>
        <form action="/update" method="post">
            <input type="hidden" name="name" value="{}" />
            <input type="text" name="value" value="{}" />
            <button type="submit">Updated</button>
        </form>
        "#,
        name, name, value
    ))
}

async fn update_entry(Form(form): Form<AddEntry>) -> Html<String> {
    let mut data = load_data();
    if data.contains_key(&form.name) {
        data.insert(form.name.clone(), form.value.clone());
        save_data(&data);
        Html(format!("✅ '{}' updated. <a href=\"/list\">Liste</a>", form.name))
    } else {
        Html(format!("❌ '{}' not found. <a href=\"/list\">Liste</a>", form.name))
    }
}

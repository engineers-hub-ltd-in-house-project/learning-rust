async fn handle_index() -> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("message", "これはサンプルです。");

    let output = tera.render("index.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_post(axum::Form(myform): axum::Form<Myform>) -> axum::response::Html<String> {
    let msg = format!("I am {}<{}>.", myform.name, myform.mail);
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("message", &msg);

    let output = tera.render("index.html", &context);
    axum::response::Html(output.unwrap())
}

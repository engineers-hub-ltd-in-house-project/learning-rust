/*
以下は templates/index.html として保存するHTMLテンプレートの内容です：

<html>
  <head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>{{title}}</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.css"
    rel="stylesheet" crossorigin="anonymous">
  </head>
  <body class="container">
    <h1 class="display-6 my-2">{{title}}</h1>
    <div class="alert alert-primary">
      <p class="my-2">{{message}}</p>
    </div>
    <form method="post" action="/post">
      <div class="mb-3">
        <label for="name" class="form-label">
          Your name:</label>
        <input type="text" class="form-control"
          name="name" id="name">
      </div>
      <div class="mb-3">
        <label for="mail" class="form-label">
          Email address</label>
        <input type="text" class="form-control"
          name="mail" id="mail">
      </div>
      <input type="submit" class="btn btn-primary"
        value="Submit">
    </form>
  </body>
</html>
*/

fn main() {
    println!("これはTeraテンプレートのサンプルです");
    println!("templates/index.htmlファイルとして保存して使用します");
}

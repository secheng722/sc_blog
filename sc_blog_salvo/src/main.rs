use salvo::catcher::Catcher;
use salvo::prelude::*;
use sc_blog_salvo::fs_helper::{self, render_md_to_catalog, render_md_to_html};
use tera::{Context, Tera};

#[handler]
async fn home(res: &mut Response) {
    res.render(Text::Html(fs_helper::read_to_string(
        "src/static/html/index.html",
    )));
}

#[handler]
async fn hello(res: &mut Response) {
    res.render(Text::Html(fs_helper::read_to_string(
        "src/static/html/hello.html",
    )));
}

#[handler]
async fn handle_404(res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.render(Text::Html(fs_helper::read_to_string(
            "src/static/html/404.html",
        )));
        ctrl.skip_rest();
    }
}

#[handler]
async fn handle_css(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    tracing::info!("css_name: {}", name);
    res.render(Text::Css(fs_helper::read_to_string(&format!(
        "src/static/css/{}.css",
        name
    ))));
}

#[handler]
async fn handle_js(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    tracing::info!("js_name: {}", name);
    res.render(Text::Js(fs_helper::read_to_string(&format!(
        "src/static/js/{}.js",
        name
    ))));
}

#[handler]
async fn blog_list(res: &mut Response) {
    //使用tera模版引擎
    let tera = match Tera::new("src/static/template/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            return;
        }
    };
    let md_info = render_md_to_catalog(Some(0), Some(5));
    let mut context = Context::new();
    context.insert("articles", &md_info);
    let rendered = tera
        .render("index.html.tera", &context)
        .expect("Failed to render template");
    res.render(Text::Html(rendered));
}

#[handler]
async fn blog_article(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    res.render(Text::Html(render_md_to_html(&name)));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = Router::new()
        .get(blog_list)
        .push(Router::with_path("/hello").get(hello))
        .push(Router::with_path("/list").get(blog_list))
        .push(Router::with_path("/article/<name>").get(blog_article))
        .push(Router::with_path("/static/css/<name>.css").get(handle_css))
        .push(Router::with_path("/static/js/<name>.js").get(handle_js));
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    let service = Service::new(router).catcher(Catcher::default().hoop(handle_404));
    Server::new(acceptor).serve(service).await;
}

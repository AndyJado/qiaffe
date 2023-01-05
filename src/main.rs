#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    femme::start();
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(|_| async { Ok("你掐不掐咖啡啊?🤏") });
    app.at("/api").nest({
        let mut api = tide::new();
        api.at("/buy").get(|_| async { Ok("来找我买是吧?") });
        api.at("/goodbye").get(|_| async { Ok("Goodbye, world") });
        api
    });
    app.listen("0.0.0.0:80").await?;
    Ok(())
}

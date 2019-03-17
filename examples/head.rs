use lynx::HttpResult;

fn main() -> HttpResult {
    env_logger::init();

    let (status, headers, _) = lynx::head("http://httpbin.org").send()?;
    println!("{:?} {:#?}", status, headers);

    Ok(())
}

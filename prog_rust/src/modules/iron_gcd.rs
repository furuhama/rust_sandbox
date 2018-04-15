use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

pub fn start_gcd_server() {
    println!("Serving on http://localhost:3000 !!!");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let content_type = "text/html".parse::<Mime>().expect("Failed to parse content type");

    response.set_mut(status::Ok);
    response.set_mut(content_type);
    response.set_mut(
        r#"
        <title>GCD calculate emulator</title>
        <h1>GCD calculate emulator</h1>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#,
    );

    Ok(response)
}

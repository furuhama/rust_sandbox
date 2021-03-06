use std;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

pub fn start_gcd_server() {
    println!("Serving on http://localhost:3000 !!!");

    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let content_type = "text/html"
        .parse::<Mime>()
        .expect("Failed to parse content type");

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

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut("form data has no 'n' parameter\n".to_string());
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter is not a number: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m)
    }

    let content_type = "text/html"
        .parse::<Mime>()
        .expect("Failed to parse content type");
    response.set_mut(status::Ok);
    response.set_mut(content_type);
    response.set_mut(format!(
        "The Greatest common divisor of the numbers {:?} is <b>{}</b>\n",
        numbers, d
    ));
    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

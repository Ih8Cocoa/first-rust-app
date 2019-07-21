extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

/// A simple web server that calculates greatest common divisor between 2 numbers
/// Made with mematic
fn main() {
    let link = "localhost:3000";
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    println!("Hello, world!");
    println!("Ready on http://{}", &link);
    Iron::new(router).http(&link).unwrap();
}

/// Get_form function to get the input form. This function requires a borrow right of an
/// Iron Request object, and produces a response
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.headers.set(html_content_type());
    response.set_mut(r#"
    <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
    </form>
    "#);
    Ok(response)
}

/// Handling POST request -> Calculate and return greatest common divisor
fn post_gcd(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match _request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form: {:?}\n", e));
            return Ok(response)
        }
        Ok(map) => map
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Form has no \"n\" parameter\n"));
            return Ok(response)
        }
        Some(nums) => nums
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        // unparsed is still needed -> only lend reading rights
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                // unparsed is not needed anymore, take it
                response.set_mut(format!("This is not a number: {:?}\n", unparsed));
                return Ok(response);
            }
            Ok(n) => numbers.push(n)
        }
    }

    let mut d = numbers[0];
    // lend reading rights of the whole "numbers" vector to the for loop,
    // which the loop will then only read from the 2nd element forward
    // It's either that or go full "unsafe {}"
    for m in &numbers[1..] {
        // Remember to deref a reference if the fn requires the value
        d = gcd(d, *m);
    }

    // Now start preparing the response
    response.set_mut(status::Ok);
    response.headers.set(html_content_type());
    response.set_mut(
        format!("The greatest common divisor of numbers {:?} is <strong>{}</strong>\n", numbers, d)
    );
    Ok(response)
}

/// Calculate GCD
fn gcd(mut n: u64, mut m: u64) -> u64{
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t= m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// generate the HTML content type
fn html_content_type() -> iron::headers::ContentType {
    let content_type: iron::mime::Mime = "text/html; charset=utf-8".parse().unwrap();

    // content_type is unneeded after this line -> transfer ownership
    return iron::headers::ContentType(content_type);
}
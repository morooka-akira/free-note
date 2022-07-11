use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    println!("Serving on http://localhost:3000...");

    server.bind("127.0.0.1:3000")
    .expect("error binding server to address")
    .run()
    .expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n" />
                <input type="text" name="m" />
                <button type="submit"/>Compute GCD</button>
            </form>
            "#
        )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        HttpResponse::BadRequest()
        .content_type("text/html")
        .body("Computing the GCD with zero is boring.")
    } else {
        let response = format!("The greatest common divisor of the numbers {} \n and {} is {}", form.n, form.m, gcd(form.n, form.m));
        HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
    }
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

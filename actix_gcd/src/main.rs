use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| 
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    );

    println!("Starting server at http://localhost:8080/");

    server.bind("127.0.0.1:8080").expect("Can not bind to port 8080").run().expect("Can not start server");

}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <html>
            <head>
                <title>GCD Calculator</title>
            </head>
            <body>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute GCD</button>
                </form>
            </body>
        </html>"#,
    )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        HttpResponse::BadRequest().content_type("text/html").body("n and m must be non-zero");
    } 

    let response = format!("The greatest common divisor of {} and {} is {}", form.n, form.m, greatest_common_divisor(form.n, form.m));

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    
    n
}

#[test]
fn test_greatest_common_divisor() {
    assert_eq!(greatest_common_divisor(14, 15), 1);
    assert_eq!(greatest_common_divisor(2 * 3 * 5 * 11 * 17,
                                        3 * 7 * 11 * 13 * 19),
               3 * 11);
}
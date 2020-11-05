mod panic;

use reqwest::blocking::Client;
use reqwest::StatusCode;

#[cfg(not(debug_assertions))]
use self::panic::panic::set_panic_hook;


fn main() {
    #[cfg(not(debug_assertions))] set_panic_hook();

    let client = Client::new();
    let res = client.get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().unwrap();
    println!("{}", body.len());
    println!("{}", body);

    let res = client.get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().unwrap();
    println!("{}", body.len());
    println!("{}", body);
}

use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn form_check() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(!response.body().is_none());
    // assert!(&resp.starts_with("Paste YT link here"));
}

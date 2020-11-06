#[cfg(test)]
mod test {
    use reqwest::blocking::Client;
    use reqwest::StatusCode;

    // use super::
    #[test]
    fn reqwest_http_user_agent() {
        let client = Client::new();
        let res = client.get("http://httpbin.org/user-agent").send().unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = res.text().unwrap();
        assert_eq!(body, "{\n  \"user-agent\": null\n}\n");
    }

    #[test]
    fn reqwest_https_user_agent() {
        let client = Client::new();
        let res = client.get("https://httpbin.org/user-agent").send().unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = res.text().unwrap();
        assert_eq!(body, "{\n  \"user-agent\": null\n}\n");
    }
}

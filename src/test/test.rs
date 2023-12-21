#[path= "../main.rs"] mod main;

mod tests {
    use super::*;
    use ntex::util::Bytes;
    use ntex::web::{test, App, Error};
    use ntex::Service;
    use ntex::{http, web};

    #[ntex::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let bytes = test::read_body(resp).await;

        assert_eq!(bytes, Bytes::from(r##"Hello world!"##));

        Ok(())
    }
}
#[cfg(all(feature = "new-e2e-tests"))]
pub mod all {
    use anyhow::Result;
    use e2e_testing::asserts::assert_http_request;
    use e2e_testing::cloud_controller;
    use e2e_testing::controller::Controller;
    use e2e_testing::metadata_extractor::AppMetadata;
    use e2e_testing::testcase::{SkipCondition, TestCase};

    fn get_url(base: &str, path: &str) -> String {
        format!("{}{}", base, path)
    }

    pub async fn http_go_works() {
        fn checks(metadata: &AppMetadata) -> Result<()> {
            return assert_http_request(metadata.base.as_str(), 200, &[], Some("Hello Fermyon!\n"));
        }

        let tc = TestCase {
            name: "http-go template".to_string(),
            appname: "http-go-test".to_string(),
            template: Some("http-go".to_string()),
            template_install_args: None,
            assertions: checks,
            plugins: None,
            deploy_args: None,
            skip_conditions: None,
            pre_build_hooks: None,
        };

        tc.run().await.unwrap();
    }
}

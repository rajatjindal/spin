#[cfg(feature = "new-e2e-tests")]
pub mod all {
    use anyhow::Result;
    use e2e_testing::asserts::assert_http_response;
    use e2e_testing::controller::Controller;
    use e2e_testing::metadata_extractor::AppMetadata;
    use e2e_testing::testcase::TestCase;
    use e2e_testing::testcase::TestCaseBuilder;
    use std::time::Duration;
    use tokio::io::BufReader;
    use tokio::process::ChildStdout;

    fn get_url(base: &str, path: &str) -> String {
        format!("{}{}", base, path)
    }

    pub async fn http_go_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(metadata.base.as_str(), 200, &[], Some("Hello Fermyon!\n")).await
        }

        let tc = TestCaseBuilder::default()
            .name("http-go-template".to_string())
            .template(Some("http-go".to_string()))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap();
    }

    pub async fn http_c_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(
                metadata.base.as_str(),
                200,
                &[],
                Some("Hello from WAGI/1\n"),
            )
            .await
        }

        let tc = TestCaseBuilder::default()
            .name("http-c-template".to_string())
            .template(Some("http-c".to_string()))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn http_rust_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(metadata.base.as_str(), 200, &[], Some("Hello, Fermyon")).await
        }

        let tc = TestCaseBuilder::default()
            .name("http-rust-template".to_string())
            .template(Some("http-rust".to_string()))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn http_zig_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(metadata.base.as_str(), 200, &[], Some("Hello World!\n")).await
        }

        let tc = TestCaseBuilder::default()
            .name("http-zig-template".to_string())
            .template(Some("http-zig".to_string()))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    #[allow(unused)]
    pub async fn http_grain_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(metadata.base.as_str(), 200, &[], Some("Hello, World\n")).await
        }

        let tc = TestCaseBuilder::default()
            .name("http-grain-template".to_string())
            .template(Some("http-grain".to_string()))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn http_ts_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(metadata.base.as_str(), 200, &[], Some("Hello from TS-SDK")).await
        }

        let tc = TestCaseBuilder::default()
            .name("http-ts-template".to_string())
            .template(Some("http-ts".to_string()))
            .template_install_args(Some(vec![
                "--git".to_string(),
                "https://github.com/fermyon/spin-js-sdk".to_string(),
                "--update".to_string(),
            ]))
            .plugins(Some(vec!["js2wasm".to_string()]))
            .pre_build_hooks(Some(vec![vec!["npm".to_string(), "install".to_string()]]))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn http_js_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(metadata.base.as_str(), 200, &[], Some("Hello from JS-SDK")).await
        }

        let tc = TestCaseBuilder::default()
            .name("http-js-template".to_string())
            .template(Some("http-js".to_string()))
            .template_install_args(Some(vec![
                "--git".to_string(),
                "https://github.com/fermyon/spin-js-sdk".to_string(),
                "--update".to_string(),
            ]))
            .plugins(Some(vec!["js2wasm".to_string()]))
            .pre_build_hooks(Some(vec![vec!["npm".to_string(), "install".to_string()]]))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn assets_routing_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(
                get_url(metadata.base.as_str(), "/static/thisshouldbemounted/1").as_str(),
                200,
                &[],
                Some("1\n"),
            )
            .await?;

            assert_http_response(
                get_url(metadata.base.as_str(), "/static/thisshouldbemounted/2").as_str(),
                200,
                &[],
                Some("2\n"),
            )
            .await?;

            assert_http_response(
                get_url(metadata.base.as_str(), "/static/thisshouldbemounted/3").as_str(),
                200,
                &[],
                Some("3\n"),
            )
            .await?;

            assert_http_response(
                get_url(metadata.base.as_str(), "/static/donotmount/a").as_str(),
                404,
                &[],
                Some("Not Found"),
            )
            .await?;

            assert_http_response(
                get_url(
                    metadata.base.as_str(),
                    "/static/thisshouldbemounted/thisshouldbeexcluded/4",
                )
                .as_str(),
                404,
                &[],
                Some("Not Found"),
            )
            .await?;

            Ok(())
        }

        let tc = TestCaseBuilder::default()
            .name("assets-test".to_string())
            .appname(Some("assets-test".to_string()))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn simple_spin_rust_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(
                get_url(metadata.base.as_str(), "/test/hello").as_str(),
                200,
                &[],
                Some("I'm a teapot"),
            )
            .await?;

            assert_http_response(
                get_url(
                    metadata.base.as_str(),
                    "/test/hello/wildcards/should/be/handled",
                )
                .as_str(),
                200,
                &[],
                Some("I'm a teapot"),
            )
            .await?;

            assert_http_response(
                get_url(metadata.base.as_str(), "/thisshouldfail").as_str(),
                404,
                &[],
                None,
            )
            .await?;

            assert_http_response(
                get_url(metadata.base.as_str(), "/test/hello/test-placement").as_str(),
                200,
                &[],
                Some("text for test"),
            )
            .await?;

            Ok(())
        }

        let tc = TestCaseBuilder::default()
            .name("simple-spin-rust-test".to_string())
            .appname(Some("simple-spin-rust-test".to_string()))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn header_env_routes_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(
                get_url(metadata.base.as_str(), "/env").as_str(),
                200,
                &[],
                Some("I'm a teapot"),
            )
            .await?;

            assert_http_response(
                get_url(metadata.base.as_str(), "/env/foo").as_str(),
                200,
                &[("env_some_key", "some_value")],
                Some("I'm a teapot"),
            )
            .await?;

            Ok(())
        }

        let tc = TestCaseBuilder::default()
            .name("headers-env-routes-test".to_string())
            .appname(Some("headers-env-routes-test".to_string()))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn header_dynamic_env_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(
                get_url(metadata.base.as_str(), "/env").as_str(),
                200,
                &[],
                Some("I'm a teapot"),
            )
            .await?;

            assert_http_response(
                get_url(metadata.base.as_str(), "/env/foo").as_str(),
                200,
                &[("env_some_key", "some_value")],
                Some("I'm a teapot"),
            )
            .await?;

            Ok(())
        }

        let tc = TestCaseBuilder::default()
            .name("headers-dynamic-env-test".to_string())
            .appname(Some("headers-dynamic-env-test".to_string()))
            .deploy_args(Some(vec!["--env".to_string(), "foo=bar".to_string()]))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    pub async fn http_rust_outbound_mysql_works(controller: &dyn Controller) {
        async fn checks(metadata: AppMetadata, _: Option<BufReader<ChildStdout>>) -> Result<()> {
            assert_http_response(
                get_url(metadata.base.as_str(), "/test_numeric_types").as_str(),
                200,
                &[],
                None,
            )
            .await?;

            assert_http_response(
                get_url(metadata.base.as_str(), "/test_character_types").as_str(),
                200,
                &[],
                None,
            )
            .await?;

            Ok(())
        }

        let tc = TestCaseBuilder::default()
            .name("http-rust-outbound-mysql".to_string())
            .appname(Some("http-rust-outbound-mysql".to_string()))
            .deploy_args(Some(vec!["--env".to_string(), "foo=bar".to_string()]))
            .assertions(
                |metadata: AppMetadata, logs_stream: Option<BufReader<ChildStdout>>| {
                    Box::pin(checks(metadata, logs_stream))
                },
            )
            .build()
            .unwrap();

        tc.run(controller).await.unwrap()
    }

    // pub async fn redis_go_works(controller: &dyn Controller) {
    //     async fn checks<'a>(
    //         _: AppMetadata,
    //         logs_stream: Option<BufReader<ChildStdout>>,
    //     ) -> Result<()> {
    //         utils::run(
    //             vec!["redis-cli", "PUBLISH", "abc", "msg-from-channel"],
    //             None,
    //             None,
    //         )
    //         .context("publishing redis msg")?;
    //         let logs = utils::get_output_from_reader(logs_stream, Duration::from_secs(2)).await;

    //         println!("{:?}", logs);
    //         Ok(())
    //     }

    //     let tc = TestCaseBuilder::default()
    //         .name("redis-go".to_string())
    //         .appname(Some("redis-go-generated".to_string()))
    //         .assertions(checks)
    //         .build()
    //         .unwrap();

    //     tc.run(controller).await.unwrap()
    // }
}

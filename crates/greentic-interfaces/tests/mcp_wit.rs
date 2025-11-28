use std::path::PathBuf;

use wit_parser::Resolve;

#[test]
fn mcp_packages_are_staged_and_parse() {
    let staged = PathBuf::from(env!("WIT_STAGING_DIR"));
    for pkg in [
        "wasix-mcp-24.11.5",
        "wasix-mcp-25.3.26",
        "wasix-mcp-25.6.18",
    ] {
        let path = staged.join(pkg).join("package.wit");
        assert!(
            path.exists(),
            "staged WIT missing for package {pkg}: {}",
            path.display()
        );

        let mut resolve = Resolve::new();
        resolve
            .push_path(&path)
            .unwrap_or_else(|err| panic!("failed to parse {pkg} ({path:?}): {err}"));
    }
}

#[test]
fn mcp_tool_and_result_shapes_compile() {
    use greentic_interfaces::bindings::wasix_mcp_24_11_5_mcp_router::exports::wasix::mcp::router as mcp24;
    let _tool24 = mcp24::Tool {
        name: "echo".into(),
        description: "test tool".into(),
        input_schema: mcp24::Value { json: "{}".into() },
        output_schema: None,
        output: Some("text/plain".into()),
        config: Some(vec![mcp24::ConfigDescriptor {
            name: "endpoint".into(),
            description: "service endpoint".into(),
            required: true,
        }]),
        secrets: Some(vec![mcp24::SecretDescriptor {
            name: "token".into(),
            description: "auth token".into(),
            required: true,
        }]),
    };

    use greentic_interfaces::bindings::wasix_mcp_25_3_26_mcp_router::exports::wasix::mcp::router as mcp25_03;
    let _result25 = mcp25_03::CallToolResult {
        content: vec![mcp25_03::Content::Audio(mcp25_03::AudioContent {
            data: "base64audio".into(),
            mime_type: "audio/wav".into(),
            annotations: None,
        })],
        progress: Some(vec![mcp25_03::ProgressNotification {
            progress: Some(0.5),
            message: Some("halfway".into()),
            annotations: None,
        }]),
        meta: Some(vec![mcp25_03::MetaEntry {
            key: "output".into(),
            value: "text/plain".into(),
        }]),
        is_error: Some(false),
    };

    use greentic_interfaces::bindings::wasix_mcp_25_6_18_mcp_router::exports::wasix::mcp::router as mcp25_06;
    let content = vec![
        mcp25_06::ContentBlock::Audio(mcp25_06::AudioContent {
            data: "base64audio".into(),
            mime_type: "audio/wav".into(),
            annotations: None,
        }),
        mcp25_06::ContentBlock::ResourceLink(mcp25_06::ResourceLinkContent {
            uri: "https://example.com/resource".into(),
            title: Some("example".into()),
            description: Some("linked resource".into()),
            mime_type: Some("text/plain".into()),
            annotations: None,
        }),
        mcp25_06::ContentBlock::EmbeddedResource(mcp25_06::EmbeddedResource {
            uri: "memory://embedded".into(),
            title: Some("embedded".into()),
            description: Some("inline resource".into()),
            mime_type: Some("text/plain".into()),
            data: "hello".into(),
            annotations: None,
        }),
    ];
    let _result = mcp25_06::ToolResult {
        content,
        structured_content: Some("{\"ok\":true}".into()),
        progress: None,
        meta: None,
        is_error: Some(false),
    };
}

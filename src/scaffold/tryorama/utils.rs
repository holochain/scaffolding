use std::path::PathBuf;

pub fn common_tests_setup(app_bundle_path_from_tests_root: &PathBuf) -> String {
    format!(
        r#"
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/' + {:?};

    // Set up the app to be installed 
    const appSource = {{ appBundleSource: {{ path: testAppPath }} }};

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();"#,
        app_bundle_path_from_tests_root,
    )
}

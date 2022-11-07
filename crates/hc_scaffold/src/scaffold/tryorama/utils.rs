use std::path::PathBuf;

pub fn common_tests_setup(dna_bundle_path_from_tests_root: &PathBuf) -> String {
    format!(
        r#"
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testDnaPath = process.cwd() + '/' + {:?};

    // Set up the array of DNAs to be installed, which only consists of the
    // test DNA referenced by path.
    const dnas: DnaSource[] = [{{ path: testDnaPath }}];

    // Add 2 players with the test DNA to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithHapps([dnas, dnas]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();
    
    "#,
        dna_bundle_path_from_tests_root,
    )
}

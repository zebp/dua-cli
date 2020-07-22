use crate::interactive::app_test::utils::{
    debug, initialized_app_and_terminal_from_fixture, sample_01_tree, sample_02_tree,
};
use anyhow::Result;
use pretty_assertions::assert_eq;

// Won't work on windows as there are backslashes in the paths :D
#[cfg_attr(windows, ignore)]
fn it_can_handle_ending_traversal_reaching_top_but_skipping_levels() -> Result<()> {
    let (_, app) = initialized_app_and_terminal_from_fixture(&["sample-01"])?;
    let expected_tree = sample_01_tree();

    assert_eq!(
        debug(app.traversal.tree),
        debug(expected_tree),
        "filesystem graph is stable and matches the directory structure"
    );
    Ok(())
}

// Won't work on windows as there are backslashes in the paths :D
#[cfg_attr(windows, ignore)]
#[test]
fn it_can_handle_ending_traversal_without_reaching_the_top() -> Result<()> {
    let (_, app) = initialized_app_and_terminal_from_fixture(&["sample-02"])?;
    let expected_tree = sample_02_tree();

    assert_eq!(
        debug(app.traversal.tree),
        debug(expected_tree),
        "filesystem graph is stable and matches the directory structure"
    );
    Ok(())
}

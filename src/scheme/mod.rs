use std::collections::BTreeMap;

use plist::Value as PlistValue;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XCScheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_upgrade_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_action: Option<BuildAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_action: Option<TestAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_action: Option<LaunchAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_action: Option<ProfileAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_action: Option<AnalyzeAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_action: Option<ArchiveAction>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildableReference {
    pub buildable_identifier: String,
    pub blueprint_identifier: String,
    pub buildable_name: String,
    pub blueprint_name: String,
    pub referenced_container: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelize_buildables: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_implicit_dependencies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_post_actions_on_failure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_architectures: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<BuildActionEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_actions: Option<Vec<ExecutionAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_actions: Option<Vec<ExecutionAction>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildActionEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_for_running: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_for_testing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_for_profiling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_for_archiving: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_for_analyzing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildable_reference: Option<BuildableReference>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_content: Option<ActionContent>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_to_invoke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_buildable: Option<BuildableReference>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_debugger_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_launcher_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_launch_scheme_args_env: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_autocreate_test_plan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_screen_capture_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_coverage_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_generate_coverage_for_specified_targets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testables: Option<Vec<TestableReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_plans: Option<Vec<TestPlanReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macro_expansion: Option<BuildableReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<Vec<CommandLineArgument>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_actions: Option<Vec<ExecutionAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_actions: Option<Vec<ExecutionAction>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestableReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelizable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_test_selection_whitelist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildable_reference: Option<BuildableReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_tests: Option<Vec<SkippedTest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_tests: Option<Vec<SelectedTest>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkippedTest {
    pub identifier: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedTest {
    pub identifier: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanReference {
    pub reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "default")]
    pub default_plan: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_debugger_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_launcher_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_working_directory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_working_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignores_persistent_state_on_launch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_document_versioning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_service_extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_location_simulation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_launch_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "appClipInvocationURLString")]
    pub app_clip_invocation_url_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_for_app_to_launch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_automatically_substyle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildable_product_runnable: Option<BuildableProductRunnable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_runnable: Option<RemoteRunnable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macro_expansion: Option<BuildableReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_scenario_reference: Option<LocationScenarioReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_kit_configuration_file_reference: Option<StoreKitConfigurationFileReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<Vec<CommandLineArgument>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_actions: Option<Vec<ExecutionAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_actions: Option<Vec<ExecutionAction>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildableProductRunnable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnable_debugging_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildable_reference: Option<BuildableReference>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteRunnable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnable_debugging_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildable_reference: Option<BuildableReference>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationScenarioReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreKitConfigurationFileReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_launch_scheme_args_env: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_tool_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_working_directory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_document_versioning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_for_app_to_launch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_automatically_substyle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "appClipInvocationURLString")]
    pub app_clip_invocation_url_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildable_product_runnable: Option<BuildableProductRunnable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<Vec<CommandLineArgument>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_actions: Option<Vec<ExecutionAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_actions: Option<Vec<ExecutionAction>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_actions: Option<Vec<ExecutionAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_actions: Option<Vec<ExecutionAction>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_archive_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_archive_in_organizer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_actions: Option<Vec<ExecutionAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_actions: Option<Vec<ExecutionAction>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandLineArgument {
    pub argument: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

// ============================================================================
// In-memory XML element tree for DOM-like navigation
// ============================================================================

#[derive(Debug, Clone)]
struct XmlElement {
    tag: String,
    attrs: Vec<(String, String)>,
    children: Vec<XmlElement>,
}

impl XmlElement {
    fn get_attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    fn get_bool_attr(&self, name: &str) -> Option<bool> {
        match self.get_attr(name)? {
            "YES" | "Yes" => Some(true),
            "NO" | "No" => Some(false),
            _ => None,
        }
    }

    fn get_child(&self, tag_name: &str) -> Option<&XmlElement> {
        self.children.iter().find(|c| c.tag == tag_name)
    }

    fn get_children(&self, tag_name: &str) -> Vec<&XmlElement> {
        self.children.iter().filter(|c| c.tag == tag_name).collect()
    }
}

/// Parse raw XML string into a tree of `XmlElement` nodes.
fn parse_xml_tree(xml: &str) -> anyhow::Result<XmlElement> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<XmlElement> = Vec::new();
    let mut root: Option<XmlElement> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(start)) => {
                let tag = String::from_utf8(start.name().as_ref().to_vec())?;
                let mut attrs = Vec::new();
                for attr in start.attributes() {
                    let attr = attr?;
                    let key = std::str::from_utf8(attr.key.as_ref())?.to_string();
                    let value = attr
                        .decode_and_unescape_value(reader.decoder())?
                        .to_string();
                    attrs.push((key, value));
                }
                stack.push(XmlElement {
                    tag,
                    attrs,
                    children: Vec::new(),
                });
            }
            Ok(Event::End(_)) => {
                let el = stack.pop().unwrap();
                if stack.is_empty() {
                    root = Some(el);
                } else {
                    stack.last_mut().unwrap().children.push(el);
                }
            }
            Ok(Event::Empty(start)) => {
                let tag = String::from_utf8(start.name().as_ref().to_vec())?;
                let mut attrs = Vec::new();
                for attr in start.attributes() {
                    let attr = attr?;
                    let key = std::str::from_utf8(attr.key.as_ref())?.to_string();
                    let value = attr
                        .decode_and_unescape_value(reader.decoder())?
                        .to_string();
                    attrs.push((key, value));
                }
                let el = XmlElement {
                    tag,
                    attrs,
                    children: Vec::new(),
                };
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(el);
                } else {
                    root = Some(el);
                }
            }
            Ok(Event::Eof) => break,
            Ok(Event::Decl(_) | Event::Comment(_) | Event::DocType(_) | Event::Text(_)) => {}
            Ok(_) => {}
            Err(err) => return Err(err.into()),
        }
        buf.clear();
    }

    root.ok_or_else(|| anyhow::anyhow!("Invalid xcscheme file: no root element found"))
}

// ============================================================================
// Parser
// ============================================================================

pub fn parse(xml: &str) -> anyhow::Result<XCScheme> {
    let root = parse_xml_tree(xml)?;
    if root.tag != "Scheme" {
        anyhow::bail!("Invalid xcscheme file: root element must be <Scheme>");
    }
    Ok(parse_scheme(&root))
}

fn parse_scheme(el: &XmlElement) -> XCScheme {
    XCScheme {
        version: el.get_attr("version").map(String::from),
        last_upgrade_version: el.get_attr("LastUpgradeVersion").map(String::from),
        build_action: el.get_child("BuildAction").map(parse_build_action),
        test_action: el.get_child("TestAction").map(parse_test_action),
        launch_action: el.get_child("LaunchAction").map(parse_launch_action),
        profile_action: el.get_child("ProfileAction").map(parse_profile_action),
        analyze_action: el.get_child("AnalyzeAction").map(parse_analyze_action),
        archive_action: el.get_child("ArchiveAction").map(parse_archive_action),
    }
}

fn parse_build_action(el: &XmlElement) -> BuildAction {
    BuildAction {
        parallelize_buildables: el.get_bool_attr("parallelizeBuildables"),
        build_implicit_dependencies: el.get_bool_attr("buildImplicitDependencies"),
        run_post_actions_on_failure: el.get_bool_attr("runPostActionsOnFailure"),
        build_architectures: el.get_attr("buildArchitectures").map(String::from),
        entries: el.get_child("BuildActionEntries").map(|entries_el| {
            entries_el
                .get_children("BuildActionEntry")
                .into_iter()
                .map(parse_build_action_entry)
                .collect()
        }),
        pre_actions: parse_execution_actions(el, "PreActions"),
        post_actions: parse_execution_actions(el, "PostActions"),
    }
}

fn parse_build_action_entry(el: &XmlElement) -> BuildActionEntry {
    BuildActionEntry {
        build_for_running: el.get_bool_attr("buildForRunning"),
        build_for_testing: el.get_bool_attr("buildForTesting"),
        build_for_profiling: el.get_bool_attr("buildForProfiling"),
        build_for_archiving: el.get_bool_attr("buildForArchiving"),
        build_for_analyzing: el.get_bool_attr("buildForAnalyzing"),
        buildable_reference: el
            .get_child("BuildableReference")
            .map(parse_buildable_reference),
    }
}

fn parse_buildable_reference(el: &XmlElement) -> BuildableReference {
    BuildableReference {
        buildable_identifier: el
            .get_attr("BuildableIdentifier")
            .unwrap_or("primary")
            .to_string(),
        blueprint_identifier: el
            .get_attr("BlueprintIdentifier")
            .unwrap_or("")
            .to_string(),
        buildable_name: el.get_attr("BuildableName").unwrap_or("").to_string(),
        blueprint_name: el.get_attr("BlueprintName").unwrap_or("").to_string(),
        referenced_container: el
            .get_attr("ReferencedContainer")
            .unwrap_or("")
            .to_string(),
    }
}

fn parse_execution_actions(parent: &XmlElement, container_name: &str) -> Option<Vec<ExecutionAction>> {
    let container_el = parent.get_child(container_name)?;
    let actions = container_el.get_children("ExecutionAction");
    if actions.is_empty() {
        return None;
    }
    Some(actions.into_iter().map(parse_execution_action).collect())
}

fn parse_execution_action(el: &XmlElement) -> ExecutionAction {
    ExecutionAction {
        action_type: el.get_attr("ActionType").map(String::from),
        action_content: el.get_child("ActionContent").map(parse_action_content),
    }
}

fn parse_action_content(el: &XmlElement) -> ActionContent {
    ActionContent {
        title: el.get_attr("title").map(String::from),
        script_text: el.get_attr("scriptText").map(String::from),
        shell_to_invoke: el.get_attr("shellToInvoke").map(String::from),
        environment_buildable: el
            .get_child("EnvironmentBuildable")
            .and_then(|e| e.get_child("BuildableReference"))
            .map(parse_buildable_reference),
    }
}

fn parse_test_action(el: &XmlElement) -> TestAction {
    TestAction {
        build_configuration: el.get_attr("buildConfiguration").map(String::from),
        selected_debugger_identifier: el.get_attr("selectedDebuggerIdentifier").map(String::from),
        selected_launcher_identifier: el.get_attr("selectedLauncherIdentifier").map(String::from),
        should_use_launch_scheme_args_env: el.get_bool_attr("shouldUseLaunchSchemeArgsEnv"),
        should_autocreate_test_plan: el.get_bool_attr("shouldAutocreateTestPlan"),
        preferred_screen_capture_format: el.get_attr("preferredScreenCaptureFormat").map(String::from),
        code_coverage_enabled: el.get_bool_attr("codeCoverageEnabled"),
        only_generate_coverage_for_specified_targets: el.get_bool_attr("onlyGenerateCoverageForSpecifiedTargets"),
        testables: el.get_child("Testables").map(|testables_el| {
            testables_el
                .get_children("TestableReference")
                .into_iter()
                .map(parse_testable_reference)
                .collect()
        }),
        test_plans: el.get_child("TestPlans").map(|test_plans_el| {
            test_plans_el
                .get_children("TestPlanReference")
                .into_iter()
                .map(parse_test_plan_reference)
                .collect()
        }),
        macro_expansion: el
            .get_child("MacroExpansion")
            .and_then(|e| e.get_child("BuildableReference"))
            .map(parse_buildable_reference),
        command_line_arguments: parse_command_line_arguments(el),
        environment_variables: parse_environment_variables(el),
        pre_actions: parse_execution_actions(el, "PreActions"),
        post_actions: parse_execution_actions(el, "PostActions"),
    }
}

fn parse_testable_reference(el: &XmlElement) -> TestableReference {
    TestableReference {
        skipped: el.get_bool_attr("skipped"),
        parallelizable: el.get_bool_attr("parallelizable"),
        use_test_selection_whitelist: el.get_bool_attr("useTestSelectionWhitelist"),
        buildable_reference: el
            .get_child("BuildableReference")
            .map(parse_buildable_reference),
        skipped_tests: el.get_child("SkippedTests").map(|skipped_tests_el| {
            skipped_tests_el
                .get_children("Test")
                .into_iter()
                .map(|test_el| SkippedTest {
                    identifier: test_el.get_attr("Identifier").unwrap_or("").to_string(),
                })
                .collect()
        }),
        selected_tests: el.get_child("SelectedTests").map(|selected_tests_el| {
            selected_tests_el
                .get_children("Test")
                .into_iter()
                .map(|test_el| SelectedTest {
                    identifier: test_el.get_attr("Identifier").unwrap_or("").to_string(),
                })
                .collect()
        }),
    }
}

fn parse_test_plan_reference(el: &XmlElement) -> TestPlanReference {
    TestPlanReference {
        reference: el.get_attr("reference").unwrap_or("").to_string(),
        default_plan: el.get_bool_attr("default"),
    }
}

fn parse_launch_action(el: &XmlElement) -> LaunchAction {
    LaunchAction {
        build_configuration: el.get_attr("buildConfiguration").map(String::from),
        selected_debugger_identifier: el.get_attr("selectedDebuggerIdentifier").map(String::from),
        selected_launcher_identifier: el.get_attr("selectedLauncherIdentifier").map(String::from),
        launch_style: el.get_attr("launchStyle").map(String::from),
        use_custom_working_directory: el.get_bool_attr("useCustomWorkingDirectory"),
        custom_working_directory: el.get_attr("customWorkingDirectory").map(String::from),
        ignores_persistent_state_on_launch: el.get_bool_attr("ignoresPersistentStateOnLaunch"),
        debug_document_versioning: el.get_bool_attr("debugDocumentVersioning"),
        debug_service_extension: el.get_attr("debugServiceExtension").map(String::from),
        allow_location_simulation: el.get_bool_attr("allowLocationSimulation"),
        custom_launch_command: el.get_attr("customLaunchCommand").map(String::from),
        app_clip_invocation_url_string: el.get_attr("appClipInvocationURLString").map(String::from),
        ask_for_app_to_launch: el.get_bool_attr("askForAppToLaunch"),
        launch_automatically_substyle: el.get_attr("launchAutomaticallySubstyle").map(String::from),
        buildable_product_runnable: el
            .get_child("BuildableProductRunnable")
            .map(parse_buildable_product_runnable),
        remote_runnable: el.get_child("RemoteRunnable").map(parse_remote_runnable),
        macro_expansion: el
            .get_child("MacroExpansion")
            .and_then(|e| e.get_child("BuildableReference"))
            .map(parse_buildable_reference),
        location_scenario_reference: el
            .get_child("LocationScenarioReference")
            .map(parse_location_scenario_reference),
        store_kit_configuration_file_reference: el
            .get_child("StoreKitConfigurationFileReference")
            .map(parse_store_kit_configuration_file_reference),
        command_line_arguments: parse_command_line_arguments(el),
        environment_variables: parse_environment_variables(el),
        pre_actions: parse_execution_actions(el, "PreActions"),
        post_actions: parse_execution_actions(el, "PostActions"),
    }
}

fn parse_buildable_product_runnable(el: &XmlElement) -> BuildableProductRunnable {
    BuildableProductRunnable {
        runnable_debugging_mode: el.get_attr("runnableDebuggingMode").map(String::from),
        buildable_reference: el
            .get_child("BuildableReference")
            .map(parse_buildable_reference),
    }
}

fn parse_remote_runnable(el: &XmlElement) -> RemoteRunnable {
    RemoteRunnable {
        runnable_debugging_mode: el.get_attr("runnableDebuggingMode").map(String::from),
        bundle_identifier: el.get_attr("BundleIdentifier").map(String::from),
        remote_path: el.get_attr("RemotePath").map(String::from),
        buildable_reference: el
            .get_child("BuildableReference")
            .map(parse_buildable_reference),
    }
}

fn parse_location_scenario_reference(el: &XmlElement) -> LocationScenarioReference {
    LocationScenarioReference {
        identifier: el.get_attr("identifier").map(String::from),
        reference_type: el.get_attr("referenceType").map(String::from),
    }
}

fn parse_store_kit_configuration_file_reference(
    el: &XmlElement,
) -> StoreKitConfigurationFileReference {
    StoreKitConfigurationFileReference {
        identifier: el.get_attr("identifier").map(String::from),
    }
}

fn parse_profile_action(el: &XmlElement) -> ProfileAction {
    ProfileAction {
        build_configuration: el.get_attr("buildConfiguration").map(String::from),
        should_use_launch_scheme_args_env: el.get_bool_attr("shouldUseLaunchSchemeArgsEnv"),
        saved_tool_identifier: el.get_attr("savedToolIdentifier").map(String::from),
        use_custom_working_directory: el.get_bool_attr("useCustomWorkingDirectory"),
        debug_document_versioning: el.get_bool_attr("debugDocumentVersioning"),
        ask_for_app_to_launch: el.get_bool_attr("askForAppToLaunch"),
        launch_automatically_substyle: el.get_attr("launchAutomaticallySubstyle").map(String::from),
        app_clip_invocation_url_string: el.get_attr("appClipInvocationURLString").map(String::from),
        buildable_product_runnable: el
            .get_child("BuildableProductRunnable")
            .map(parse_buildable_product_runnable),
        command_line_arguments: parse_command_line_arguments(el),
        environment_variables: parse_environment_variables(el),
        pre_actions: parse_execution_actions(el, "PreActions"),
        post_actions: parse_execution_actions(el, "PostActions"),
    }
}

fn parse_analyze_action(el: &XmlElement) -> AnalyzeAction {
    AnalyzeAction {
        build_configuration: el.get_attr("buildConfiguration").map(String::from),
        pre_actions: parse_execution_actions(el, "PreActions"),
        post_actions: parse_execution_actions(el, "PostActions"),
    }
}

fn parse_archive_action(el: &XmlElement) -> ArchiveAction {
    ArchiveAction {
        build_configuration: el.get_attr("buildConfiguration").map(String::from),
        custom_archive_name: el.get_attr("customArchiveName").map(String::from),
        reveal_archive_in_organizer: el.get_bool_attr("revealArchiveInOrganizer"),
        pre_actions: parse_execution_actions(el, "PreActions"),
        post_actions: parse_execution_actions(el, "PostActions"),
    }
}

fn parse_command_line_arguments(el: &XmlElement) -> Option<Vec<CommandLineArgument>> {
    let args_el = el.get_child("CommandLineArguments")?;
    let args = args_el.get_children("CommandLineArgument");
    if args.is_empty() {
        return None;
    }
    Some(
        args.into_iter()
            .map(|arg_el| CommandLineArgument {
                argument: arg_el.get_attr("argument").unwrap_or("").to_string(),
                is_enabled: arg_el.get_bool_attr("isEnabled"),
            })
            .collect(),
    )
}

fn parse_environment_variables(el: &XmlElement) -> Option<Vec<EnvironmentVariable>> {
    let vars_el = el.get_child("EnvironmentVariables")?;
    let vars = vars_el.get_children("EnvironmentVariable");
    if vars.is_empty() {
        return None;
    }
    Some(
        vars.into_iter()
            .map(|var_el| EnvironmentVariable {
                key: var_el.get_attr("key").unwrap_or("").to_string(),
                value: var_el.get_attr("value").unwrap_or("").to_string(),
                is_enabled: var_el.get_bool_attr("isEnabled"),
            })
            .collect(),
    )
}

// ============================================================================
// Builder
// ============================================================================

fn get_indent(depth: usize) -> String {
    "   ".repeat(depth)
}

fn bool_to_string(value: bool) -> &'static str {
    if value {
        "YES"
    } else {
        "NO"
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn build(scheme: &XCScheme) -> String {
    let mut lines = Vec::<String>::new();
    lines.push("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string());
    lines.push("<Scheme".to_string());

    let mut scheme_attrs = Vec::new();
    if let Some(ref luv) = scheme.last_upgrade_version {
        scheme_attrs.push(format!("LastUpgradeVersion = \"{}\"", luv));
    }
    if let Some(ref v) = scheme.version {
        scheme_attrs.push(format!("version = \"{}\"", v));
    }
    for attr in &scheme_attrs {
        lines.push(format!("   {}", attr));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref build_action) = scheme.build_action {
        build_build_action(&mut lines, build_action, 1);
    }
    if let Some(ref test_action) = scheme.test_action {
        build_test_action(&mut lines, test_action, 1);
    }
    if let Some(ref launch_action) = scheme.launch_action {
        build_launch_action(&mut lines, launch_action, 1);
    }
    if let Some(ref profile_action) = scheme.profile_action {
        build_profile_action(&mut lines, profile_action, 1);
    }
    if let Some(ref analyze_action) = scheme.analyze_action {
        build_analyze_action(&mut lines, analyze_action, 1);
    }
    if let Some(ref archive_action) = scheme.archive_action {
        build_archive_action(&mut lines, archive_action, 1);
    }

    lines.push("</Scheme>".to_string());
    lines.join("\n") + "\n"
}

fn build_build_action(lines: &mut Vec<String>, action: &BuildAction, depth: usize) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(v) = action.parallelize_buildables {
        attrs.push(format!("parallelizeBuildables = \"{}\"", bool_to_string(v)));
    }
    if let Some(v) = action.build_implicit_dependencies {
        attrs.push(format!(
            "buildImplicitDependencies = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(v) = action.run_post_actions_on_failure {
        attrs.push(format!(
            "runPostActionsOnFailure = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(ref v) = action.build_architectures {
        attrs.push(format!("buildArchitectures = \"{}\"", v));
    }

    lines.push(format!("{indent}<BuildAction"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref pre_actions) = action.pre_actions
        && !pre_actions.is_empty() {
            build_execution_actions(lines, "PreActions", pre_actions, depth + 1);
        }
    if let Some(ref post_actions) = action.post_actions
        && !post_actions.is_empty() {
            build_execution_actions(lines, "PostActions", post_actions, depth + 1);
        }

    if let Some(ref entries) = action.entries
        && !entries.is_empty() {
            lines.push(format!("{}<BuildActionEntries>", get_indent(depth + 1)));
            for entry in entries {
                build_build_action_entry(lines, entry, depth + 2);
            }
            lines.push(format!("{}</BuildActionEntries>", get_indent(depth + 1)));
        }

    lines.push(format!("{indent}</BuildAction>"));
}

fn build_build_action_entry(lines: &mut Vec<String>, entry: &BuildActionEntry, depth: usize) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(v) = entry.build_for_testing {
        attrs.push(format!("buildForTesting = \"{}\"", bool_to_string(v)));
    }
    if let Some(v) = entry.build_for_running {
        attrs.push(format!("buildForRunning = \"{}\"", bool_to_string(v)));
    }
    if let Some(v) = entry.build_for_profiling {
        attrs.push(format!("buildForProfiling = \"{}\"", bool_to_string(v)));
    }
    if let Some(v) = entry.build_for_archiving {
        attrs.push(format!("buildForArchiving = \"{}\"", bool_to_string(v)));
    }
    if let Some(v) = entry.build_for_analyzing {
        attrs.push(format!("buildForAnalyzing = \"{}\"", bool_to_string(v)));
    }

    lines.push(format!("{indent}<BuildActionEntry"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref buildable_ref) = entry.buildable_reference {
        build_buildable_reference(lines, buildable_ref, depth + 1);
    }

    lines.push(format!("{indent}</BuildActionEntry>"));
}

fn build_buildable_reference(
    lines: &mut Vec<String>,
    reference: &BuildableReference,
    depth: usize,
) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<BuildableReference"));
    lines.push(format!(
        "{indent}   BuildableIdentifier = \"{}\"",
        reference.buildable_identifier
    ));
    lines.push(format!(
        "{indent}   BlueprintIdentifier = \"{}\"",
        reference.blueprint_identifier
    ));
    lines.push(format!(
        "{indent}   BuildableName = \"{}\"",
        escape_xml(&reference.buildable_name)
    ));
    lines.push(format!(
        "{indent}   BlueprintName = \"{}\"",
        escape_xml(&reference.blueprint_name)
    ));
    lines.push(format!(
        "{indent}   ReferencedContainer = \"{}\">",
        escape_xml(&reference.referenced_container)
    ));
    lines.push(format!("{indent}</BuildableReference>"));
}

fn build_execution_actions(
    lines: &mut Vec<String>,
    container_name: &str,
    actions: &[ExecutionAction],
    depth: usize,
) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<{container_name}>"));
    for action in actions {
        build_execution_action(lines, action, depth + 1);
    }
    lines.push(format!("{indent}</{container_name}>"));
}

fn build_execution_action(lines: &mut Vec<String>, action: &ExecutionAction, depth: usize) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<ExecutionAction"));
    if let Some(ref action_type) = action.action_type {
        lines.push(format!("{indent}   ActionType = \"{action_type}\">"));
    } else {
        lines.last_mut().unwrap().push('>');
    }
    if let Some(ref content) = action.action_content {
        build_action_content(lines, content, depth + 1);
    }
    lines.push(format!("{indent}</ExecutionAction>"));
}

fn build_action_content(lines: &mut Vec<String>, content: &ActionContent, depth: usize) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(ref v) = content.title {
        attrs.push(format!("title = \"{}\"", escape_xml(v)));
    }
    if let Some(ref v) = content.script_text {
        attrs.push(format!("scriptText = \"{}\"", escape_xml(v)));
    }
    if let Some(ref v) = content.shell_to_invoke {
        attrs.push(format!("shellToInvoke = \"{}\"", v));
    }

    lines.push(format!("{indent}<ActionContent"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }

    if let Some(ref env_buildable) = content.environment_buildable {
        lines.last_mut().unwrap().push('>');
        lines.push(format!("{}<EnvironmentBuildable>", get_indent(depth + 1)));
        build_buildable_reference(lines, env_buildable, depth + 2);
        lines.push(format!("{}</EnvironmentBuildable>", get_indent(depth + 1)));
        lines.push(format!("{indent}</ActionContent>"));
    } else {
        lines.last_mut().unwrap().push('>');
        lines.push(format!("{indent}</ActionContent>"));
    }
}

fn build_test_action(lines: &mut Vec<String>, action: &TestAction, depth: usize) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(ref v) = action.build_configuration {
        attrs.push(format!("buildConfiguration = \"{}\"", v));
    }
    if let Some(ref v) = action.selected_debugger_identifier {
        attrs.push(format!("selectedDebuggerIdentifier = \"{}\"", v));
    }
    if let Some(ref v) = action.selected_launcher_identifier {
        attrs.push(format!("selectedLauncherIdentifier = \"{}\"", v));
    }
    if let Some(v) = action.should_use_launch_scheme_args_env {
        attrs.push(format!(
            "shouldUseLaunchSchemeArgsEnv = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(v) = action.should_autocreate_test_plan {
        attrs.push(format!(
            "shouldAutocreateTestPlan = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(ref v) = action.preferred_screen_capture_format {
        attrs.push(format!("preferredScreenCaptureFormat = \"{}\"", v));
    }
    if let Some(v) = action.code_coverage_enabled {
        attrs.push(format!("codeCoverageEnabled = \"{}\"", bool_to_string(v)));
    }
    if let Some(v) = action.only_generate_coverage_for_specified_targets {
        attrs.push(format!(
            "onlyGenerateCoverageForSpecifiedTargets = \"{}\"",
            bool_to_string(v)
        ));
    }

    lines.push(format!("{indent}<TestAction"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref pre_actions) = action.pre_actions
        && !pre_actions.is_empty() {
            build_execution_actions(lines, "PreActions", pre_actions, depth + 1);
        }
    if let Some(ref post_actions) = action.post_actions
        && !post_actions.is_empty() {
            build_execution_actions(lines, "PostActions", post_actions, depth + 1);
        }

    if let Some(ref test_plans) = action.test_plans
        && !test_plans.is_empty() {
            lines.push(format!("{}<TestPlans>", get_indent(depth + 1)));
            for plan in test_plans {
                build_test_plan_reference(lines, plan, depth + 2);
            }
            lines.push(format!("{}</TestPlans>", get_indent(depth + 1)));
        }

    if let Some(ref macro_expansion) = action.macro_expansion {
        lines.push(format!("{}<MacroExpansion>", get_indent(depth + 1)));
        build_buildable_reference(lines, macro_expansion, depth + 2);
        lines.push(format!("{}</MacroExpansion>", get_indent(depth + 1)));
    }

    if let Some(ref testables) = action.testables {
        if !testables.is_empty() {
            lines.push(format!("{}<Testables>", get_indent(depth + 1)));
            for testable in testables {
                build_testable_reference(lines, testable, depth + 2);
            }
            lines.push(format!("{}</Testables>", get_indent(depth + 1)));
        } else {
            // Output empty Testables if explicitly set
            lines.push(format!("{}<Testables>", get_indent(depth + 1)));
            lines.push(format!("{}</Testables>", get_indent(depth + 1)));
        }
    }

    if let Some(ref cmd_args) = action.command_line_arguments
        && !cmd_args.is_empty() {
            build_command_line_arguments(lines, cmd_args, depth + 1);
        }
    if let Some(ref env_vars) = action.environment_variables
        && !env_vars.is_empty() {
            build_environment_variables(lines, env_vars, depth + 1);
        }

    lines.push(format!("{indent}</TestAction>"));
}

fn build_testable_reference(
    lines: &mut Vec<String>,
    reference: &TestableReference,
    depth: usize,
) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(v) = reference.skipped {
        attrs.push(format!("skipped = \"{}\"", bool_to_string(v)));
    }
    if let Some(v) = reference.parallelizable {
        attrs.push(format!("parallelizable = \"{}\"", bool_to_string(v)));
    }
    if let Some(v) = reference.use_test_selection_whitelist {
        attrs.push(format!(
            "useTestSelectionWhitelist = \"{}\"",
            bool_to_string(v)
        ));
    }

    lines.push(format!("{indent}<TestableReference"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref buildable_ref) = reference.buildable_reference {
        build_buildable_reference(lines, buildable_ref, depth + 1);
    }

    if let Some(ref skipped_tests) = reference.skipped_tests
        && !skipped_tests.is_empty() {
            lines.push(format!("{}<SkippedTests>", get_indent(depth + 1)));
            for test in skipped_tests {
                lines.push(format!("{}<Test", get_indent(depth + 2)));
                lines.push(format!(
                    "{}   Identifier = \"{}\">",
                    get_indent(depth + 2),
                    test.identifier
                ));
                lines.push(format!("{}</Test>", get_indent(depth + 2)));
            }
            lines.push(format!("{}</SkippedTests>", get_indent(depth + 1)));
        }

    if let Some(ref selected_tests) = reference.selected_tests
        && !selected_tests.is_empty() {
            lines.push(format!("{}<SelectedTests>", get_indent(depth + 1)));
            for test in selected_tests {
                lines.push(format!("{}<Test", get_indent(depth + 2)));
                lines.push(format!(
                    "{}   Identifier = \"{}\">",
                    get_indent(depth + 2),
                    test.identifier
                ));
                lines.push(format!("{}</Test>", get_indent(depth + 2)));
            }
            lines.push(format!("{}</SelectedTests>", get_indent(depth + 1)));
        }

    lines.push(format!("{indent}</TestableReference>"));
}

fn build_test_plan_reference(lines: &mut Vec<String>, plan: &TestPlanReference, depth: usize) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<TestPlanReference"));
    if let Some(v) = plan.default_plan {
        lines.push(format!("{indent}   default = \"{}\"", bool_to_string(v)));
    }
    lines.push(format!(
        "{indent}   reference = \"{}\">",
        plan.reference
    ));
    lines.push(format!("{indent}</TestPlanReference>"));
}

fn build_launch_action(lines: &mut Vec<String>, action: &LaunchAction, depth: usize) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(ref v) = action.build_configuration {
        attrs.push(format!("buildConfiguration = \"{}\"", v));
    }
    if let Some(ref v) = action.app_clip_invocation_url_string {
        attrs.push(format!(
            "appClipInvocationURLString = \"{}\"",
            escape_xml(v)
        ));
    }
    if let Some(ref v) = action.selected_debugger_identifier {
        attrs.push(format!("selectedDebuggerIdentifier = \"{}\"", v));
    }
    if let Some(ref v) = action.selected_launcher_identifier {
        attrs.push(format!("selectedLauncherIdentifier = \"{}\"", v));
    }
    if let Some(ref v) = action.launch_style {
        attrs.push(format!("launchStyle = \"{}\"", v));
    }
    if let Some(v) = action.ask_for_app_to_launch {
        // LaunchAction uses "Yes"/"No" (not "YES"/"NO")
        attrs.push(format!(
            "askForAppToLaunch = \"{}\"",
            if v { "Yes" } else { "No" }
        ));
    }
    if let Some(v) = action.use_custom_working_directory {
        attrs.push(format!(
            "useCustomWorkingDirectory = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(ref v) = action.custom_working_directory {
        attrs.push(format!("customWorkingDirectory = \"{}\"", v));
    }
    if let Some(v) = action.ignores_persistent_state_on_launch {
        attrs.push(format!(
            "ignoresPersistentStateOnLaunch = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(v) = action.debug_document_versioning {
        attrs.push(format!(
            "debugDocumentVersioning = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(ref v) = action.debug_service_extension {
        attrs.push(format!("debugServiceExtension = \"{}\"", v));
    }
    if let Some(v) = action.allow_location_simulation {
        attrs.push(format!(
            "allowLocationSimulation = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(ref v) = action.custom_launch_command {
        attrs.push(format!("customLaunchCommand = \"{}\"", escape_xml(v)));
    }
    if let Some(ref v) = action.launch_automatically_substyle {
        attrs.push(format!("launchAutomaticallySubstyle = \"{}\"", v));
    }

    lines.push(format!("{indent}<LaunchAction"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref pre_actions) = action.pre_actions
        && !pre_actions.is_empty() {
            build_execution_actions(lines, "PreActions", pre_actions, depth + 1);
        }
    if let Some(ref post_actions) = action.post_actions
        && !post_actions.is_empty() {
            build_execution_actions(lines, "PostActions", post_actions, depth + 1);
        }

    if let Some(ref runnable) = action.buildable_product_runnable {
        build_buildable_product_runnable(lines, runnable, depth + 1);
    }
    if let Some(ref remote) = action.remote_runnable {
        build_remote_runnable(lines, remote, depth + 1);
    }
    if let Some(ref macro_expansion) = action.macro_expansion {
        lines.push(format!("{}<MacroExpansion>", get_indent(depth + 1)));
        build_buildable_reference(lines, macro_expansion, depth + 2);
        lines.push(format!("{}</MacroExpansion>", get_indent(depth + 1)));
    }
    if let Some(ref location_ref) = action.location_scenario_reference {
        build_location_scenario_reference(lines, location_ref, depth + 1);
    }

    if let Some(ref cmd_args) = action.command_line_arguments
        && !cmd_args.is_empty() {
            build_command_line_arguments(lines, cmd_args, depth + 1);
        }
    if let Some(ref env_vars) = action.environment_variables
        && !env_vars.is_empty() {
            build_environment_variables(lines, env_vars, depth + 1);
        }

    if let Some(ref store_kit_ref) = action.store_kit_configuration_file_reference {
        build_store_kit_configuration_file_reference(lines, store_kit_ref, depth + 1);
    }

    lines.push(format!("{indent}</LaunchAction>"));
}

fn build_buildable_product_runnable(
    lines: &mut Vec<String>,
    runnable: &BuildableProductRunnable,
    depth: usize,
) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<BuildableProductRunnable"));
    if let Some(ref v) = runnable.runnable_debugging_mode {
        lines.push(format!("{indent}   runnableDebuggingMode = \"{v}\">"));
    } else {
        lines.last_mut().unwrap().push('>');
    }
    if let Some(ref buildable_ref) = runnable.buildable_reference {
        build_buildable_reference(lines, buildable_ref, depth + 1);
    }
    lines.push(format!("{indent}</BuildableProductRunnable>"));
}

fn build_remote_runnable(lines: &mut Vec<String>, runnable: &RemoteRunnable, depth: usize) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(ref v) = runnable.runnable_debugging_mode {
        attrs.push(format!("runnableDebuggingMode = \"{}\"", v));
    }
    if let Some(ref v) = runnable.bundle_identifier {
        attrs.push(format!("BundleIdentifier = \"{}\"", v));
    }
    if let Some(ref v) = runnable.remote_path {
        attrs.push(format!("RemotePath = \"{}\"", v));
    }

    lines.push(format!("{indent}<RemoteRunnable"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref buildable_ref) = runnable.buildable_reference {
        build_buildable_reference(lines, buildable_ref, depth + 1);
    }
    lines.push(format!("{indent}</RemoteRunnable>"));
}

fn build_location_scenario_reference(
    lines: &mut Vec<String>,
    reference: &LocationScenarioReference,
    depth: usize,
) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<LocationScenarioReference"));
    if let Some(ref v) = reference.identifier {
        lines.push(format!("{indent}   identifier = \"{}\"", v));
    }
    if let Some(ref v) = reference.reference_type {
        lines.push(format!("{indent}   referenceType = \"{v}\">"));
    } else {
        lines.last_mut().unwrap().push('>');
    }
    lines.push(format!("{indent}</LocationScenarioReference>"));
}

fn build_store_kit_configuration_file_reference(
    lines: &mut Vec<String>,
    reference: &StoreKitConfigurationFileReference,
    depth: usize,
) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<StoreKitConfigurationFileReference"));
    if let Some(ref v) = reference.identifier {
        lines.push(format!("{indent}   identifier = \"{v}\">"));
    } else {
        lines.last_mut().unwrap().push('>');
    }
    lines.push(format!("{indent}</StoreKitConfigurationFileReference>"));
}

fn build_profile_action(lines: &mut Vec<String>, action: &ProfileAction, depth: usize) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(v) = action.ask_for_app_to_launch {
        // ProfileAction uses "YES"/"NO"
        attrs.push(format!(
            "askForAppToLaunch = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(ref v) = action.build_configuration {
        attrs.push(format!("buildConfiguration = \"{}\"", v));
    }
    if let Some(ref v) = action.app_clip_invocation_url_string {
        attrs.push(format!(
            "appClipInvocationURLString = \"{}\"",
            escape_xml(v)
        ));
    }
    if let Some(v) = action.should_use_launch_scheme_args_env {
        attrs.push(format!(
            "shouldUseLaunchSchemeArgsEnv = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(ref v) = action.saved_tool_identifier {
        attrs.push(format!("savedToolIdentifier = \"{}\"", v));
    }
    if let Some(v) = action.use_custom_working_directory {
        attrs.push(format!(
            "useCustomWorkingDirectory = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(v) = action.debug_document_versioning {
        attrs.push(format!(
            "debugDocumentVersioning = \"{}\"",
            bool_to_string(v)
        ));
    }
    if let Some(ref v) = action.launch_automatically_substyle {
        attrs.push(format!("launchAutomaticallySubstyle = \"{}\"", v));
    }

    lines.push(format!("{indent}<ProfileAction"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref pre_actions) = action.pre_actions
        && !pre_actions.is_empty() {
            build_execution_actions(lines, "PreActions", pre_actions, depth + 1);
        }
    if let Some(ref post_actions) = action.post_actions
        && !post_actions.is_empty() {
            build_execution_actions(lines, "PostActions", post_actions, depth + 1);
        }

    if let Some(ref runnable) = action.buildable_product_runnable {
        build_buildable_product_runnable(lines, runnable, depth + 1);
    }

    if let Some(ref cmd_args) = action.command_line_arguments
        && !cmd_args.is_empty() {
            build_command_line_arguments(lines, cmd_args, depth + 1);
        }
    if let Some(ref env_vars) = action.environment_variables
        && !env_vars.is_empty() {
            build_environment_variables(lines, env_vars, depth + 1);
        }

    lines.push(format!("{indent}</ProfileAction>"));
}

fn build_analyze_action(lines: &mut Vec<String>, action: &AnalyzeAction, depth: usize) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<AnalyzeAction"));
    if let Some(ref v) = action.build_configuration {
        lines.push(format!("{indent}   buildConfiguration = \"{v}\">"));
    } else {
        lines.last_mut().unwrap().push('>');
    }
    if let Some(ref pre_actions) = action.pre_actions
        && !pre_actions.is_empty() {
            build_execution_actions(lines, "PreActions", pre_actions, depth + 1);
        }
    if let Some(ref post_actions) = action.post_actions
        && !post_actions.is_empty() {
            build_execution_actions(lines, "PostActions", post_actions, depth + 1);
        }
    lines.push(format!("{indent}</AnalyzeAction>"));
}

fn build_archive_action(lines: &mut Vec<String>, action: &ArchiveAction, depth: usize) {
    let indent = get_indent(depth);

    let mut attrs = Vec::new();
    if let Some(ref v) = action.build_configuration {
        attrs.push(format!("buildConfiguration = \"{}\"", v));
    }
    if let Some(ref v) = action.custom_archive_name {
        attrs.push(format!("customArchiveName = \"{}\"", escape_xml(v)));
    }
    if let Some(v) = action.reveal_archive_in_organizer {
        attrs.push(format!(
            "revealArchiveInOrganizer = \"{}\"",
            bool_to_string(v)
        ));
    }

    lines.push(format!("{indent}<ArchiveAction"));
    for attr in &attrs {
        lines.push(format!("{indent}   {attr}"));
    }
    lines.last_mut().unwrap().push('>');

    if let Some(ref pre_actions) = action.pre_actions
        && !pre_actions.is_empty() {
            build_execution_actions(lines, "PreActions", pre_actions, depth + 1);
        }
    if let Some(ref post_actions) = action.post_actions
        && !post_actions.is_empty() {
            build_execution_actions(lines, "PostActions", post_actions, depth + 1);
        }

    lines.push(format!("{indent}</ArchiveAction>"));
}

fn build_command_line_arguments(
    lines: &mut Vec<String>,
    args: &[CommandLineArgument],
    depth: usize,
) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<CommandLineArguments>"));
    for arg in args {
        let inner_indent = get_indent(depth + 1);
        lines.push(format!("{inner_indent}<CommandLineArgument"));
        lines.push(format!(
            "{inner_indent}   argument = \"{}\"",
            escape_xml(&arg.argument)
        ));
        if let Some(v) = arg.is_enabled {
            lines.push(format!(
                "{inner_indent}   isEnabled = \"{}\">",
                bool_to_string(v)
            ));
        } else {
            lines.last_mut().unwrap().push('>');
        }
        lines.push(format!("{inner_indent}</CommandLineArgument>"));
    }
    lines.push(format!("{indent}</CommandLineArguments>"));
}

fn build_environment_variables(
    lines: &mut Vec<String>,
    vars: &[EnvironmentVariable],
    depth: usize,
) {
    let indent = get_indent(depth);
    lines.push(format!("{indent}<EnvironmentVariables>"));
    for var in vars {
        let inner_indent = get_indent(depth + 1);
        lines.push(format!("{inner_indent}<EnvironmentVariable"));
        lines.push(format!(
            "{inner_indent}   key = \"{}\"",
            escape_xml(&var.key)
        ));
        lines.push(format!(
            "{inner_indent}   value = \"{}\"",
            escape_xml(&var.value)
        ));
        if let Some(v) = var.is_enabled {
            lines.push(format!(
                "{inner_indent}   isEnabled = \"{}\">",
                bool_to_string(v)
            ));
        } else {
            lines.last_mut().unwrap().push('>');
        }
        lines.push(format!("{inner_indent}</EnvironmentVariable>"));
    }
    lines.push(format!("{indent}</EnvironmentVariables>"));
}

// ============================================================================
// Management types and functions (unchanged)
// ============================================================================

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemeUserState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hint: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuppressBuildableAutocreation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct XCSchemeManagement {
    #[serde(rename = "SchemeUserState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_user_state: Option<BTreeMap<String, SchemeUserState>>,
    #[serde(rename = "SuppressBuildableAutocreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_buildable_autocreation: Option<BTreeMap<String, SuppressBuildableAutocreation>>,
}

pub fn parse_management(plist_string: &str) -> anyhow::Result<XCSchemeManagement> {
    let value = plist::Value::from_reader_xml(plist_string.as_bytes())?;
    let dict = value
        .into_dictionary()
        .ok_or_else(|| anyhow::anyhow!("Expected plist dictionary"))?;

    let mut result = XCSchemeManagement::default();

    if let Some(user_state) = dict
        .get("SchemeUserState")
        .and_then(PlistValue::as_dictionary)
    {
        let mut map = BTreeMap::new();
        for (key, value) in user_state {
            if let Some(entry) = value.as_dictionary() {
                map.insert(
                    key.clone(),
                    SchemeUserState {
                        is_shown: entry.get("isShown").and_then(PlistValue::as_boolean),
                        order_hint: entry
                            .get("orderHint")
                            .and_then(PlistValue::as_signed_integer),
                    },
                );
            }
        }
        result.scheme_user_state = Some(map);
    }

    if let Some(suppress) = dict
        .get("SuppressBuildableAutocreation")
        .and_then(PlistValue::as_dictionary)
    {
        let mut map = BTreeMap::new();
        for (key, value) in suppress {
            if let Some(entry) = value.as_dictionary() {
                map.insert(
                    key.clone(),
                    SuppressBuildableAutocreation {
                        primary: entry.get("primary").and_then(PlistValue::as_boolean),
                    },
                );
            }
        }
        result.suppress_buildable_autocreation = Some(map);
    }

    Ok(result)
}

pub fn build_management(management: &XCSchemeManagement) -> anyhow::Result<String> {
    let mut root = plist::Dictionary::new();

    if let Some(user_state) = &management.scheme_user_state {
        let mut map = plist::Dictionary::new();
        for (key, value) in user_state {
            let mut entry = plist::Dictionary::new();
            if let Some(is_shown) = value.is_shown {
                entry.insert("isShown".to_string(), PlistValue::Boolean(is_shown));
            }
            if let Some(order_hint) = value.order_hint {
                entry.insert(
                    "orderHint".to_string(),
                    PlistValue::Integer(order_hint.into()),
                );
            }
            map.insert(key.clone(), PlistValue::Dictionary(entry));
        }
        root.insert("SchemeUserState".to_string(), PlistValue::Dictionary(map));
    }

    if let Some(suppress) = &management.suppress_buildable_autocreation {
        let mut map = plist::Dictionary::new();
        for (key, value) in suppress {
            let mut entry = plist::Dictionary::new();
            if let Some(primary) = value.primary {
                entry.insert("primary".to_string(), PlistValue::Boolean(primary));
            }
            map.insert(key.clone(), PlistValue::Dictionary(entry));
        }
        root.insert(
            "SuppressBuildableAutocreation".to_string(),
            PlistValue::Dictionary(map),
        );
    }

    let mut bytes = Vec::new();
    PlistValue::Dictionary(root).to_writer_xml(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}

#[cfg(test)]
mod tests {
    use super::{build, parse};

    #[test]
    fn parse_scheme_root() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Scheme
   LastUpgradeVersion = "1600"
   version = "1.7">
</Scheme>
"#;

        let scheme = parse(xml).unwrap();
        assert_eq!(scheme.version.as_deref(), Some("1.7"));
        assert_eq!(scheme.last_upgrade_version.as_deref(), Some("1600"));

        let out = build(&scheme);
        assert!(out.contains("<Scheme"));
    }

    #[test]
    fn parse_and_build_full_scheme() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Scheme
   LastUpgradeVersion = "0830"
   version = "2.0">
   <BuildAction
      parallelizeBuildables = "YES"
      buildImplicitDependencies = "YES">
      <BuildActionEntries>
         <BuildActionEntry
            buildForTesting = "YES"
            buildForRunning = "YES"
            buildForProfiling = "YES"
            buildForArchiving = "YES"
            buildForAnalyzing = "YES">
            <BuildableReference
               BuildableIdentifier = "primary"
               BlueprintIdentifier = "23766C111EAA3484007A9026"
               BuildableName = "iOS.app"
               BlueprintName = "iOS"
               ReferencedContainer = "container:Project.xcodeproj">
            </BuildableReference>
         </BuildActionEntry>
      </BuildActionEntries>
   </BuildAction>
   <TestAction
      buildConfiguration = "Debug"
      selectedDebuggerIdentifier = "Xcode.DebuggerFoundation.Debugger.LLDB"
      selectedLauncherIdentifier = "Xcode.DebuggerFoundation.Launcher.LLDB"
      shouldUseLaunchSchemeArgsEnv = "YES">
      <Testables>
      </Testables>
   </TestAction>
   <LaunchAction
      buildConfiguration = "Debug"
      selectedDebuggerIdentifier = "Xcode.DebuggerFoundation.Debugger.LLDB"
      selectedLauncherIdentifier = "Xcode.DebuggerFoundation.Launcher.LLDB"
      launchStyle = "0"
      useCustomWorkingDirectory = "NO"
      ignoresPersistentStateOnLaunch = "NO"
      debugDocumentVersioning = "YES"
      debugServiceExtension = "internal"
      allowLocationSimulation = "YES">
      <BuildableProductRunnable
         runnableDebuggingMode = "0">
         <BuildableReference
            BuildableIdentifier = "primary"
            BlueprintIdentifier = "23766C111EAA3484007A9026"
            BuildableName = "iOS.app"
            BlueprintName = "iOS"
            ReferencedContainer = "container:Project.xcodeproj">
         </BuildableReference>
      </BuildableProductRunnable>
   </LaunchAction>
   <ProfileAction
      buildConfiguration = "Release"
      shouldUseLaunchSchemeArgsEnv = "YES"
      savedToolIdentifier = ""
      useCustomWorkingDirectory = "NO"
      debugDocumentVersioning = "YES">
      <BuildableProductRunnable
         runnableDebuggingMode = "0">
         <BuildableReference
            BuildableIdentifier = "primary"
            BlueprintIdentifier = "23766C111EAA3484007A9026"
            BuildableName = "iOS.app"
            BlueprintName = "iOS"
            ReferencedContainer = "container:Project.xcodeproj">
         </BuildableReference>
      </BuildableProductRunnable>
   </ProfileAction>
   <AnalyzeAction
      buildConfiguration = "Debug">
   </AnalyzeAction>
   <ArchiveAction
      buildConfiguration = "Release"
      revealArchiveInOrganizer = "YES">
   </ArchiveAction>
</Scheme>
"#;

        let scheme = parse(xml).unwrap();
        assert_eq!(scheme.version.as_deref(), Some("2.0"));
        assert!(scheme.build_action.is_some());
        assert!(scheme.test_action.is_some());
        assert!(scheme.launch_action.is_some());
        assert!(scheme.profile_action.is_some());
        assert!(scheme.analyze_action.is_some());
        assert!(scheme.archive_action.is_some());

        let built = build(&scheme);
        assert_eq!(xml, built);
    }
}

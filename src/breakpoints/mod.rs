use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XCBreakpointList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<BreakpointProxy>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointProxy {
    #[serde(rename = "breakpointExtensionID")]
    pub breakpoint_extension_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoint_content: Option<BreakpointContent>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_be_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_after_running_actions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_column_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_column_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_line_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_line_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmark_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmark_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_on_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<BreakpointActionProxy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<BreakpointLocation>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointActionProxy {
    #[serde(rename = "actionExtensionID")]
    pub action_extension_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_content: Option<BreakpointActionContent>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointActionContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conveyance_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_until_done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_be_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_after_running_actions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_line_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_line_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_column_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_column_number: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Bucket")]
struct BucketXml {
    #[serde(rename = "@uuid")]
    uuid: Option<String>,
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "@version")]
    version: Option<String>,
    #[serde(rename = "Breakpoints")]
    breakpoints: Option<BreakpointsXml>,
}

#[derive(Debug, Deserialize)]
struct BreakpointsXml {
    #[serde(rename = "BreakpointProxy", default)]
    entries: Vec<BreakpointProxyXml>,
}

#[derive(Debug, Deserialize)]
struct BreakpointProxyXml {
    #[serde(rename = "@BreakpointExtensionID")]
    breakpoint_extension_id: Option<String>,
    #[serde(rename = "BreakpointContent")]
    breakpoint_content: Option<BreakpointContentXml>,
}

#[derive(Debug, Deserialize)]
struct BreakpointContentXml {
    #[serde(rename = "@uuid")]
    uuid: Option<String>,
    #[serde(rename = "@shouldBeEnabled")]
    should_be_enabled: Option<String>,
    #[serde(rename = "@ignoreCount")]
    ignore_count: Option<i64>,
    #[serde(rename = "@continueAfterRunningActions")]
    continue_after_running_actions: Option<String>,
    #[serde(rename = "@filePath")]
    file_path: Option<String>,
    #[serde(rename = "@startingColumnNumber")]
    starting_column_number: Option<String>,
    #[serde(rename = "@endingColumnNumber")]
    ending_column_number: Option<String>,
    #[serde(rename = "@startingLineNumber")]
    starting_line_number: Option<String>,
    #[serde(rename = "@endingLineNumber")]
    ending_line_number: Option<String>,
    #[serde(rename = "@landmarkName")]
    landmark_name: Option<String>,
    #[serde(rename = "@landmarkType")]
    landmark_type: Option<String>,
    #[serde(rename = "@condition")]
    condition: Option<String>,
    #[serde(rename = "@scope")]
    scope: Option<String>,
    #[serde(rename = "@symbolName")]
    symbol_name: Option<String>,
    #[serde(rename = "@moduleName")]
    module_name: Option<String>,
    #[serde(rename = "@exceptionType")]
    exception_type: Option<String>,
    #[serde(rename = "@stopOnStyle")]
    stop_on_style: Option<String>,
    #[serde(rename = "Actions")]
    actions: Option<ActionsXml>,
    #[serde(rename = "Locations")]
    locations: Option<LocationsXml>,
}

#[derive(Debug, Deserialize)]
struct ActionsXml {
    #[serde(rename = "BreakpointActionProxy", default)]
    entries: Vec<BreakpointActionProxyXml>,
}

#[derive(Debug, Deserialize)]
struct BreakpointActionProxyXml {
    #[serde(rename = "@ActionExtensionID")]
    action_extension_id: Option<String>,
    #[serde(rename = "ActionContent")]
    action_content: Option<BreakpointActionContentXml>,
}

#[derive(Debug, Deserialize)]
struct BreakpointActionContentXml {
    #[serde(rename = "@consoleCommand")]
    console_command: Option<String>,
    #[serde(rename = "@message")]
    message: Option<String>,
    #[serde(rename = "@conveyanceType")]
    conveyance_type: Option<String>,
    #[serde(rename = "@shellCommand")]
    shell_command: Option<String>,
    #[serde(rename = "@shellArguments")]
    shell_arguments: Option<String>,
    #[serde(rename = "@waitUntilDone")]
    wait_until_done: Option<String>,
    #[serde(rename = "@script")]
    script: Option<String>,
    #[serde(rename = "@soundName")]
    sound_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LocationsXml {
    #[serde(rename = "BreakpointLocationProxy", default)]
    entries: Vec<BreakpointLocationProxyXml>,
}

#[derive(Debug, Deserialize)]
struct BreakpointLocationProxyXml {
    #[serde(rename = "@uuid")]
    uuid: Option<String>,
    #[serde(rename = "@shouldBeEnabled")]
    should_be_enabled: Option<String>,
    #[serde(rename = "@ignoreCount")]
    ignore_count: Option<i64>,
    #[serde(rename = "@continueAfterRunningActions")]
    continue_after_running_actions: Option<String>,
    #[serde(rename = "@symbolName")]
    symbol_name: Option<String>,
    #[serde(rename = "@moduleName")]
    module_name: Option<String>,
    #[serde(rename = "@urlString")]
    url_string: Option<String>,
    #[serde(rename = "@startingLineNumber")]
    starting_line_number: Option<String>,
    #[serde(rename = "@endingLineNumber")]
    ending_line_number: Option<String>,
    #[serde(rename = "@startingColumnNumber")]
    starting_column_number: Option<String>,
    #[serde(rename = "@endingColumnNumber")]
    ending_column_number: Option<String>,
    #[serde(rename = "BreakpointLocationContent")]
    content: Option<BreakpointLocationContentXml>,
}

#[derive(Debug, Deserialize)]
struct BreakpointLocationContentXml {
    #[serde(rename = "@uuid")]
    uuid: Option<String>,
    #[serde(rename = "@shouldBeEnabled")]
    should_be_enabled: Option<String>,
    #[serde(rename = "@ignoreCount")]
    ignore_count: Option<i64>,
    #[serde(rename = "@continueAfterRunningActions")]
    continue_after_running_actions: Option<String>,
    #[serde(rename = "@symbolName")]
    symbol_name: Option<String>,
    #[serde(rename = "@moduleName")]
    module_name: Option<String>,
    #[serde(rename = "@urlString")]
    url_string: Option<String>,
    #[serde(rename = "@startingLineNumber")]
    starting_line_number: Option<String>,
    #[serde(rename = "@endingLineNumber")]
    ending_line_number: Option<String>,
    #[serde(rename = "@startingColumnNumber")]
    starting_column_number: Option<String>,
    #[serde(rename = "@endingColumnNumber")]
    ending_column_number: Option<String>,
}

pub fn parse(xml: &str) -> anyhow::Result<XCBreakpointList> {
    let bucket: BucketXml = from_str(xml)?;

    let breakpoints = bucket.breakpoints.map(|container| {
        container
            .entries
            .into_iter()
            .map(|entry| BreakpointProxy {
                breakpoint_extension_id: entry.breakpoint_extension_id.unwrap_or_default(),
                breakpoint_content: entry.breakpoint_content.map(content_from_xml),
            })
            .collect::<Vec<_>>()
    });

    Ok(XCBreakpointList {
        uuid: bucket.uuid,
        r#type: bucket.r#type,
        version: bucket.version,
        breakpoints,
    })
}

fn content_from_xml(xml: BreakpointContentXml) -> BreakpointContent {
    BreakpointContent {
        uuid: xml.uuid,
        should_be_enabled: parse_bool(xml.should_be_enabled.as_deref()),
        ignore_count: xml.ignore_count,
        continue_after_running_actions: parse_bool(xml.continue_after_running_actions.as_deref()),
        file_path: xml.file_path,
        starting_column_number: xml.starting_column_number,
        ending_column_number: xml.ending_column_number,
        starting_line_number: xml.starting_line_number,
        ending_line_number: xml.ending_line_number,
        landmark_name: xml.landmark_name,
        landmark_type: xml.landmark_type,
        condition: xml.condition,
        scope: xml.scope,
        symbol_name: xml.symbol_name,
        module_name: xml.module_name,
        exception_type: xml.exception_type,
        stop_on_style: xml.stop_on_style,
        actions: xml.actions.map(|actions| {
            actions
                .entries
                .into_iter()
                .map(|action| BreakpointActionProxy {
                    action_extension_id: action.action_extension_id.unwrap_or_default(),
                    action_content: action
                        .action_content
                        .map(|content| BreakpointActionContent {
                            console_command: content.console_command,
                            message: content.message,
                            conveyance_type: content.conveyance_type,
                            shell_command: content.shell_command,
                            shell_arguments: content.shell_arguments,
                            wait_until_done: parse_bool(content.wait_until_done.as_deref()),
                            script: content.script,
                            sound_name: content.sound_name,
                        }),
                })
                .collect()
        }),
        locations: xml.locations.map(|locations| {
            locations
                .entries
                .into_iter()
                .map(location_from_proxy)
                .collect()
        }),
    }
}

fn location_from_proxy(proxy: BreakpointLocationProxyXml) -> BreakpointLocation {
    let source = proxy.content.map(|content| BreakpointLocation {
        uuid: content.uuid,
        should_be_enabled: parse_bool(content.should_be_enabled.as_deref()),
        ignore_count: content.ignore_count,
        continue_after_running_actions: parse_bool(
            content.continue_after_running_actions.as_deref(),
        ),
        symbol_name: content.symbol_name,
        module_name: content.module_name,
        url_string: content.url_string,
        starting_line_number: content.starting_line_number,
        ending_line_number: content.ending_line_number,
        starting_column_number: content.starting_column_number,
        ending_column_number: content.ending_column_number,
    });

    source.unwrap_or(BreakpointLocation {
        uuid: proxy.uuid,
        should_be_enabled: parse_bool(proxy.should_be_enabled.as_deref()),
        ignore_count: proxy.ignore_count,
        continue_after_running_actions: parse_bool(proxy.continue_after_running_actions.as_deref()),
        symbol_name: proxy.symbol_name,
        module_name: proxy.module_name,
        url_string: proxy.url_string,
        starting_line_number: proxy.starting_line_number,
        ending_line_number: proxy.ending_line_number,
        starting_column_number: proxy.starting_column_number,
        ending_column_number: proxy.ending_column_number,
    })
}

pub fn build(list: &XCBreakpointList) -> String {
    let mut lines = Vec::<String>::new();

    lines.push("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string());
    lines.push("<Bucket".to_string());

    if let Some(uuid) = &list.uuid {
        lines.push(format!("   uuid = \"{}\"", uuid));
    }
    if let Some(typ) = &list.r#type {
        lines.push(format!("   type = \"{}\"", typ));
    }
    if let Some(version) = &list.version {
        lines.push(format!("   version = \"{}\"", version));
    }
    if let Some(last) = lines.last_mut() {
        last.push('>');
    }

    if let Some(breakpoints) = &list.breakpoints
        && !breakpoints.is_empty() {
            lines.push(format!("{}<Breakpoints>", indent(1)));
            for bp in breakpoints {
                lines.extend(build_breakpoint_proxy(bp, 2));
            }
            lines.push(format!("{}</Breakpoints>", indent(1)));
        }

    lines.push("</Bucket>".to_string());
    lines.join("\n") + "\n"
}

fn build_breakpoint_proxy(proxy: &BreakpointProxy, depth: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let i = indent(depth);

    lines.push(format!("{}<BreakpointProxy", i));
    lines.push(format!(
        "{}   BreakpointExtensionID = \"{}\">",
        i, proxy.breakpoint_extension_id
    ));

    if let Some(content) = &proxy.breakpoint_content {
        lines.extend(build_breakpoint_content(content, depth + 1));
    }

    lines.push(format!("{}</BreakpointProxy>", i));
    lines
}

fn build_breakpoint_content(content: &BreakpointContent, depth: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let i = indent(depth);

    lines.push(format!("{}<BreakpointContent", i));
    push_opt_attr(&mut lines, &i, "uuid", content.uuid.as_deref());
    push_opt_attr_bool(&mut lines, &i, "shouldBeEnabled", content.should_be_enabled);
    push_opt_attr_i64(&mut lines, &i, "ignoreCount", content.ignore_count);
    push_opt_attr_bool(
        &mut lines,
        &i,
        "continueAfterRunningActions",
        content.continue_after_running_actions,
    );
    push_opt_attr(
        &mut lines,
        &i,
        "filePath",
        content
            .file_path
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "startingColumnNumber",
        content.starting_column_number.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "endingColumnNumber",
        content.ending_column_number.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "startingLineNumber",
        content.starting_line_number.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "endingLineNumber",
        content.ending_line_number.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "landmarkName",
        content
            .landmark_name
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "landmarkType",
        content.landmark_type.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "condition",
        content
            .condition
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(&mut lines, &i, "scope", content.scope.as_deref());
    push_opt_attr(
        &mut lines,
        &i,
        "symbolName",
        content
            .symbol_name
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "moduleName",
        content
            .module_name
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "exceptionType",
        content.exception_type.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "stopOnStyle",
        content.stop_on_style.as_deref(),
    );

    let has_children = content
        .actions
        .as_ref()
        .map(|a| !a.is_empty())
        .unwrap_or(false)
        || content
            .locations
            .as_ref()
            .map(|l| !l.is_empty())
            .unwrap_or(false);

    if let Some(last) = lines.last_mut() {
        last.push('>');
    }

    if has_children {
        if let Some(actions) = &content.actions
            && !actions.is_empty() {
                lines.push(format!("{}<Actions>", indent(depth + 1)));
                for action in actions {
                    lines.extend(build_action_proxy(action, depth + 2));
                }
                lines.push(format!("{}</Actions>", indent(depth + 1)));
            }

        if let Some(locations) = &content.locations
            && !locations.is_empty() {
                lines.push(format!("{}<Locations>", indent(depth + 1)));
                for location in locations {
                    lines.extend(build_location(location, depth + 2));
                }
                lines.push(format!("{}</Locations>", indent(depth + 1)));
            }
    }

    lines.push(format!("{}</BreakpointContent>", i));
    lines
}

fn build_action_proxy(proxy: &BreakpointActionProxy, depth: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let i = indent(depth);

    lines.push(format!("{}<BreakpointActionProxy", i));
    lines.push(format!(
        "{}   ActionExtensionID = \"{}\">",
        i, proxy.action_extension_id
    ));

    if let Some(content) = &proxy.action_content {
        lines.extend(build_action_content(content, depth + 1));
    }

    lines.push(format!("{}</BreakpointActionProxy>", i));
    lines
}

fn build_action_content(content: &BreakpointActionContent, depth: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let i = indent(depth);

    lines.push(format!("{}<ActionContent", i));
    push_opt_attr(
        &mut lines,
        &i,
        "consoleCommand",
        content
            .console_command
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "message",
        content.message.as_deref().map(escape_xml_owned).as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "conveyanceType",
        content.conveyance_type.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "shellCommand",
        content
            .shell_command
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "shellArguments",
        content
            .shell_arguments
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr_bool(&mut lines, &i, "waitUntilDone", content.wait_until_done);
    push_opt_attr(
        &mut lines,
        &i,
        "script",
        content.script.as_deref().map(escape_xml_owned).as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &i,
        "soundName",
        content
            .sound_name
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );

    if let Some(last) = lines.last_mut() {
        last.push('>');
    }
    lines.push(format!("{}</ActionContent>", i));
    lines
}

fn build_location(location: &BreakpointLocation, depth: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let i = indent(depth);

    lines.push(format!("{}<BreakpointLocationProxy>", i));
    lines.push(format!("{}<BreakpointLocationContent", indent(depth + 1)));

    let inner_indent = indent(depth + 1);
    push_opt_attr(&mut lines, &inner_indent, "uuid", location.uuid.as_deref());
    push_opt_attr_bool(
        &mut lines,
        &inner_indent,
        "shouldBeEnabled",
        location.should_be_enabled,
    );
    push_opt_attr_i64(
        &mut lines,
        &inner_indent,
        "ignoreCount",
        location.ignore_count,
    );
    push_opt_attr_bool(
        &mut lines,
        &inner_indent,
        "continueAfterRunningActions",
        location.continue_after_running_actions,
    );
    push_opt_attr(
        &mut lines,
        &inner_indent,
        "symbolName",
        location
            .symbol_name
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &inner_indent,
        "moduleName",
        location
            .module_name
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &inner_indent,
        "urlString",
        location
            .url_string
            .as_deref()
            .map(escape_xml_owned)
            .as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &inner_indent,
        "startingLineNumber",
        location.starting_line_number.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &inner_indent,
        "endingLineNumber",
        location.ending_line_number.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &inner_indent,
        "startingColumnNumber",
        location.starting_column_number.as_deref(),
    );
    push_opt_attr(
        &mut lines,
        &inner_indent,
        "endingColumnNumber",
        location.ending_column_number.as_deref(),
    );

    if let Some(last) = lines.last_mut() {
        last.push('>');
    }

    lines.push(format!("{}</BreakpointLocationContent>", indent(depth + 1)));
    lines.push(format!("{}</BreakpointLocationProxy>", i));
    lines
}

fn push_opt_attr(lines: &mut Vec<String>, indent: &str, name: &str, value: Option<&str>) {
    if let Some(v) = value {
        lines.push(format!("{}   {} = \"{}\"", indent, name, v));
    }
}

fn push_opt_attr_bool(lines: &mut Vec<String>, indent: &str, name: &str, value: Option<bool>) {
    if let Some(v) = value {
        lines.push(format!(
            "{}   {} = \"{}\"",
            indent,
            name,
            if v { "Yes" } else { "No" }
        ));
    }
}

fn push_opt_attr_i64(lines: &mut Vec<String>, indent: &str, name: &str, value: Option<i64>) {
    if let Some(v) = value {
        lines.push(format!("{}   {} = \"{}\"", indent, name, v));
    }
}

fn parse_bool(value: Option<&str>) -> Option<bool> {
    match value {
        Some("Yes") | Some("YES") => Some(true),
        Some("No") | Some("NO") => Some(false),
        _ => None,
    }
}

fn escape_xml_owned(value: &str) -> String {
    crate::util::escape_xml(value)
}

fn indent(depth: usize) -> String {
    "   ".repeat(depth)
}

#[cfg(test)]
mod tests {
    use super::{build, parse};

    #[test]
    fn round_trip_simple() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Bucket uuid = "A" type = "1" version = "2.0"></Bucket>
"#;

        let parsed = parse(xml).unwrap();
        assert_eq!(parsed.uuid.as_deref(), Some("A"));

        let rebuilt = build(&parsed);
        assert!(rebuilt.contains("<Bucket"));
    }
}

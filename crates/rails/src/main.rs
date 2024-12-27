use fancy_regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(Debug, Clone, Serialize)]
struct Router {
    pub routes: Vec<Route>,
    pub constraints: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Serialize)]
struct Route {
    section: Option<String>,
    id: usize,
    path_original: String,
    path: String,
    params: Vec<String>,
    constraints: BTreeMap<String, String>,
    methods: Vec<String>,
    action: String,
    prefix: Option<String>,
}

// Also handles conversion from Rails URI format to Wayfind.
fn parse_uri(value: &str) -> (String, Vec<String>) {
    let mut path = String::new();

    let mut params = vec![];
    let mut segment = String::new();
    let mut in_param = false;

    let chars: Vec<char> = value.chars().collect();
    let len = chars.len();

    for (index, &char) in chars.iter().enumerate() {
        match char {
            ':' | '*' => {
                in_param = true;

                if char == '*' {
                    segment.push_str("{*");
                } else {
                    segment.push('{');
                }

                let mut param = String::new();
                let mut next = index + 1;

                while next < len && (chars[next].is_ascii_alphanumeric() || chars[next] == '_') {
                    param.push(chars[next]);
                    next += 1;
                }

                if !param.is_empty() {
                    params.push(param);
                }
            }
            _ if in_param && (char.is_ascii_alphanumeric() || char == '_') => {
                segment.push(char);

                if index == len - 1 {
                    segment.push('}');
                    path.push_str(&segment);
                    segment.clear();
                }
            }
            _ if in_param => {
                if segment.starts_with('{') {
                    segment.push('}');
                }

                path.push_str(&segment);
                segment.clear();

                in_param = false;
                path.push(char);
            }
            _ => {
                if !segment.is_empty() {
                    path.push_str(&segment);
                    segment.clear();
                }

                path.push(char);
            }
        }
    }

    if !segment.is_empty() {
        if in_param {
            segment.push('}');
        }

        path.push_str(&segment);
    }

    (path, params)
}

fn parse_constraints(constraint: &str, params: &[String]) -> BTreeMap<String, String> {
    let constraint = constraint
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}');

    if constraint.is_empty() {
        return BTreeMap::new();
    }

    let mut constraints = BTreeMap::new();

    let parts: Vec<&str> = constraint.split(", :").collect();
    for (index, part) in parts.iter().enumerate() {
        let part = if index == 0 {
            *part
        } else {
            &format!(":{part}")
        };

        if let Some((key, value)) = part.split_once("=>") {
            let key = key
                .trim()
                .trim_start_matches([':', '*'])
                .trim_matches(|c| c == '"' || c == '\'')
                .to_owned();

            if params.contains(&key) {
                let value = value.trim();
                let end = value.len();

                // Handle static, symbol and regex constraints
                let value = if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('/') && value.ends_with('/'))
                {
                    &value[1..end - 1]
                } else if value.starts_with(':') {
                    &value[1..end]
                } else {
                    value
                };

                // Ruby supports octal sequences in regex
                // Convert them to hex to make them work with fancy-regex
                let value = value
                    .replace("[^\\000-\\040\\177", "[^\\x00-\\x20\\x7f")
                    .replace("\\0", "\\x00");

                // Verify it actually is valid regex
                Regex::new(&value).unwrap();

                constraints.insert(key, value.clone());
            }
        }
    }

    constraints
}

fn parse_router(input: &str) -> Router {
    let mut routes = vec![];
    let mut section = None;
    let mut route = None;

    for line in input.lines() {
        let line = line.trim();

        if line.starts_with("[ Routes for ") {
            section = Some(line[13..line.len() - 2].to_owned());
            continue;
        }

        if line.starts_with("--[ Route ") {
            if let Some(route) = route.take() {
                routes.push(route);
            }

            let id = line.split_whitespace().nth(2).unwrap().parse().unwrap();

            route = Some(Route {
                id,
                section: section.clone(),
                prefix: None,
                path: String::new(),
                path_original: String::new(),
                params: vec![],
                constraints: BTreeMap::new(),
                methods: vec![],
                action: String::new(),
            });

            continue;
        }

        if let Some((key, value)) = line.split_once('|') {
            let key = key.trim().to_lowercase().replace(' ', "");
            let value = value.trim().to_owned();

            if let Some(route) = route.as_mut() {
                match key.as_str() {
                    "prefix" => {
                        if !value.is_empty() {
                            route.prefix = Some(value.clone());
                        }
                    }
                    "verb" => {
                        let methods: Vec<String> = value
                            .split('|')
                            .map(|v| v.trim().to_owned())
                            .filter(|v| !v.is_empty())
                            .collect();

                        if !methods.is_empty() {
                            route.methods = methods;
                        }
                    }
                    "uri" => {
                        route.path_original.clone_from(&value);

                        let (path, params) = parse_uri(&value);
                        route.path = path;
                        route.params = params;
                    }
                    "controller#action" => {
                        let mut in_parens = 0;
                        let mut start = None;
                        let end = value.len();

                        for (index, char) in value.chars().enumerate() {
                            match char {
                                '(' => in_parens += 1,
                                ')' => in_parens -= 1,
                                '{' if in_parens == 0 && value[index..].starts_with("{:") => {
                                    start = Some(index);
                                    break;
                                }
                                _ => {}
                            }
                        }

                        if let Some(start) = start {
                            let action = value[..start].trim().to_owned();
                            route.action = action;

                            let constraint = value[start..end].trim().to_owned();
                            route.constraints = parse_constraints(&constraint, &route.params);
                        } else {
                            let action = value.trim().to_owned();
                            route.action = action;
                        }
                    }
                    _ => unreachable!("Unknown key: {key}"),
                }
            }
        }
    }

    if let Some(route) = route {
        routes.push(route);
    }

    // Create constraints.
    let mut id = 0;
    let mut constraints = BTreeMap::new();

    for route in &routes {
        for value in route.constraints.values() {
            if !constraints.contains_key(value) {
                constraints.insert(value.to_string(), id);
                id += 1;
            }
        }
    }

    for route in &mut routes {
        let mut path = route.path.clone();

        for (param, constraint) in &route.constraints {
            let name = constraints.get(constraint).unwrap();

            if path.contains(&format!("{{*{param}}}")) {
                path = path.replace(&format!("{{*{param}}}"), &format!("{{*{param}:{name}}}"));
            } else if path.contains(&format!("{{{param}}}")) {
                path = path.replace(&format!("{{{param}}}"), &format!("{{{param}:{name}}}"));
            }
        }

        route.path = path;
    }

    // Add section prefixes to routes.
    for route in &mut routes {
        if let Some(section) = &route.section {
            let section = section.to_lowercase().replace("::", "_");
            if route.path == "/" {
                route.path = format!("/{section}");
            } else {
                route.path = format!("/{section}{}", route.path);
            }
        }
    }

    // Dedupe by action, method + path.
    let mut unique: BTreeMap<(String, Vec<String>, String), Route> = BTreeMap::new();

    for route in routes {
        let key = (
            route.action.clone(),
            route.methods.clone(),
            route.path.clone(),
        );

        unique.entry(key).or_insert(route);
    }

    Router {
        routes: unique.into_values().collect(),
        constraints,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let content = fs::read_to_string(path).unwrap();
    let router = parse_router(&content);

    let json = serde_json::to_string_pretty(&router).unwrap();
    println!("{json}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_json_snapshot;

    #[test]
    fn test_no_verb() {
        let input = r"
            --[ Route 8 ]-------------------------------------------------------------------
            Prefix            | letter_opener_web
            Verb              |
            URI               | /rails/letter_opener
            Controller#Action | LetterOpenerWeb::Engine
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 8,
              "path_original": "/rails/letter_opener",
              "path": "/rails/letter_opener",
              "params": [],
              "constraints": {},
              "methods": [],
              "action": "LetterOpenerWeb::Engine",
              "prefix": "letter_opener_web"
            }
          ],
          "constraints": {}
        }
        "#);
    }

    #[test]
    fn test_multiple_verbs() {
        let input = r"
            --[ Route 226 ]-----------------------------------------------------------------
            Prefix            | decline_invite
            Verb              | GET|POST
            URI               | /-/invites/:id/decline(.:format)
            Controller#Action | invites#decline {:id=>/[A-Za-z0-9_-]+/}
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 226,
              "path_original": "/-/invites/:id/decline(.:format)",
              "path": "/-/invites/{id:0}/decline(.{format})",
              "params": [
                "id",
                "format"
              ],
              "constraints": {
                "id": "[A-Za-z0-9_-]+"
              },
              "methods": [
                "GET",
                "POST"
              ],
              "action": "invites#decline",
              "prefix": "decline_invite"
            }
          ],
          "constraints": {
            "[A-Za-z0-9_-]+": 0
          }
        }
        "#);
    }

    #[test]
    fn test_redirect_action() {
        let input = r"
            --[ Route 248 ]-----------------------------------------------------------------
            Prefix            |
            Verb              | GET
            URI               | /-/s/:username(.:format)
            Controller#Action | redirect(301, users/%{username}/snippets) {:username=>/[a-zA-Z.0-9_\-]+(?<!\.atom)/}
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 248,
              "path_original": "/-/s/:username(.:format)",
              "path": "/-/s/{username:0}(.{format})",
              "params": [
                "username",
                "format"
              ],
              "constraints": {
                "username": "[a-zA-Z.0-9_\\-]+(?<!\\.atom)"
              },
              "methods": [
                "GET"
              ],
              "action": "redirect(301, users/%{username}/snippets)",
              "prefix": null
            }
          ],
          "constraints": {
            "[a-zA-Z.0-9_\\-]+(?<!\\.atom)": 0
          }
        }
        "#);
    }

    #[test]
    fn test_unknown_param() {
        let input = r"
            --[ Route 2072 ]----------------------------------------------------------------
            Prefix            |
            Verb              | GET
            URI               | /.well-known/change-password(.:format)
            Controller#Action | redirect(301, -/user_settings/password/edit) {:status=>302}
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 2072,
              "path_original": "/.well-known/change-password(.:format)",
              "path": "/.well-known/change-password(.{format})",
              "params": [
                "format"
              ],
              "constraints": {},
              "methods": [
                "GET"
              ],
              "action": "redirect(301, -/user_settings/password/edit)",
              "prefix": null
            }
          ],
          "constraints": {}
        }
        "#);
    }

    #[test]
    fn test_sections() {
        let input = r"
            --[ Route 10 ]------------------------------------------------------------------
            Prefix            |
            Verb              | GET
            URI               | /*path(.:format)
            Controller#Action | lookbook/application#not_found
            [ Routes for Toogle::Engine ]
            --[ Route 1 ]-------------------------------------------------------------------
            Prefix            | definitions
            Verb              | GET
            URI               | /definitions(.:format)
            Controller#Action | toogle/definitions#index
            --[ Route 2 ]-------------------------------------------------------------------
            Prefix            | features
            Verb              | GET
            URI               | /
            Controller#Action | toogle/features#index
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 10,
              "path_original": "/*path(.:format)",
              "path": "/{*path}(.{format})",
              "params": [
                "path",
                "format"
              ],
              "constraints": {},
              "methods": [
                "GET"
              ],
              "action": "lookbook/application#not_found",
              "prefix": null
            },
            {
              "section": "Toogle::Engine",
              "id": 1,
              "path_original": "/definitions(.:format)",
              "path": "/toogle_engine/definitions(.{format})",
              "params": [
                "format"
              ],
              "constraints": {},
              "methods": [
                "GET"
              ],
              "action": "toogle/definitions#index",
              "prefix": "definitions"
            },
            {
              "section": "Toogle::Engine",
              "id": 2,
              "path_original": "/",
              "path": "/toogle_engine",
              "params": [],
              "constraints": {},
              "methods": [
                "GET"
              ],
              "action": "toogle/features#index",
              "prefix": "features"
            }
          ],
          "constraints": {}
        }
        "#);
    }

    #[test]
    fn test_proc() {
        let input = r#"
            [ Routes for Lookbook::Engine ]
            --[ Route 1 ]-------------------------------------------------------------------
            Prefix            | cable
            Verb              |
            URI               | /cable
            Controller#Action | #<ActionCable::Server::Base:0x00007f8ae79eadd0 @config=#<ActionCable::Server::Configuration:0x00007f8ae79eae70 @log_tags=[], @connection_class=#<Proc:0x00007f8abff35790 /home/gitpod/.asdf/installs/ruby/3.2.4/lib/ruby/gems/3.2.0/gems/lookbook-2.3.4/lib/lookbook/cable/cable.rb:48 (lambda)>, @worker_pool_size=4, @disable_request_forgery_protection=false, @allow_same_origin_as_host=true, @cable={"adapter"=>"async"}, @mount_path=nil, @logger=#<ActiveSupport::Logger:0x00007f8b063f45b0 @level=0, @progname=nil, @default_formatter=#<Logger::Formatter:0x00007f8b069cff48 @datetime_format=nil>, @formatter=#<ActiveSupport::Logger::SimpleFormatter:0x00007f8b069cb8a8 @datetime_format=nil, @thread_key="activesupport_tagged_logging_tags:36020">, @logdev=#<Logger::LogDevice:0x00007f8b063f5140 @shift_period_suffix="%Y%m%d", @shift_size=1048576, @shift_age=0, @filename="/workspace/gitlab-development-kit/gitlab/log/development.log", @dev=#<File:/workspace/gitlab-development-kit/gitlab/log/development.log>, @binmode=false, @mon_data=#<Monitor:0x00007f8b069cfa98>, @mon_data_owner_object_id=22800>>>, @mutex=#<Monitor:0x00007f8abff356a0>, @pubsub=nil, @worker_pool=nil, @event_loop=nil, @remote_connections=nil>
        "#;

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r##"
        {
          "routes": [
            {
              "section": "Lookbook::Engine",
              "id": 1,
              "path_original": "/cable",
              "path": "/lookbook_engine/cable",
              "params": [],
              "constraints": {},
              "methods": [],
              "action": "#<ActionCable::Server::Base:0x00007f8ae79eadd0 @config=#<ActionCable::Server::Configuration:0x00007f8ae79eae70 @log_tags=[], @connection_class=#<Proc:0x00007f8abff35790 /home/gitpod/.asdf/installs/ruby/3.2.4/lib/ruby/gems/3.2.0/gems/lookbook-2.3.4/lib/lookbook/cable/cable.rb:48 (lambda)>, @worker_pool_size=4, @disable_request_forgery_protection=false, @allow_same_origin_as_host=true, @cable={\"adapter\"=>\"async\"}, @mount_path=nil, @logger=#<ActiveSupport::Logger:0x00007f8b063f45b0 @level=0, @progname=nil, @default_formatter=#<Logger::Formatter:0x00007f8b069cff48 @datetime_format=nil>, @formatter=#<ActiveSupport::Logger::SimpleFormatter:0x00007f8b069cb8a8 @datetime_format=nil, @thread_key=\"activesupport_tagged_logging_tags:36020\">, @logdev=#<Logger::LogDevice:0x00007f8b063f5140 @shift_period_suffix=\"%Y%m%d\", @shift_size=1048576, @shift_age=0, @filename=\"/workspace/gitlab-development-kit/gitlab/log/development.log\", @dev=#<File:/workspace/gitlab-development-kit/gitlab/log/development.log>, @binmode=false, @mon_data=#<Monitor:0x00007f8b069cfa98>, @mon_data_owner_object_id=22800>>>, @mutex=#<Monitor:0x00007f8abff356a0>, @pubsub=nil, @worker_pool=nil, @event_loop=nil, @remote_connections=nil>",
              "prefix": "cable"
            }
          ],
          "constraints": {}
        }
        "##);
    }

    #[test]
    fn test_params() {
        let input = r"
            --[ Route 2071 ]----------------------------------------------------------------
            Prefix            |
            Verb              | DELETE
            URI               | /*namespace_id/:project_id(.:format)
            Controller#Action | application#route_not_found {:project_id=>/(?!((?i-mx:\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\/folders|files|find_file|gitlab\-lfs\/objects|info\/lfs\/objects|new|preview|raw|refs|tree|update|wikis))\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254})(?-mix:(?<!\.git|\.atom)))/, :namespace_id=>/(?-mix:(?!((?i-mx:\-|\.well\-known|404\.html|422\.html|500\.html|502\.html|503\.html|admin|api|apple\-touch\-icon\.png|assets|dashboard|deploy\.html|explore|favicon\.ico|favicon\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\.txt|s|search|sitemap|sitemap\.xml|sitemap\.xml\.gz|slash\-command\-logo\.png|snippets|unsubscribes|uploads|users|v2))\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254}[a-zA-Z0-9_\-]|[a-zA-Z0-9_])(?-mix:(?<!\.git|\.atom))))(?:\/(?!(?i-mx:\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\/folders|files|find_file|gitlab\-lfs\/objects|info\/lfs\/objects|new|preview|raw|refs|tree|update|wikis)\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254}[a-zA-Z0-9_\-]|[a-zA-Z0-9_])(?-mix:(?<!\.git|\.atom))))*/}
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 2071,
              "path_original": "/*namespace_id/:project_id(.:format)",
              "path": "/{*namespace_id:0}/{project_id:1}(.{format})",
              "params": [
                "namespace_id",
                "project_id",
                "format"
              ],
              "constraints": {
                "namespace_id": "(?-mix:(?!((?i-mx:\\-|\\.well\\-known|404\\.html|422\\.html|500\\.html|502\\.html|503\\.html|admin|api|apple\\-touch\\-icon\\.png|assets|dashboard|deploy\\.html|explore|favicon\\.ico|favicon\\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\\.txt|s|search|sitemap|sitemap\\.xml|sitemap\\.xml\\.gz|slash\\-command\\-logo\\.png|snippets|unsubscribes|uploads|users|v2))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))(?:\\/(?!(?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis)\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))*",
                "project_id": "(?!((?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254})(?-mix:(?<!\\.git|\\.atom)))"
              },
              "methods": [
                "DELETE"
              ],
              "action": "application#route_not_found",
              "prefix": null
            }
          ],
          "constraints": {
            "(?!((?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254})(?-mix:(?<!\\.git|\\.atom)))": 1,
            "(?-mix:(?!((?i-mx:\\-|\\.well\\-known|404\\.html|422\\.html|500\\.html|502\\.html|503\\.html|admin|api|apple\\-touch\\-icon\\.png|assets|dashboard|deploy\\.html|explore|favicon\\.ico|favicon\\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\\.txt|s|search|sitemap|sitemap\\.xml|sitemap\\.xml\\.gz|slash\\-command\\-logo\\.png|snippets|unsubscribes|uploads|users|v2))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))(?:\\/(?!(?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis)\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))*": 0
          }
        }
        "#);
    }

    #[test]
    fn test_octal() {
        let input = r"
            --[ Route 1791 ]----------------------------------------------------------------
            Prefix            | logs_tree_namespace_project_ref
            Verb              | GET
            URI               | /*namespace_id/:project_id/-/refs/:id/logs_tree
            Controller#Action | projects/refs#logs_tree {:project_id=>/(?!((?i-mx:\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\/folders|files|find_file|gitlab\-lfs\/objects|info\/lfs\/objects|new|preview|raw|refs|tree|update|wikis))\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254})(?-mix:(?<!\.git|\.atom)))/, :namespace_id=>/(?-mix:(?!((?i-mx:\-|\.well\-known|404\.html|422\.html|500\.html|502\.html|503\.html|admin|api|apple\-touch\-icon\.png|assets|dashboard|deploy\.html|explore|favicon\.ico|favicon\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\.txt|s|search|sitemap|sitemap\.xml|sitemap\.xml\.gz|slash\-command\-logo\.png|snippets|unsubscribes|uploads|users|v2))\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254}[a-zA-Z0-9_\-]|[a-zA-Z0-9_])(?-mix:(?<!\.git|\.atom))))(?:\/(?!(?i-mx:\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\/folders|files|find_file|gitlab\-lfs\/objects|info\/lfs\/objects|new|preview|raw|refs|tree|update|wikis)\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254}[a-zA-Z0-9_\-]|[a-zA-Z0-9_])(?-mix:(?<!\.git|\.atom))))*/, :id=>/(?!\/|.*(?:[\/.]\.|\/\/|@\{|\\))[^\000-\040\177~^:?*\[]+(?<!\.lock)(?<![\/.])/}
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 1791,
              "path_original": "/*namespace_id/:project_id/-/refs/:id/logs_tree",
              "path": "/{*namespace_id:1}/{project_id:2}/-/refs/{id:0}/logs_tree",
              "params": [
                "namespace_id",
                "project_id",
                "id"
              ],
              "constraints": {
                "id": "(?!\\/|.*(?:[\\/.]\\.|\\/\\/|@\\{|\\\\))[^\\x00-\\x20\\x7f~^:?*\\[]+(?<!\\.lock)(?<![\\/.])",
                "namespace_id": "(?-mix:(?!((?i-mx:\\-|\\.well\\-known|404\\.html|422\\.html|500\\.html|502\\.html|503\\.html|admin|api|apple\\-touch\\-icon\\.png|assets|dashboard|deploy\\.html|explore|favicon\\.ico|favicon\\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\\.txt|s|search|sitemap|sitemap\\.xml|sitemap\\.xml\\.gz|slash\\-command\\-logo\\.png|snippets|unsubscribes|uploads|users|v2))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))(?:\\/(?!(?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis)\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))*",
                "project_id": "(?!((?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254})(?-mix:(?<!\\.git|\\.atom)))"
              },
              "methods": [
                "GET"
              ],
              "action": "projects/refs#logs_tree",
              "prefix": "logs_tree_namespace_project_ref"
            }
          ],
          "constraints": {
            "(?!((?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254})(?-mix:(?<!\\.git|\\.atom)))": 2,
            "(?!\\/|.*(?:[\\/.]\\.|\\/\\/|@\\{|\\\\))[^\\x00-\\x20\\x7f~^:?*\\[]+(?<!\\.lock)(?<![\\/.])": 0,
            "(?-mix:(?!((?i-mx:\\-|\\.well\\-known|404\\.html|422\\.html|500\\.html|502\\.html|503\\.html|admin|api|apple\\-touch\\-icon\\.png|assets|dashboard|deploy\\.html|explore|favicon\\.ico|favicon\\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\\.txt|s|search|sitemap|sitemap\\.xml|sitemap\\.xml\\.gz|slash\\-command\\-logo\\.png|snippets|unsubscribes|uploads|users|v2))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))(?:\\/(?!(?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis)\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))*": 1
          }
        }
        "#);
    }

    #[test]
    fn test_string_constraint() {
        let input = r#"
            --[ Route 2079 ]----------------------------------------------------------------
            Prefix            |
            Verb              | GET|POST
            URI               | /health_check(/:checks)(.:format)
            Controller#Action | health_check/health_check#index {:format=>"txt"}
        "#;

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 2079,
              "path_original": "/health_check(/:checks)(.:format)",
              "path": "/health_check(/{checks})(.{format:0})",
              "params": [
                "checks",
                "format"
              ],
              "constraints": {
                "format": "txt"
              },
              "methods": [
                "GET",
                "POST"
              ],
              "action": "health_check/health_check#index",
              "prefix": null
            }
          ],
          "constraints": {
            "txt": 0
          }
        }
        "#);
    }

    #[test]
    fn test_symbol_constraint() {
        let input = r"
            --[ Route 517 ]-----------------------------------------------------------------
            Prefix            | group_security_merge_commit_reports
            Verb              | GET
            URI               | /groups/*group_id/-/security/merge_commit_reports(.:format)
            Controller#Action | groups/security/merge_commit_reports#index {:format=>:csv}
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 517,
              "path_original": "/groups/*group_id/-/security/merge_commit_reports(.:format)",
              "path": "/groups/{*group_id}/-/security/merge_commit_reports(.{format:0})",
              "params": [
                "group_id",
                "format"
              ],
              "constraints": {
                "format": "csv"
              },
              "methods": [
                "GET"
              ],
              "action": "groups/security/merge_commit_reports#index",
              "prefix": "group_security_merge_commit_reports"
            }
          ],
          "constraints": {
            "csv": 0
          }
        }
        "#);
    }

    #[test]
    fn test_complex() {
        let input = r"
            --[ Route 1793 ]----------------------------------------------------------------
            Prefix            | namespace_project_network
            Verb              | GET
            URI               | /*namespace_id/:project_id/-/network/:id
            Controller#Action | projects/network#show {:project_id=>/(?!((?i-mx:\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\/folders|files|find_file|gitlab\-lfs\/objects|info\/lfs\/objects|new|preview|raw|refs|tree|update|wikis))\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254})(?-mix:(?<!\.git|\.atom)))/, :id=>/(?!\/|.*(?:[\/.]\.|\/\/|@\{|\\))[^\000-\040\177~^:?*\[]+(?<!\.lock)(?<![\/.])/, :namespace_id=>/(?-mix:(?!((?i-mx:\-|\.well\-known|404\.html|422\.html|500\.html|502\.html|503\.html|admin|api|apple\-touch\-icon\.png|assets|dashboard|deploy\.html|explore|favicon\.ico|favicon\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\.txt|s|search|sitemap|sitemap\.xml|sitemap\.xml\.gz|slash\-command\-logo\.png|snippets|unsubscribes|uploads|users|v2))\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254}[a-zA-Z0-9_\-]|[a-zA-Z0-9_])(?-mix:(?<!\.git|\.atom))))(?:\/(?!(?i-mx:\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\/folders|files|find_file|gitlab\-lfs\/objects|info\/lfs\/objects|new|preview|raw|refs|tree|update|wikis)\/)(?-mix:(?:[a-zA-Z0-9_\.][a-zA-Z0-9_\-\.]{0,254}[a-zA-Z0-9_\-]|[a-zA-Z0-9_])(?-mix:(?<!\.git|\.atom))))*/}
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 1793,
              "path_original": "/*namespace_id/:project_id/-/network/:id",
              "path": "/{*namespace_id:1}/{project_id:2}/-/network/{id:0}",
              "params": [
                "namespace_id",
                "project_id",
                "id"
              ],
              "constraints": {
                "id": "(?!\\/|.*(?:[\\/.]\\.|\\/\\/|@\\{|\\\\))[^\\x00-\\x20\\x7f~^:?*\\[]+(?<!\\.lock)(?<![\\/.])",
                "namespace_id": "(?-mix:(?!((?i-mx:\\-|\\.well\\-known|404\\.html|422\\.html|500\\.html|502\\.html|503\\.html|admin|api|apple\\-touch\\-icon\\.png|assets|dashboard|deploy\\.html|explore|favicon\\.ico|favicon\\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\\.txt|s|search|sitemap|sitemap\\.xml|sitemap\\.xml\\.gz|slash\\-command\\-logo\\.png|snippets|unsubscribes|uploads|users|v2))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))(?:\\/(?!(?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis)\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))*",
                "project_id": "(?!((?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254})(?-mix:(?<!\\.git|\\.atom)))"
              },
              "methods": [
                "GET"
              ],
              "action": "projects/network#show",
              "prefix": "namespace_project_network"
            }
          ],
          "constraints": {
            "(?!((?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254})(?-mix:(?<!\\.git|\\.atom)))": 2,
            "(?!\\/|.*(?:[\\/.]\\.|\\/\\/|@\\{|\\\\))[^\\x00-\\x20\\x7f~^:?*\\[]+(?<!\\.lock)(?<![\\/.])": 0,
            "(?-mix:(?!((?i-mx:\\-|\\.well\\-known|404\\.html|422\\.html|500\\.html|502\\.html|503\\.html|admin|api|apple\\-touch\\-icon\\.png|assets|dashboard|deploy\\.html|explore|favicon\\.ico|favicon\\.png|files|groups|health_check|help|import|jwt|login|oauth|profile|projects|public|robots\\.txt|s|search|sitemap|sitemap\\.xml|sitemap\\.xml\\.gz|slash\\-command\\-logo\\.png|snippets|unsubscribes|uploads|users|v2))\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))(?:\\/(?!(?i-mx:\\-|badges|blame|blob|builds|commits|create|create_dir|edit|environments\\/folders|files|find_file|gitlab\\-lfs\\/objects|info\\/lfs\\/objects|new|preview|raw|refs|tree|update|wikis)\\/)(?-mix:(?:[a-zA-Z0-9_\\.][a-zA-Z0-9_\\-\\.]{0,254}[a-zA-Z0-9_\\-]|[a-zA-Z0-9_])(?-mix:(?<!\\.git|\\.atom))))*": 1
          }
        }
        "#);
    }

    #[test]
    fn test_duplicate() {
        let input = r"
            --[ Route 249 ]-----------------------------------------------------------------
            Prefix            | profile_usage_quotas
            Verb              | GET
            URI               | /-/profile/usage_quotas(.:format)
            Controller#Action | profiles/usage_quotas#index
            --[ Route 292 ]-----------------------------------------------------------------
            Prefix            |
            Verb              | GET
            URI               | /-/profile/usage_quotas(.:format)
            Controller#Action | profiles/usage_quotas#index
        ";

        let routes = parse_router(input);
        assert_json_snapshot!(routes, @r#"
        {
          "routes": [
            {
              "section": null,
              "id": 249,
              "path_original": "/-/profile/usage_quotas(.:format)",
              "path": "/-/profile/usage_quotas(.{format})",
              "params": [
                "format"
              ],
              "constraints": {},
              "methods": [
                "GET"
              ],
              "action": "profiles/usage_quotas#index",
              "prefix": "profile_usage_quotas"
            }
          ],
          "constraints": {}
        }
        "#);
    }
}

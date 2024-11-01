use std::error::Error;
use wayfind::Router;

#[path = "../benches/gitlab_routes.rs"]
pub mod gitlab_routes;

#[test]
fn test_display_multibyte() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert("/👨‍👩‍👧", 1)?; // Family: Man, Woman, Girl
    insta::assert_snapshot!(router, @"/👨‍👩‍👧");

    router.insert("/👨‍👩‍👦", 1)?; // Family: Man, Woman, Boy
    insta::assert_snapshot!(router, @r#"
    /👨‍👩‍�
    ├─ � [*]
    ╰─ � [*]
    "#);

    router.insert("/👩‍👩‍👧", 1)?; // Family: Woman, Woman, Girl
    insta::assert_snapshot!(router, @r#"
    /�
    ├─ �‍👩‍👧 [*]
    ╰─ �‍👩‍�
       ├─ � [*]
       ╰─ � [*]
    "#);

    router.insert("/👩‍👩‍👦", 1)?; // Family: Woman, Woman, Boy
    insta::assert_snapshot!(router, @r#"
    /�
    ├─ �‍👩‍�
    │  ├─ � [*]
    │  ╰─ � [*]
    ╰─ �‍👩‍�
       ├─ � [*]
       ╰─ � [*]
    "#);

    router.insert("/👨‍👨‍👧", 1)?; // Family: Man, Man, Girl
    insta::assert_snapshot!(router, @r#"
    /�
    ├─ �‍👩‍�
    │  ├─ � [*]
    │  ╰─ � [*]
    ╰─ �‍�
       ├─ �‍👧 [*]
       ╰─ �‍�
          ├─ � [*]
          ╰─ � [*]
    "#);

    router.insert("/👨‍👨‍👦", 1)?; // Family: Man, Man, Boy
    insta::assert_snapshot!(router, @r#"
    /�
    ├─ �‍👩‍�
    │  ├─ � [*]
    │  ╰─ � [*]
    ╰─ �‍�
       ├─ �‍�
       │  ├─ � [*]
       │  ╰─ � [*]
       ╰─ �‍�
          ├─ � [*]
          ╰─ � [*]
    "#);

    Ok(())
}

#[test]
fn test_display_gitlab() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    for route in gitlab_routes::routes() {
        router.insert(route, true)?;
    }

    insta::assert_snapshot!(router, @r#"
    /
    ├─ dashboard [*]
    │  ╰─ /
    │     ├─ todos [*]
    │     │  ╰─ /
    │     │     ├─ bulk_restore [*]
    │     │     ├─ destroy_all [*]
    │     │     ├─ vue [*]
    │     │     ╰─ {id} [*]
    │     │        ╰─ /restore [*]
    │     ├─ activity [*]
    │     ├─ projects [*]
    │     │  ╰─ /
    │     │     ├─ contributed [*]
    │     │     ├─ personal [*]
    │     │     ├─ removed [*]
    │     │     ├─ starred [*]
    │     │     ╰─ member [*]
    │     ├─ snippets [*]
    │     ├─ groups [*]
    │     ├─ issues [*]
    │     ├─ labels [*]
    │     ╰─ m
    │        ├─ erge_requests [*]
    │        │  ╰─ /search [*]
    │        ╰─ ilestones [*]
    ├─ jwt/auth [*]
    ├─ explore [*]
    │  ╰─ /
    │     ├─ dependencies [*]
    │     ├─ projects [*]
    │     │  ╰─ /
    │     │     ├─ t
    │     │     │  ├─ rending [*]
    │     │     │  ╰─ opics [*]
    │     │     │     ╰─ /
    │     │     │        ╰─ {topic_name}
    │     │     │           ╰─ (.
    │     │     │              ╰─ {format}
    │     │     │                 ╰─ ) [*]
    │     │     ╰─ starred [*]
    │     ├─ snippets [*]
    │     ├─ catalog [*]
    │     │  ╰─ /
    │     │     ╰─ {*full_path} [*]
    │     ╰─ groups [*]
    ├─ groups [*]
    │  ╰─ /
    │     ├─ new [*]
    │     ├─ {*group_id}
    │     │  ╰─ /-/
    │     │     ├─ m
    │     │     │  ├─ erge_requests/bulk_update [*]
    │     │     │  ╰─ ilestones [*]
    │     │     │     ╰─ /
    │     │     │        ├─ new [*]
    │     │     │        ╰─ {id} [*]
    │     │     │           ╰─ /
    │     │     │              ├─ merge_requests [*]
    │     │     │              ├─ participants [*]
    │     │     │              ├─ issues [*]
    │     │     │              ├─ labels [*]
    │     │     │              ╰─ edit [*]
    │     │     ├─ notification_setting [*]
    │     │     ├─ u
    │     │     │  ├─ sage_quotas [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ pending_members [*]
    │     │     │  │     ╰─ subscription_history(.
    │     │     │  │        ╰─ {format}
    │     │     │  │           ╰─ ) [*]
    │     │     │  ╰─ ploads [*]
    │     │     │     ╰─ /
    │     │     │        ├─ authorize [*]
    │     │     │        ╰─ {secret}
    │     │     │           ╰─ /
    │     │     │              ╰─ {filename}
    │     │     │                 ╰─ (.
    │     │     │                    ╰─ {format}
    │     │     │                       ╰─ ) [*]
    │     │     ├─ variables [*]
    │     │     ├─ b
    │     │     │  ├─ illings [*]
    │     │     │  │  ╰─ /refresh_seats [*]
    │     │     │  ╰─ oards [*]
    │     │     │     ╰─ /
    │     │     │        ╰─ {id} [*]
    │     │     ├─ h
    │     │     │  ├─ ooks [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ {hook_id}
    │     │     │  │     │  ╰─ /hook_logs/
    │     │     │  │     │     ╰─ {id} [*]
    │     │     │  │     │        ╰─ /retry [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ edit [*]
    │     │     │  │           ╰─ test [*]
    │     │     │  ╰─ arbor/repositories [*]
    │     │     │     ╰─ /
    │     │     │        ├─ {id} [*]
    │     │     │        ╰─ {repository_id}
    │     │     │           ╰─ /artifacts [*]
    │     │     │              ╰─ /
    │     │     │                 ╰─ {artifact_id}
    │     │     │                    ╰─ /tags [*]
    │     │     ├─ group_
    │     │     │  ├─ members [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ bulk_reassignment_file [*]
    │     │     │  │     ├─ request_access [*]
    │     │     │  │     ├─ export_csv [*]
    │     │     │  │     ├─ leave [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ approve_access_request [*]
    │     │     │  │           ├─ resend_invite [*]
    │     │     │  │           ├─ override [*]
    │     │     │  │           ├─ unban [*]
    │     │     │  │           ╰─ ban [*]
    │     │     │  ╰─ links/
    │     │     │     ╰─ {id} [*]
    │     │     ├─ epic
    │     │     │  ├─ _boards [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ {id} [*]
    │     │     │  ╰─ s [*]
    │     │     │     ╰─ /
    │     │     │        ├─ bulk_update [*]
    │     │     │        ├─ new [*]
    │     │     │        ├─ {epic_id}
    │     │     │        │  ╰─ /
    │     │     │        │     ├─ related_epic_links [*]
    │     │     │        │     │  ╰─ /
    │     │     │        │     │     ╰─ {id} [*]
    │     │     │        │     ├─ issues [*]
    │     │     │        │     │  ╰─ /
    │     │     │        │     │     ╰─ {id} [*]
    │     │     │        │     ├─ links [*]
    │     │     │        │     │  ╰─ /
    │     │     │        │     │     ╰─ {id} [*]
    │     │     │        │     ╰─ notes [*]
    │     │     │        │        ╰─ /
    │     │     │        │           ╰─ {id} [*]
    │     │     │        │              ╰─ /toggle_award_emoji [*]
    │     │     │        ╰─ {id} [*]
    │     │     │           ╰─ /
    │     │     │              ├─ realtime_changes [*]
    │     │     │              ├─ edit [*]
    │     │     │              ├─ toggle_
    │     │     │              │  ├─ subscription [*]
    │     │     │              │  ╰─ award_emoji [*]
    │     │     │              ╰─ d
    │     │     │                 ├─ iscussions [*]
    │     │     │                 ╰─ escriptions/
    │     │     │                    ╰─ {version_id} [*]
    │     │     │                       ╰─ /diff [*]
    │     │     ├─ l
    │     │     │  ├─ abels [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ toggle_subscription [*]
    │     │     │  │           ╰─ edit [*]
    │     │     │  ╰─ dap
    │     │     │     ├─ /sync [*]
    │     │     │     ╰─ _group_links [*]
    │     │     │        ╰─ /
    │     │     │           ╰─ {id} [*]
    │     │     ├─ a
    │     │     │  ├─ dd_ons/discover_duo_pro [*]
    │     │     │  ├─ chievements [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new [*]
    │     │     │  │     ╰─ {id}
    │     │     │  │        ╰─ /edit [*]
    │     │     │  ├─ nalytics [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ type_of_work/tasks_by_type [*]
    │     │     │  │     │  ╰─ /top_labels [*]
    │     │     │  │     ├─ merge_request_analytics [*]
    │     │     │  │     ├─ productivity_analytics [*]
    │     │     │  │     ├─ value_stream_analytics [*]
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ value_streams [*]
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     │     ├─ new [*]
    │     │     │  │     │     │     ├─ {value_stream_id}
    │     │     │  │     │     │     │  ╰─ /stages [*]
    │     │     │  │     │     │     │     ╰─ /
    │     │     │  │     │     │     │        ╰─ {id}
    │     │     │  │     │     │     │           ╰─ /
    │     │     │  │     │     │     │              ├─ average [*]
    │     │     │  │     │     │     │              │  ╰─ _duration_chart [*]
    │     │     │  │     │     │     │              ├─ records [*]
    │     │     │  │     │     │     │              ├─ median [*]
    │     │     │  │     │     │     │              ╰─ count [*]
    │     │     │  │     │     │     ╰─ {id} [*]
    │     │     │  │     │     │        ╰─ /edit [*]
    │     │     │  │     │     ├─ time_summary [*]
    │     │     │  │     │     ├─ cycle_times [*]
    │     │     │  │     │     ├─ lead_times [*]
    │     │     │  │     │     ╰─ summary [*]
    │     │     │  │     ├─ repository_analytics [*]
    │     │     │  │     ├─ c
    │     │     │  │     │  ├─ overage_reports [*]
    │     │     │  │     │  ├─ ycle_analytics [*]
    │     │     │  │     │  ╰─ i_cd [*]
    │     │     │  │     ╰─ d
    │     │     │  │        ├─ evops_adoption [*]
    │     │     │  │        ╰─ ashboards(/
    │     │     │  │           ╰─ {*vueroute}
    │     │     │  │              ╰─ ) [*]
    │     │     │  ├─ vatar [*]
    │     │     │  ╰─ u
    │     │     │     ├─ tocomplete_sources/
    │     │     │     │  ├─ vulnerabilities [*]
    │     │     │     │  ├─ commands [*]
    │     │     │     │  ├─ labels [*]
    │     │     │     │  ├─ epics [*]
    │     │     │     │  ├─ wikis [*]
    │     │     │     │  ├─ m
    │     │     │     │  │  ├─ ilestones [*]
    │     │     │     │  │  ╰─ e
    │     │     │     │  │     ├─ rge_requests [*]
    │     │     │     │  │     ╰─ mbers [*]
    │     │     │     │  ╰─ i
    │     │     │     │     ├─ terations [*]
    │     │     │     │     ╰─ ssues [*]
    │     │     │     ╰─ dit_events [*]
    │     │     ├─ c
    │     │     │  ├─ rm/
    │     │     │  │  ├─ organizations [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ╰─ {id}
    │     │     │  │  │        ╰─ /edit [*]
    │     │     │  │  ╰─ contacts [*]
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ new [*]
    │     │     │  │        ╰─ {id}
    │     │     │  │           ╰─ /edit [*]
    │     │     │  ├─ ustom_emoji [*]
    │     │     │  │  ╰─ /new [*]
    │     │     │  ├─ hildren [*]
    │     │     │  ├─ lusters [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new_cluster_docs [*]
    │     │     │  │     ├─ c
    │     │     │  │     │  ├─ reate_user [*]
    │     │     │  │     │  ╰─ onnect [*]
    │     │     │  │     ├─ {cluster_id}
    │     │     │  │     │  ╰─ /integration/create_or_update [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ cl
    │     │     │  │           │  ├─ uster_status [*]
    │     │     │  │           │  ╰─ ear_cache [*]
    │     │     │  │           ├─ environments [*]
    │     │     │  │           ╰─ metrics [*]
    │     │     │  │              ╰─ _dashboard [*]
    │     │     │  ├─ adences(/
    │     │     │  │  ╰─ {*vueroute}
    │     │     │  │     ╰─ ) [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ new [*]
    │     │     │  │           ├─ {iteration_cadence_id}
    │     │     │  │           │  ╰─ /iterations [*]
    │     │     │  │           │     ╰─ /
    │     │     │  │           │        ├─ new [*]
    │     │     │  │           │        ╰─ {id} [*]
    │     │     │  │           │           ╰─ /edit [*]
    │     │     │  │           ╰─ {id} [*]
    │     │     │  │              ╰─ /edit [*]
    │     │     │  ╰─ o
    │     │     │     ├─ nt
    │     │     │     │  ├─ ribution_analytics [*]
    │     │     │     │  ╰─ ainer_registries [*]
    │     │     │     │     ╰─ /
    │     │     │     │        ╰─ {id} [*]
    │     │     │     ╰─ mment_templates [*]
    │     │     │        ╰─ /
    │     │     │           ╰─ {id} [*]
    │     │     ├─ d
    │     │     │  ├─ ep
    │     │     │  │  ├─ endenc
    │     │     │  │  │  ├─ y_proxy [*]
    │     │     │  │  │  ╰─ ies [*]
    │     │     │  │  │     ╰─ /l
    │     │     │  │  │        ├─ ocations [*]
    │     │     │  │  │        ╰─ icenses [*]
    │     │     │  │  ╰─ loy_tokens/
    │     │     │  │     ╰─ {id}
    │     │     │  │        ╰─ /revoke [*]
    │     │     │  ╰─ iscover [*]
    │     │     ├─ i
    │     │     │  ├─ terations [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  │        ╰─ /edit [*]
    │     │     │  ├─ mport [*]
    │     │     │  ├─ ssues
    │     │     │  │  ├─ /bulk_update [*]
    │     │     │  │  ╰─ _analytics [*]
    │     │     │  ╰─ n
    │     │     │     ├─ frastructure_registry [*]
    │     │     │     ╰─ sights [*]
    │     │     │        ╰─ /query [*]
    │     │     ├─ p
    │     │     │  ├─ ush_rules [*]
    │     │     │  ├─ ackages [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ {id} [*]
    │     │     │  ╰─ r
    │     │     │     ├─ eview_markdown [*]
    │     │     │     ╰─ otected_
    │     │     │        ├─ environments [*]
    │     │     │        │  ╰─ /
    │     │     │        │     ╰─ {id} [*]
    │     │     │        ╰─ branches [*]
    │     │     │           ╰─ /
    │     │     │              ╰─ {id} [*]
    │     │     ├─ r
    │     │     │  ├─ unners [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ dashboard [*]
    │     │     │  │     ├─ new [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ pause [*]
    │     │     │  │           ├─ edit [*]
    │     │     │  │           ╰─ re
    │     │     │  │              ├─ gister [*]
    │     │     │  │              ╰─ sume [*]
    │     │     │  ├─ e
    │     │     │  │  ├─ leases [*]
    │     │     │  │  ╰─ store [*]
    │     │     │  ╰─ oadmap [*]
    │     │     ├─ s
    │     │     │  ├─ hared_projects [*]
    │     │     │  ├─ cim_oauth [*]
    │     │     │  ├─ aml [*]
    │     │     │  │  ├─ /
    │     │     │  │  │  ├─ callback [*]
    │     │     │  │  │  ├─ u
    │     │     │  │  │  │  ├─ pdate_microsoft_application [*]
    │     │     │  │  │  │  ╰─ nlink [*]
    │     │     │  │  │  ╰─ sso [*]
    │     │     │  │  ╰─ _group_links [*]
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ {id} [*]
    │     │     │  ╰─ e
    │     │     │     ├─ at_usage [*]
    │     │     │     ├─ rvice_accounts(/
    │     │     │     │  ╰─ {*vueroute}
    │     │     │     │     ╰─ ) [*]
    │     │     │     │        ╰─ /
    │     │     │     │           ├─ new [*]
    │     │     │     │           ╰─ {id} [*]
    │     │     │     │              ╰─ /edit [*]
    │     │     │     ├─ curity/
    │     │     │     │  ├─ vulnerabilities [*]
    │     │     │     │  ├─ policies [*]
    │     │     │     │  │  ╰─ /
    │     │     │     │  │     ├─ schema [*]
    │     │     │     │  │     ├─ new [*]
    │     │     │     │  │     ╰─ {id}
    │     │     │     │  │        ╰─ /edit [*]
    │     │     │     │  ├─ merge_commit_reports(.
    │     │     │     │  │  ╰─ {format}
    │     │     │     │  │     ╰─ ) [*]
    │     │     │     │  ├─ c
    │     │     │     │  │  ├─ ompliance_
    │     │     │     │  │  │  ├─ standards_adherence_reports(.
    │     │     │     │  │  │  │  ╰─ {format}
    │     │     │     │  │  │  │     ╰─ ) [*]
    │     │     │     │  │  │  ├─ project_framework_reports(.
    │     │     │     │  │  │  │  ╰─ {format}
    │     │     │     │  │  │  │     ╰─ ) [*]
    │     │     │     │  │  │  ├─ framework_reports(.
    │     │     │     │  │  │  │  ╰─ {format}
    │     │     │     │  │  │  │     ╰─ ) [*]
    │     │     │     │  │  │  ├─ violation_reports(.
    │     │     │     │  │  │  │  ╰─ {format}
    │     │     │     │  │  │  │     ╰─ ) [*]
    │     │     │     │  │  │  ╰─ dashboard(/
    │     │     │     │  │  │     ╰─ {*vueroute}
    │     │     │     │  │  │        ╰─ ) [*]
    │     │     │     │  │  ╰─ redentials [*]
    │     │     │     │  │     ╰─ /
    │     │     │     │  │        ╰─ {id} [*]
    │     │     │     │  │           ╰─ /revoke [*]
    │     │     │     │  ╰─ d
    │     │     │     │     ├─ ashboard [*]
    │     │     │     │     ╰─ iscover [*]
    │     │     │     ╰─ ttings/
    │     │     │        ├─ packages_and_registries [*]
    │     │     │        ├─ domain_verification [*]
    │     │     │        │  ╰─ /
    │     │     │        │     ├─ new [*]
    │     │     │        │     ╰─ {id} [*]
    │     │     │        │        ╰─ /
    │     │     │        │           ├─ clean_certificate [*]
    │     │     │        │           ├─ retry_auto_ssl [*]
    │     │     │        │           ╰─ verify [*]
    │     │     │        ├─ gitlab_duo_usage [*]
    │     │     │        ├─ merge_requests [*]
    │     │     │        ├─ integrations [*]
    │     │     │        │  ╰─ /
    │     │     │        │     ╰─ {id} [*]
    │     │     │        │        ╰─ /
    │     │     │        │           ├─ reset [*]
    │     │     │        │           ├─ edit [*]
    │     │     │        │           ╰─ test [*]
    │     │     │        ├─ workspaces [*]
    │     │     │        ├─ ci_cd [*]
    │     │     │        │  ╰─ /
    │     │     │        │     ├─ deploy_token/create [*]
    │     │     │        │     ├─ r
    │     │     │        │     │  ├─ eset_registration_token [*]
    │     │     │        │     │  ╰─ unner_setup_scripts [*]
    │     │     │        │     ╰─ update_auto_devops [*]
    │     │     │        ├─ slack [*]
    │     │     │        │  ╰─ /slack_auth [*]
    │     │     │        ├─ a
    │     │     │        │  ├─ ccess_tokens [*]
    │     │     │        │  │  ╰─ /
    │     │     │        │  │     ╰─ {id}
    │     │     │        │  │        ╰─ /revoke [*]
    │     │     │        │  ├─ nalytics [*]
    │     │     │        │  ╰─ pplications [*]
    │     │     │        │     ╰─ /
    │     │     │        │        ├─ new [*]
    │     │     │        │        ╰─ {id} [*]
    │     │     │        │           ╰─ /
    │     │     │        │              ├─ renew [*]
    │     │     │        │              ╰─ edit [*]
    │     │     │        ╰─ r
    │     │     │           ├─ oles_and_permissions [*]
    │     │     │           │  ╰─ /
    │     │     │           │     ├─ new [*]
    │     │     │           │     ╰─ {id} [*]
    │     │     │           │        ╰─ /edit [*]
    │     │     │           ╰─ epo
    │     │     │              ├─ sitory [*]
    │     │     │              │  ╰─ /deploy_token/create [*]
    │     │     │              ╰─ rting [*]
    │     │     ├─ t
    │     │     │  ├─ erraform_module_registry [*]
    │     │     │  ├─ wo_factor_auth [*]
    │     │     │  ╰─ odos [*]
    │     │     ╰─ w
    │     │        ├─ ikis [*]
    │     │        │  ╰─ /
    │     │        │     ├─ -/confluence [*]
    │     │        │     ├─ git_access [*]
    │     │        │     ├─ templates [*]
    │     │        │     ├─ pages [*]
    │     │        │     ├─ new [*]
    │     │        │     ├─ {*id}
    │     │        │     │  ╰─ /
    │     │        │     │     ├─ preview_markdown [*]
    │     │        │     │     ├─ history [*]
    │     │        │     │     ├─ diff [*]
    │     │        │     │     ├─ edit [*]
    │     │        │     │     ╰─ raw [*]
    │     │        │     ╰─ {*id} [*]
    │     │        ╰─ ork_items [*]
    │     │           ╰─ /
    │     │              ╰─ {iid} [*]
    │     │                 ╰─ /descriptions/
    │     │                    ╰─ {version_id} [*]
    │     │                       ╰─ /diff [*]
    │     ╰─ {*id}
    │        ├─ /-/
    │        │  ├─ unfoldered_environment_names(.
    │        │  │  ╰─ {format}
    │        │  │     ╰─ ) [*]
    │        │  ├─ merge_requests(.
    │        │  │  ╰─ {format}
    │        │  │     ╰─ ) [*]
    │        │  ├─ a
    │        │  │  ├─ ctivity(.
    │        │  │  │  ╰─ {format}
    │        │  │  │     ╰─ ) [*]
    │        │  │  ╰─ rchived(.
    │        │  │     ╰─ {format}
    │        │  │        ╰─ ) [*]
    │        │  ├─ projects(.
    │        │  │  ╰─ {format}
    │        │  │     ╰─ ) [*]
    │        │  ├─ transfer(.
    │        │  │  ╰─ {format}
    │        │  │     ╰─ ) [*]
    │        │  ├─ i
    │        │  │  ├─ nactive(.
    │        │  │  │  ╰─ {format}
    │        │  │  │     ╰─ ) [*]
    │        │  │  ╰─ ssues(.
    │        │  │     ╰─ {format}
    │        │  │        ╰─ ) [*]
    │        │  ├─ shared(.
    │        │  │  ╰─ {format}
    │        │  │     ╰─ ) [*]
    │        │  ├─ d
    │        │  │  ├─ ownload_export(.
    │        │  │  │  ╰─ {format}
    │        │  │  │     ╰─ ) [*]
    │        │  │  ╰─ etails(.
    │        │  │     ╰─ {format}
    │        │  │        ╰─ ) [*]
    │        │  ╰─ e
    │        │     ├─ xport(.
    │        │     │  ╰─ {format}
    │        │     │     ╰─ ) [*]
    │        │     ╰─ dit(.
    │        │        ╰─ {format}
    │        │           ╰─ ) [*]
    │        ╰─ (.
    │           ╰─ {format}
    │              ╰─ ) [*]
    ├─ s
    │  ├─ itemap [*]
    │  ├─ earch [*]
    │  │  ╰─ /
    │  │     ├─ a
    │  │     │  ├─ ggregations [*]
    │  │     │  ╰─ utocomplete [*]
    │  │     ├─ opensearch [*]
    │  │     ├─ settings [*]
    │  │     ╰─ count [*]
    │  ╰─ nippets
    │     ├─ (/
    │     │  ╰─ {*rest}
    │     │     ╰─ ) [*]
    │     ╰─ /
    │        ╰─ {id}
    │           ╰─ /raw [*]
    ├─ he
    │  ├─ lp [*]
    │  │  ╰─ /
    │  │     ├─ instance_configuration [*]
    │  │     ├─ shortcuts [*]
    │  │     ├─ d
    │  │     │  ├─ ocs [*]
    │  │     │  ╰─ rawers/
    │  │     │     ╰─ {*markdown_file} [*]
    │  │     ╰─ {*path} [*]
    │  ╰─ alth_check(/
    │     ╰─ {checks}
    │        ╰─ )(.
    │           ╰─ {format}
    │              ╰─ ) [*]
    ├─ v2 [*]
    │  ╰─ /
    │     ╰─ {*group_id}
    │        ╰─ /dependency_proxy/containers/
    │           ╰─ {*image}
    │              ╰─ /
    │                 ├─ manifests/
    │                 │  ├─ {*tag}
    │                 │  │  ╰─ /upload [*]
    │                 │  │     ╰─ /authorize [*]
    │                 │  ╰─ {*tag} [*]
    │                 ╰─ blobs/
    │                    ╰─ {sha} [*]
    │                       ╰─ /upload [*]
    │                          ╰─ /authorize [*]
    ├─ .well-known/
    │  ├─ change-password [*]
    │  ├─ terraform.json [*]
    │  ├─ security.txt [*]
    │  ├─ webfinger [*]
    │  ╰─ o
    │     ├─ auth-authorization-server [*]
    │     ╰─ penid-configuration [*]
    ├─ import/
    │  ├─ url/validate [*]
    │  ├─ manifest [*]
    │  │  ╰─ /
    │  │     ├─ realtime_changes [*]
    │  │     ├─ status [*]
    │  │     ├─ upload [*]
    │  │     ╰─ new [*]
    │  ├─ fogbugz [*]
    │  │  ╰─ /
    │  │     ├─ realtime_changes [*]
    │  │     ├─ callback [*]
    │  │     ├─ user_map [*]
    │  │     ├─ status [*]
    │  │     ╰─ new [*]
    │  ├─ history [*]
    │  ├─ source_users/
    │  │  ╰─ {id} [*]
    │  │     ╰─ /
    │  │        ├─ decline [*]
    │  │        ╰─ accept [*]
    │  ├─ git
    │  │  ├─ lab_
    │  │  │  ├─ group [*]
    │  │  │  │  ╰─ /authorize [*]
    │  │  │  ╰─ project [*]
    │  │  │     ╰─ /
    │  │  │        ├─ authorize [*]
    │  │  │        ╰─ new [*]
    │  │  ├─ hub [*]
    │  │  │  ├─ _group/status [*]
    │  │  │  ╰─ /
    │  │  │     ├─ personal_access_token [*]
    │  │  │     ├─ realtime_changes [*]
    │  │  │     ├─ failures [*]
    │  │  │     ├─ details [*]
    │  │  │     ├─ status [*]
    │  │  │     ├─ new [*]
    │  │  │     ╰─ c
    │  │  │        ├─ ounts [*]
    │  │  │        ╰─ a
    │  │  │           ├─ llback [*]
    │  │  │           ╰─ ncel [*]
    │  │  │              ╰─ _all [*]
    │  │  ╰─ ea [*]
    │  │     ╰─ /
    │  │        ├─ personal_access_token [*]
    │  │        ├─ realtime_changes [*]
    │  │        ├─ status [*]
    │  │        ╰─ new [*]
    │  ╰─ b
    │     ├─ ulk_imports [*]
    │     │  ╰─ /
    │     │     ├─ realtime_changes [*]
    │     │     ├─ configure [*]
    │     │     ├─ history [*]
    │     │     ├─ status [*]
    │     │     ╰─ {id}
    │     │        ╰─ /history [*]
    │     │           ╰─ /
    │     │              ╰─ {entity_id}
    │     │                 ╰─ /failures [*]
    │     ╰─ itbucket [*]
    │        ├─ _server [*]
    │        │  ╰─ /
    │        │     ├─ realtime_changes [*]
    │        │     ├─ status [*]
    │        │     ├─ new [*]
    │        │     ╰─ c
    │        │        ├─ onfigure [*]
    │        │        ╰─ allback [*]
    │        ╰─ /
    │           ├─ realtime_changes [*]
    │           ├─ callback [*]
    │           ╰─ status [*]
    ├─ oauth/
    │  ├─ d
    │  │  ├─ iscovery/keys [*]
    │  │  ╰─ evice [*]
    │  │     ╰─ /confirm [*]
    │  ├─ introspect [*]
    │  ├─ userinfo [*]
    │  ├─ revoke [*]
    │  ├─ token [*]
    │  │  ╰─ /info [*]
    │  ├─ geo/
    │  │  ├─ callback [*]
    │  │  ├─ logout [*]
    │  │  ╰─ auth [*]
    │  ╰─ a
    │     ├─ pplications [*]
    │     │  ╰─ /
    │     │     ├─ new [*]
    │     │     ╰─ {id} [*]
    │     │        ╰─ /
    │     │           ├─ renew [*]
    │     │           ╰─ edit [*]
    │     ╰─ uthorize [*]
    │        ├─ /native [*]
    │        ├─ d_applications [*]
    │        │  ╰─ /
    │        │     ╰─ {id} [*]
    │        ╰─ _device [*]
    ├─ rails/
    │  ├─ features [*]
    │  │  ╰─ /
    │  │     ├─ definitions [*]
    │  │     ╰─ {id} [*]
    │  ├─ mailers [*]
    │  │  ╰─ /
    │  │     ╰─ {path} [*]
    │  ├─ info [*]
    │  │  ╰─ /
    │  │     ├─ properties [*]
    │  │     ╰─ routes [*]
    │  ╰─ l
    │     ├─ ookbook [*]
    │     │  ╰─ /
    │     │     ├─ cable [*]
    │     │     ├─ embed [*]
    │     │     │  ╰─ /
    │     │     │     ╰─ {*path} [*]
    │     │     ├─ inspect/
    │     │     │  ╰─ {*path} [*]
    │     │     ├─ p
    │     │     │  ├─ review
    │     │     │  │  ├─ s [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ {*path} [*]
    │     │     │  ╰─ ages [*]
    │     │     │     ╰─ /
    │     │     │        ╰─ {*path} [*]
    │     │     ╰─ {*path} [*]
    │     ╰─ etter_opener [*]
    │        ╰─ /
    │           ├─ clear [*]
    │           ╰─ {id}
    │              ├─ /
    │              │  ├─ delete [*]
    │              │  ╰─ attachments/
    │              │     ╰─ {file} [*]
    │              ╰─ (/
    │                 ╰─ {style}
    │                    ╰─ ) [*]
    ├─ -/
    │  ├─ g
    │  │  ├─ oogle_api/auth/callback [*]
    │  │  ╰─ raphql-explorer [*]
    │  ├─ ex
    │  │  ├─ ternal_redirect [*]
    │  │  ╰─ periment/
    │  │     ╰─ {id} [*]
    │  ├─ kubernetes [*]
    │  │  ╰─ /
    │  │     ╰─ {agent_id}
    │  │        ╰─ (/
    │  │           ╰─ {*vueroute}
    │  │              ╰─ ) [*]
    │  ├─ whats_new [*]
    │  ├─ liveness [*]
    │  ├─ user
    │  │  ├─ _settings/
    │  │  │  ├─ identities [*]
    │  │  │  │  ╰─ /new [*]
    │  │  │  ├─ gpg_keys [*]
    │  │  │  │  ╰─ /
    │  │  │  │     ╰─ {id} [*]
    │  │  │  │        ╰─ /revoke [*]
    │  │  │  ├─ ssh_keys [*]
    │  │  │  │  ╰─ /
    │  │  │  │     ╰─ {id} [*]
    │  │  │  │        ╰─ /revoke [*]
    │  │  │  ├─ a
    │  │  │  │  ├─ ctive_sessions [*]
    │  │  │  │  │  ╰─ /
    │  │  │  │  │     ├─ saml [*]
    │  │  │  │  │     ╰─ {id} [*]
    │  │  │  │  ├─ uthentication_log [*]
    │  │  │  │  ╰─ pplications [*]
    │  │  │  ╰─ p
    │  │  │     ├─ assword [*]
    │  │  │     │  ╰─ /
    │  │  │     │     ├─ reset [*]
    │  │  │     │     ├─ edit [*]
    │  │  │     │     ╰─ new [*]
    │  │  │     ├─ rofile [*]
    │  │  │     ╰─ ersonal_access_tokens [*]
    │  │  │        ╰─ /
    │  │  │           ╰─ {id}
    │  │  │              ╰─ /revoke [*]
    │  │  ╰─ s/
    │  │     ├─ broadcast_message_dismissals [*]
    │  │     ├─ p
    │  │     │  ├─ roject_callouts [*]
    │  │     │  ╰─ ins [*]
    │  │     ├─ group_callouts [*]
    │  │     ├─ callouts [*]
    │  │     ╰─ terms [*]
    │  │        ╰─ /
    │  │           ╰─ {id}
    │  │              ╰─ /
    │  │                 ├─ decline [*]
    │  │                 ╰─ accept [*]
    │  ├─ a
    │  │  ├─ buse_reports [*]
    │  │  │  ╰─ /add_category [*]
    │  │  ├─ cme-challenge [*]
    │  │  ╰─ utocomplete/
    │  │     ├─ deploy_keys_with_owners [*]
    │  │     ├─ namespace_routes [*]
    │  │     ├─ group_subgroups [*]
    │  │     ├─ award_emojis [*]
    │  │     ├─ users [*]
    │  │     │  ╰─ /
    │  │     │     ╰─ {id} [*]
    │  │     ├─ merge_request_
    │  │     │  ├─ source_branches [*]
    │  │     │  ╰─ target_branches [*]
    │  │     ╰─ project
    │  │        ├─ _
    │  │        │  ├─ groups [*]
    │  │        │  ╰─ routes [*]
    │  │        ╰─ s [*]
    │  ├─ c
    │  │  ├─ ustomers_dot/proxy/graphql [*]
    │  │  ├─ haos/
    │  │  │  ├─ cpu_spin [*]
    │  │  │  ├─ db_spin [*]
    │  │  │  ├─ leakmem [*]
    │  │  │  ├─ sleep [*]
    │  │  │  ├─ kill [*]
    │  │  │  ├─ quit [*]
    │  │  │  ╰─ gc [*]
    │  │  ╰─ ountr
    │  │     ├─ y_states [*]
    │  │     ╰─ ies [*]
    │  ├─ i
    │  │  ├─ de [*]
    │  │  │  ├─ ntity_verification [*]
    │  │  │  │  ╰─ /
    │  │  │  │     ├─ s
    │  │  │  │     │  ├─ end_phone_verification_code [*]
    │  │  │  │     │  ╰─ uccess [*]
    │  │  │  │     ├─ toggle_phone_exemption [*]
    │  │  │  │     ╰─ verif
    │  │  │  │        ├─ y_
    │  │  │  │        │  ├─ phone_verification_code [*]
    │  │  │  │        │  ╰─ credit_card [*]
    │  │  │  │        │     ╰─ _captcha [*]
    │  │  │  │        ╰─ ication_state [*]
    │  │  │  ╰─ /
    │  │  │     ├─ reset_oauth_application_settings [*]
    │  │  │     ├─ oauth_redirect [*]
    │  │  │     ╰─ project [*]
    │  │  │        ╰─ /
    │  │  │           ╰─ {project_id} [*]
    │  │  │              ╰─ /
    │  │  │                 ├─ blob [*]
    │  │  │                 │  ╰─ /
    │  │  │                 │     ├─ {*branch}
    │  │  │                 │     │  ╰─ /- [*]
    │  │  │                 │     │     ╰─ /
    │  │  │                 │     │        ╰─ {*path} [*]
    │  │  │                 │     ╰─ {*branch} [*]
    │  │  │                 ├─ edit [*]
    │  │  │                 │  ╰─ /
    │  │  │                 │     ├─ {*branch}
    │  │  │                 │     │  ╰─ /- [*]
    │  │  │                 │     │     ╰─ /
    │  │  │                 │     │        ╰─ {*path} [*]
    │  │  │                 │     ╰─ {*branch} [*]
    │  │  │                 ├─ tree [*]
    │  │  │                 │  ╰─ /
    │  │  │                 │     ├─ {*branch}
    │  │  │                 │     │  ╰─ /- [*]
    │  │  │                 │     │     ╰─ /
    │  │  │                 │     │        ╰─ {*path} [*]
    │  │  │                 │     ╰─ {*branch} [*]
    │  │  │                 ╰─ merge_requests/
    │  │  │                    ╰─ {merge_request_id} [*]
    │  │  ╰─ nvites/
    │  │     ╰─ {id} [*]
    │  │        ╰─ /
    │  │           ├─ decline [*]
    │  │           ╰─ accept [*]
    │  ├─ j
    │  │  ├─ ira_connect [*]
    │  │  │  ╰─ /
    │  │  │     ├─ workspaces/search [*]
    │  │  │     ├─ oauth_
    │  │  │     │  ├─ application_id [*]
    │  │  │     │  ╰─ callbacks [*]
    │  │  │     ├─ app_descriptor [*]
    │  │  │     ├─ installations [*]
    │  │  │     ├─ subscriptions [*]
    │  │  │     │  ╰─ /
    │  │  │     │     ╰─ {id} [*]
    │  │  │     ├─ repositories/
    │  │  │     │  ├─ associate [*]
    │  │  │     │  ╰─ search [*]
    │  │  │     ├─ public_keys/
    │  │  │     │  ╰─ {id} [*]
    │  │  │     ├─ branches/
    │  │  │     │  ├─ route [*]
    │  │  │     │  ╰─ new [*]
    │  │  │     ╰─ events/
    │  │  │        ├─ uninstalled [*]
    │  │  │        ╰─ installed [*]
    │  │  ╰─ wks [*]
    │  ├─ m
    │  │  ├─ a
    │  │  │  ├─ ilgun/webhooks [*]
    │  │  │  ╰─ nifest [*]
    │  │  ╰─ e
    │  │     ├─ mbers/mailgun/permanent_failures [*]
    │  │     ╰─ trics [*]
    │  │        ╰─ /system [*]
    │  ├─ o
    │  │  ├─ rganizations [*]
    │  │  │  ╰─ /
    │  │  │     ├─ preview_markdown [*]
    │  │  │     ├─ new [*]
    │  │  │     ╰─ {organization_path} [*]
    │  │  │        ╰─ /
    │  │  │           ├─ settings/general [*]
    │  │  │           ├─ activity [*]
    │  │  │           ├─ groups [*]
    │  │  │           │  ├─ /
    │  │  │           │  │  ├─ new [*]
    │  │  │           │  │  ╰─ {*id}
    │  │  │           │  │     ╰─ /edit [*]
    │  │  │           │  ╰─ _and_projects [*]
    │  │  │           ├─ users [*]
    │  │  │           ╰─ projects/
    │  │  │              ╰─ {*namespace_id}
    │  │  │                 ╰─ /
    │  │  │                    ╰─ {id}
    │  │  │                       ╰─ /edit [*]
    │  │  ├─ perations [*]
    │  │  │  ╰─ /environments [*]
    │  │  ╰─ ffline [*]
    │  ├─ p
    │  │  ├─ hone_verification/telesign_callback [*]
    │  │  ├─ eek/results [*]
    │  │  ├─ ush_from_secondary/
    │  │  │  ╰─ {geo_node_id}
    │  │  │     ╰─ /
    │  │  │        ├─ {*repository_path}
    │  │  │        │  ╰─ /
    │  │  │        │     ├─ info/
    │  │  │        │     │  ├─ lfs/
    │  │  │        │     │  │  ├─ objects [*]
    │  │  │        │     │  │  │  ╰─ /
    │  │  │        │     │  │  │     ├─ batch [*]
    │  │  │        │     │  │  │     ╰─ {*oid} [*]
    │  │  │        │     │  │  ╰─ locks [*]
    │  │  │        │     │  │     ╰─ /
    │  │  │        │     │  │        ├─ verify [*]
    │  │  │        │     │  │        ├─ new [*]
    │  │  │        │     │  │        ╰─ {id} [*]
    │  │  │        │     │  │           ╰─ /
    │  │  │        │     │  │              ├─ unlock [*]
    │  │  │        │     │  │              ╰─ edit [*]
    │  │  │        │     │  ╰─ refs [*]
    │  │  │        │     ├─ ssh-
    │  │  │        │     │  ├─ receive-pack [*]
    │  │  │        │     │  ╰─ upload-pack [*]
    │  │  │        │     ╰─ git
    │  │  │        │        ├─ lab-lfs/objects/
    │  │  │        │        │  ├─ {*oid}
    │  │  │        │        │  │  ╰─ /
    │  │  │        │        │  │     ╰─ {size} [*]
    │  │  │        │        │  │        ╰─ /authorize [*]
    │  │  │        │        │  ╰─ {*oid} [*]
    │  │  │        │        ╰─ -
    │  │  │        │           ├─ receive-pack [*]
    │  │  │        │           ╰─ upload-pack [*]
    │  │  │        ╰─ {*repository_path} [*]
    │  │  ╰─ rofile/
    │  │     ├─ join_early_access_program [*]
    │  │     ├─ two_factor_auth [*]
    │  │     │  ╰─ /
    │  │     │     ├─ c
    │  │     │     │  ├─ reate_webauthn [*]
    │  │     │     │  ╰─ odes [*]
    │  │     │     ╰─ skip [*]
    │  │     ├─ notifications [*]
    │  │     ├─ preferences [*]
    │  │     ├─ billings [*]
    │  │     ├─ emails [*]
    │  │     │  ╰─ /
    │  │     │     ├─ confirmation [*]
    │  │     │     │  ╰─ /new [*]
    │  │     │     ╰─ {id} [*]
    │  │     │        ╰─ /resend_confirmation_instructions [*]
    │  │     ├─ webauthn_registrations/
    │  │     │  ╰─ {id} [*]
    │  │     ├─ groups/
    │  │     │  ╰─ {*id}
    │  │     │     ╰─ /notifications(.
    │  │     │        ╰─ {format}
    │  │     │           ╰─ ) [*]
    │  │     ├─ reset_
    │  │     │  ├─ incoming_email_token [*]
    │  │     │  ├─ static_object_token [*]
    │  │     │  ╰─ feed_token [*]
    │  │     ├─ slack/
    │  │     │  ├─ slack_link [*]
    │  │     │  ╰─ edit [*]
    │  │     ├─ a
    │  │     │  ├─ pplications [*]
    │  │     │  ├─ udit_log [*]
    │  │     │  ├─ ccount [*]
    │  │     │  │  ╰─ /unlink [*]
    │  │     │  ╰─ vatar [*]
    │  │     ├─ c
    │  │     │  ├─ hat_names [*]
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ deny [*]
    │  │     │  │     ├─ new [*]
    │  │     │  │     ╰─ {id} [*]
    │  │     │  ╰─ omment_templates [*]
    │  │     │     ╰─ /
    │  │     │        ╰─ {id} [*]
    │  │     ╰─ u
    │  │        ├─ pdate_username [*]
    │  │        ╰─ sage_quotas [*]
    │  ├─ r
    │  │  ├─ unner_setup/platforms [*]
    │  │  ╰─ e
    │  │     ├─ adiness [*]
    │  │     ╰─ mote_development/workspaces
    │  │        ├─ _feature_flag [*]
    │  │        ╰─ (/
    │  │           ╰─ {*vueroute}
    │  │              ╰─ ) [*]
    │  │                 ╰─ /
    │  │                    ├─ new [*]
    │  │                    ├─ {workspace_id}
    │  │                    │  ╰─ /workspaces [*]
    │  │                    │     ╰─ /new [*]
    │  │                    ╰─ {id} [*]
    │  │                       ╰─ /edit [*]
    │  ├─ s
    │  │  ├─ ubscriptions [*]
    │  │  │  ╰─ /
    │  │  │     ├─ validate_payment_method [*]
    │  │  │     ├─ hand_raise_leads [*]
    │  │  │     ├─ groups [*]
    │  │  │     │  ╰─ /
    │  │  │     │     ├─ new [*]
    │  │  │     │     ╰─ {id} [*]
    │  │  │     │        ╰─ /edit [*]
    │  │  │     ├─ new [*]
    │  │  │     ├─ payment_
    │  │  │     │  ├─ method [*]
    │  │  │     │  ╰─ form [*]
    │  │  │     ╰─ buy_
    │  │  │        ├─ minutes [*]
    │  │  │        ╰─ storage [*]
    │  │  ├─ nippets [*]
    │  │  │  ╰─ /
    │  │  │     ├─ preview_markdown [*]
    │  │  │     ├─ new [*]
    │  │  │     ├─ {snippet_id}
    │  │  │     │  ╰─ /
    │  │  │     │     ├─ notes [*]
    │  │  │     │     │  ╰─ /
    │  │  │     │     │     ╰─ {id} [*]
    │  │  │     │     │        ╰─ /
    │  │  │     │     │           ├─ toggle_award_emoji [*]
    │  │  │     │     │           ╰─ delete_attachment [*]
    │  │  │     │     ╰─ raw/
    │  │  │     │        ╰─ {ref}
    │  │  │     │           ╰─ /
    │  │  │     │              ╰─ {*path} [*]
    │  │  │     ╰─ {id} [*]
    │  │  │        ╰─ /
    │  │  │           ├─ toggle_award_emoji [*]
    │  │  │           ├─ mark_as_spam [*]
    │  │  │           ├─ edit [*]
    │  │  │           ╰─ raw [*]
    │  │  ├─ martcard/
    │  │  │  ├─ extract_certificate [*]
    │  │  │  ├─ verify_certificate [*]
    │  │  │  ╰─ auth [*]
    │  │  ├─ andbox/
    │  │  │  ├─ mermaid [*]
    │  │  │  ╰─ swagger [*]
    │  │  ├─ /
    │  │  │  ╰─ {username} [*]
    │  │  ╰─ e
    │  │     ├─ curity [*]
    │  │     │  ╰─ /
    │  │     │     ├─ vulnerabilities [*]
    │  │     │     ├─ dashboard [*]
    │  │     │     │  ╰─ /settings [*]
    │  │     │     ╰─ projects [*]
    │  │     │        ╰─ /
    │  │     │           ╰─ {id} [*]
    │  │     ╰─ nt_notifications/
    │  │        ╰─ {id}
    │  │           ╰─ /unsubscribe [*]
    │  ├─ t
    │  │  ├─ imelogs [*]
    │  │  ╰─ r
    │  │     ├─ ack_namespace_visits [*]
    │  │     ╰─ ial
    │  │        ├─ _registrations [*]
    │  │        │  ╰─ /new [*]
    │  │        ╰─ s [*]
    │  │           ╰─ /
    │  │              ├─ duo_
    │  │              │  ├─ enterprise [*]
    │  │              │  │  ╰─ /new [*]
    │  │              │  ╰─ pro [*]
    │  │              │     ╰─ /new [*]
    │  │              ╰─ new [*]
    │  ╰─ {model}
    │     ╰─ /
    │        ╰─ {model_id}
    │           ╰─ /uploads/
    │              ╰─ {secret}
    │                 ╰─ /
    │                    ╰─ {filename} [*]
    ├─ a
    │  ├─ dmin [*]
    │  │  ╰─ /
    │  │     ├─ namespace_limits [*]
    │  │     │  ╰─ /export_usage [*]
    │  │     ├─ organizations [*]
    │  │     ├─ version_check [*]
    │  │     ├─ topics [*]
    │  │     │  ╰─ /
    │  │     │     ├─ preview_markdown [*]
    │  │     │     ├─ merge [*]
    │  │     │     ├─ new [*]
    │  │     │     ├─ {topic_id}
    │  │     │     │  ╰─ /avatar [*]
    │  │     │     ╰─ {id} [*]
    │  │     │        ╰─ /edit [*]
    │  │     ├─ jobs [*]
    │  │     │  ╰─ /cancel_all [*]
    │  │     ├─ us
    │  │     │  ├─ age_trends [*]
    │  │     │  ╰─ er
    │  │     │     ├─ _permission_exports [*]
    │  │     │     ╰─ s [*]
    │  │     │        ╰─ /
    │  │     │           ├─ new [*]
    │  │     │           ├─ {user_id}
    │  │     │           │  ╰─ /
    │  │     │           │     ├─ i
    │  │     │           │     │  ├─ dentities [*]
    │  │     │           │     │  │  ╰─ /
    │  │     │           │     │  │     ├─ new [*]
    │  │     │           │     │  │     ╰─ {id} [*]
    │  │     │           │     │  │        ╰─ /edit [*]
    │  │     │           │     │  ╰─ mpersonation_tokens [*]
    │  │     │           │     │     ╰─ /
    │  │     │           │     │        ╰─ {id}
    │  │     │           │     │           ╰─ /revoke [*]
    │  │     │           │     ╰─ keys/
    │  │     │           │        ╰─ {id} [*]
    │  │     │           ╰─ {id} [*]
    │  │     │              ╰─ /
    │  │     │                 ├─ trust [*]
    │  │     │                 ├─ edit [*]
    │  │     │                 ├─ keys [*]
    │  │     │                 ├─ re
    │  │     │                 │  ├─ set_runners_minutes [*]
    │  │     │                 │  ├─ ject [*]
    │  │     │                 │  ╰─ move/
    │  │     │                 │     ╰─ {email_id} [*]
    │  │     │                 ├─ un
    │  │     │                 │  ├─ trust [*]
    │  │     │                 │  ├─ lock [*]
    │  │     │                 │  ╰─ b
    │  │     │                 │     ├─ lock [*]
    │  │     │                 │     ╰─ an [*]
    │  │     │                 ├─ a
    │  │     │                 │  ├─ ctivate [*]
    │  │     │                 │  ╰─ pprove [*]
    │  │     │                 ├─ b
    │  │     │                 │  ├─ lock [*]
    │  │     │                 │  ╰─ an [*]
    │  │     │                 ├─ c
    │  │     │                 │  ├─ ard_match [*]
    │  │     │                 │  ╰─ onfirm [*]
    │  │     │                 ├─ d
    │  │     │                 │  ├─ isable_two_factor [*]
    │  │     │                 │  ╰─ e
    │  │     │                 │     ├─ stroy_identity_verification_exemption [*]
    │  │     │                 │     ╰─ activate [*]
    │  │     │                 ├─ i
    │  │     │                 │  ├─ dentity_verification_exemption [*]
    │  │     │                 │  ╰─ mpersonate [*]
    │  │     │                 ╰─ p
    │  │     │                    ├─ hone_match [*]
    │  │     │                    ╰─ rojects [*]
    │  │     ├─ a
    │  │     │  ├─ pplication
    │  │     │  │  ├─ _settings [*]
    │  │     │  │  │  ╰─ /
    │  │     │  │  │     ├─ lets_encrypt_terms_of_service [*]
    │  │     │  │  │     ├─ metrics_and_profiling [*]
    │  │     │  │  │     ├─ integrations [*]
    │  │     │  │  │     │  ╰─ /
    │  │     │  │  │     │     ╰─ {id} [*]
    │  │     │  │  │     │        ╰─ /
    │  │     │  │  │     │           ├─ overrides [*]
    │  │     │  │  │     │           ├─ reset [*]
    │  │     │  │  │     │           ├─ edit [*]
    │  │     │  │  │     │           ╰─ test [*]
    │  │     │  │  │     ├─ preferences [*]
    │  │     │  │  │     ├─ templates [*]
    │  │     │  │  │     ├─ ge
    │  │     │  │  │     │  ├─ neral [*]
    │  │     │  │  │     │  ╰─ o [*]
    │  │     │  │  │     ├─ a
    │  │     │  │  │     │  ├─ ppearance [*]
    │  │     │  │  │     │  │  ╰─ /
    │  │     │  │  │     │  │     ├─ header_logos [*]
    │  │     │  │  │     │  │     ├─ favicon [*]
    │  │     │  │  │     │  │     ├─ logo [*]
    │  │     │  │  │     │  │     ╰─ p
    │  │     │  │  │     │  │        ├─ review_sign_in [*]
    │  │     │  │  │     │  │        ╰─ wa_icon [*]
    │  │     │  │  │     │  ├─ dvanced_search [*]
    │  │     │  │  │     │  ╰─ nalytics [*]
    │  │     │  │  │     ├─ c
    │  │     │  │  │     │  ├─ lear_repository_check_states [*]
    │  │     │  │  │     │  ╰─ i_cd [*]
    │  │     │  │  │     ├─ n
    │  │     │  │  │     │  ├─ amespace_storage [*]
    │  │     │  │  │     │  ╰─ etwork [*]
    │  │     │  │  │     ├─ r
    │  │     │  │  │     │  ├─ e
    │  │     │  │  │     │  │  ├─ po
    │  │     │  │  │     │  │  │  ├─ sitory [*]
    │  │     │  │  │     │  │  │  ╰─ rting [*]
    │  │     │  │  │     │  │  ╰─ set_
    │  │     │  │  │     │  │     ├─ error_tracking_access_token [*]
    │  │     │  │  │     │  │     ├─ health_check_token [*]
    │  │     │  │  │     │  │     ╰─ registration_token [*]
    │  │     │  │  │     │  ╰─ oles_and_permissions [*]
    │  │     │  │  │     │     ╰─ /
    │  │     │  │  │     │        ├─ new [*]
    │  │     │  │  │     │        ╰─ {id} [*]
    │  │     │  │  │     │           ╰─ /edit [*]
    │  │     │  │  │     ├─ s
    │  │     │  │  │     │  ├─ lack [*]
    │  │     │  │  │     │  │  ├─ /slack_auth [*]
    │  │     │  │  │     │  │  ╰─ _app_manifest_
    │  │     │  │  │     │  │     ├─ download [*]
    │  │     │  │  │     │  │     ╰─ share [*]
    │  │     │  │  │     │  ├─ cim_oauth [*]
    │  │     │  │  │     │  ╰─ e
    │  │     │  │  │     │     ├─ curity_and_compliance [*]
    │  │     │  │  │     │     ╰─ at_link_payload [*]
    │  │     │  │  │     ╰─ u
    │  │     │  │  │        ├─ pdate_microsoft_application [*]
    │  │     │  │  │        ╰─ sage_data [*]
    │  │     │  │  ╰─ s [*]
    │  │     │  │     ╰─ /
    │  │     │  │        ├─ new [*]
    │  │     │  │        ╰─ {id} [*]
    │  │     │  │           ╰─ /
    │  │     │  │              ├─ renew [*]
    │  │     │  │              ╰─ edit [*]
    │  │     │  ├─ buse_reports [*]
    │  │     │  │  ╰─ /
    │  │     │  │     ╰─ {id} [*]
    │  │     │  │        ╰─ /moderate_user [*]
    │  │     │  ├─ udit_log
    │  │     │  │  ├─ s [*]
    │  │     │  │  ╰─ _reports(.
    │  │     │  │     ╰─ {format}
    │  │     │  │        ╰─ ) [*]
    │  │     │  ╰─ i/
    │  │     │     ├─ self_hosted_models [*]
    │  │     │     │  ╰─ /
    │  │     │     │     ├─ terms_and_conditions [*]
    │  │     │     │     ├─ new [*]
    │  │     │     │     ╰─ {id} [*]
    │  │     │     │        ╰─ /edit [*]
    │  │     │     ╰─ feature_settings [*]
    │  │     │        ╰─ /
    │  │     │           ╰─ {id} [*]
    │  │     │              ╰─ /edit [*]
    │  │     ├─ b
    │  │     │  ├─ roadcast_messages [*]
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ preview [*]
    │  │     │  │     ╰─ {id} [*]
    │  │     │  │        ╰─ /edit [*]
    │  │     │  ╰─ ackground_
    │  │     │     ├─ migrations [*]
    │  │     │     │  ╰─ /
    │  │     │     │     ├─ {background_migration_id}
    │  │     │     │     │  ╰─ /batched_jobs/
    │  │     │     │     │     ╰─ {id} [*]
    │  │     │     │     ╰─ {id} [*]
    │  │     │     │        ╰─ /
    │  │     │     │           ├─ re
    │  │     │     │           │  ├─ sume [*]
    │  │     │     │           │  ╰─ try [*]
    │  │     │     │           ╰─ pause [*]
    │  │     │     ╰─ jobs [*]
    │  │     ├─ c
    │  │     │  ├─ lusters [*]
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ new_cluster_docs [*]
    │  │     │  │     ├─ c
    │  │     │  │     │  ├─ reate_user [*]
    │  │     │  │     │  ╰─ onnect [*]
    │  │     │  │     ├─ {cluster_id}
    │  │     │  │     │  ╰─ /integration/create_or_update [*]
    │  │     │  │     ╰─ {id} [*]
    │  │     │  │        ╰─ /
    │  │     │  │           ├─ cl
    │  │     │  │           │  ├─ uster_status [*]
    │  │     │  │           │  ╰─ ear_cache [*]
    │  │     │  │           ├─ environments [*]
    │  │     │  │           ╰─ metrics [*]
    │  │     │  │              ╰─ _dashboard [*]
    │  │     │  ├─ i/variables [*]
    │  │     │  ├─ redentials [*]
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ {credential_id}
    │  │     │  │     │  ╰─ /resources/
    │  │     │  │     │     ╰─ {resource_id}
    │  │     │  │     │        ╰─ /revoke [*]
    │  │     │  │     ╰─ {id} [*]
    │  │     │  │        ╰─ /revoke [*]
    │  │     │  ╰─ o
    │  │     │     ├─ de_suggestions [*]
    │  │     │     ╰─ horts [*]
    │  │     ├─ d
    │  │     │  ├─ ashboard/stats [*]
    │  │     │  ╰─ e
    │  │     │     ├─ v_ops_report [*]
    │  │     │     │  ╰─ s [*]
    │  │     │     ╰─ ploy_keys [*]
    │  │     │        ╰─ /
    │  │     │           ├─ new [*]
    │  │     │           ╰─ {id} [*]
    │  │     │              ╰─ /edit [*]
    │  │     ├─ e
    │  │     │  ├─ lasticsearch/
    │  │     │  │  ├─ cancel_index_deletion [*]
    │  │     │  │  ├─ trigger_reindexing [*]
    │  │     │  │  ├─ retry_migration [*]
    │  │     │  │  ╰─ enqueue_index [*]
    │  │     │  ╰─ mail [*]
    │  │     ├─ g
    │  │     │  ├─ italy_servers [*]
    │  │     │  ├─ eo [*]
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ replication [*]
    │  │     │  │     │  ╰─ /
    │  │     │  │     │     ╰─ {replicable_name_plural} [*]
    │  │     │  │     ╰─ s
    │  │     │  │        ├─ ettings [*]
    │  │     │  │        ╰─ ites [*]
    │  │     │  │           ╰─ /
    │  │     │  │              ├─ new [*]
    │  │     │  │              ╰─ {id} [*]
    │  │     │  │                 ╰─ /
    │  │     │  │                    ├─ replication [*]
    │  │     │  │                    │  ╰─ /
    │  │     │  │                    │     ╰─ {replicable_name_plural} [*]
    │  │     │  │                    ╰─ edit [*]
    │  │     │  ╰─ roups [*]
    │  │     │     ╰─ /
    │  │     │        ├─ new [*]
    │  │     │        ╰─ {*id}
    │  │     │           ├─ (.
    │  │     │           │  ╰─ {format}
    │  │     │           │     ╰─ ) [*]
    │  │     │           ╰─ /
    │  │     │              ├─ reset_runners_minutes(.
    │  │     │              │  ╰─ {format}
    │  │     │              │     ╰─ ) [*]
    │  │     │              ├─ members_update(.
    │  │     │              │  ╰─ {format}
    │  │     │              │     ╰─ ) [*]
    │  │     │              ╰─ edit(.
    │  │     │                 ╰─ {format}
    │  │     │                    ╰─ ) [*]
    │  │     ├─ h
    │  │     │  ├─ ealth_check [*]
    │  │     │  ╰─ ooks [*]
    │  │     │     ╰─ /
    │  │     │        ├─ {hook_id}
    │  │     │        │  ╰─ /hook_logs/
    │  │     │        │     ╰─ {id} [*]
    │  │     │        │        ╰─ /retry [*]
    │  │     │        ╰─ {id} [*]
    │  │     │           ╰─ /
    │  │     │              ├─ edit [*]
    │  │     │              ╰─ test [*]
    │  │     ├─ i
    │  │     │  ├─ n
    │  │     │  │  ├─ stance_review [*]
    │  │     │  │  ╰─ itial_setup [*]
    │  │     │  │     ╰─ /new [*]
    │  │     │  ╰─ mpersonation [*]
    │  │     ├─ l
    │  │     │  ├─ icense [*]
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ sync_seat_link [*]
    │  │     │  │     ├─ usage_export [*]
    │  │     │  │     ╰─ download [*]
    │  │     │  ╰─ abels [*]
    │  │     │     ╰─ /
    │  │     │        ├─ new [*]
    │  │     │        ╰─ {id} [*]
    │  │     │           ╰─ /edit [*]
    │  │     ├─ p
    │  │     │  ├─ lan_limits [*]
    │  │     │  ├─ ush_rule [*]
    │  │     │  ╰─ rojects [*]
    │  │     │     ╰─ /
    │  │     │        ╰─ {*namespace_id}
    │  │     │           ╰─ /
    │  │     │              ├─ {id} [*]
    │  │     │              │  ╰─ /
    │  │     │              │     ├─ repository_check [*]
    │  │     │              │     ├─ transfer [*]
    │  │     │              │     ╰─ edit [*]
    │  │     │              ╰─ {project_id}
    │  │     │                 ╰─ /runner_projects [*]
    │  │     │                    ╰─ /
    │  │     │                       ╰─ {id} [*]
    │  │     ├─ r
    │  │     │  ├─ unners [*]
    │  │     │  │  ╰─ /
    │  │     │  │     ├─ runner_setup_scripts [*]
    │  │     │  │     ├─ dashboard [*]
    │  │     │  │     ├─ tag_list [*]
    │  │     │  │     ├─ new [*]
    │  │     │  │     ╰─ {id} [*]
    │  │     │  │        ╰─ /
    │  │     │  │           ├─ pause [*]
    │  │     │  │           ├─ edit [*]
    │  │     │  │           ╰─ re
    │  │     │  │              ├─ gister [*]
    │  │     │  │              ╰─ sume [*]
    │  │     │  ╰─ ole_promotion_requests [*]
    │  │     ╰─ s
    │  │        ├─ ubscription [*]
    │  │        ├─ ystem_info [*]
    │  │        ├─ pam_logs [*]
    │  │        │  ╰─ /
    │  │        │     ╰─ {id} [*]
    │  │        │        ╰─ /mark_as_ham [*]
    │  │        ├─ ession [*]
    │  │        │  ╰─ /
    │  │        │     ├─ destroy [*]
    │  │        │     ╰─ new [*]
    │  │        ╰─ idekiq [*]
    │  ╰─ pi/
    │     ├─ v4/geo/graphql [*]
    │     ╰─ graphql [*]
    ├─ f
    │  ├─ iles/note/
    │  │  ╰─ {id}
    │  │     ╰─ /
    │  │        ╰─ {filename} [*]
    │  ╰─ avicon.
    │     ├─ ico [*]
    │     ╰─ png [*]
    ├─ p
    │  ├─ rojects [*]
    │  │  ╰─ /
    │  │     ├─ new [*]
    │  │     ╰─ {id} [*]
    │  ╰─ ublic [*]
    │     ╰─ /projects [*]
    ├─ u
    │  ├─ nsubscribes/
    │  │  ╰─ {email} [*]
    │  ├─ ploads/
    │  │  ├─ -/system/
    │  │  │  ├─ temp/
    │  │  │  │  ╰─ {secret}
    │  │  │  │     ╰─ /
    │  │  │  │        ╰─ {filename} [*]
    │  │  │  ╰─ {model}
    │  │  │     ╰─ /
    │  │  │        ├─ {mounted_as}
    │  │  │        │  ╰─ /
    │  │  │        │     ╰─ {id}
    │  │  │        │        ╰─ /
    │  │  │        │           ╰─ {filename} [*]
    │  │  │        ╰─ {id}
    │  │  │           ╰─ /
    │  │  │              ╰─ {secret}
    │  │  │                 ╰─ /
    │  │  │                    ╰─ {filename} [*]
    │  │  ╰─ {model} [*]
    │  │     ╰─ /authorize [*]
    │  ╰─ sers [*]
    │     ╰─ /
    │        ├─ resend_verification_code [*]
    │        ├─ identity_verification [*]
    │        │  ╰─ /
    │        │     ├─ s
    │        │     │  ├─ end_phone_verification_code [*]
    │        │     │  ╰─ uccess [*]
    │        │     ├─ toggle_phone_exemption [*]
    │        │     ├─ arkose_labs_challenge [*]
    │        │     ├─ res
    │        │     │  ├─ end_email_code [*]
    │        │     │  ╰─ tricted [*]
    │        │     ╰─ verif
    │        │        ├─ ication_state [*]
    │        │        ╰─ y_
    │        │           ├─ phone_verification_code [*]
    │        │           ├─ arkose_labs_session [*]
    │        │           ├─ credit_card [*]
    │        │           │  ╰─ _captcha [*]
    │        │           ╰─ email_code [*]
    │        ├─ password [*]
    │        │  ╰─ /
    │        │     ├─ complexity [*]
    │        │     ├─ edit [*]
    │        │     ╰─ new [*]
    │        ├─ u
    │        │  ├─ pdate_email [*]
    │        │  ╰─ nlock [*]
    │        │     ╰─ /new [*]
    │        ├─ edit [*]
    │        ├─ s
    │        │  ├─ uccessful_verification [*]
    │        │  ╰─ ign_
    │        │     ├─ out [*]
    │        │     ├─ in [*]
    │        │     ╰─ up [*]
    │        │        ╰─ /
    │        │           ├─ company [*]
    │        │           │  ╰─ /new [*]
    │        │           ├─ welcome [*]
    │        │           ╰─ groups [*]
    │        │              ╰─ /new [*]
    │        ├─ a
    │        │  ├─ lmost_there [*]
    │        │  ╰─ uth [*]
    │        │     ╰─ /
    │        │        ├─ kerberos/negotiate [*]
    │        │        ╰─ geo/sign_
    │        │           ├─ out [*]
    │        │           ╰─ in [*]
    │        ├─ c
    │        │  ├─ onfirmation [*]
    │        │  │  ╰─ /new [*]
    │        │  ╰─ ancel [*]
    │        ╰─ {username} [*]
    │           ╰─ /
    │              ├─ projects [*]
    │              ├─ unfollow [*]
    │              ├─ exists [*]
    │              ├─ follow [*]
    │              │  ├─ ers [*]
    │              │  ╰─ ing [*]
    │              ├─ groups [*]
    │              ├─ a
    │              │  ├─ ctivity [*]
    │              │  ╰─ vailable_
    │              │     ├─ project_templates [*]
    │              │     ╰─ group_templates [*]
    │              ├─ c
    │              │  ├─ ontributed [*]
    │              │  ╰─ alendar [*]
    │              │     ╰─ _activities [*]
    │              ╰─ s
    │                 ├─ nippets [*]
    │                 ╰─ tarred [*]
    ├─ {username} [*]
    │  ╰─ .
    │     ├─ keys [*]
    │     ╰─ gpg [*]
    ├─ {*repository_path}
    │  ╰─ /
    │     ├─ info/
    │     │  ├─ lfs/
    │     │  │  ├─ objects [*]
    │     │  │  │  ╰─ /
    │     │  │  │     ├─ batch [*]
    │     │  │  │     ╰─ {*oid} [*]
    │     │  │  ╰─ locks [*]
    │     │  │     ╰─ /
    │     │  │        ├─ verify [*]
    │     │  │        ├─ new [*]
    │     │  │        ╰─ {id} [*]
    │     │  │           ╰─ /
    │     │  │              ├─ unlock [*]
    │     │  │              ╰─ edit [*]
    │     │  ╰─ refs [*]
    │     ├─ ssh-
    │     │  ├─ receive-pack [*]
    │     │  ╰─ upload-pack [*]
    │     ╰─ git
    │        ├─ lab-lfs/objects/
    │        │  ├─ {*oid}
    │        │  │  ╰─ /
    │        │  │     ╰─ {size} [*]
    │        │  │        ╰─ /authorize [*]
    │        │  ╰─ {*oid} [*]
    │        ╰─ -
    │           ├─ receive-pack [*]
    │           ╰─ upload-pack [*]
    ├─ {*namespace_id}
    │  ╰─ /
    │     ├─ {project_id} [*]
    │     │  ╰─ /
    │     │     ├─ uploads [*]
    │     │     │  ╰─ /
    │     │     │     ├─ authorize [*]
    │     │     │     ╰─ {secret}
    │     │     │        ╰─ /
    │     │     │           ╰─ {filename} [*]
    │     │     ├─ de
    │     │     │  ├─ scription_templates/names/
    │     │     │  │  ╰─ {template_type}
    │     │     │  │     ╰─ (.
    │     │     │  │        ╰─ {format}
    │     │     │  │           ╰─ ) [*]
    │     │     │  ╰─ pendencies(/
    │     │     │     ╰─ {*rest}
    │     │     │        ╰─ ) [*]
    │     │     ├─ v
    │     │     │  ├─ ulnerability_feedback(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ╰─ ariables(/
    │     │     │     ╰─ {*rest}
    │     │     │        ╰─ ) [*]
    │     │     ├─ hooks(/
    │     │     │  ╰─ {*rest}
    │     │     │     ╰─ ) [*]
    │     │     ├─ wikis(/
    │     │     │  ╰─ {*rest}
    │     │     │     ╰─ ) [*]
    │     │     ├─ -/
    │     │     │  ├─ quality/test_cases [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ new [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  ├─ jobs [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ artifacts/
    │     │     │  │     │  ╰─ {*ref_name_and_path} [*]
    │     │     │  │     ├─ {job_id}
    │     │     │  │     │  ╰─ /artifacts/
    │     │     │  │     │     ├─ download [*]
    │     │     │  │     │     ├─ keep [*]
    │     │     │  │     │     ├─ external_file/
    │     │     │  │     │     │  ╰─ {*path} [*]
    │     │     │  │     │     ├─ browse(/
    │     │     │  │     │     │  ╰─ {*path}
    │     │     │  │     │     │     ╰─ ) [*]
    │     │     │  │     │     ├─ file/
    │     │     │  │     │     │  ╰─ {*path} [*]
    │     │     │  │     │     ╰─ raw/
    │     │     │  │     │        ╰─ {*path} [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ unschedule [*]
    │     │     │  │           ├─ cancel [*]
    │     │     │  │           ├─ status [*]
    │     │     │  │           ├─ viewer [*]
    │     │     │  │           ├─ erase [*]
    │     │     │  │           ├─ p
    │     │     │  │           │  ├─ roxy [*]
    │     │     │  │           │  │  ╰─ .ws/authorize [*]
    │     │     │  │           │  ╰─ lay [*]
    │     │     │  │           ├─ t
    │     │     │  │           │  ├─ e
    │     │     │  │           │  │  ├─ st_report_summary [*]
    │     │     │  │           │  │  ╰─ rminal [*]
    │     │     │  │           │  │     ╰─ .ws/authorize [*]
    │     │     │  │           │  ╰─ race(.
    │     │     │  │           │     ╰─ {format}
    │     │     │  │           │        ╰─ ) [*]
    │     │     │  │           ╰─ r
    │     │     │  │              ├─ etry [*]
    │     │     │  │              ╰─ aw [*]
    │     │     │  ├─ h
    │     │     │  │  ├─ ooks [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ {hook_id}
    │     │     │  │  │     │  ╰─ /hook_logs/
    │     │     │  │  │     │     ╰─ {id} [*]
    │     │     │  │  │     │        ╰─ /retry [*]
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ edit [*]
    │     │     │  │  │           ╰─ test [*]
    │     │     │  │  ╰─ arbor/repositories [*]
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ {id} [*]
    │     │     │  │        ╰─ {repository_id}
    │     │     │  │           ╰─ /artifacts [*]
    │     │     │  │              ╰─ /
    │     │     │  │                 ╰─ {artifact_id}
    │     │     │  │                    ╰─ /tags [*]
    │     │     │  ├─ de
    │     │     │  │  ├─ sign_management/designs/
    │     │     │  │  │  ╰─ {design_id}
    │     │     │  │  │     ╰─ (/
    │     │     │  │  │        ╰─ {sha}
    │     │     │  │  │           ╰─ )/r
    │     │     │  │  │              ├─ aw_image [*]
    │     │     │  │  │              ╰─ esized_image/
    │     │     │  │  │                 ╰─ {id} [*]
    │     │     │  │  ╰─ p
    │     │     │  │     ├─ loy_
    │     │     │  │     │  ├─ keys [*]
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ├─ enabled_keys [*]
    │     │     │  │     │  │     ├─ new [*]
    │     │     │  │     │  │     ├─ available_p
    │     │     │  │     │  │     │  ├─ roject_keys [*]
    │     │     │  │     │  │     │  ╰─ ublic_keys [*]
    │     │     │  │     │  │     ╰─ {id} [*]
    │     │     │  │     │  │        ╰─ /
    │     │     │  │     │  │           ├─ disable [*]
    │     │     │  │     │  │           ╰─ e
    │     │     │  │     │  │              ├─ nable [*]
    │     │     │  │     │  │              ╰─ dit [*]
    │     │     │  │     │  ╰─ tokens/
    │     │     │  │     │     ╰─ {id}
    │     │     │  │     │        ╰─ /revoke [*]
    │     │     │  │     ╰─ endencies [*]
    │     │     │  ├─ ne
    │     │     │  │  ├─ twork/
    │     │     │  │  │  ╰─ {id} [*]
    │     │     │  │  ╰─ w/
    │     │     │  │     ╰─ {*id} [*]
    │     │     │  ├─ on
    │     │     │  │  ├─ call_schedules [*]
    │     │     │  │  ╰─ _demand_scans [*]
    │     │     │  │     ╰─ /
    │     │     │  │        ├─ new [*]
    │     │     │  │        ╰─ {id}
    │     │     │  │           ╰─ /edit [*]
    │     │     │  ├─ a
    │     │     │  │  ├─ vatar [*]
    │     │     │  │  ├─ ws [*]
    │     │     │  │  │  ╰─ /configuration [*]
    │     │     │  │  ├─ lert_management [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  │        ╰─ /details(/
    │     │     │  │  │           ╰─ {*page}
    │     │     │  │  │              ╰─ ) [*]
    │     │     │  │  ├─ nalytics/
    │     │     │  │  │  ├─ value_stream_analytics [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ value_streams [*]
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ new [*]
    │     │     │  │  │  │     │     ├─ {id} [*]
    │     │     │  │  │  │     │     │  ╰─ /edit [*]
    │     │     │  │  │  │     │     ╰─ {value_stream_id}
    │     │     │  │  │  │     │        ╰─ /stages [*]
    │     │     │  │  │  │     │           ╰─ /
    │     │     │  │  │  │     │              ╰─ {id}
    │     │     │  │  │  │     │                 ╰─ /
    │     │     │  │  │  │     │                    ├─ average [*]
    │     │     │  │  │  │     │                    │  ╰─ _duration_chart [*]
    │     │     │  │  │  │     │                    ├─ records [*]
    │     │     │  │  │  │     │                    ├─ median [*]
    │     │     │  │  │  │     │                    ╰─ count [*]
    │     │     │  │  │  │     ├─ time_summary [*]
    │     │     │  │  │  │     ╰─ summary [*]
    │     │     │  │  │  ├─ merge_request_analytics [*]
    │     │     │  │  │  ├─ issues_analytics [*]
    │     │     │  │  │  ├─ code_reviews [*]
    │     │     │  │  │  ╰─ dashboards(/
    │     │     │  │  │     ╰─ {*vueroute}
    │     │     │  │  │        ╰─ ) [*]
    │     │     │  │  ├─ pprover
    │     │     │  │  │  ├─ _groups/
    │     │     │  │  │  │  ╰─ {id} [*]
    │     │     │  │  │  ╰─ s/
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  ├─ r
    │     │     │  │  │  ├─ tifacts [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ╰─ {id} [*]
    │     │     │  │  │  ╰─ chive/
    │     │     │  │  │     ╰─ {id}
    │     │     │  │  │        ╰─ .
    │     │     │  │  │           ╰─ {format} [*]
    │     │     │  │  ╰─ u
    │     │     │  │     ├─ dit_events [*]
    │     │     │  │     ╰─ to
    │     │     │  │        ├─ mations [*]
    │     │     │  │        ╰─ complete_sources/
    │     │     │  │           ├─ vulnerabilities [*]
    │     │     │  │           ├─ snippets [*]
    │     │     │  │           ├─ labels [*]
    │     │     │  │           ├─ epics [*]
    │     │     │  │           ├─ wikis [*]
    │     │     │  │           ├─ co
    │     │     │  │           │  ├─ mmands [*]
    │     │     │  │           │  ╰─ ntacts [*]
    │     │     │  │           ├─ i
    │     │     │  │           │  ├─ terations [*]
    │     │     │  │           │  ╰─ ssues [*]
    │     │     │  │           ╰─ m
    │     │     │  │              ├─ ilestones [*]
    │     │     │  │              ╰─ e
    │     │     │  │                 ├─ rge_requests [*]
    │     │     │  │                 ╰─ mbers [*]
    │     │     │  ├─ b
    │     │     │  │  ├─ ranches [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ diverging_commit_counts [*]
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ├─ {state} [*]
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  ├─ oards [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  ├─ adges/release(.
    │     │     │  │  │  ╰─ {format}
    │     │     │  │  │     ╰─ ) [*]
    │     │     │  │  ╰─ l
    │     │     │  │     ├─ ame
    │     │     │  │     │  ├─ _page/
    │     │     │  │     │  │  ╰─ {*id} [*]
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ {*id}
    │     │     │  │     │     │  ╰─ /streaming [*]
    │     │     │  │     │     ╰─ {*id} [*]
    │     │     │  │     ╰─ ob/
    │     │     │  │        ├─ {*id}
    │     │     │  │        │  ╰─ /diff [*]
    │     │     │  │        ╰─ {*id} [*]
    │     │     │  ├─ c
    │     │     │  │  ├─ ycle_analytics [*]
    │     │     │  │  ├─ adences(/
    │     │     │  │  │  ╰─ {*vueroute}
    │     │     │  │  │     ╰─ ) [*]
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ new [*]
    │     │     │  │  │           ├─ {iteration_cadence_id}
    │     │     │  │  │           │  ╰─ /iterations [*]
    │     │     │  │  │           │     ╰─ /
    │     │     │  │  │           │        ╰─ {id} [*]
    │     │     │  │  │           ╰─ {id} [*]
    │     │     │  │  │              ╰─ /edit [*]
    │     │     │  │  ├─ luster
    │     │     │  │  │  ├─ s [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ new_cluster_docs [*]
    │     │     │  │  │  │     ├─ c
    │     │     │  │  │  │     │  ├─ reate_user [*]
    │     │     │  │  │  │     │  ╰─ onnect [*]
    │     │     │  │  │  │     ├─ {cluster_id}
    │     │     │  │  │  │     │  ╰─ /integration/create_or_update [*]
    │     │     │  │  │  │     ╰─ {id} [*]
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  │           ├─ cl
    │     │     │  │  │  │           │  ├─ uster_status [*]
    │     │     │  │  │  │           │  ╰─ ear_cache [*]
    │     │     │  │  │  │           ├─ environments [*]
    │     │     │  │  │  │           ╰─ metrics [*]
    │     │     │  │  │  │              ╰─ _dashboard [*]
    │     │     │  │  │  ╰─ _agents/
    │     │     │  │  │     ╰─ {name} [*]
    │     │     │  │  ├─ reate
    │     │     │  │  │  ├─ _dir/
    │     │     │  │  │  │  ╰─ {*id} [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ {*id} [*]
    │     │     │  │  ├─ i/
    │     │     │  │  │  ├─ editor [*]
    │     │     │  │  │  ├─ lint [*]
    │     │     │  │  │  ├─ daily_build_group_report_results(.
    │     │     │  │  │  │  ╰─ {format}
    │     │     │  │  │  │     ╰─ ) [*]
    │     │     │  │  │  ╰─ prometheus_metrics/histograms(.
    │     │     │  │  │     ╰─ {format}
    │     │     │  │  │        ╰─ ) [*]
    │     │     │  │  ╰─ om
    │     │     │  │     ├─ m
    │     │     │  │     │  ├─ ent_templates [*]
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ╰─ {id} [*]
    │     │     │  │     │  ╰─ it
    │     │     │  │     │     ├─ s [*]
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     │     ├─ {*id}
    │     │     │  │     │     │     │  ╰─ /signatures [*]
    │     │     │  │     │     │     ╰─ {*id} [*]
    │     │     │  │     │     ╰─ /
    │     │     │  │     │        ╰─ {id} [*]
    │     │     │  │     │           ╰─ /
    │     │     │  │     │              ├─ merge_requests [*]
    │     │     │  │     │              ├─ cherry_pick [*]
    │     │     │  │     │              ├─ pipelines [*]
    │     │     │  │     │              ├─ branches [*]
    │     │     │  │     │              ├─ revert [*]
    │     │     │  │     │              ╰─ diff_f
    │     │     │  │     │                 ├─ or_path [*]
    │     │     │  │     │                 ╰─ iles [*]
    │     │     │  │     ╰─ pare [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ diff_for_path [*]
    │     │     │  │           ├─ signatures [*]
    │     │     │  │           ╰─ {from}
    │     │     │  │              ╰─ ...
    │     │     │  │                 ╰─ {to} [*]
    │     │     │  ├─ e
    │     │     │  │  ├─ scalation_policies [*]
    │     │     │  │  ├─ rror_tracking [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ projects [*]
    │     │     │  │  │     ╰─ {issue_id} [*]
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ stack_trace [*]
    │     │     │  │  │           ╰─ details [*]
    │     │     │  │  ├─ nvironments [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ search [*]
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ├─ folders/
    │     │     │  │  │     │  ╰─ {*id}
    │     │     │  │  │     │     ╰─ (.
    │     │     │  │  │     │        ╰─ {format}
    │     │     │  │  │     │           ╰─ ) [*]
    │     │     │  │  │     ├─ {environment_id}
    │     │     │  │  │     │  ╰─ /deployments [*]
    │     │     │  │  │     │     ╰─ /
    │     │     │  │  │     │        ╰─ {id} [*]
    │     │     │  │  │     │           ╰─ /
    │     │     │  │  │     │              ├─ additional_metrics [*]
    │     │     │  │  │     │              ╰─ metrics [*]
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ cancel_auto_stop [*]
    │     │     │  │  │           ├─ terminal [*]
    │     │     │  │  │           │  ╰─ .ws/authorize [*]
    │     │     │  │  │           ├─ edit [*]
    │     │     │  │  │           ├─ stop [*]
    │     │     │  │  │           ├─ prometheus/api/v1/
    │     │     │  │  │           │  ╰─ {*proxy_path} [*]
    │     │     │  │  │           ╰─ k8s(/
    │     │     │  │  │              ╰─ {*vueroute}
    │     │     │  │  │                 ╰─ ) [*]
    │     │     │  │  ╰─ dit/
    │     │     │  │     ╰─ {*id} [*]
    │     │     │  ├─ f
    │     │     │  │  ├─ eature_flags [*]
    │     │     │  │  │  ├─ _
    │     │     │  │  │  │  ├─ client/reset_token [*]
    │     │     │  │  │  │  ╰─ user_lists [*]
    │     │     │  │  │  │     ╰─ /
    │     │     │  │  │  │        ├─ new [*]
    │     │     │  │  │  │        ╰─ {iid} [*]
    │     │     │  │  │  │           ╰─ /edit [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ├─ {feature_flag_iid}
    │     │     │  │  │     │  ╰─ /issues [*]
    │     │     │  │  │     │     ╰─ /
    │     │     │  │  │     │        ╰─ {id} [*]
    │     │     │  │  │     ╰─ {iid} [*]
    │     │     │  │  │        ╰─ /edit [*]
    │     │     │  │  ├─ orks [*]
    │     │     │  │  │  ╰─ /new [*]
    │     │     │  │  ╰─ i
    │     │     │  │     ├─ nd_file/
    │     │     │  │     │  ╰─ {*id} [*]
    │     │     │  │     ╰─ les/
    │     │     │  │        ╰─ {*id} [*]
    │     │     │  ├─ g
    │     │     │  │  ├─ oogle_cloud [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ artifact_registry [*]
    │     │     │  │  │     │  ╰─ /projects/
    │     │     │  │  │     │     ╰─ {project}
    │     │     │  │  │     │        ╰─ /locations/
    │     │     │  │  │     │           ╰─ {location}
    │     │     │  │  │     │              ╰─ /repositories/
    │     │     │  │  │     │                 ╰─ {repository}
    │     │     │  │  │     │                    ╰─ /dockerImages/
    │     │     │  │  │     │                       ╰─ {image} [*]
    │     │     │  │  │     ├─ service_accounts [*]
    │     │     │  │  │     ├─ configuration [*]
    │     │     │  │  │     ├─ revoke_oauth [*]
    │     │     │  │  │     ├─ d
    │     │     │  │  │     │  ├─ eployments [*]
    │     │     │  │  │     │  │  ╰─ /cloud_
    │     │     │  │  │     │  │     ├─ storage [*]
    │     │     │  │  │     │  │     ╰─ run [*]
    │     │     │  │  │     │  ╰─ atabases [*]
    │     │     │  │  │     │     ╰─ /new/
    │     │     │  │  │     │        ╰─ {product} [*]
    │     │     │  │  │     ╰─ gcp_regions [*]
    │     │     │  │  ╰─ r
    │     │     │  │     ├─ oup_links/
    │     │     │  │     │  ╰─ {id} [*]
    │     │     │  │     ╰─ aphs/
    │     │     │  │        ╰─ {id} [*]
    │     │     │  │           ╰─ /
    │     │     │  │              ├─ languages [*]
    │     │     │  │              ╰─ c
    │     │     │  │                 ├─ ommits [*]
    │     │     │  │                 ├─ harts [*]
    │     │     │  │                 ╰─ i [*]
    │     │     │  ├─ i
    │     │     │  │  ├─ terations [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  ├─ mport [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ jira [*]
    │     │     │  │  │     ╰─ new [*]
    │     │     │  │  ├─ ssues [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ service_desk [*]
    │     │     │  │  │     ├─ bulk_update [*]
    │     │     │  │  │     ├─ export_csv [*]
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ├─ i
    │     │     │  │  │     │  ├─ mport_csv [*]
    │     │     │  │  │     │  ╰─ ncident/
    │     │     │  │  │     │     ╰─ {id}
    │     │     │  │  │     │        ╰─ (/
    │     │     │  │  │     │           ╰─ {incident_tab}
    │     │     │  │  │     │              ╰─ ) [*]
    │     │     │  │  │     ├─ {issue_id}
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ feature_flags [*]
    │     │     │  │  │     │     │  ╰─ /
    │     │     │  │  │     │     │     ╰─ {id} [*]
    │     │     │  │  │     │     ╰─ links [*]
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ╰─ {id} [*]
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ edit [*]
    │     │     │  │  │           ├─ toggle_
    │     │     │  │  │           │  ├─ subscription [*]
    │     │     │  │  │           │  ╰─ award_emoji [*]
    │     │     │  │  │           ├─ re
    │     │     │  │  │           │  ├─ altime_changes [*]
    │     │     │  │  │           │  ├─ lated_branches [*]
    │     │     │  │  │           │  ╰─ order [*]
    │     │     │  │  │           ├─ c
    │     │     │  │  │           │  ├─ reate_merge_request [*]
    │     │     │  │  │           │  ╰─ an_create_branch [*]
    │     │     │  │  │           ├─ d
    │     │     │  │  │           │  ├─ iscussions [*]
    │     │     │  │  │           │  ╰─ es
    │     │     │  │  │           │     ├─ criptions/
    │     │     │  │  │           │     │  ╰─ {version_id} [*]
    │     │     │  │  │           │     │     ╰─ /diff [*]
    │     │     │  │  │           │     ╰─ igns(/
    │     │     │  │  │           │        ╰─ {*vueroute}
    │     │     │  │  │           │           ╰─ ) [*]
    │     │     │  │  │           ├─ m
    │     │     │  │  │           │  ├─ ark_as_spam [*]
    │     │     │  │  │           │  ╰─ ove [*]
    │     │     │  │  │           ╰─ {incident_tab} [*]
    │     │     │  │  ╰─ n
    │     │     │  │     ├─ cident
    │     │     │  │     │  ├─ _management/timeline_events/preview_markdown [*]
    │     │     │  │     │  ╰─ s [*]
    │     │     │  │     │     ╰─ /integrations/pagerduty [*]
    │     │     │  │     ├─ frastructure_registry [*]
    │     │     │  │     ╰─ tegrations/
    │     │     │  │        ├─ slash_commands [*]
    │     │     │  │        │  ╰─ /confirm [*]
    │     │     │  │        ├─ zentao/issues [*]
    │     │     │  │        │  ╰─ /
    │     │     │  │        │     ╰─ {id} [*]
    │     │     │  │        ╰─ jira/issues [*]
    │     │     │  │           ╰─ /
    │     │     │  │              ╰─ {id} [*]
    │     │     │  ├─ l
    │     │     │  │  ├─ abels [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ set_priorities [*]
    │     │     │  │  │     ├─ generate [*]
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ toggle_subscription [*]
    │     │     │  │  │           ├─ remove_priority [*]
    │     │     │  │  │           ├─ promote [*]
    │     │     │  │  │           ╰─ edit [*]
    │     │     │  │  ├─ earn_gitlab [*]
    │     │     │  │  │  ╰─ /end_tutorial [*]
    │     │     │  │  ╰─ ogs [*]
    │     │     │  ├─ m
    │     │     │  │  ├─ attermost [*]
    │     │     │  │  │  ╰─ /new [*]
    │     │     │  │  ├─ i
    │     │     │  │  │  ├─ lestones [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ new [*]
    │     │     │  │  │  │     ╰─ {id} [*]
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  │           ├─ merge_requests [*]
    │     │     │  │  │  │           ├─ issues [*]
    │     │     │  │  │  │           ├─ labels [*]
    │     │     │  │  │  │           ├─ edit [*]
    │     │     │  │  │  │           ╰─ p
    │     │     │  │  │  │              ├─ articipants [*]
    │     │     │  │  │  │              ╰─ romote [*]
    │     │     │  │  │  ╰─ rror [*]
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ update_now [*]
    │     │     │  │  │        ╰─ ssh_host_keys(.
    │     │     │  │  │           ╰─ {format}
    │     │     │  │  │              ╰─ ) [*]
    │     │     │  │  ├─ l/
    │     │     │  │  │  ├─ preview_markdown [*]
    │     │     │  │  │  ├─ experiments [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ╰─ {iid} [*]
    │     │     │  │  │  ├─ models [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ new [*]
    │     │     │  │  │  │     ├─ {model_id} [*]
    │     │     │  │  │  │     ╰─ {model_model_id}
    │     │     │  │  │  │        ╰─ /versions/
    │     │     │  │  │  │           ╰─ {model_version_id} [*]
    │     │     │  │  │  ├─ candidates/
    │     │     │  │  │  │  ╰─ {iid} [*]
    │     │     │  │  │  ╰─ agents(/
    │     │     │  │  │     ╰─ {*vueroute}
    │     │     │  │  │        ╰─ ) [*]
    │     │     │  │  │           ╰─ /
    │     │     │  │  │              ├─ new [*]
    │     │     │  │  │              ╰─ {id} [*]
    │     │     │  │  │                 ╰─ /edit [*]
    │     │     │  │  ╰─ e
    │     │     │  │     ├─ rge
    │     │     │  │     │  ├─ d_branches [*]
    │     │     │  │     │  ╰─ _
    │     │     │  │     │     ├─ trains [*]
    │     │     │  │     │     ╰─ requests [*]
    │     │     │  │     │        ╰─ /
    │     │     │  │     │           ├─ diff_for_path [*]
    │     │     │  │     │           ├─ bulk_update [*]
    │     │     │  │     │           ├─ export_csv [*]
    │     │     │  │     │           ├─ new [*]
    │     │     │  │     │           │  ╰─ /
    │     │     │  │     │           │     ├─ target_projects [*]
    │     │     │  │     │           │     ├─ branch_
    │     │     │  │     │           │     │  ├─ from [*]
    │     │     │  │     │           │     │  ╰─ to [*]
    │     │     │  │     │           │     ├─ pipelines [*]
    │     │     │  │     │           │     ╰─ diff
    │     │     │  │     │           │        ├─ _for_path [*]
    │     │     │  │     │           │        ╰─ s [*]
    │     │     │  │     │           ├─ {merge_request_id}
    │     │     │  │     │           │  ╰─ /
    │     │     │  │     │           │     ├─ drafts [*]
    │     │     │  │     │           │     │  ╰─ /
    │     │     │  │     │           │     │     ├─ discard [*]
    │     │     │  │     │           │     │     ├─ publish [*]
    │     │     │  │     │           │     │     ╰─ {id} [*]
    │     │     │  │     │           │     ╰─ approver
    │     │     │  │     │           │        ├─ s [*]
    │     │     │  │     │           │        │  ╰─ /
    │     │     │  │     │           │        │     ╰─ {id} [*]
    │     │     │  │     │           │        ╰─ _groups/
    │     │     │  │     │           │           ╰─ {id} [*]
    │     │     │  │     │           ╰─ {id} [*]
    │     │     │  │     │              ╰─ /
    │     │     │  │     │                 ├─ license_scanning_reports [*]
    │     │     │  │     │                 │  ╰─ _collapsed [*]
    │     │     │  │     │                 ├─ e
    │     │     │  │     │                 │  ├─ xposed_artifacts [*]
    │     │     │  │     │                 │  ╰─ dit [*]
    │     │     │  │     │                 ├─ widget [*]
    │     │     │  │     │                 ├─ pipeline
    │     │     │  │     │                 │  ├─ _status [*]
    │     │     │  │     │                 │  ╰─ s [*]
    │     │     │  │     │                 ├─ me
    │     │     │  │     │                 │  ├─ trics_reports [*]
    │     │     │  │     │                 │  ╰─ rge [*]
    │     │     │  │     │                 ├─ re
    │     │     │  │     │                 │  ├─ solve_conflicts [*]
    │     │     │  │     │                 │  ├─ move_wip [*]
    │     │     │  │     │                 │  ├─ ports [*]
    │     │     │  │     │                 │  ╰─ base [*]
    │     │     │  │     │                 ├─ a
    │     │     │  │     │                 │  ├─ ccessibility_reports [*]
    │     │     │  │     │                 │  ├─ ssign_related_issues [*]
    │     │     │  │     │                 │  ╰─ pi_fuzzing_reports [*]
    │     │     │  │     │                 ├─ c
    │     │     │  │     │                 │  ├─ i_environments_status [*]
    │     │     │  │     │                 │  ├─ a
    │     │     │  │     │                 │  │  ├─ ncel_auto_merge [*]
    │     │     │  │     │                 │  │  ╰─ ched_widget [*]
    │     │     │  │     │                 │  ╰─ o
    │     │     │  │     │                 │     ├─ n
    │     │     │  │     │                 │     │  ├─ flict
    │     │     │  │     │                 │     │  │  ├─ _for_path [*]
    │     │     │  │     │                 │     │  │  ╰─ s [*]
    │     │     │  │     │                 │     │  ╰─ t
    │     │     │  │     │                 │     │     ├─ ainer_scanning_reports [*]
    │     │     │  │     │                 │     │     ╰─ ext_commits [*]
    │     │     │  │     │                 │     ├─ mmit
    │     │     │  │     │                 │     │  ├─ _change_content [*]
    │     │     │  │     │                 │     │  ╰─ s [*]
    │     │     │  │     │                 │     ├─ dequality_
    │     │     │  │     │                 │     │  ├─ mr_diff_reports [*]
    │     │     │  │     │                 │     │  ╰─ reports [*]
    │     │     │  │     │                 │     ╰─ verage_
    │     │     │  │     │                 │        ├─ fuzzing_reports [*]
    │     │     │  │     │                 │        ╰─ reports [*]
    │     │     │  │     │                 ├─ d
    │     │     │  │     │                 │  ├─ ast_reports [*]
    │     │     │  │     │                 │  ├─ i
    │     │     │  │     │                 │  │  ├─ scussions [*]
    │     │     │  │     │                 │  │  ╰─ ff
    │     │     │  │     │                 │  │     ├─ _
    │     │     │  │     │                 │  │     │  ├─ for_path [*]
    │     │     │  │     │                 │  │     │  ╰─ by_file_hash/
    │     │     │  │     │                 │  │     │     ╰─ {file_hash} [*]
    │     │     │  │     │                 │  │     ╰─ s [*]
    │     │     │  │     │                 │  │        ╰─ _
    │     │     │  │     │                 │  │           ├─ metadata [*]
    │     │     │  │     │                 │  │           ├─ stream [*]
    │     │     │  │     │                 │  │           ╰─ batch [*]
    │     │     │  │     │                 │  ╰─ e
    │     │     │  │     │                 │     ├─ pendency_scanning_reports [*]
    │     │     │  │     │                 │     ╰─ scriptions/
    │     │     │  │     │                 │        ╰─ {version_id} [*]
    │     │     │  │     │                 │           ╰─ /diff [*]
    │     │     │  │     │                 ├─ s
    │     │     │  │     │                 │  ├─ ec
    │     │     │  │     │                 │  │  ├─ ret_detection_reports [*]
    │     │     │  │     │                 │  │  ╰─ urity_reports [*]
    │     │     │  │     │                 │  ╰─ a
    │     │     │  │     │                 │     ├─ ml_approval [*]
    │     │     │  │     │                 │     ╰─ st_reports [*]
    │     │     │  │     │                 ╰─ t
    │     │     │  │     │                    ├─ oggle_
    │     │     │  │     │                    │  ├─ subscription [*]
    │     │     │  │     │                    │  ╰─ award_emoji [*]
    │     │     │  │     │                    ╰─ e
    │     │     │  │     │                       ├─ rraform_reports [*]
    │     │     │  │     │                       ╰─ st_reports [*]
    │     │     │  │     ╰─ trics [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ╰─ {id} [*]
    │     │     │  ├─ p
    │     │     │  │  ├─ ush_rules/
    │     │     │  │  │  ╰─ {id} [*]
    │     │     │  │  ├─ ipeline
    │     │     │  │  │  ├─ s [*]
    │     │     │  │  │  │  ├─ (/
    │     │     │  │  │  │  │  ╰─ {*ref}
    │     │     │  │  │  │  │     ╰─ )/latest [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ├─ settings [*]
    │     │     │  │  │  │     ├─ charts [*]
    │     │     │  │  │  │     ├─ new [*]
    │     │     │  │  │  │     ├─ {pipeline_id}
    │     │     │  │  │  │     │  ╰─ /
    │     │     │  │  │  │     │     ├─ tests/
    │     │     │  │  │  │     │     │  ├─ summary [*]
    │     │     │  │  │  │     │     │  ╰─ {suite_name} [*]
    │     │     │  │  │  │     │     ├─ validate_account [*]
    │     │     │  │  │  │     │     ╰─ stages/
    │     │     │  │  │  │     │        ╰─ {stage_name}
    │     │     │  │  │  │     │           ╰─ /play_manual [*]
    │     │     │  │  │  │     ╰─ {id} [*]
    │     │     │  │  │  │        ╰─ /
    │     │     │  │  │  │           ├─ manual_variables [*]
    │     │     │  │  │  │           ├─ test_report [*]
    │     │     │  │  │  │           ├─ failures [*]
    │     │     │  │  │  │           ├─ builds [*]
    │     │     │  │  │  │           ├─ retry [*]
    │     │     │  │  │  │           ├─ d
    │     │     │  │  │  │           │  ├─ ownloadable_artifacts [*]
    │     │     │  │  │  │           │  ╰─ ag [*]
    │     │     │  │  │  │           ├─ license
    │     │     │  │  │  │           │  ├─ _count [*]
    │     │     │  │  │  │           │  ╰─ s [*]
    │     │     │  │  │  │           ├─ c
    │     │     │  │  │  │           │  ├─ odequality_report [*]
    │     │     │  │  │  │           │  ╰─ ancel [*]
    │     │     │  │  │  │           ╰─ s
    │     │     │  │  │  │              ├─ ecurity [*]
    │     │     │  │  │  │              ╰─ ta
    │     │     │  │  │  │                 ├─ tus [*]
    │     │     │  │  │  │                 ╰─ ge [*]
    │     │     │  │  │  ╰─ _schedules [*]
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ new [*]
    │     │     │  │  │        ╰─ {id} [*]
    │     │     │  │  │           ╰─ /
    │     │     │  │  │              ├─ take_ownership [*]
    │     │     │  │  │              ├─ edit [*]
    │     │     │  │  │              ╰─ play [*]
    │     │     │  │  ├─ ackage
    │     │     │  │  │  ├─ s [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ╰─ {id} [*]
    │     │     │  │  │  ╰─ _files/
    │     │     │  │  │     ╰─ {id}
    │     │     │  │  │        ╰─ /download [*]
    │     │     │  │  ╰─ r
    │     │     │  │     ├─ o
    │     │     │  │     │  ├─ ject_members [*]
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ├─ request_access [*]
    │     │     │  │     │  │     ├─ leave [*]
    │     │     │  │     │  │     ╰─ {id} [*]
    │     │     │  │     │  │        ╰─ /
    │     │     │  │     │  │           ├─ approve_access_request [*]
    │     │     │  │     │  │           ╰─ resend_invite [*]
    │     │     │  │     │  ╰─ tected_
    │     │     │  │     │     ├─ environments [*]
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     │     ├─ search [*]
    │     │     │  │     │     │     ╰─ {id} [*]
    │     │     │  │     │     ├─ branches [*]
    │     │     │  │     │     │  ╰─ /
    │     │     │  │     │     │     ╰─ {id} [*]
    │     │     │  │     │     ╰─ tags [*]
    │     │     │  │     │        ╰─ /
    │     │     │  │     │           ╰─ {id} [*]
    │     │     │  │     ╰─ eview
    │     │     │  │        ├─ _markdown [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ╰─ {*id} [*]
    │     │     │  ├─ r
    │     │     │  │  ├─ unners [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ├─ toggle_
    │     │     │  │  │     │  ├─ shared_runners [*]
    │     │     │  │  │     │  ╰─ group_runners [*]
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ pause [*]
    │     │     │  │  │           ├─ edit [*]
    │     │     │  │  │           ╰─ re
    │     │     │  │  │              ├─ gister [*]
    │     │     │  │  │              ╰─ sume [*]
    │     │     │  │  ├─ aw/
    │     │     │  │  │  ╰─ {*id} [*]
    │     │     │  │  ╰─ e
    │     │     │  │     ├─ quirements_management/requirements [*]
    │     │     │  │     │  ╰─ /import_csv [*]
    │     │     │  │     │     ╰─ /authorize [*]
    │     │     │  │     ├─ pository [*]
    │     │     │  │     ├─ leases [*]
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ├─ outbox [*]
    │     │     │  │     │     ├─ inbox [*]
    │     │     │  │     │     ├─ new [*]
    │     │     │  │     │     ├─ permalink/latest(/
    │     │     │  │     │     │  ╰─ {*suffix_path}
    │     │     │  │     │     │     ╰─ ) [*]
    │     │     │  │     │     ╰─ {tag} [*]
    │     │     │  │     │        ╰─ /
    │     │     │  │     │           ├─ downloads/
    │     │     │  │     │           │  ╰─ {*filepath} [*]
    │     │     │  │     │           ╰─ e
    │     │     │  │     │              ├─ dit [*]
    │     │     │  │     │              ╰─ vidences/
    │     │     │  │     │                 ╰─ {id} [*]
    │     │     │  │     ╰─ fs/
    │     │     │  │        ├─ switch [*]
    │     │     │  │        ╰─ {id}
    │     │     │  │           ╰─ /logs_tree [*]
    │     │     │  │              ╰─ /
    │     │     │  │                 ╰─ {*path} [*]
    │     │     │  ├─ s
    │     │     │  │  ├─ ubscriptions [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  ├─ nippets [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ├─ {snippet_id}
    │     │     │  │  │     │  ╰─ /raw/
    │     │     │  │  │     │     ╰─ {ref}
    │     │     │  │  │     │        ╰─ /
    │     │     │  │  │     │           ╰─ {*path} [*]
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  │        ╰─ /
    │     │     │  │  │           ├─ toggle_award_emoji [*]
    │     │     │  │  │           ├─ mark_as_spam [*]
    │     │     │  │  │           ├─ edit [*]
    │     │     │  │  │           ╰─ raw [*]
    │     │     │  │  ├─ tarrers [*]
    │     │     │  │  ├─ chema/
    │     │     │  │  │  ╰─ {branch}
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ╰─ {*filename} [*]
    │     │     │  │  ╰─ e
    │     │     │  │     ├─ rvice_desk/custom_email [*]
    │     │     │  │     ├─ ttings/
    │     │     │  │     │  ├─ packages_and_registries [*]
    │     │     │  │     │  │  ╰─ /cleanup_image_tags [*]
    │     │     │  │     │  ├─ merge_requests [*]
    │     │     │  │     │  ├─ integrations [*]
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ├─ {integration_id}
    │     │     │  │     │  │     │  ╰─ /hook_logs/
    │     │     │  │     │  │     │     ╰─ {id} [*]
    │     │     │  │     │  │     │        ╰─ /retry [*]
    │     │     │  │     │  │     ╰─ {id} [*]
    │     │     │  │     │  │        ╰─ /
    │     │     │  │     │  │           ├─ edit [*]
    │     │     │  │     │  │           ╰─ test [*]
    │     │     │  │     │  ├─ operations [*]
    │     │     │  │     │  │  ╰─ /reset_
    │     │     │  │     │  │     ├─ pagerduty_token [*]
    │     │     │  │     │  │     ╰─ alerting_token [*]
    │     │     │  │     │  ├─ repository [*]
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ├─ deploy_token/create [*]
    │     │     │  │     │  │     ├─ branch_rules [*]
    │     │     │  │     │  │     ╰─ cleanup [*]
    │     │     │  │     │  ├─ ci_cd [*]
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ├─ deploy_token/create [*]
    │     │     │  │     │  │     ╰─ r
    │     │     │  │     │  │        ├─ unner_setup_scripts [*]
    │     │     │  │     │  │        ╰─ eset_
    │     │     │  │     │  │           ├─ registration_token [*]
    │     │     │  │     │  │           ╰─ cache [*]
    │     │     │  │     │  ├─ slack [*]
    │     │     │  │     │  │  ╰─ /
    │     │     │  │     │  │     ├─ slack_auth [*]
    │     │     │  │     │  │     ╰─ edit [*]
    │     │     │  │     │  ╰─ a
    │     │     │  │     │     ├─ nalytics [*]
    │     │     │  │     │     ╰─ ccess_tokens [*]
    │     │     │  │     │        ╰─ /
    │     │     │  │     │           ╰─ {id}
    │     │     │  │     │              ╰─ /revoke [*]
    │     │     │  │     ╰─ c
    │     │     │  │        ├─ rets(/
    │     │     │  │        │  ╰─ {*vueroute}
    │     │     │  │        │     ╰─ ) [*]
    │     │     │  │        ╰─ urity/
    │     │     │  │           ├─ vulnerabilit
    │     │     │  │           │  ├─ y_report [*]
    │     │     │  │           │  ╰─ ies/
    │     │     │  │           │     ├─ new [*]
    │     │     │  │           │     ├─ {vulnerability_id}
    │     │     │  │           │     │  ╰─ /notes [*]
    │     │     │  │           │     │     ╰─ /
    │     │     │  │           │     │        ╰─ {id} [*]
    │     │     │  │           │     │           ╰─ /toggle_award_emoji [*]
    │     │     │  │           │     ╰─ {id} [*]
    │     │     │  │           │        ╰─ /discussions [*]
    │     │     │  │           ├─ scanned_resources [*]
    │     │     │  │           ├─ policies [*]
    │     │     │  │           │  ╰─ /
    │     │     │  │           │     ├─ schema [*]
    │     │     │  │           │     ├─ new [*]
    │     │     │  │           │     ╰─ {id}
    │     │     │  │           │        ╰─ /edit [*]
    │     │     │  │           ├─ configuration [*]
    │     │     │  │           │  ╰─ /
    │     │     │  │           │     ├─ corpus_management [*]
    │     │     │  │           │     ├─ s
    │     │     │  │           │     │  ├─ ecret_detection [*]
    │     │     │  │           │     │  ╰─ ast [*]
    │     │     │  │           │     ├─ profile_library [*]
    │     │     │  │           │     │  ╰─ /dast_s
    │     │     │  │           │     │     ├─ canner_profiles/
    │     │     │  │           │     │     │  ├─ new [*]
    │     │     │  │           │     │     │  ╰─ {id}
    │     │     │  │           │     │     │     ╰─ /edit [*]
    │     │     │  │           │     │     ╰─ ite_profiles/
    │     │     │  │           │     │        ├─ new [*]
    │     │     │  │           │     │        ╰─ {id}
    │     │     │  │           │     │           ╰─ /edit [*]
    │     │     │  │           │     ├─ api_fuzzing [*]
    │     │     │  │           │     ╰─ dast [*]
    │     │     │  │           ╰─ d
    │     │     │  │              ├─ ashboard [*]
    │     │     │  │              ╰─ iscover [*]
    │     │     │  ├─ t
    │     │     │  │  ├─ erraform [*]
    │     │     │  │  │  ╰─ _module_registry [*]
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ╰─ {id} [*]
    │     │     │  │  ├─ a
    │     │     │  │  │  ├─ rget_branch_rules [*]
    │     │     │  │  │  │  ╰─ /
    │     │     │  │  │  │     ╰─ {id} [*]
    │     │     │  │  │  ╰─ gs [*]
    │     │     │  │  │     ╰─ /
    │     │     │  │  │        ├─ new [*]
    │     │     │  │  │        ╰─ {id} [*]
    │     │     │  │  ╰─ r
    │     │     │  │     ├─ iggers [*]
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ {id} [*]
    │     │     │  │     ├─ acing [*]
    │     │     │  │     │  ╰─ /
    │     │     │  │     │     ╰─ {id} [*]
    │     │     │  │     ╰─ ee/
    │     │     │  │        ╰─ {*id} [*]
    │     │     │  ├─ u
    │     │     │  │  ├─ sage_quotas [*]
    │     │     │  │  ╰─ pdate/
    │     │     │  │     ╰─ {*id} [*]
    │     │     │  ├─ v
    │     │     │  │  ├─ ulnerability_feedback [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ count [*]
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  ╰─ a
    │     │     │  │     ├─ lue_stream_analytics [*]
    │     │     │  │     │  ╰─ /events/
    │     │     │  │     │     ├─ staging [*]
    │     │     │  │     │     ├─ review [*]
    │     │     │  │     │     ├─ issue [*]
    │     │     │  │     │     ├─ code [*]
    │     │     │  │     │     ├─ p
    │     │     │  │     │     │  ├─ roduction [*]
    │     │     │  │     │     │  ╰─ lan [*]
    │     │     │  │     │     ╰─ test [*]
    │     │     │  │     ╰─ riables [*]
    │     │     │  ├─ w
    │     │     │  │  ├─ ikis [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ├─ -/confluence [*]
    │     │     │  │  │     ├─ git_access [*]
    │     │     │  │  │     ├─ templates [*]
    │     │     │  │  │     ├─ pages [*]
    │     │     │  │  │     ├─ new [*]
    │     │     │  │  │     ├─ {*id}
    │     │     │  │  │     │  ╰─ /
    │     │     │  │  │     │     ├─ preview_markdown [*]
    │     │     │  │  │     │     ├─ history [*]
    │     │     │  │  │     │     ├─ diff [*]
    │     │     │  │  │     │     ├─ edit [*]
    │     │     │  │  │     │     ╰─ raw [*]
    │     │     │  │  │     ╰─ {*id} [*]
    │     │     │  │  ╰─ ork_items/
    │     │     │  │     ├─ import_csv [*]
    │     │     │  │     │  ╰─ /authorize [*]
    │     │     │  │     ╰─ {iid} [*]
    │     │     │  │        ╰─ /designs(/
    │     │     │  │           ╰─ {*vueroute}
    │     │     │  │              ╰─ ) [*]
    │     │     │  ╰─ {noteable_type}
    │     │     │     ╰─ /
    │     │     │        ╰─ {noteable_id}
    │     │     │           ╰─ /discussions/
    │     │     │              ╰─ {id} [*]
    │     │     │                 ╰─ /resolve [*]
    │     │     ├─ fi
    │     │     │  ├─ nd_file(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ╰─ les(/
    │     │     │     ╰─ {*rest}
    │     │     │        ╰─ ) [*]
    │     │     ├─ a
    │     │     │  ├─ udit_events(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ╰─ lert
    │     │     │     ├─ s/notify [*]
    │     │     │     │  ╰─ /
    │     │     │     │     ╰─ {name}
    │     │     │     │        ╰─ /
    │     │     │     │           ╰─ {endpoint_identifier} [*]
    │     │     │     ╰─ _management(/
    │     │     │        ╰─ {*rest}
    │     │     │           ╰─ ) [*]
    │     │     ├─ b
    │     │     │  ├─ adges [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ {*ref}
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ coverage(.
    │     │     │  │           │  ╰─ {format}
    │     │     │  │           │     ╰─ ) [*]
    │     │     │  │           ╰─ pipeline(.
    │     │     │  │              ╰─ {format}
    │     │     │  │                 ╰─ ) [*]
    │     │     │  ├─ uilds [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ artifacts/
    │     │     │  │     │  ╰─ {*ref_name_and_path} [*]
    │     │     │  │     ├─ {build_id}
    │     │     │  │     │  ╰─ /artifacts/
    │     │     │  │     │     ├─ download [*]
    │     │     │  │     │     ├─ browse(/
    │     │     │  │     │     │  ╰─ {*path}
    │     │     │  │     │     │     ╰─ ) [*]
    │     │     │  │     │     ├─ file/
    │     │     │  │     │     │  ╰─ {*path} [*]
    │     │     │  │     │     ╰─ raw/
    │     │     │  │     │        ╰─ {*path} [*]
    │     │     │  │     ╰─ {id} [*]
    │     │     │  │        ╰─ /raw [*]
    │     │     │  ╰─ l
    │     │     │     ├─ ame/
    │     │     │     │  ╰─ {*id} [*]
    │     │     │     ╰─ ob/
    │     │     │        ╰─ {*id} [*]
    │     │     ├─ c
    │     │     │  ├─ ycle_analytics(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ├─ lusters(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ╰─ o
    │     │     │     ├─ ntainer_registry [*]
    │     │     │     │  ╰─ /
    │     │     │     │     ╰─ {id} [*]
    │     │     │     ╰─ m
    │     │     │        ├─ pare(/
    │     │     │        │  ╰─ {*rest}
    │     │     │        │     ╰─ ) [*]
    │     │     │        ╰─ mit
    │     │     │           ├─ s(/
    │     │     │           │  ╰─ {*rest}
    │     │     │           │     ╰─ ) [*]
    │     │     │           ╰─ (/
    │     │     │              ╰─ {*rest}
    │     │     │                 ╰─ ) [*]
    │     │     ├─ e
    │     │     │  ├─ rror_tracking(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ├─ nvironments(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ╰─ dit/
    │     │     │     ╰─ {*id} [*]
    │     │     ├─ i
    │     │     │  ├─ nsights [*]
    │     │     │  │  ╰─ /query [*]
    │     │     │  ├─ de_terminals
    │     │     │  │  ├─ (.
    │     │     │  │  │  ╰─ {format}
    │     │     │  │  │     ╰─ ) [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ├─ check_config(.
    │     │     │  │     │  ╰─ {format}
    │     │     │  │     │     ╰─ ) [*]
    │     │     │  │     ╰─ {id}
    │     │     │  │        ├─ (.
    │     │     │  │        │  ╰─ {format}
    │     │     │  │        │     ╰─ ) [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ cancel(.
    │     │     │  │           │  ╰─ {format}
    │     │     │  │           │     ╰─ ) [*]
    │     │     │  │           ╰─ retry(.
    │     │     │  │              ╰─ {format}
    │     │     │  │                 ╰─ ) [*]
    │     │     │  ╰─ ssues(/
    │     │     │     ╰─ {*rest}
    │     │     │        ╰─ ) [*]
    │     │     ├─ m
    │     │     │  ├─ erge_requests(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ├─ attermost(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ╰─ irror(/
    │     │     │     ╰─ {*rest}
    │     │     │        ╰─ ) [*]
    │     │     ├─ n
    │     │     │  ├─ ew/
    │     │     │  │  ╰─ {*id} [*]
    │     │     │  ╰─ ote
    │     │     │     ├─ s [*]
    │     │     │     │  ╰─ /
    │     │     │     │     ╰─ {id} [*]
    │     │     │     │        ╰─ /
    │     │     │     │           ├─ outdated_line_change [*]
    │     │     │     │           ├─ toggle_award_emoji [*]
    │     │     │     │           ├─ delete_attachment [*]
    │     │     │     │           ╰─ resolve [*]
    │     │     │     ╰─ able/
    │     │     │        ╰─ {target_type}
    │     │     │           ╰─ /
    │     │     │              ╰─ {target_id}
    │     │     │                 ╰─ /notes [*]
    │     │     ├─ p
    │     │     │  ├─ ipeline
    │     │     │  │  ├─ _schedules(/
    │     │     │  │  │  ╰─ {*rest}
    │     │     │  │  │     ╰─ ) [*]
    │     │     │  │  ╰─ s(/
    │     │     │  │     ╰─ {*rest}
    │     │     │  │        ╰─ ) [*]
    │     │     │  ├─ ro
    │     │     │  │  ├─ tected_environments(/
    │     │     │  │  │  ╰─ {*rest}
    │     │     │  │  │     ╰─ ) [*]
    │     │     │  │  ╰─ metheus/
    │     │     │  │     ├─ alerts/
    │     │     │  │     │  ├─ notify [*]
    │     │     │  │     │  ╰─ {id}
    │     │     │  │     │     ╰─ /metrics_dashboard [*]
    │     │     │  │     ╰─ metrics [*]
    │     │     │  │        ╰─ /
    │     │     │  │           ├─ validate_query [*]
    │     │     │  │           ├─ active_common [*]
    │     │     │  │           ├─ new [*]
    │     │     │  │           ╰─ {id} [*]
    │     │     │  │              ╰─ /edit [*]
    │     │     │  ╰─ a
    │     │     │     ├─ th_locks [*]
    │     │     │     │  ╰─ /
    │     │     │     │     ├─ toggle [*]
    │     │     │     │     ╰─ {id} [*]
    │     │     │     ╰─ ges [*]
    │     │     │        ╰─ /
    │     │     │           ├─ domains [*]
    │     │     │           │  ╰─ /
    │     │     │           │     ├─ new [*]
    │     │     │           │     ╰─ {id} [*]
    │     │     │           │        ╰─ /
    │     │     │           │           ├─ clean_certificate [*]
    │     │     │           │           ├─ retry_auto_ssl [*]
    │     │     │           │           ├─ verify [*]
    │     │     │           │           ╰─ edit [*]
    │     │     │           ╰─ new [*]
    │     │     ├─ r
    │     │     │  ├─ unner
    │     │     │  │  ├─ _projects [*]
    │     │     │  │  │  ╰─ /
    │     │     │  │  │     ╰─ {id} [*]
    │     │     │  │  ╰─ s(/
    │     │     │  │     ╰─ {*rest}
    │     │     │  │        ╰─ ) [*]
    │     │     │  ├─ aw/
    │     │     │  │  ╰─ {*id} [*]
    │     │     │  ╰─ e
    │     │     │     ├─ fs/
    │     │     │     │  ├─ switch [*]
    │     │     │     │  ╰─ {id}
    │     │     │     │     ╰─ /logs_tree [*]
    │     │     │     │        ╰─ /
    │     │     │     │           ╰─ {*path} [*]
    │     │     │     ├─ pository [*]
    │     │     │     ├─ store [*]
    │     │     │     ╰─ gistry/repository/
    │     │     │        ╰─ {repository_id}
    │     │     │           ╰─ /tags [*]
    │     │     │              ╰─ /
    │     │     │                 ├─ bulk_destroy [*]
    │     │     │                 ╰─ {id} [*]
    │     │     ├─ s
    │     │     │  ├─ nippets
    │     │     │  │  ├─ (/
    │     │     │  │  │  ╰─ {*rest}
    │     │     │  │  │     ╰─ ) [*]
    │     │     │  │  ╰─ /
    │     │     │  │     ╰─ {id}
    │     │     │  │        ╰─ /raw [*]
    │     │     │  ╰─ e
    │     │     │     ├─ curity(/
    │     │     │     │  ╰─ {*rest}
    │     │     │     │     ╰─ ) [*]
    │     │     │     ╰─ rv
    │     │     │        ├─ erless(/
    │     │     │        │  ╰─ {*rest}
    │     │     │        │     ╰─ ) [*]
    │     │     │        ╰─ ice_
    │     │     │           ├─ ping/web_ide_pipelines_count [*]
    │     │     │           ╰─ desk [*]
    │     │     ├─ t
    │     │     │  ├─ odos [*]
    │     │     │  ├─ emplates/
    │     │     │  │  ╰─ {template_type}
    │     │     │  │     ├─ (.
    │     │     │  │     │  ╰─ {format}
    │     │     │  │     │     ╰─ ) [*]
    │     │     │  │     ╰─ /
    │     │     │  │        ╰─ {key}
    │     │     │  │           ╰─ (.
    │     │     │  │              ╰─ {format}
    │     │     │  │                 ╰─ ) [*]
    │     │     │  ├─ ags(/
    │     │     │  │  ╰─ {*rest}
    │     │     │  │     ╰─ ) [*]
    │     │     │  ╰─ r
    │     │     │     ├─ iggers(/
    │     │     │     │  ╰─ {*rest}
    │     │     │     │     ╰─ ) [*]
    │     │     │     ╰─ ee/
    │     │     │        ╰─ {*id} [*]
    │     │     ╰─ {*all} [*]
    │     ╰─ {id} [*]
    │        ╰─ /
    │           ├─ new_issuable_address [*]
    │           ├─ generate_new_export [*]
    │           ├─ download_export [*]
    │           ├─ housekeeping [*]
    │           ├─ un
    │           │  ├─ foldered_environment_names [*]
    │           │  ╰─ archive [*]
    │           ├─ e
    │           │  ├─ xport [*]
    │           │  ╰─ dit [*]
    │           ├─ re
    │           │  ├─ fs [*]
    │           │  ╰─ move_
    │           │     ├─ export [*]
    │           │     ╰─ fork [*]
    │           ├─ a
    │           │  ├─ ctivity [*]
    │           │  ╰─ rchive [*]
    │           ╰─ t
    │              ├─ oggle_star [*]
    │              ╰─ ransfer [*]
    ├─ {*id}
    │  ╰─ (.
    │     ╰─ {format}
    │        ╰─ ) [*]
    ├─ {*repository_path} [*]
    ╰─ {*unmatched_route} [*]
    "#);

    Ok(())
}

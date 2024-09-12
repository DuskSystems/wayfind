#[must_use]
#[allow(clippy::too_many_lines)]
pub fn routes() -> impl IntoIterator<Item = &'static str> {
    [
        // Route 1
        "/favicon.png{/}",
        // Route 2
        "/favicon.ico{/}",
        // Route 3
        "/rails/mailers{/}",
        // Route 4
        "/rails/mailers/{path}{/}",
        // Route 5
        "/rails/info/properties{/}",
        // Route 6
        "/rails/info/routes{/}",
        // Route 7
        "/rails/info{/}",
        // Route 8
        // NOTE: See LetterOpenerWeb::Engine
        // "/rails/letter_opener{/}",
        // Route 9
        // NOTE: See Lookbook::Engine
        // "/rails/lookbook{/}",
        // Route 10
        // NOTE: See Toogle::Engine
        // "/rails/features{/}",
        // Route 11
        "/oauth/authorize/native{/}",
        // Route 12
        "/oauth/authorize{/}",
        // Route 13
        // "/oauth/authorize{/}",
        // Route 14
        // "/oauth/authorize{/}",
        // Route 15
        "/oauth/token{/}",
        // Route 16
        "/oauth/revoke{/}",
        // Route 17
        "/oauth/introspect{/}",
        // Route 18
        "/oauth/applications{/}",
        // Route 19
        // "/oauth/applications{/}",
        // Route 20
        "/oauth/applications/new{/}",
        // Route 21
        "/oauth/applications/{id}/edit{/}",
        // Route 22
        "/oauth/applications/{id}{/}",
        // Route 23
        // "/oauth/applications/{id}{/}",
        // Route 24
        // "/oauth/applications/{id}{/}",
        // Route 25
        // "/oauth/applications/{id}{/}",
        // Route 26
        "/oauth/authorized_applications{/}",
        // Route 27
        "/oauth/authorized_applications/{id}{/}",
        // Route 28
        "/oauth/token/info{/}",
        // Route 29
        "/oauth/applications/{id}/renew{/}",
        // Route 30
        "/oauth/geo/auth{/}",
        // Route 31
        "/oauth/geo/callback{/}",
        // Route 32
        "/oauth/geo/logout{/}",
        // Route 33
        "/oauth/userinfo{/}",
        // Route 34
        // "/oauth/userinfo{/}",
        // Route 35
        "/oauth/discovery/keys{/}",
        // Route 36
        "/.well-known/openid-configuration{/}",
        // Route 37
        "/.well-known/oauth-authorization-server{/}",
        // Route 38
        "/.well-known/webfinger{/}",
        // Route 39
        "/oauth/authorize_device{/}",
        // Route 40
        "/oauth/device{/}",
        // Route 41
        // "/oauth/device{/}",
        // Route 42
        // "/oauth/userinfo{/}",
        // Route 43
        // "/oauth/discovery/keys{/}",
        // Route 44
        // "/.well-known/openid-configuration{/}",
        // Route 45
        // "/.well-known/webfinger{/}",
        // Route 46
        // "/oauth/token{/}",
        // Route 47
        // "/oauth/revoke{/}",
        // Route 48
        "/-/jira_connect/oauth_application_id{/}",
        // Route 49
        "/-/jira_connect/subscriptions{/}",
        // Route 50
        "/-/jira_connect/subscriptions/{id}{/}",
        // Route 51
        "/users/sign_up/welcome{/}",
        // Route 52
        // "/users/sign_up/welcome{/}",
        // Route 53
        // "/users/sign_up/welcome{/}",
        // Route 54
        "/users/sign_up/company/new{/}",
        // Route 55
        "/users/sign_up/company{/}",
        // Route 56
        "/users/sign_up/groups{/}",
        // Route 57
        "/users/sign_up/groups/new{/}",
        // Route 58
        "/search{/}",
        // Route 59
        "/search/autocomplete{/}",
        // Route 60
        "/search/settings{/}",
        // Route 61
        "/search/count{/}",
        // Route 62
        "/search/opensearch{/}",
        // Route 63
        "/search/aggregations{/}",
        // Route 64
        "/jwt/auth{/}",
        // Route 65
        // "/jwt/auth{/}",
        // Route 66
        "/health_check/{checks?}{/}",
        // Route 67
        "/.well-known/terraform.json{/}",
        // Route 68
        "/-/autocomplete/users{/}",
        // Route 69
        "/-/autocomplete/users/{id}{/}",
        // Route 70
        "/-/autocomplete/projects{/}",
        // Route 71
        "/-/autocomplete/award_emojis{/}",
        // Route 72
        "/-/autocomplete/merge_request_target_branches{/}",
        // Route 73
        "/-/autocomplete/merge_request_source_branches{/}",
        // Route 74
        "/-/autocomplete/deploy_keys_with_owners{/}",
        // Route 75
        "/-/autocomplete/project_groups{/}",
        // Route 76
        "/-/autocomplete/project_routes{/}",
        // Route 77
        "/-/autocomplete/namespace_routes{/}",
        // Route 78
        "/-/autocomplete/group_subgroups{/}",
        // Route 79
        "/-/sandbox/mermaid{/}",
        // Route 80
        "/-/sandbox/swagger{/}",
        // Route 81
        "/-/{model}/{model_id}/uploads/{secret}/{filename}{/}",
        // Route 82
        "/-/whats_new{/}",
        // Route 83
        "/-/offline{/}",
        // Route 84
        "/-/manifest{/}",
        // Route 85
        "/-/kubernetes{/}",
        // Route 86
        "/-/kubernetes/{agent_id}/{*vueroute?}{/}",
        // Route 87
        "/-/liveness{/}",
        // Route 88
        "/-/readiness{/}",
        // Route 89
        "/-/metrics{/}",
        // Route 90
        "/-/metrics/system{/}",
        // Route 91
        // NOTE: See Peek::Railtie
        // "/-/peek{/}",
        // Route 92
        "/-/runner_setup/platforms{/}",
        // Route 93
        "/-/acme-challenge{/}",
        // Route 94
        "/-/ide{/}",
        // Route 95
        "/-/ide/project{/}",
        // Route 96
        "/-/ide/oauth_redirect{/}",
        // Route 97
        "/-/ide/project/{project_id}/edit{/}",
        // Route 98
        "/-/ide/project/{project_id}/edit/{*branch}/-/{*path}{/}",
        // Route 99
        "/-/ide/project/{project_id}/edit/{*branch}/-{/}",
        // Route 100
        "/-/ide/project/{project_id}/edit/{*branch}{/}",
        // Route 101
        "/-/ide/project/{project_id}/tree{/}",
        // Route 102
        "/-/ide/project/{project_id}/tree/{*branch}/-/{*path}{/}",
        // Route 103
        "/-/ide/project/{project_id}/tree/{*branch}/-{/}",
        // Route 104
        "/-/ide/project/{project_id}/tree/{*branch}{/}",
        // Route 105
        "/-/ide/project/{project_id}/blob{/}",
        // Route 106
        "/-/ide/project/{project_id}/blob/{*branch}/-/{*path}{/}",
        // Route 107
        "/-/ide/project/{project_id}/blob/{*branch}/-{/}",
        // Route 108
        "/-/ide/project/{project_id}/blob/{*branch}{/}",
        // Route 109
        "/-/ide/project/{project_id}/merge_requests/{merge_request_id}{/}",
        // Route 110
        "/-/ide/project/{project_id}{/}",
        // Route 111
        "/-/ide/reset_oauth_application_settings{/}",
        // Route 112
        "/-/operations{/}",
        // Route 113
        // "/-/operations{/}",
        // Route 114
        // "/-/operations{/}",
        // Route 115
        "/-/operations/environments{/}",
        // Route 116
        // "/-/operations/environments{/}",
        // Route 117
        // "/-/operations/environments{/}",
        // Route 118
        "/-/jira_connect{/}",
        // Route 119
        "/-/jira_connect/app_descriptor{/}",
        // Route 120
        "/-/jira_connect/events/installed{/}",
        // Route 121
        "/-/jira_connect/events/uninstalled{/}",
        // Route 122
        // "/-/jira_connect/subscriptions{/}",
        // Route 123
        // "/-/jira_connect/subscriptions{/}",
        // Route 124
        // "/-/jira_connect/subscriptions/{id}{/}",
        // Route 125
        "/-/jira_connect/branches/route{/}",
        // Route 126
        "/-/jira_connect/branches/new{/}",
        // Route 127
        "/-/jira_connect/public_keys/{id}{/}",
        // Route 128
        "/-/jira_connect/workspaces/search{/}",
        // Route 129
        "/-/jira_connect/repositories/search{/}",
        // Route 130
        "/-/jira_connect/repositories/associate{/}",
        // Route 131
        "/-/jira_connect/installations{/}",
        // Route 132
        // "/-/jira_connect/installations{/}",
        // Route 133
        "/-/jira_connect/oauth_callbacks{/}",
        // Route 134
        // "/-/jira_connect/oauth_application_id{/}",
        // Route 135
        "/-/organizations/preview_markdown{/}",
        // Route 136
        "/-/organizations/{organization_path}/activity{/}",
        // Route 137
        "/-/organizations/{organization_path}/groups_and_projects{/}",
        // Route 138
        "/-/organizations/{organization_path}/users{/}",
        // Route 139
        "/-/organizations/{organization_path}/settings/general{/}",
        // Route 140
        "/-/organizations/{organization_path}/groups/new{/}",
        // Route 141
        "/-/organizations/{organization_path}/groups{/}",
        // Route 142
        // "/-/organizations/{organization_path}/groups{/}",
        // Route 143
        "/-/organizations/{organization_path}/groups/{*id}/edit{/}",
        // Route 144
        "/-/organizations/{organization_path}/projects/{*namespace_id}/{id}/edit{/}",
        // Route 145
        "/-/organizations{/}",
        // Route 146
        "/-/organizations/new{/}",
        // Route 147
        "/-/organizations/{organization_path}{/}",
        // Route 148
        "/-/remote_development/workspaces/{*vueroute?}/{workspace_id}/workspaces{/}",
        // Route 149
        "/-/remote_development/workspaces/{*vueroute?}/{workspace_id}/workspaces/new{/}",
        // Route 150
        "/-/remote_development/workspaces/{*vueroute?}{/}",
        // Route 151
        // "/-/remote_development/workspaces/{*vueroute?}{/}",
        // Route 152
        "/-/remote_development/workspaces/{*vueroute?}/new{/}",
        // Route 153
        "/-/remote_development/workspaces/{*vueroute?}/{id}/edit{/}",
        // Route 154
        "/-/remote_development/workspaces/{*vueroute?}/{id}{/}",
        // Route 155
        // "/-/remote_development/workspaces/{*vueroute?}/{id}{/}",
        // Route 156
        // "/-/remote_development/workspaces/{*vueroute?}/{id}{/}",
        // Route 157
        // "/-/remote_development/workspaces/{*vueroute?}/{id}{/}",
        // Route 158
        "/-/remote_development/workspaces_feature_flag{/}",
        // Route 159
        "/-/security{/}",
        // Route 160
        "/-/security/dashboard/settings{/}",
        // Route 161
        "/-/security/dashboard{/}",
        // Route 162
        "/-/security/projects{/}",
        // Route 163
        // "/-/security/projects{/}",
        // Route 164
        "/-/security/projects/{id}{/}",
        // Route 165
        "/-/security/vulnerabilities{/}",
        // Route 166
        "/-/smartcard/auth{/}",
        // Route 167
        "/-/smartcard/extract_certificate{/}",
        // Route 168
        "/-/smartcard/verify_certificate{/}",
        // Route 169
        "/-/trial_registrations{/}",
        // Route 170
        "/-/trial_registrations/new{/}",
        // Route 171
        "/-/countries{/}",
        // Route 172
        "/-/country_states{/}",
        // Route 173
        "/-/subscriptions/buy_minutes{/}",
        // Route 174
        "/-/subscriptions/buy_storage{/}",
        // Route 175
        "/-/subscriptions/payment_form{/}",
        // Route 176
        "/-/subscriptions/payment_method{/}",
        // Route 177
        "/-/subscriptions/validate_payment_method{/}",
        // Route 178
        "/-/subscriptions/groups{/}",
        // Route 179
        "/-/subscriptions/groups/new{/}",
        // Route 180
        "/-/subscriptions/groups/{id}/edit{/}",
        // Route 181
        "/-/subscriptions/groups/{id}{/}",
        // Route 182
        // "/-/subscriptions/groups/{id}{/}",
        // Route 183
        "/-/subscriptions/hand_raise_leads{/}",
        // Route 184
        "/-/subscriptions/new{/}",
        // Route 185
        "/-/subscriptions{/}",
        // Route 186
        "/-/trials{/}",
        // Route 187
        "/-/trials/new{/}",
        // Route 188
        "/-/trials/duo_pro/new{/}",
        // Route 189
        "/-/trials/duo_pro{/}",
        // Route 190
        "/-/trials/duo_enterprise/new{/}",
        // Route 191
        "/-/trials/duo_enterprise{/}",
        // Route 192
        "/-/phone_verification/telesign_callback{/}",
        // Route 193
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/refs{/}",
        // Route 194
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/git-upload-pack{/}",
        // Route 195
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/git-receive-pack{/}",
        // Route 196
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/ssh-upload-pack{/}",
        // Route 197
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/ssh-receive-pack{/}",
        // Route 198
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/objects/batch{/}",
        // Route 199
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/objects{/}",
        // Route 200
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/objects/{*oid}{/}",
        // Route 201
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks/{id}/unlock{/}",
        // Route 202
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks/verify{/}",
        // Route 203
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks{/}",
        // Route 204
        // "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks{/}",
        // Route 205
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks/new{/}",
        // Route 206
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks/{id}/edit{/}",
        // Route 207
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks/{id}{/}",
        // Route 208
        // "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks/{id}{/}",
        // Route 209
        // "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks/{id}{/}",
        // Route 210
        // "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/lfs/locks/{id}{/}",
        // Route 211
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/gitlab-lfs/objects/{*oid}{/}",
        // Route 212
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/gitlab-lfs/objects/{*oid}/{size}/authorize{/}",
        // Route 213
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}/gitlab-lfs/objects/{*oid}/{size}{/}",
        // Route 214
        "/-/push_from_secondary/{geo_node_id}/{*repository_path}{/}",
        // Route 215
        // "/-/push_from_secondary/{geo_node_id}/{*repository_path}/info/refs{/}",
        // Route 216
        "/-/chaos/leakmem{/}",
        // Route 217
        "/-/chaos/cpu_spin{/}",
        // Route 218
        "/-/chaos/db_spin{/}",
        // Route 219
        "/-/chaos/sleep{/}",
        // Route 220
        "/-/chaos/kill{/}",
        // Route 221
        "/-/chaos/quit{/}",
        // Route 222
        "/-/chaos/gc{/}",
        // Route 223
        "/-/invites/{id}/accept{/}",
        // Route 224
        "/-/invites/{id}/decline{/}",
        // Route 225
        "/-/invites/{id}{/}",
        // Route 226
        "/-/sent_notifications/{id}/unsubscribe{/}",
        // Route 227
        "/-/abuse_reports/add_category{/}",
        // Route 228
        "/-/abuse_reports{/}",
        // Route 229
        "/-/jwks{/}",
        // Route 230
        "/-/snippets/{id}/raw{/}",
        // Route 231
        "/-/snippets/{id}/mark_as_spam{/}",
        // Route 232
        "/-/snippets/preview_markdown{/}",
        // Route 233
        "/-/snippets/{snippet_id}/notes/{id}/delete_attachment{/}",
        // Route 234
        "/-/snippets/{snippet_id}/notes/{id}/toggle_award_emoji{/}",
        // Route 235
        "/-/snippets/{snippet_id}/notes{/}",
        // Route 236
        // "/-/snippets/{snippet_id}/notes{/}",
        // Route 237
        "/-/snippets/{snippet_id}/notes/{id}{/}",
        // Route 238
        // "/-/snippets/{snippet_id}/notes/{id}{/}",
        // Route 239
        // "/-/snippets/{snippet_id}/notes/{id}{/}",
        // Route 240
        "/-/snippets/{id}/toggle_award_emoji{/}",
        // Route 241
        "/-/snippets{/}",
        // Route 242
        "/-/snippets/new{/}",
        // Route 243
        "/-/snippets/{id}/edit{/}",
        // Route 244
        "/-/snippets/{id}{/}",
        // Route 245
        "/-/snippets/{snippet_id}/raw/{ref}/{*path}{/}",
        // Route 246
        "/-/s/{username}{/}",
        // Route 247
        "/-/profile/usage_quotas{/}",
        // Route 248
        "/-/profile/billings{/}",
        // Route 249
        "/-/profile/emails/confirmation/new{/}",
        // Route 250
        "/-/profile/emails/confirmation{/}",
        // Route 251
        // "/-/profile/emails/confirmation{/}",
        // Route 252
        "/-/profile/audit_log{/}",
        // Route 253
        "/-/profile/applications{/}",
        // Route 254
        "/-/profile/reset_incoming_email_token{/}",
        // Route 255
        "/-/profile/reset_feed_token{/}",
        // Route 256
        "/-/profile/reset_static_object_token{/}",
        // Route 257
        "/-/profile/update_username{/}",
        // Route 258
        "/-/profile/join_early_access_program{/}",
        // Route 259
        "/-/profile/account/unlink{/}",
        // Route 260
        "/-/profile/account{/}",
        // Route 261
        "/-/profile/groups/{*id}/notifications.{format?}{/}",
        // Route 262
        // "/-/profile/groups/{*id}/notifications.{format?}{/}",
        // Route 263
        "/-/profile/notifications{/}",
        // Route 264
        // "/-/profile/notifications{/}",
        // Route 265
        // "/-/profile/notifications{/}",
        // Route 266
        "/-/profile/slack/slack_link{/}",
        // Route 267
        "/-/profile/slack/edit{/}",
        // Route 268
        "/-/profile/preferences{/}",
        // Route 269
        // "/-/profile/preferences{/}",
        // Route 270
        // "/-/profile/preferences{/}",
        // Route 271
        "/-/profile/comment_templates{/}",
        // Route 272
        "/-/profile/comment_templates/{id}{/}",
        // Route 273
        "/-/profile/emails/{id}/resend_confirmation_instructions{/}",
        // Route 274
        "/-/profile/emails{/}",
        // Route 275
        // "/-/profile/emails{/}",
        // Route 276
        "/-/profile/emails/{id}{/}",
        // Route 277
        "/-/profile/chat_names/deny{/}",
        // Route 278
        "/-/profile/chat_names{/}",
        // Route 279
        // "/-/profile/chat_names{/}",
        // Route 280
        "/-/profile/chat_names/new{/}",
        // Route 281
        "/-/profile/chat_names/{id}{/}",
        // Route 282
        "/-/profile/avatar{/}",
        // Route 283
        "/-/profile/two_factor_auth/codes{/}",
        // Route 284
        "/-/profile/two_factor_auth/skip{/}",
        // Route 285
        "/-/profile/two_factor_auth/create_webauthn{/}",
        // Route 286
        "/-/profile/two_factor_auth{/}",
        // Route 287
        // "/-/profile/two_factor_auth{/}",
        // Route 288
        // "/-/profile/two_factor_auth{/}",
        // Route 289
        "/-/profile/webauthn_registrations/{id}{/}",
        // Route 290
        // "/-/profile/usage_quotas{/}",
        // Route 291
        "/-/user_settings/active_sessions/saml{/}",
        // Route 292
        "/-/user_settings/authentication_log{/}",
        // Route 293
        "/-/user_settings/applications{/}",
        // Route 294
        "/-/user_settings/active_sessions{/}",
        // Route 295
        "/-/user_settings/active_sessions/{id}{/}",
        // Route 296
        "/-/user_settings/profile{/}",
        // Route 297
        // "/-/user_settings/profile{/}",
        // Route 298
        // "/-/user_settings/profile{/}",
        // Route 299
        "/-/user_settings/identities/new{/}",
        // Route 300
        "/-/user_settings/identities{/}",
        // Route 301
        "/-/user_settings/password/reset{/}",
        // Route 302
        "/-/user_settings/password/new{/}",
        // Route 303
        "/-/user_settings/password/edit{/}",
        // Route 304
        "/-/user_settings/password{/}",
        // Route 305
        // "/-/user_settings/password{/}",
        // Route 306
        // "/-/user_settings/password{/}",
        // Route 307
        "/-/user_settings/personal_access_tokens/{id}/revoke{/}",
        // Route 308
        "/-/user_settings/personal_access_tokens{/}",
        // Route 309
        // "/-/user_settings/personal_access_tokens{/}",
        // Route 310
        "/-/user_settings/gpg_keys/{id}/revoke{/}",
        // Route 311
        "/-/user_settings/gpg_keys{/}",
        // Route 312
        // "/-/user_settings/gpg_keys{/}",
        // Route 313
        "/-/user_settings/gpg_keys/{id}{/}",
        // Route 314
        "/-/user_settings/ssh_keys/{id}/revoke{/}",
        // Route 315
        "/-/user_settings/ssh_keys{/}",
        // Route 316
        // "/-/user_settings/ssh_keys{/}",
        // Route 317
        "/-/user_settings/ssh_keys/{id}{/}",
        // Route 318
        // "/-/user_settings/ssh_keys/{id}{/}",
        // Route 319
        "/-/mailgun/webhooks{/}",
        // Route 320
        "/-/members/mailgun/permanent_failures{/}",
        // Route 321
        "/-/timelogs{/}",
        // Route 322
        "/-/track_namespace_visits{/}",
        // Route 323
        "/-/external_redirect{/}",
        // Route 324
        "/groups{/}",
        // Route 325
        // "/groups{/}",
        // Route 326
        "/groups/new{/}",
        // Route 327
        "/groups/{*group_id}/-/wikis/git_access{/}",
        // Route 328
        "/groups/{*group_id}/-/wikis/pages{/}",
        // Route 329
        "/groups/{*group_id}/-/wikis/templates{/}",
        // Route 330
        "/groups/{*group_id}/-/wikis/new{/}",
        // Route 331
        "/groups/{*group_id}/-/wikis{/}",
        // Route 332
        // "/groups/{*group_id}/-/wikis{/}",
        // Route 333
        "/groups/{*group_id}/-/wikis/-/confluence{/}",
        // Route 334
        "/groups/{*group_id}/-/wikis/{*id}/edit{/}",
        // Route 335
        "/groups/{*group_id}/-/wikis/{*id}/history{/}",
        // Route 336
        "/groups/{*group_id}/-/wikis/{*id}/diff{/}",
        // Route 337
        "/groups/{*group_id}/-/wikis/{*id}/raw{/}",
        // Route 338
        "/groups/{*group_id}/-/wikis/{*id}/preview_markdown{/}",
        // Route 339
        "/groups/{*group_id}/-/wikis/{*id}{/}",
        // Route 340
        // "/groups/{*group_id}/-/wikis/{*id}{/}",
        // Route 341
        // "/groups/{*group_id}/-/wikis/{*id}{/}",
        // Route 342
        "/groups/{*group_id}/-/settings/reporting{/}",
        // Route 343
        "/groups/{*group_id}/-/settings/domain_verification/{id}/verify{/}",
        // Route 344
        "/groups/{*group_id}/-/settings/domain_verification/{id}/retry_auto_ssl{/}",
        // Route 345
        "/groups/{*group_id}/-/settings/domain_verification/{id}/clean_certificate{/}",
        // Route 346
        "/groups/{*group_id}/-/settings/domain_verification{/}",
        // Route 347
        // "/groups/{*group_id}/-/settings/domain_verification{/}",
        // Route 348
        "/groups/{*group_id}/-/settings/domain_verification/new{/}",
        // Route 349
        "/groups/{*group_id}/-/settings/domain_verification/{id}{/}",
        // Route 350
        // "/groups/{*group_id}/-/settings/domain_verification/{id}{/}",
        // Route 351
        // "/groups/{*group_id}/-/settings/domain_verification/{id}{/}",
        // Route 352
        // "/groups/{*group_id}/-/settings/domain_verification/{id}{/}",
        // Route 353
        "/groups/{*group_id}/-/settings/merge_requests{/}",
        // Route 354
        // "/groups/{*group_id}/-/settings/merge_requests{/}",
        // Route 355
        "/groups/{*group_id}/-/settings/roles_and_permissions{/}",
        // Route 356
        "/groups/{*group_id}/-/settings/roles_and_permissions/new{/}",
        // Route 357
        "/groups/{*group_id}/-/settings/roles_and_permissions/{id}/edit{/}",
        // Route 358
        "/groups/{*group_id}/-/settings/roles_and_permissions/{id}{/}",
        // Route 359
        "/groups/{*group_id}/-/settings/analytics{/}",
        // Route 360
        // "/groups/{*group_id}/-/settings/analytics{/}",
        // Route 361
        // "/groups/{*group_id}/-/settings/analytics{/}",
        // Route 362
        "/groups/{*group_id}/-/settings/gitlab_duo_usage{/}",
        // Route 363
        "/groups/{*group_id}/-/settings/workspaces{/}",
        // Route 364
        "/groups/{*group_id}/-/group_members/{id}/override{/}",
        // Route 365
        "/groups/{*group_id}/-/group_members/{id}/unban{/}",
        // Route 366
        "/groups/{*group_id}/-/group_members/{id}/ban{/}",
        // Route 367
        "/groups/{*group_id}/-/group_members/export_csv{/}",
        // Route 368
        "/groups/{*group_id}/-/group_members/request_access{/}",
        // Route 369
        // "/groups/{*group_id}/-/group_members/request_access{/}",
        // Route 370
        "/groups/{*group_id}/-/group_members/{id}/approve_access_request{/}",
        // Route 371
        "/groups/{*group_id}/-/two_factor_auth{/}",
        // Route 372
        "/groups/{*group_id}/-/analytics{/}",
        // Route 373
        "/groups/{*group_id}/-/contribution_analytics{/}",
        // Route 374
        "/groups/{*group_id}/-/analytics/ci_cd{/}",
        // Route 375
        "/groups/{*group_id}/-/analytics/dashboards/{*vueroute?}{/}",
        // Route 376
        "/groups/{*group_id}/-/analytics/devops_adoption{/}",
        // Route 377
        "/groups/{*group_id}/-/analytics/productivity_analytics{/}",
        // Route 378
        "/groups/{*group_id}/-/analytics/coverage_reports{/}",
        // Route 379
        "/groups/{*group_id}/-/analytics/merge_request_analytics{/}",
        // Route 380
        "/groups/{*group_id}/-/analytics/repository_analytics{/}",
        // Route 381
        "/groups/{*group_id}/-/analytics/value_stream_analytics{/}",
        // Route 382
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/average_duration_chart{/}",
        // Route 383
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/median{/}",
        // Route 384
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/average{/}",
        // Route 385
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/records{/}",
        // Route 386
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/count{/}",
        // Route 387
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages{/}",
        // Route 388
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams{/}",
        // Route 389
        // "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams{/}",
        // Route 390
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/new{/}",
        // Route 391
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{id}/edit{/}",
        // Route 392
        "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{id}{/}",
        // Route 393
        // "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{id}{/}",
        // Route 394
        // "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{id}{/}",
        // Route 395
        // "/groups/{*group_id}/-/analytics/value_stream_analytics/value_streams/{id}{/}",
        // Route 396
        "/groups/{*group_id}/-/analytics/value_stream_analytics/summary{/}",
        // Route 397
        "/groups/{*group_id}/-/analytics/value_stream_analytics/time_summary{/}",
        // Route 398
        "/groups/{*group_id}/-/analytics/value_stream_analytics/lead_times{/}",
        // Route 399
        "/groups/{*group_id}/-/analytics/value_stream_analytics/cycle_times{/}",
        // Route 400
        "/groups/{*group_id}/-/analytics/cycle_analytics{/}",
        // Route 401
        "/groups/{*group_id}/-/analytics/type_of_work/tasks_by_type/top_labels{/}",
        // Route 402
        "/groups/{*group_id}/-/analytics/type_of_work/tasks_by_type{/}",
        // Route 403
        "/groups/{*group_id}/-/ldap/sync{/}",
        // Route 404
        "/groups/{*group_id}/-/issues_analytics{/}",
        // Route 405
        "/groups/{*group_id}/-/insights/query{/}",
        // Route 406
        "/groups/{*group_id}/-/insights{/}",
        // Route 407
        "/groups/{*group_id}/-/notification_setting{/}",
        // Route 408
        // "/groups/{*group_id}/-/notification_setting{/}",
        // Route 409
        "/groups/{*group_id}/-/ldap_group_links{/}",
        // Route 410
        // "/groups/{*group_id}/-/ldap_group_links{/}",
        // Route 411
        "/groups/{*group_id}/-/ldap_group_links/{id}{/}",
        // Route 412
        "/groups/{*group_id}/-/saml_group_links{/}",
        // Route 413
        // "/groups/{*group_id}/-/saml_group_links{/}",
        // Route 414
        "/groups/{*group_id}/-/saml_group_links/{id}{/}",
        // Route 415
        "/groups/{*group_id}/-/audit_events{/}",
        // Route 416
        "/groups/{*group_id}/-/usage_quotas/pending_members{/}",
        // Route 417
        "/groups/{*group_id}/-/usage_quotas/subscription_history.{format?}{/}",
        // Route 418
        "/groups/{*group_id}/-/usage_quotas{/}",
        // Route 419
        "/groups/{*group_id}/-/hooks/{id}/test{/}",
        // Route 420
        "/groups/{*group_id}/-/hooks/{hook_id}/hook_logs/{id}/retry{/}",
        // Route 421
        "/groups/{*group_id}/-/hooks/{hook_id}/hook_logs/{id}{/}",
        // Route 422
        "/groups/{*group_id}/-/hooks{/}",
        // Route 423
        // "/groups/{*group_id}/-/hooks{/}",
        // Route 424
        "/groups/{*group_id}/-/hooks/{id}/edit{/}",
        // Route 425
        "/groups/{*group_id}/-/hooks/{id}{/}",
        // Route 426
        // "/groups/{*group_id}/-/hooks/{id}{/}",
        // Route 427
        // "/groups/{*group_id}/-/hooks/{id}{/}",
        // Route 428
        "/groups/{*group_id}/-/autocomplete_sources/epics{/}",
        // Route 429
        "/groups/{*group_id}/-/autocomplete_sources/iterations{/}",
        // Route 430
        "/groups/{*group_id}/-/autocomplete_sources/vulnerabilities{/}",
        // Route 431
        "/groups/{*group_id}/-/autocomplete_sources/wikis{/}",
        // Route 432
        "/groups/{*group_id}/-/billings/refresh_seats{/}",
        // Route 433
        "/groups/{*group_id}/-/billings{/}",
        // Route 434
        "/groups/{*group_id}/-/seat_usage{/}",
        // Route 435
        "/groups/{*group_id}/-/comment_templates{/}",
        // Route 436
        "/groups/{*group_id}/-/comment_templates/{id}{/}",
        // Route 437
        "/groups/{*group_id}/-/epics/{id}/descriptions/{version_id}/diff{/}",
        // Route 438
        "/groups/{*group_id}/-/epics/{id}/descriptions/{version_id}{/}",
        // Route 439
        "/groups/{*group_id}/-/epics/{id}/discussions{/}",
        // Route 440
        "/groups/{*group_id}/-/epics/{id}/realtime_changes{/}",
        // Route 441
        "/groups/{*group_id}/-/epics/{id}/toggle_subscription{/}",
        // Route 442
        "/groups/{*group_id}/-/epics/{epic_id}/issues{/}",
        // Route 443
        // "/groups/{*group_id}/-/epics/{epic_id}/issues{/}",
        // Route 444
        "/groups/{*group_id}/-/epics/{epic_id}/issues/{id}{/}",
        // Route 445
        // "/groups/{*group_id}/-/epics/{epic_id}/issues/{id}{/}",
        // Route 446
        // "/groups/{*group_id}/-/epics/{epic_id}/issues/{id}{/}",
        // Route 447
        "/groups/{*group_id}/-/epics/{epic_id}/notes/{id}/toggle_award_emoji{/}",
        // Route 448
        "/groups/{*group_id}/-/epics/{epic_id}/notes{/}",
        // Route 449
        // "/groups/{*group_id}/-/epics/{epic_id}/notes{/}",
        // Route 450
        "/groups/{*group_id}/-/epics/{epic_id}/notes/{id}{/}",
        // Route 451
        // "/groups/{*group_id}/-/epics/{epic_id}/notes/{id}{/}",
        // Route 452
        // "/groups/{*group_id}/-/epics/{epic_id}/notes/{id}{/}",
        // Route 453
        "/groups/{*group_id}/-/epics/{epic_id}/links{/}",
        // Route 454
        // "/groups/{*group_id}/-/epics/{epic_id}/links{/}",
        // Route 455
        "/groups/{*group_id}/-/epics/{epic_id}/links/{id}{/}",
        // Route 456
        // "/groups/{*group_id}/-/epics/{epic_id}/links/{id}{/}",
        // Route 457
        // "/groups/{*group_id}/-/epics/{epic_id}/links/{id}{/}",
        // Route 458
        "/groups/{*group_id}/-/epics/{epic_id}/related_epic_links{/}",
        // Route 459
        // "/groups/{*group_id}/-/epics/{epic_id}/related_epic_links{/}",
        // Route 460
        "/groups/{*group_id}/-/epics/{epic_id}/related_epic_links/{id}{/}",
        // Route 461
        "/groups/{*group_id}/-/epics/bulk_update{/}",
        // Route 462
        "/groups/{*group_id}/-/epics/{id}/toggle_award_emoji{/}",
        // Route 463
        "/groups/{*group_id}/-/epics{/}",
        // Route 464
        // "/groups/{*group_id}/-/epics{/}",
        // Route 465
        "/groups/{*group_id}/-/epics/new{/}",
        // Route 466
        "/groups/{*group_id}/-/epics/{id}/edit{/}",
        // Route 467
        "/groups/{*group_id}/-/epics/{id}{/}",
        // Route 468
        // "/groups/{*group_id}/-/epics/{id}{/}",
        // Route 469
        // "/groups/{*group_id}/-/epics/{id}{/}",
        // Route 470
        // "/groups/{*group_id}/-/epics/{id}{/}",
        // Route 471
        "/groups/{*group_id}/-/iterations{/}",
        // Route 472
        "/groups/{*group_id}/-/iterations/new{/}",
        // Route 473
        "/groups/{*group_id}/-/iterations/{id}/edit{/}",
        // Route 474
        "/groups/{*group_id}/-/iterations/{id}{/}",
        // Route 475
        "/groups/{*group_id}/-/cadences/{*vueroute?}/{iteration_cadence_id}/iterations{/}",
        // Route 476
        "/groups/{*group_id}/-/cadences/{*vueroute?}/{iteration_cadence_id}/iterations/new{/}",
        // Route 477
        "/groups/{*group_id}/-/cadences/{*vueroute?}/{iteration_cadence_id}/iterations/{id}/edit{/}",
        // Route 478
        "/groups/{*group_id}/-/cadences/{*vueroute?}/{iteration_cadence_id}/iterations/{id}{/}",
        // Route 479
        "/groups/{*group_id}/-/cadences/{*vueroute?}{/}",
        // Route 480
        // "/groups/{*group_id}/-/cadences/{*vueroute?}{/}",
        // Route 481
        "/groups/{*group_id}/-/cadences/{*vueroute?}/new{/}",
        // Route 482
        "/groups/{*group_id}/-/cadences/{*vueroute?}/{id}/edit{/}",
        // Route 483
        "/groups/{*group_id}/-/cadences/{*vueroute?}/{id}{/}",
        // Route 484
        // "/groups/{*group_id}/-/cadences/{*vueroute?}/{id}{/}",
        // Route 485
        // "/groups/{*group_id}/-/cadences/{*vueroute?}/{id}{/}",
        // Route 486
        // "/groups/{*group_id}/-/cadences/{*vueroute?}/{id}{/}",
        // Route 487
        "/groups/{*group_id}/-/issues/bulk_update{/}",
        // Route 488
        "/groups/{*group_id}/-/merge_requests/bulk_update{/}",
        // Route 489
        "/groups/{*group_id}/-/todos{/}",
        // Route 490
        "/groups/{*group_id}/-/epic_boards{/}",
        // Route 491
        "/groups/{*group_id}/-/epic_boards/{id}{/}",
        // Route 492
        "/groups/{*group_id}/-/protected_environments{/}",
        // Route 493
        "/groups/{*group_id}/-/protected_environments/{id}{/}",
        // Route 494
        // "/groups/{*group_id}/-/protected_environments/{id}{/}",
        // Route 495
        // "/groups/{*group_id}/-/protected_environments/{id}{/}",
        // Route 496
        "/groups/{*group_id}/-/security/dashboard{/}",
        // Route 497
        "/groups/{*group_id}/-/security/vulnerabilities{/}",
        // Route 498
        "/groups/{*group_id}/-/security/compliance_dashboard/{*vueroute?}{/}",
        // Route 499
        "/groups/{*group_id}/-/security/discover{/}",
        // Route 500
        "/groups/{*group_id}/-/security/credentials/{id}/revoke{/}",
        // Route 501
        "/groups/{*group_id}/-/security/credentials{/}",
        // Route 502
        "/groups/{*group_id}/-/security/credentials/{id}{/}",
        // Route 503
        "/groups/{*group_id}/-/security/policies/schema{/}",
        // Route 504
        "/groups/{*group_id}/-/security/policies{/}",
        // Route 505
        "/groups/{*group_id}/-/security/policies/new{/}",
        // Route 506
        "/groups/{*group_id}/-/security/policies/{id}/edit{/}",
        // Route 507
        "/groups/{*group_id}/-/security/merge_commit_reports.{format?}{/}",
        // Route 508
        "/groups/{*group_id}/-/security/compliance_project_framework_reports.{format?}{/}",
        // Route 509
        "/groups/{*group_id}/-/security/compliance_violation_reports.{format?}{/}",
        // Route 510
        "/groups/{*group_id}/-/security/compliance_standards_adherence_reports.{format?}{/}",
        // Route 511
        "/groups/{*group_id}/-/security/compliance_framework_reports.{format?}{/}",
        // Route 512
        "/groups/{*group_id}/-/add_ons/discover_duo_pro{/}",
        // Route 513
        "/groups/{*group_id}/-/dependencies/licenses{/}",
        // Route 514
        "/groups/{*group_id}/-/dependencies/locations{/}",
        // Route 515
        "/groups/{*group_id}/-/dependencies{/}",
        // Route 516
        "/groups/{*group_id}/-/push_rules{/}",
        // Route 517
        // "/groups/{*group_id}/-/push_rules{/}",
        // Route 518
        "/groups/{*group_id}/-/protected_branches{/}",
        // Route 519
        "/groups/{*group_id}/-/protected_branches/{id}{/}",
        // Route 520
        // "/groups/{*group_id}/-/protected_branches/{id}{/}",
        // Route 521
        // "/groups/{*group_id}/-/protected_branches/{id}{/}",
        // Route 522
        "/groups/{*group_id}/-/saml/callback{/}",
        // Route 523
        "/groups/{*group_id}/-/saml/sso{/}",
        // Route 524
        // "/groups/{*group_id}/-/saml/sso{/}",
        // Route 525
        "/groups/{*group_id}/-/saml/unlink{/}",
        // Route 526
        "/groups/{*group_id}/-/saml/update_microsoft_application{/}",
        // Route 527
        "/groups/{*group_id}/-/saml{/}",
        // Route 528
        // "/groups/{*group_id}/-/saml{/}",
        // Route 529
        // "/groups/{*group_id}/-/saml{/}",
        // Route 530
        // "/groups/{*group_id}/-/saml{/}",
        // Route 531
        "/groups/{*group_id}/-/scim_oauth{/}",
        // Route 532
        "/groups/{*group_id}/-/roadmap{/}",
        // Route 533
        "/groups/{*group_id}/-/restore{/}",
        // Route 534
        "/groups/{*group_id}/-/service_accounts/{*vueroute?}{/}",
        // Route 535
        // "/groups/{*group_id}/-/service_accounts/{*vueroute?}{/}",
        // Route 536
        "/groups/{*group_id}/-/service_accounts/{*vueroute?}/new{/}",
        // Route 537
        "/groups/{*group_id}/-/service_accounts/{*vueroute?}/{id}/edit{/}",
        // Route 538
        "/groups/{*group_id}/-/service_accounts/{*vueroute?}/{id}{/}",
        // Route 539
        // "/groups/{*group_id}/-/service_accounts/{*vueroute?}/{id}{/}",
        // Route 540
        // "/groups/{*group_id}/-/service_accounts/{*vueroute?}/{id}{/}",
        // Route 541
        // "/groups/{*group_id}/-/service_accounts/{*vueroute?}/{id}{/}",
        // Route 542
        "/groups/{*group_id}/-/work_items/{iid}/descriptions/{version_id}/diff{/}",
        // Route 543
        "/groups/{*group_id}/-/work_items/{iid}/descriptions/{version_id}{/}",
        // Route 544
        "/groups/{*group_id}/-/discover{/}",
        // Route 545
        "/groups/{*group_id}/-/runners/dashboard{/}",
        // Route 546
        "/groups/{*id}/-/edit.{format?}{/}",
        // Route 547
        "/groups/{*id}/-/issues.{format?}{/}",
        // Route 548
        // "/groups/{*id}/-/issues.{format?}{/}",
        // Route 549
        "/groups/{*id}/-/merge_requests.{format?}{/}",
        // Route 550
        "/groups/{*id}/-/projects.{format?}{/}",
        // Route 551
        "/groups/{*id}/-/details.{format?}{/}",
        // Route 552
        "/groups/{*id}/-/activity.{format?}{/}",
        // Route 553
        "/groups/{*id}/-/transfer.{format?}{/}",
        // Route 554
        "/groups/{*id}/-/export.{format?}{/}",
        // Route 555
        "/groups/{*id}/-/download_export.{format?}{/}",
        // Route 556
        "/groups/{*id}/-/unfoldered_environment_names.{format?}{/}",
        // Route 557
        "/groups/{*id}/-/shared.{format?}{/}",
        // Route 558
        "/groups/{*id}/-/inactive.{format?}{/}",
        // Route 559
        "/groups/{*id}/-/archived.{format?}{/}",
        // Route 560
        "/groups/{*id}/{name}.{format?}{/}",
        // Route 561
        "/groups/{*group_id}/-/settings/ci_cd/reset_registration_token{/}",
        // Route 562
        "/groups/{*group_id}/-/settings/ci_cd/update_auto_devops{/}",
        // Route 563
        "/groups/{*group_id}/-/settings/ci_cd/deploy_token/create{/}",
        // Route 564
        "/groups/{*group_id}/-/settings/ci_cd/runner_setup_scripts{/}",
        // Route 565
        "/groups/{*group_id}/-/settings/ci_cd{/}",
        // Route 566
        // "/groups/{*group_id}/-/settings/ci_cd{/}",
        // Route 567
        // "/groups/{*group_id}/-/settings/ci_cd{/}",
        // Route 568
        "/groups/{*group_id}/-/settings/repository/deploy_token/create{/}",
        // Route 569
        "/groups/{*group_id}/-/settings/repository{/}",
        // Route 570
        "/groups/{*group_id}/-/settings/access_tokens/{id}/revoke{/}",
        // Route 571
        "/groups/{*group_id}/-/settings/access_tokens{/}",
        // Route 572
        // "/groups/{*group_id}/-/settings/access_tokens{/}",
        // Route 573
        "/groups/{*group_id}/-/settings/integrations/{id}/test{/}",
        // Route 574
        "/groups/{*group_id}/-/settings/integrations/{id}/reset{/}",
        // Route 575
        "/groups/{*group_id}/-/settings/integrations{/}",
        // Route 576
        "/groups/{*group_id}/-/settings/integrations/{id}/edit{/}",
        // Route 577
        "/groups/{*group_id}/-/settings/integrations/{id}{/}",
        // Route 578
        // "/groups/{*group_id}/-/settings/integrations/{id}{/}",
        // Route 579
        "/groups/{*group_id}/-/settings/slack/slack_auth{/}",
        // Route 580
        "/groups/{*group_id}/-/settings/slack{/}",
        // Route 581
        "/groups/{*group_id}/-/settings/applications/{id}/renew{/}",
        // Route 582
        "/groups/{*group_id}/-/settings/applications{/}",
        // Route 583
        // "/groups/{*group_id}/-/settings/applications{/}",
        // Route 584
        "/groups/{*group_id}/-/settings/applications/new{/}",
        // Route 585
        "/groups/{*group_id}/-/settings/applications/{id}/edit{/}",
        // Route 586
        "/groups/{*group_id}/-/settings/applications/{id}{/}",
        // Route 587
        // "/groups/{*group_id}/-/settings/applications/{id}{/}",
        // Route 588
        // "/groups/{*group_id}/-/settings/applications/{id}{/}",
        // Route 589
        // "/groups/{*group_id}/-/settings/applications/{id}{/}",
        // Route 590
        "/groups/{*group_id}/-/settings/packages_and_registries{/}",
        // Route 591
        // "/groups/{*group_id}/-/usage_quotas{/}",
        // Route 592
        "/groups/{*group_id}/-/variables{/}",
        // Route 593
        // "/groups/{*group_id}/-/variables{/}",
        // Route 594
        // "/groups/{*group_id}/-/variables{/}",
        // Route 595
        "/groups/{*group_id}/-/children{/}",
        // Route 596
        "/groups/{*group_id}/-/shared_projects{/}",
        // Route 597
        "/groups/{*group_id}/-/labels/{id}/toggle_subscription{/}",
        // Route 598
        "/groups/{*group_id}/-/labels{/}",
        // Route 599
        // "/groups/{*group_id}/-/labels{/}",
        // Route 600
        "/groups/{*group_id}/-/labels/new{/}",
        // Route 601
        "/groups/{*group_id}/-/labels/{id}/edit{/}",
        // Route 602
        "/groups/{*group_id}/-/labels/{id}{/}",
        // Route 603
        // "/groups/{*group_id}/-/labels/{id}{/}",
        // Route 604
        // "/groups/{*group_id}/-/labels/{id}{/}",
        // Route 605
        "/groups/{*group_id}/-/custom_emoji{/}",
        // Route 606
        "/groups/{*group_id}/-/custom_emoji/new{/}",
        // Route 607
        "/groups/{*group_id}/-/packages{/}",
        // Route 608
        "/groups/{*group_id}/-/packages/{id}{/}",
        // Route 609
        "/groups/{*group_id}/-/terraform_module_registry{/}",
        // Route 610
        "/groups/{*group_id}/-/infrastructure_registry{/}",
        // Route 611
        "/groups/{*group_id}/-/milestones/{id}/issues{/}",
        // Route 612
        "/groups/{*group_id}/-/milestones/{id}/merge_requests{/}",
        // Route 613
        "/groups/{*group_id}/-/milestones/{id}/participants{/}",
        // Route 614
        "/groups/{*group_id}/-/milestones/{id}/labels{/}",
        // Route 615
        "/groups/{*group_id}/-/milestones{/}",
        // Route 616
        // "/groups/{*group_id}/-/milestones{/}",
        // Route 617
        "/groups/{*group_id}/-/milestones/new{/}",
        // Route 618
        "/groups/{*group_id}/-/milestones/{id}/edit{/}",
        // Route 619
        "/groups/{*group_id}/-/milestones/{id}{/}",
        // Route 620
        // "/groups/{*group_id}/-/milestones/{id}{/}",
        // Route 621
        // "/groups/{*group_id}/-/milestones/{id}{/}",
        // Route 622
        // "/groups/{*group_id}/-/milestones/{id}{/}",
        // Route 623
        "/groups/{*group_id}/-/releases{/}",
        // Route 624
        "/groups/{*group_id}/-/deploy_tokens/{id}/revoke{/}",
        // Route 625
        "/groups/{*group_id}/-/avatar{/}",
        // Route 626
        "/groups/{*group_id}/-/import{/}",
        // Route 627
        "/groups/{*group_id}/-/clusters/connect{/}",
        // Route 628
        "/groups/{*group_id}/-/clusters/new_cluster_docs{/}",
        // Route 629
        "/groups/{*group_id}/-/clusters/create_user{/}",
        // Route 630
        "/groups/{*group_id}/-/clusters/{cluster_id}/integration/create_or_update{/}",
        // Route 631
        "/groups/{*group_id}/-/clusters/{id}/metrics{/}",
        // Route 632
        "/groups/{*group_id}/-/clusters/{id}/environments{/}",
        // Route 633
        "/groups/{*group_id}/-/clusters/{id}/metrics_dashboard{/}",
        // Route 634
        "/groups/{*group_id}/-/clusters/{id}/cluster_status{/}",
        // Route 635
        "/groups/{*group_id}/-/clusters/{id}/clear_cache{/}",
        // Route 636
        "/groups/{*group_id}/-/clusters{/}",
        // Route 637
        "/groups/{*group_id}/-/clusters/{id}{/}",
        // Route 638
        // "/groups/{*group_id}/-/clusters/{id}{/}",
        // Route 639
        // "/groups/{*group_id}/-/clusters/{id}{/}",
        // Route 640
        // "/groups/{*group_id}/-/clusters/{id}{/}",
        // Route 641
        "/groups/{*group_id}/-/group_members/{id}/resend_invite{/}",
        // Route 642
        "/groups/{*group_id}/-/group_members/bulk_reassignment_file{/}",
        // Route 643
        "/groups/{*group_id}/-/group_members/leave{/}",
        // Route 644
        // "/groups/{*group_id}/-/group_members/request_access{/}",
        // Route 645
        // "/groups/{*group_id}/-/group_members/request_access{/}",
        // Route 646
        // "/groups/{*group_id}/-/group_members/{id}/approve_access_request{/}",
        // Route 647
        "/groups/{*group_id}/-/group_members{/}",
        // Route 648
        "/groups/{*group_id}/-/group_members/{id}{/}",
        // Route 649
        // "/groups/{*group_id}/-/group_members/{id}{/}",
        // Route 650
        // "/groups/{*group_id}/-/group_members/{id}{/}",
        // Route 651
        "/groups/{*group_id}/-/group_links/{id}{/}",
        // Route 652
        // "/groups/{*group_id}/-/group_links/{id}{/}",
        // Route 653
        // "/groups/{*group_id}/-/group_links/{id}{/}",
        // Route 654
        "/groups/{*group_id}/-/uploads/{secret}/{filename}.{format?}{/}",
        // Route 655
        "/groups/{*group_id}/-/uploads/authorize{/}",
        // Route 656
        "/groups/{*group_id}/-/uploads{/}",
        // Route 657
        "/groups/{*group_id}/-/boards{/}",
        // Route 658
        "/groups/{*group_id}/-/boards/{id}{/}",
        // Route 659
        "/groups/{*group_id}/-/runners/{id}/register{/}",
        // Route 660
        "/groups/{*group_id}/-/runners/{id}/resume{/}",
        // Route 661
        "/groups/{*group_id}/-/runners/{id}/pause{/}",
        // Route 662
        "/groups/{*group_id}/-/runners{/}",
        // Route 663
        "/groups/{*group_id}/-/runners/new{/}",
        // Route 664
        "/groups/{*group_id}/-/runners/{id}/edit{/}",
        // Route 665
        "/groups/{*group_id}/-/runners/{id}{/}",
        // Route 666
        // "/groups/{*group_id}/-/runners/{id}{/}",
        // Route 667
        // "/groups/{*group_id}/-/runners/{id}{/}",
        // Route 668
        // "/groups/{*group_id}/-/runners/{id}{/}",
        // Route 669
        "/groups/{*group_id}/-/container_registries{/}",
        // Route 670
        "/groups/{*group_id}/-/container_registries/{id}{/}",
        // Route 671
        "/groups/{*group_id}/-/dependency_proxy{/}",
        // Route 672
        // "/groups/{*group_id}/-/dependency_proxy{/}",
        // Route 673
        // "/groups/{*group_id}/-/dependency_proxy{/}",
        // Route 674
        "/groups/{*group_id}/-/harbor/repositories/{repository_id}/artifacts/{artifact_id}/tags{/}",
        // Route 675
        "/groups/{*group_id}/-/harbor/repositories/{repository_id}/artifacts{/}",
        // Route 676
        "/groups/{*group_id}/-/harbor/repositories{/}",
        // Route 677
        "/groups/{*group_id}/-/harbor/repositories/{id}{/}",
        // Route 678
        "/groups/{*group_id}/-/autocomplete_sources/members{/}",
        // Route 679
        "/groups/{*group_id}/-/autocomplete_sources/issues{/}",
        // Route 680
        "/groups/{*group_id}/-/autocomplete_sources/merge_requests{/}",
        // Route 681
        "/groups/{*group_id}/-/autocomplete_sources/labels{/}",
        // Route 682
        "/groups/{*group_id}/-/autocomplete_sources/commands{/}",
        // Route 683
        "/groups/{*group_id}/-/autocomplete_sources/milestones{/}",
        // Route 684
        "/groups/{*group_id}/-/crm/contacts{/}",
        // Route 685
        "/groups/{*group_id}/-/crm/contacts/new{/}",
        // Route 686
        "/groups/{*group_id}/-/crm/contacts/{id}/edit{/}",
        // Route 687
        "/groups/{*group_id}/-/crm/organizations{/}",
        // Route 688
        "/groups/{*group_id}/-/crm/organizations/new{/}",
        // Route 689
        "/groups/{*group_id}/-/crm/organizations/{id}/edit{/}",
        // Route 690
        "/groups/{*group_id}/-/achievements{/}",
        // Route 691
        "/groups/{*group_id}/-/achievements/new{/}",
        // Route 692
        "/groups/{*group_id}/-/achievements/{id}/edit{/}",
        // Route 693
        "/groups/{*group_id}/-/work_items{/}",
        // Route 694
        "/groups/{*group_id}/-/work_items/{iid}{/}",
        // Route 695
        "/groups/{*group_id}/-/preview_markdown{/}",
        // Route 696
        "/{*id?}/{name}.{format?}{/}",
        // Route 697
        // "/{*id?}/{name}.{format?}{/}",
        // Route 698
        // "/{*id?}/{name}.{format?}{/}",
        // Route 699
        // "/{*id?}/{name}.{format?}{/}",
        // Route 700
        "/v2{/}",
        // Route 701
        "/v2/{*group_id}/dependency_proxy/containers/{*image}/manifests/{*tag}{/}",
        // Route 702
        "/v2/{*group_id}/dependency_proxy/containers/{*image}/blobs/{sha}{/}",
        // Route 703
        "/v2/{*group_id}/dependency_proxy/containers/{*image}/blobs/{sha}/upload/authorize{/}",
        // Route 704
        "/v2/{*group_id}/dependency_proxy/containers/{*image}/blobs/{sha}/upload{/}",
        // Route 705
        "/v2/{*group_id}/dependency_proxy/containers/{*image}/manifests/{*tag}/upload/authorize{/}",
        // Route 706
        "/v2/{*group_id}/dependency_proxy/containers/{*image}/manifests/{*tag}/upload{/}",
        // Route 707
        "/projects{/}",
        // Route 708
        // "/projects{/}",
        // Route 709
        "/projects/new{/}",
        // Route 710
        "/projects/{id}{/}",
        // Route 711
        "/{*repository_path}/info/refs{/}",
        // Route 712
        "/{*repository_path}/git-upload-pack{/}",
        // Route 713
        "/{*repository_path}/git-receive-pack{/}",
        // Route 714
        "/{*repository_path}/ssh-upload-pack{/}",
        // Route 715
        "/{*repository_path}/ssh-receive-pack{/}",
        // Route 716
        "/{*repository_path}/info/lfs/objects/batch{/}",
        // Route 717
        "/{*repository_path}/info/lfs/objects{/}",
        // Route 718
        "/{*repository_path}/info/lfs/objects/{*oid}{/}",
        // Route 719
        "/{*repository_path}/info/lfs/locks/{id}/unlock{/}",
        // Route 720
        "/{*repository_path}/info/lfs/locks/verify{/}",
        // Route 721
        "/{*repository_path}/info/lfs/locks{/}",
        // Route 722
        // "/{*repository_path}/info/lfs/locks{/}",
        // Route 723
        "/{*repository_path}/info/lfs/locks/new{/}",
        // Route 724
        "/{*repository_path}/info/lfs/locks/{id}/edit{/}",
        // Route 725
        "/{*repository_path}/info/lfs/locks/{id}{/}",
        // Route 726
        // "/{*repository_path}/info/lfs/locks/{id}{/}",
        // Route 727
        // "/{*repository_path}/info/lfs/locks/{id}{/}",
        // Route 728
        // "/{*repository_path}/info/lfs/locks/{id}{/}",
        // Route 729
        "/{*repository_path}/gitlab-lfs/objects/{*oid}{/}",
        // Route 730
        "/{*repository_path}/gitlab-lfs/objects/{*oid}/{size}/authorize{/}",
        // Route 731
        "/{*repository_path}/gitlab-lfs/objects/{*oid}/{size}{/}",
        // Route 732
        "/{*repository_path}{/}",
        // Route 733
        // "/{*repository_path}/info/refs{/}",
        // Route 734
        "/api/v4/geo/graphql{/}",
        // Route 735
        "/api/graphql{/}",
        // Route 736
        // NOTE: See GraphiQL::Rails::Engine
        // "/-/graphql-explorer{/}",
        // Route 737
        "/",
        // Route 738
        "/{*namespace_id}/{project_id}/-/releases/outbox{/}",
        // Route 739
        "/{*namespace_id}/{project_id}/-/releases/inbox{/}",
        // Route 740
        "/{*namespace_id}/{project_id}/-/releases{/}",
        // Route 741
        "/-/customers_dot/proxy/graphql{/}",
        // Route 742
        "/oauth/device/confirm{/}",
        // Route 743
        "/admin/sidekiq{/}",
        // Route 744
        "/help{/}",
        // Route 745
        "/help/shortcuts{/}",
        // Route 746
        "/help/instance_configuration{/}",
        // Route 747
        "/help/drawers/{*markdown_file}{/}",
        // Route 748
        "/help/docs{/}",
        // Route 749
        "/help/{*path}{/}",
        // Route 750
        "/-/google_api/auth/callback{/}",
        // Route 751
        "/import/history{/}",
        // Route 752
        "/import/url/validate{/}",
        // Route 753
        "/import/github/personal_access_token{/}",
        // Route 754
        "/import/github/status{/}",
        // Route 755
        "/import/github/details{/}",
        // Route 756
        "/import/github/callback{/}",
        // Route 757
        "/import/github/realtime_changes{/}",
        // Route 758
        "/import/github/failures{/}",
        // Route 759
        "/import/github/cancel{/}",
        // Route 760
        "/import/github/cancel_all{/}",
        // Route 761
        "/import/github/counts{/}",
        // Route 762
        "/import/github/new{/}",
        // Route 763
        "/import/github{/}",
        // Route 764
        "/import/gitea/personal_access_token{/}",
        // Route 765
        "/import/gitea/status{/}",
        // Route 766
        "/import/gitea/realtime_changes{/}",
        // Route 767
        "/import/gitea/new{/}",
        // Route 768
        "/import/gitea{/}",
        // Route 769
        "/import/bitbucket/status{/}",
        // Route 770
        "/import/bitbucket/callback{/}",
        // Route 771
        "/import/bitbucket/realtime_changes{/}",
        // Route 772
        "/import/bitbucket{/}",
        // Route 773
        "/import/bitbucket_server/configure{/}",
        // Route 774
        "/import/bitbucket_server/status{/}",
        // Route 775
        "/import/bitbucket_server/callback{/}",
        // Route 776
        "/import/bitbucket_server/realtime_changes{/}",
        // Route 777
        "/import/bitbucket_server/new{/}",
        // Route 778
        "/import/bitbucket_server{/}",
        // Route 779
        "/import/fogbugz/status{/}",
        // Route 780
        "/import/fogbugz/callback{/}",
        // Route 781
        "/import/fogbugz/realtime_changes{/}",
        // Route 782
        "/import/fogbugz/user_map{/}",
        // Route 783
        // "/import/fogbugz/user_map{/}",
        // Route 784
        "/import/fogbugz/new{/}",
        // Route 785
        "/import/fogbugz{/}",
        // Route 786
        "/import/gitlab_project{/}",
        // Route 787
        "/import/gitlab_project/authorize{/}",
        // Route 788
        "/import/gitlab_project/new{/}",
        // Route 789
        // "/import/gitlab_project{/}",
        // Route 790
        "/import/gitlab_group/authorize{/}",
        // Route 791
        "/import/gitlab_group{/}",
        // Route 792
        "/import/github_group/status{/}",
        // Route 793
        "/import/bulk_imports/configure{/}",
        // Route 794
        "/import/bulk_imports/status{/}",
        // Route 795
        "/import/bulk_imports/realtime_changes{/}",
        // Route 796
        "/import/bulk_imports/history{/}",
        // Route 797
        "/import/bulk_imports{/}",
        // Route 798
        "/import/bulk_imports/{id}/history{/}",
        // Route 799
        "/import/bulk_imports/{id}/history/{entity_id}/failures{/}",
        // Route 800
        "/import/manifest/status{/}",
        // Route 801
        "/import/manifest/realtime_changes{/}",
        // Route 802
        "/import/manifest/upload{/}",
        // Route 803
        "/import/manifest/new{/}",
        // Route 804
        "/import/manifest{/}",
        // Route 805
        "/import/source_users/{id}{/}",
        // Route 806
        "/import/source_users/{id}/accept{/}",
        // Route 807
        "/import/source_users/{id}/decline{/}",
        // Route 808
        "/uploads/-/system/{model}/{mounted_as}/{id}/{filename}{/}",
        // Route 809
        // "/uploads/-/system/{model}/{mounted_as}/{id}/{filename}{/}",
        // Route 810
        "/uploads/-/system/{model}/{id}/{secret}/{filename}{/}",
        // Route 811
        "/uploads/-/system/temp/{secret}/{filename}{/}",
        // Route 812
        // "/uploads/-/system/{model}/{mounted_as}/{id}/{filename}{/}",
        // Route 813
        "/uploads/{model}{/}",
        // Route 814
        "/uploads/{model}/authorize{/}",
        // Route 815
        // "/uploads/-/system/{model}/{mounted_as}/{id}/{filename}{/}",
        // Route 816
        // "/uploads/-/system/{model}/{mounted_as}/{id}/{filename}{/}",
        // Route 817
        "/files/note/{id}/{filename}{/}",
        // Route 818
        "/explore/dependencies{/}",
        // Route 819
        "/explore/projects/trending{/}",
        // Route 820
        "/explore/projects/starred{/}",
        // Route 821
        "/explore/projects/topics{/}",
        // Route 822
        "/explore/projects/topics/{topic_name}.{format?}{/}",
        // Route 823
        "/explore/projects{/}",
        // Route 824
        "/explore/groups{/}",
        // Route 825
        "/explore/catalog{/}",
        // Route 826
        "/explore/catalog/{*full_path}{/}",
        // Route 827
        "/explore/snippets{/}",
        // Route 828
        "/explore{/}",
        // Route 829
        "/public{/}",
        // Route 830
        "/public/projects{/}",
        // Route 831
        "/admin/users/{id}/identity_verification_exemption{/}",
        // Route 832
        "/admin/users/{id}/destroy_identity_verification_exemption{/}",
        // Route 833
        "/admin/users/{id}/reset_runners_minutes{/}",
        // Route 834
        "/admin/users/{id}/card_match{/}",
        // Route 835
        "/admin/users/{id}/phone_match{/}",
        // Route 836
        "/admin/groups/{*id}/reset_runners_minutes.{format?}{/}",
        // Route 837
        "/admin/push_rule{/}",
        // Route 838
        // "/admin/push_rule{/}",
        // Route 839
        // "/admin/push_rule{/}",
        // Route 840
        "/admin/email{/}",
        // Route 841
        // "/admin/email{/}",
        // Route 842
        "/admin/audit_logs{/}",
        // Route 843
        "/admin/audit_log_reports.{format?}{/}",
        // Route 844
        "/admin/credentials/{credential_id}/resources/{resource_id}/revoke{/}",
        // Route 845
        "/admin/credentials/{id}/revoke{/}",
        // Route 846
        "/admin/credentials{/}",
        // Route 847
        "/admin/credentials/{id}{/}",
        // Route 848
        "/admin/user_permission_exports{/}",
        // Route 849
        "/admin/license/download{/}",
        // Route 850
        "/admin/license/sync_seat_link{/}",
        // Route 851
        "/admin/license/usage_export{/}",
        // Route 852
        "/admin/license{/}",
        // Route 853
        // "/admin/license{/}",
        // Route 854
        // "/admin/license{/}",
        // Route 855
        "/admin/subscription{/}",
        // Route 856
        "/admin/role_promotion_requests{/}",
        // Route 857
        "/admin/code_suggestions{/}",
        // Route 858
        "/admin/ai/self_hosted_models/terms_and_conditions{/}",
        // Route 859
        // "/admin/ai/self_hosted_models/terms_and_conditions{/}",
        // Route 860
        "/admin/ai/self_hosted_models{/}",
        // Route 861
        // "/admin/ai/self_hosted_models{/}",
        // Route 862
        "/admin/ai/self_hosted_models/new{/}",
        // Route 863
        "/admin/ai/self_hosted_models/{id}/edit{/}",
        // Route 864
        "/admin/ai/self_hosted_models/{id}{/}",
        // Route 865
        // "/admin/ai/self_hosted_models/{id}{/}",
        // Route 866
        // "/admin/ai/self_hosted_models/{id}{/}",
        // Route 867
        "/admin/ai/feature_settings{/}",
        // Route 868
        // "/admin/ai/feature_settings{/}",
        // Route 869
        "/admin/ai/feature_settings/{id}/edit{/}",
        // Route 870
        "/admin/ai/feature_settings/{id}{/}",
        // Route 871
        // "/admin/ai/feature_settings/{id}{/}",
        // Route 872
        "/admin/application_settings/seat_link_payload{/}",
        // Route 873
        "/admin/application_settings/templates{/}",
        // Route 874
        "/admin/application_settings/advanced_search{/}",
        // Route 875
        "/admin/application_settings/security_and_compliance{/}",
        // Route 876
        "/admin/application_settings/namespace_storage{/}",
        // Route 877
        "/admin/application_settings/analytics{/}",
        // Route 878
        "/admin/application_settings/geo{/}",
        // Route 879
        "/admin/application_settings/update_microsoft_application{/}",
        // Route 880
        "/admin/application_settings/scim_oauth{/}",
        // Route 881
        "/admin/application_settings/roles_and_permissions{/}",
        // Route 882
        "/admin/application_settings/roles_and_permissions/new{/}",
        // Route 883
        "/admin/application_settings/roles_and_permissions/{id}/edit{/}",
        // Route 884
        "/admin/application_settings/roles_and_permissions/{id}{/}",
        // Route 885
        "/admin/geo{/}",
        // Route 886
        "/admin/geo/sites/{id}/replication{/}",
        // Route 887
        "/admin/geo/sites/{id}/replication/{replicable_name_plural}{/}",
        // Route 888
        "/admin/geo/sites{/}",
        // Route 889
        // "/admin/geo/sites{/}",
        // Route 890
        "/admin/geo/sites/new{/}",
        // Route 891
        "/admin/geo/sites/{id}/edit{/}",
        // Route 892
        "/admin/geo/sites/{id}{/}",
        // Route 893
        // "/admin/geo/sites/{id}{/}",
        // Route 894
        "/admin/geo/replication{/}",
        // Route 895
        "/admin/geo/replication/{replicable_name_plural}{/}",
        // Route 896
        "/admin/geo/settings{/}",
        // Route 897
        // "/admin/geo/settings{/}",
        // Route 898
        // "/admin/geo/settings{/}",
        // Route 899
        "/admin/elasticsearch/enqueue_index{/}",
        // Route 900
        "/admin/elasticsearch/trigger_reindexing{/}",
        // Route 901
        "/admin/elasticsearch/cancel_index_deletion{/}",
        // Route 902
        "/admin/elasticsearch/retry_migration{/}",
        // Route 903
        "/admin/namespace_limits{/}",
        // Route 904
        "/admin/namespace_limits/export_usage{/}",
        // Route 905
        "/admin/runners/dashboard{/}",
        // Route 906
        "/admin/users/{user_id}/keys/{id}{/}",
        // Route 907
        // "/admin/users/{user_id}/keys/{id}{/}",
        // Route 908
        "/admin/users/{user_id}/identities{/}",
        // Route 909
        // "/admin/users/{user_id}/identities{/}",
        // Route 910
        "/admin/users/{user_id}/identities/new{/}",
        // Route 911
        "/admin/users/{user_id}/identities/{id}/edit{/}",
        // Route 912
        "/admin/users/{user_id}/identities/{id}{/}",
        // Route 913
        // "/admin/users/{user_id}/identities/{id}{/}",
        // Route 914
        // "/admin/users/{user_id}/identities/{id}{/}",
        // Route 915
        "/admin/users/{user_id}/impersonation_tokens/{id}/revoke{/}",
        // Route 916
        "/admin/users/{user_id}/impersonation_tokens{/}",
        // Route 917
        // "/admin/users/{user_id}/impersonation_tokens{/}",
        // Route 918
        "/admin/users/{id}/projects{/}",
        // Route 919
        "/admin/users/{id}/keys{/}",
        // Route 920
        "/admin/users/{id}/block{/}",
        // Route 921
        "/admin/users/{id}/unblock{/}",
        // Route 922
        "/admin/users/{id}/ban{/}",
        // Route 923
        "/admin/users/{id}/unban{/}",
        // Route 924
        "/admin/users/{id}/deactivate{/}",
        // Route 925
        "/admin/users/{id}/activate{/}",
        // Route 926
        "/admin/users/{id}/unlock{/}",
        // Route 927
        "/admin/users/{id}/confirm{/}",
        // Route 928
        "/admin/users/{id}/approve{/}",
        // Route 929
        "/admin/users/{id}/trust{/}",
        // Route 930
        "/admin/users/{id}/untrust{/}",
        // Route 931
        "/admin/users/{id}/reject{/}",
        // Route 932
        "/admin/users/{id}/impersonate{/}",
        // Route 933
        "/admin/users/{id}/disable_two_factor{/}",
        // Route 934
        "/admin/users/{id}/remove/{email_id}{/}",
        // Route 935
        "/admin/users{/}",
        // Route 936
        // "/admin/users{/}",
        // Route 937
        "/admin/users/new{/}",
        // Route 938
        "/admin/users/{id}/edit{/}",
        // Route 939
        "/admin/users/{id}{/}",
        // Route 940
        // "/admin/users/{id}{/}",
        // Route 941
        // "/admin/users/{id}{/}",
        // Route 942
        // "/admin/users/{id}{/}",
        // Route 943
        "/admin/session/destroy{/}",
        // Route 944
        "/admin/session/new{/}",
        // Route 945
        "/admin/session{/}",
        // Route 946
        "/admin/impersonation{/}",
        // Route 947
        "/admin/initial_setup/new{/}",
        // Route 948
        "/admin/initial_setup{/}",
        // Route 949
        // "/admin/initial_setup{/}",
        // Route 950
        "/admin/abuse_reports/{id}/moderate_user{/}",
        // Route 951
        "/admin/abuse_reports{/}",
        // Route 952
        "/admin/abuse_reports/{id}{/}",
        // Route 953
        // "/admin/abuse_reports/{id}{/}",
        // Route 954
        // "/admin/abuse_reports/{id}{/}",
        // Route 955
        // "/admin/abuse_reports/{id}{/}",
        // Route 956
        "/admin/gitaly_servers{/}",
        // Route 957
        "/admin/spam_logs/{id}/mark_as_ham{/}",
        // Route 958
        "/admin/spam_logs{/}",
        // Route 959
        "/admin/spam_logs/{id}{/}",
        // Route 960
        "/admin/applications/{id}/renew{/}",
        // Route 961
        "/admin/applications{/}",
        // Route 962
        // "/admin/applications{/}",
        // Route 963
        "/admin/applications/new{/}",
        // Route 964
        "/admin/applications/{id}/edit{/}",
        // Route 965
        "/admin/applications/{id}{/}",
        // Route 966
        // "/admin/applications/{id}{/}",
        // Route 967
        // "/admin/applications/{id}{/}",
        // Route 968
        // "/admin/applications/{id}{/}",
        // Route 969
        "/admin/groups{/}",
        // Route 970
        // "/admin/groups{/}",
        // Route 971
        "/admin/groups/new{/}",
        // Route 972
        "/admin/organizations{/}",
        // Route 973
        "/admin/groups/{*id}/members_update.{format?}{/}",
        // Route 974
        "/admin/groups/{*id}/edit.{format?}{/}",
        // Route 975
        "/admin/groups/{*id}/{name}.{format?}{/}",
        // Route 976
        // "/admin/groups/{*id}/{name}.{format?}{/}",
        // Route 977
        // "/admin/groups/{*id}/{name}.{format?}{/}",
        // Route 978
        // "/admin/groups/{*id}/{name}.{format?}{/}",
        // Route 979
        "/admin/topics/{topic_id}/avatar{/}",
        // Route 980
        "/admin/topics/preview_markdown{/}",
        // Route 981
        "/admin/topics/merge{/}",
        // Route 982
        "/admin/topics{/}",
        // Route 983
        // "/admin/topics{/}",
        // Route 984
        "/admin/topics/new{/}",
        // Route 985
        "/admin/topics/{id}/edit{/}",
        // Route 986
        "/admin/topics/{id}{/}",
        // Route 987
        // "/admin/topics/{id}{/}",
        // Route 988
        // "/admin/topics/{id}{/}",
        // Route 989
        "/admin/deploy_keys{/}",
        // Route 990
        // "/admin/deploy_keys{/}",
        // Route 991
        "/admin/deploy_keys/new{/}",
        // Route 992
        "/admin/deploy_keys/{id}/edit{/}",
        // Route 993
        "/admin/deploy_keys/{id}{/}",
        // Route 994
        // "/admin/deploy_keys/{id}{/}",
        // Route 995
        // "/admin/deploy_keys/{id}{/}",
        // Route 996
        "/admin/hooks/{id}/test{/}",
        // Route 997
        "/admin/hooks/{hook_id}/hook_logs/{id}/retry{/}",
        // Route 998
        "/admin/hooks/{hook_id}/hook_logs/{id}{/}",
        // Route 999
        "/admin/hooks{/}",
        // Route 1000
        // "/admin/hooks{/}",
        // Route 1001
        "/admin/hooks/{id}/edit{/}",
        // Route 1002
        "/admin/hooks/{id}{/}",
        // Route 1003
        // "/admin/hooks/{id}{/}",
        // Route 1004
        // "/admin/hooks/{id}{/}",
        // Route 1005
        "/admin/broadcast_messages/preview{/}",
        // Route 1006
        "/admin/broadcast_messages{/}",
        // Route 1007
        // "/admin/broadcast_messages{/}",
        // Route 1008
        "/admin/broadcast_messages/{id}/edit{/}",
        // Route 1009
        "/admin/broadcast_messages/{id}{/}",
        // Route 1010
        // "/admin/broadcast_messages/{id}{/}",
        // Route 1011
        // "/admin/broadcast_messages/{id}{/}",
        // Route 1012
        "/admin/instance_review{/}",
        // Route 1013
        "/admin/background_migrations/{background_migration_id}/batched_jobs/{id}{/}",
        // Route 1014
        "/admin/background_migrations/{id}/pause{/}",
        // Route 1015
        "/admin/background_migrations/{id}/resume{/}",
        // Route 1016
        "/admin/background_migrations/{id}/retry{/}",
        // Route 1017
        "/admin/background_migrations{/}",
        // Route 1018
        "/admin/background_migrations/{id}{/}",
        // Route 1019
        "/admin/health_check{/}",
        // Route 1020
        "/admin/background_jobs{/}",
        // Route 1021
        "/admin/system_info{/}",
        // Route 1022
        "/admin/projects{/}",
        // Route 1023
        "/admin/usage_trends{/}",
        // Route 1024
        "/admin/dev_ops_reports{/}",
        // Route 1025
        "/admin/dev_ops_report{/}",
        // Route 1026
        "/admin/cohorts{/}",
        // Route 1027
        "/admin/projects/{*namespace_id}/{id}/transfer{/}",
        // Route 1028
        "/admin/projects/{*namespace_id}/{id}/repository_check{/}",
        // Route 1029
        "/admin/projects/{*namespace_id}/{id}/edit{/}",
        // Route 1030
        "/admin/projects/{*namespace_id}/{id}{/}",
        // Route 1031
        // "/admin/projects/{*namespace_id}/{id}{/}",
        // Route 1032
        // "/admin/projects/{*namespace_id}/{id}{/}",
        // Route 1033
        "/admin/projects/{*namespace_id}/{project_id}/runner_projects{/}",
        // Route 1034
        "/admin/projects/{*namespace_id}/{project_id}/runner_projects/{id}{/}",
        // Route 1035
        // "/admin/projects/{*namespace_id}/{id}{/}",
        // Route 1036
        // "/admin/projects/{*namespace_id}/{id}{/}",
        // Route 1037
        "/admin/application_settings/integrations/{id}/overrides{/}",
        // Route 1038
        "/admin/application_settings/integrations/{id}/test{/}",
        // Route 1039
        "/admin/application_settings/integrations/{id}/reset{/}",
        // Route 1040
        "/admin/application_settings/integrations/{id}/edit{/}",
        // Route 1041
        "/admin/application_settings/integrations/{id}{/}",
        // Route 1042
        // "/admin/application_settings/integrations/{id}{/}",
        // Route 1043
        "/admin/application_settings/slack/slack_auth{/}",
        // Route 1044
        "/admin/application_settings/slack{/}",
        // Route 1045
        "/admin/application_settings/usage_data{/}",
        // Route 1046
        "/admin/application_settings/reset_registration_token{/}",
        // Route 1047
        "/admin/application_settings/reset_health_check_token{/}",
        // Route 1048
        "/admin/application_settings/reset_error_tracking_access_token{/}",
        // Route 1049
        "/admin/application_settings/clear_repository_check_states{/}",
        // Route 1050
        "/admin/application_settings/general{/}",
        // Route 1051
        "/admin/application_settings/integrations{/}",
        // Route 1052
        "/admin/application_settings/repository{/}",
        // Route 1053
        "/admin/application_settings/ci_cd{/}",
        // Route 1054
        "/admin/application_settings/reporting{/}",
        // Route 1055
        "/admin/application_settings/metrics_and_profiling{/}",
        // Route 1056
        "/admin/application_settings/network{/}",
        // Route 1057
        "/admin/application_settings/preferences{/}",
        // Route 1058
        "/admin/application_settings/lets_encrypt_terms_of_service{/}",
        // Route 1059
        "/admin/application_settings/slack_app_manifest_download{/}",
        // Route 1060
        "/admin/application_settings/slack_app_manifest_share{/}",
        // Route 1061
        "/admin/application_settings/appearance/preview_sign_in{/}",
        // Route 1062
        "/admin/application_settings/appearance/logo{/}",
        // Route 1063
        "/admin/application_settings/appearance/pwa_icon{/}",
        // Route 1064
        "/admin/application_settings/appearance/header_logos{/}",
        // Route 1065
        "/admin/application_settings/appearance/favicon{/}",
        // Route 1066
        "/admin/application_settings/appearance{/}",
        // Route 1067
        // "/admin/application_settings/appearance{/}",
        // Route 1068
        // "/admin/application_settings/appearance{/}",
        // Route 1069
        // "/admin/application_settings/appearance{/}",
        // Route 1070
        "/admin/application_settings{/}",
        // Route 1071
        // "/admin/application_settings{/}",
        // Route 1072
        "/admin/plan_limits{/}",
        // Route 1073
        "/admin/labels{/}",
        // Route 1074
        // "/admin/labels{/}",
        // Route 1075
        "/admin/labels/new{/}",
        // Route 1076
        "/admin/labels/{id}/edit{/}",
        // Route 1077
        "/admin/labels/{id}{/}",
        // Route 1078
        // "/admin/labels/{id}{/}",
        // Route 1079
        // "/admin/labels/{id}{/}",
        // Route 1080
        // "/admin/labels/{id}{/}",
        // Route 1081
        "/admin/runners/{id}/register{/}",
        // Route 1082
        "/admin/runners/{id}/resume{/}",
        // Route 1083
        "/admin/runners/{id}/pause{/}",
        // Route 1084
        "/admin/runners/tag_list{/}",
        // Route 1085
        "/admin/runners/runner_setup_scripts{/}",
        // Route 1086
        "/admin/runners{/}",
        // Route 1087
        "/admin/runners/new{/}",
        // Route 1088
        "/admin/runners/{id}/edit{/}",
        // Route 1089
        "/admin/runners/{id}{/}",
        // Route 1090
        // "/admin/runners/{id}{/}",
        // Route 1091
        // "/admin/runners/{id}{/}",
        // Route 1092
        // "/admin/runners/{id}{/}",
        // Route 1093
        "/admin/jobs/cancel_all{/}",
        // Route 1094
        "/admin/jobs{/}",
        // Route 1095
        "/admin/ci/variables{/}",
        // Route 1096
        // "/admin/ci/variables{/}",
        // Route 1097
        // "/admin/ci/variables{/}",
        // Route 1098
        "/admin/clusters/connect{/}",
        // Route 1099
        "/admin/clusters/new_cluster_docs{/}",
        // Route 1100
        "/admin/clusters/create_user{/}",
        // Route 1101
        "/admin/clusters/{cluster_id}/integration/create_or_update{/}",
        // Route 1102
        "/admin/clusters/{id}/metrics{/}",
        // Route 1103
        "/admin/clusters/{id}/environments{/}",
        // Route 1104
        "/admin/clusters/{id}/metrics_dashboard{/}",
        // Route 1105
        "/admin/clusters/{id}/cluster_status{/}",
        // Route 1106
        "/admin/clusters/{id}/clear_cache{/}",
        // Route 1107
        "/admin/clusters{/}",
        // Route 1108
        "/admin/clusters/{id}{/}",
        // Route 1109
        // "/admin/clusters/{id}{/}",
        // Route 1110
        // "/admin/clusters/{id}{/}",
        // Route 1111
        // "/admin/clusters/{id}{/}",
        // Route 1112
        "/admin/dashboard/stats{/}",
        // Route 1113
        "/admin{/}",
        // Route 1114
        "/admin/version_check{/}",
        // Route 1115
        "/dashboard/projects/removed{/}",
        // Route 1116
        "/dashboard/projects{/}",
        // Route 1117
        "/dashboard/issues{/}",
        // Route 1118
        // "/dashboard/issues{/}",
        // Route 1119
        "/dashboard/merge_requests{/}",
        // Route 1120
        "/dashboard/activity{/}",
        // Route 1121
        "/dashboard/merge_requests/search{/}",
        // Route 1122
        "/dashboard/milestones{/}",
        // Route 1123
        "/dashboard/labels{/}",
        // Route 1124
        "/dashboard/groups{/}",
        // Route 1125
        "/dashboard/snippets{/}",
        // Route 1126
        "/dashboard/todos/vue{/}",
        // Route 1127
        "/dashboard/todos/destroy_all{/}",
        // Route 1128
        "/dashboard/todos/bulk_restore{/}",
        // Route 1129
        "/dashboard/todos/{id}/restore{/}",
        // Route 1130
        "/dashboard/todos{/}",
        // Route 1131
        "/dashboard/todos/{id}{/}",
        // Route 1132
        "/dashboard/projects/starred{/}",
        // Route 1133
        "/dashboard/projects/contributed{/}",
        // Route 1134
        "/dashboard/projects/personal{/}",
        // Route 1135
        "/dashboard/projects/member{/}",
        // Route 1136
        // "/dashboard/projects{/}",
        // Route 1137
        "/dashboard{/}",
        // Route 1138
        "/users/identity_verification/verification_state{/}",
        // Route 1139
        "/users/identity_verification/verify_email_code{/}",
        // Route 1140
        "/users/identity_verification/resend_email_code{/}",
        // Route 1141
        "/users/identity_verification/send_phone_verification_code{/}",
        // Route 1142
        "/users/identity_verification/verify_phone_verification_code{/}",
        // Route 1143
        "/users/identity_verification/verify_arkose_labs_session{/}",
        // Route 1144
        "/users/identity_verification/toggle_phone_exemption{/}",
        // Route 1145
        "/users/identity_verification/arkose_labs_challenge{/}",
        // Route 1146
        "/users/identity_verification/verify_credit_card{/}",
        // Route 1147
        "/users/identity_verification/verify_credit_card_captcha{/}",
        // Route 1148
        "/users/identity_verification/success{/}",
        // Route 1149
        "/users/identity_verification/restricted{/}",
        // Route 1150
        "/users/identity_verification{/}",
        // Route 1151
        "/-/identity_verification/verification_state{/}",
        // Route 1152
        "/-/identity_verification/send_phone_verification_code{/}",
        // Route 1153
        "/-/identity_verification/verify_phone_verification_code{/}",
        // Route 1154
        "/-/identity_verification/toggle_phone_exemption{/}",
        // Route 1155
        "/-/identity_verification/verify_credit_card{/}",
        // Route 1156
        "/-/identity_verification/verify_credit_card_captcha{/}",
        // Route 1157
        "/-/identity_verification/success{/}",
        // Route 1158
        "/-/identity_verification{/}",
        // Route 1159
        "/users/auth/kerberos/negotiate{/}",
        // Route 1160
        "/users/password/complexity{/}",
        // Route 1161
        "/users/{username}/available_project_templates{/}",
        // Route 1162
        "/users/{username}/available_group_templates{/}",
        // Route 1163
        "/unsubscribes/{email}{/}",
        // Route 1164
        // "/unsubscribes/{email}{/}",
        // Route 1165
        "/users/sign_in{/}",
        // Route 1166
        // "/users/sign_in{/}",
        // Route 1167
        "/users/sign_out{/}",
        // Route 1168
        "/users/password/new{/}",
        // Route 1169
        "/users/password/edit{/}",
        // Route 1170
        "/users/password{/}",
        // Route 1171
        // "/users/password{/}",
        // Route 1172
        // "/users/password{/}",
        // Route 1173
        "/users/cancel{/}",
        // Route 1174
        "/users/sign_up{/}",
        // Route 1175
        "/users/edit{/}",
        // Route 1176
        "/users{/}",
        // Route 1177
        // "/users{/}",
        // Route 1178
        // "/users{/}",
        // Route 1179
        // "/users{/}",
        // Route 1180
        "/users/confirmation/new{/}",
        // Route 1181
        "/users/confirmation{/}",
        // Route 1182
        // "/users/confirmation{/}",
        // Route 1183
        "/users/unlock/new{/}",
        // Route 1184
        "/users/unlock{/}",
        // Route 1185
        // "/users/unlock{/}",
        // Route 1186
        "/users/auth/geo/sign_in{/}",
        // Route 1187
        // "/users/auth/geo/sign_in{/}",
        // Route 1188
        "/users/auth/geo/sign_out{/}",
        // Route 1189
        "/users/almost_there{/}",
        // Route 1190
        "/users/resend_verification_code{/}",
        // Route 1191
        "/users/successful_verification{/}",
        // Route 1192
        "/users/update_email{/}",
        // Route 1193
        "/users/auth{/}",
        // Route 1194
        "/-/users/terms/{id}/accept{/}",
        // Route 1195
        "/-/users/terms/{id}/decline{/}",
        // Route 1196
        "/-/users/terms{/}",
        // Route 1197
        "/-/users/callouts{/}",
        // Route 1198
        "/-/users/group_callouts{/}",
        // Route 1199
        "/-/users/project_callouts{/}",
        // Route 1200
        "/-/users/broadcast_message_dismissals{/}",
        // Route 1201
        "/-/users/pins{/}",
        // Route 1202
        // "/-/users/pins{/}",
        // Route 1203
        "/users/{username}/calendar{/}",
        // Route 1204
        "/users/{username}/calendar_activities{/}",
        // Route 1205
        "/users/{username}/groups{/}",
        // Route 1206
        "/users/{username}/projects{/}",
        // Route 1207
        "/users/{username}/contributed{/}",
        // Route 1208
        "/users/{username}/starred{/}",
        // Route 1209
        "/users/{username}/snippets{/}",
        // Route 1210
        "/users/{username}/followers{/}",
        // Route 1211
        "/users/{username}/following{/}",
        // Route 1212
        "/users/{username}/exists{/}",
        // Route 1213
        "/users/{username}/activity{/}",
        // Route 1214
        "/users/{username}/follow{/}",
        // Route 1215
        "/users/{username}/unfollow{/}",
        // Route 1216
        "/users/{username}{/}",
        // Route 1217
        "/{username}.keys{/}",
        // Route 1218
        "/{username}.gpg{/}",
        // Route 1219
        "/{username}{/}",
        // Route 1220
        "/{*namespace_id}/{project_id}/-/google_cloud/artifact_registry{/}",
        // Route 1221
        "/{*namespace_id}/{project_id}/-/google_cloud/artifact_registry/projects/{project}/locations/{location}/repositories/{repository}/dockerImages/{image}{/}",
        // Route 1222
        "/{*namespace_id}/{project_id}/-/requirements_management/requirements/import_csv{/}",
        // Route 1223
        "/{*namespace_id}/{project_id}/-/requirements_management/requirements/import_csv/authorize{/}",
        // Route 1224
        "/{*namespace_id}/{project_id}/-/requirements_management/requirements{/}",
        // Route 1225
        "/{*namespace_id}/{project_id}/-/quality/test_cases{/}",
        // Route 1226
        "/{*namespace_id}/{project_id}/-/quality/test_cases/new{/}",
        // Route 1227
        "/{*namespace_id}/{project_id}/-/quality/test_cases/{id}{/}",
        // Route 1228
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/epics{/}",
        // Route 1229
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/iterations{/}",
        // Route 1230
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/vulnerabilities{/}",
        // Route 1231
        "/{*namespace_id}/{project_id}/-/target_branch_rules{/}",
        // Route 1232
        // "/{*namespace_id}/{project_id}/-/target_branch_rules{/}",
        // Route 1233
        "/{*namespace_id}/{project_id}/-/target_branch_rules/{id}{/}",
        // Route 1234
        "/{*namespace_id}/{project_id}/-/comment_templates{/}",
        // Route 1235
        "/{*namespace_id}/{project_id}/-/comment_templates/{id}{/}",
        // Route 1236
        "/{*namespace_id}/{project_id}/-/automations{/}",
        // Route 1237
        "/{*namespace_id}/{project_id}/-/subscriptions{/}",
        // Route 1238
        "/{*namespace_id}/{project_id}/-/subscriptions/{id}{/}",
        // Route 1239
        "/{*namespace_id}/{project_id}/-/learn_gitlab/end_tutorial{/}",
        // Route 1240
        "/{*namespace_id}/{project_id}/-/learn_gitlab{/}",
        // Route 1241
        "/{*namespace_id}/{project_id}/-/protected_environments/search{/}",
        // Route 1242
        "/{*namespace_id}/{project_id}/-/protected_environments{/}",
        // Route 1243
        "/{*namespace_id}/{project_id}/-/protected_environments/{id}{/}",
        // Route 1244
        // "/{*namespace_id}/{project_id}/-/protected_environments/{id}{/}",
        // Route 1245
        // "/{*namespace_id}/{project_id}/-/protected_environments/{id}{/}",
        // Route 1246
        "/{*namespace_id}/{project_id}/-/audit_events{/}",
        // Route 1247
        "/{*namespace_id}/{project_id}/-/security/dashboard{/}",
        // Route 1248
        "/{*namespace_id}/{project_id}/-/security/vulnerability_report{/}",
        // Route 1249
        "/{*namespace_id}/{project_id}/-/security/policies/schema{/}",
        // Route 1250
        "/{*namespace_id}/{project_id}/-/security/policies{/}",
        // Route 1251
        "/{*namespace_id}/{project_id}/-/security/policies/new{/}",
        // Route 1252
        "/{*namespace_id}/{project_id}/-/security/policies/{id}/edit{/}",
        // Route 1253
        "/{*namespace_id}/{project_id}/-/security/configuration/corpus_management{/}",
        // Route 1254
        "/{*namespace_id}/{project_id}/-/security/configuration/api_fuzzing{/}",
        // Route 1255
        "/{*namespace_id}/{project_id}/-/security/configuration/profile_library/dast_site_profiles/new{/}",
        // Route 1256
        "/{*namespace_id}/{project_id}/-/security/configuration/profile_library/dast_site_profiles/{id}/edit{/}",
        // Route 1257
        "/{*namespace_id}/{project_id}/-/security/configuration/profile_library/dast_scanner_profiles/new{/}",
        // Route 1258
        "/{*namespace_id}/{project_id}/-/security/configuration/profile_library/dast_scanner_profiles/{id}/edit{/}",
        // Route 1259
        "/{*namespace_id}/{project_id}/-/security/configuration/profile_library{/}",
        // Route 1260
        "/{*namespace_id}/{project_id}/-/security/configuration/dast{/}",
        // Route 1261
        "/{*namespace_id}/{project_id}/-/security/configuration/secret_detection{/}",
        // Route 1262
        "/{*namespace_id}/{project_id}/-/security/discover{/}",
        // Route 1263
        "/{*namespace_id}/{project_id}/-/security/scanned_resources{/}",
        // Route 1264
        "/{*namespace_id}/{project_id}/-/security/vulnerabilities/{id}/discussions{/}",
        // Route 1265
        "/{*namespace_id}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes/{id}/toggle_award_emoji{/}",
        // Route 1266
        "/{*namespace_id}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes{/}",
        // Route 1267
        // "/{*namespace_id}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes{/}",
        // Route 1268
        "/{*namespace_id}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes/{id}{/}",
        // Route 1269
        // "/{*namespace_id}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes/{id}{/}",
        // Route 1270
        // "/{*namespace_id}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes/{id}{/}",
        // Route 1271
        "/{*namespace_id}/{project_id}/-/security/vulnerabilities/new{/}",
        // Route 1272
        "/{*namespace_id}/{project_id}/-/security/vulnerabilities/{id}{/}",
        // Route 1273
        "/{*namespace_id}/{project_id}/-/analytics/code_reviews{/}",
        // Route 1274
        "/{*namespace_id}/{project_id}/-/analytics/issues_analytics{/}",
        // Route 1275
        "/{*namespace_id}/{project_id}/-/analytics/merge_request_analytics{/}",
        // Route 1276
        "/{*namespace_id}/{project_id}/-/analytics/dashboards/{*vueroute?}{/}",
        // Route 1277
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams{/}",
        // Route 1278
        // "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams{/}",
        // Route 1279
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/new{/}",
        // Route 1280
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{id}/edit{/}",
        // Route 1281
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{id}{/}",
        // Route 1282
        // "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{id}{/}",
        // Route 1283
        // "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{id}{/}",
        // Route 1284
        // "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{id}{/}",
        // Route 1285
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/average_duration_chart{/}",
        // Route 1286
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/time_summary{/}",
        // Route 1287
        "/{*namespace_id}/{project_id}/-/approvers/{id}{/}",
        // Route 1288
        "/{*namespace_id}/{project_id}/-/approver_groups/{id}{/}",
        // Route 1289
        "/{*namespace_id}/{project_id}/-/push_rules/{id}{/}",
        // Route 1290
        // "/{*namespace_id}/{project_id}/-/push_rules/{id}{/}",
        // Route 1291
        "/{*namespace_id}/{project_id}/-/vulnerability_feedback{/}",
        // Route 1292
        // "/{*namespace_id}/{project_id}/-/vulnerability_feedback{/}",
        // Route 1293
        "/{*namespace_id}/{project_id}/-/vulnerability_feedback/{id}{/}",
        // Route 1294
        // "/{*namespace_id}/{project_id}/-/vulnerability_feedback/{id}{/}",
        // Route 1295
        // "/{*namespace_id}/{project_id}/-/vulnerability_feedback/{id}{/}",
        // Route 1296
        "/{*namespace_id}/{project_id}/-/vulnerability_feedback/count{/}",
        // Route 1297
        "/{*namespace_id}/{project_id}/-/dependencies{/}",
        // Route 1298
        "/{*namespace_id}/{project_id}/-/feature_flags/{feature_flag_iid}/issues{/}",
        // Route 1299
        // "/{*namespace_id}/{project_id}/-/feature_flags/{feature_flag_iid}/issues{/}",
        // Route 1300
        "/{*namespace_id}/{project_id}/-/feature_flags/{feature_flag_iid}/issues/{id}{/}",
        // Route 1301
        "/{*namespace_id}/{project_id}/-/feature_flags{/}",
        // Route 1302
        // "/{*namespace_id}/{project_id}/-/feature_flags{/}",
        // Route 1303
        "/{*namespace_id}/{project_id}/-/feature_flags/new{/}",
        // Route 1304
        "/{*namespace_id}/{project_id}/-/feature_flags/{iid}/edit{/}",
        // Route 1305
        "/{*namespace_id}/{project_id}/-/feature_flags/{iid}{/}",
        // Route 1306
        // "/{*namespace_id}/{project_id}/-/feature_flags/{iid}{/}",
        // Route 1307
        // "/{*namespace_id}/{project_id}/-/feature_flags/{iid}{/}",
        // Route 1308
        // "/{*namespace_id}/{project_id}/-/feature_flags/{iid}{/}",
        // Route 1309
        "/{*namespace_id}/{project_id}/-/on_demand_scans{/}",
        // Route 1310
        "/{*namespace_id}/{project_id}/-/on_demand_scans/new{/}",
        // Route 1311
        "/{*namespace_id}/{project_id}/-/on_demand_scans/{id}/edit{/}",
        // Route 1312
        "/{*namespace_id}/{project_id}/-/integrations/jira/issues{/}",
        // Route 1313
        "/{*namespace_id}/{project_id}/-/integrations/jira/issues/{id}{/}",
        // Route 1314
        "/{*namespace_id}/{project_id}/-/integrations/zentao/issues{/}",
        // Route 1315
        "/{*namespace_id}/{project_id}/-/integrations/zentao/issues/{id}{/}",
        // Route 1316
        "/{*namespace_id}/{project_id}/-/iterations{/}",
        // Route 1317
        "/{*namespace_id}/{project_id}/-/iterations/{id}{/}",
        // Route 1318
        "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}/{iteration_cadence_id}/iterations{/}",
        // Route 1319
        "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}/{iteration_cadence_id}/iterations/{id}{/}",
        // Route 1320
        "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}{/}",
        // Route 1321
        // "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}{/}",
        // Route 1322
        "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}/new{/}",
        // Route 1323
        "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}/{id}/edit{/}",
        // Route 1324
        "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}/{id}{/}",
        // Route 1325
        // "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}/{id}{/}",
        // Route 1326
        // "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}/{id}{/}",
        // Route 1327
        // "/{*namespace_id}/{project_id}/-/cadences/{*vueroute?}/{id}{/}",
        // Route 1328
        "/{*namespace_id}/{project_id}/-/oncall_schedules{/}",
        // Route 1329
        "/{*namespace_id}/{project_id}/-/escalation_policies{/}",
        // Route 1330
        "/{*namespace_id}/{project_id}/-/settings/analytics{/}",
        // Route 1331
        // "/{*namespace_id}/{project_id}/-/settings/analytics{/}",
        // Route 1332
        // "/{*namespace_id}/{project_id}/-/settings/analytics{/}",
        // Route 1333
        "/{*namespace_id}/{project_id}/-/secrets/{*vueroute?}{/}",
        // Route 1334
        "/{*namespace_id}/{project_id}/-/tracing{/}",
        // Route 1335
        "/{*namespace_id}/{project_id}/-/tracing/{id}{/}",
        // Route 1336
        "/{*namespace_id}/{project_id}/-/metrics{/}",
        // Route 1337
        "/{*namespace_id}/{project_id}/-/metrics/{id}{/}",
        // Route 1338
        "/{*namespace_id}/{project_id}/-/logs{/}",
        // Route 1339
        "/{*namespace_id}/{project_id}/-/ml/agents/{*vueroute?}{/}",
        // Route 1340
        // "/{*namespace_id}/{project_id}/-/ml/agents/{*vueroute?}{/}",
        // Route 1341
        "/{*namespace_id}/{project_id}/-/ml/agents/{*vueroute?}/new{/}",
        // Route 1342
        "/{*namespace_id}/{project_id}/-/ml/agents/{*vueroute?}/{id}/edit{/}",
        // Route 1343
        "/{*namespace_id}/{project_id}/-/ml/agents/{*vueroute?}/{id}{/}",
        // Route 1344
        // "/{*namespace_id}/{project_id}/-/ml/agents/{*vueroute?}/{id}{/}",
        // Route 1345
        // "/{*namespace_id}/{project_id}/-/ml/agents/{*vueroute?}/{id}{/}",
        // Route 1346
        // "/{*namespace_id}/{project_id}/-/ml/agents/{*vueroute?}/{id}{/}",
        // Route 1347
        "/{*namespace_id}/{project_id}/-/merge_trains{/}",
        // Route 1348
        "/{*namespace_id}/{project_id}/path_locks/toggle{/}",
        // Route 1349
        "/{*namespace_id}/{project_id}/path_locks{/}",
        // Route 1350
        "/{*namespace_id}/{project_id}/path_locks/{id}{/}",
        // Route 1351
        "/{*namespace_id}/{project_id}/restore{/}",
        // Route 1352
        "/{*namespace_id}/{project_id}/insights/query{/}",
        // Route 1353
        "/{*namespace_id}/{project_id}/insights{/}",
        // Route 1354
        "/{*namespace_id}/{project_id}/-/preview_markdown{/}",
        // Route 1355
        "/{*namespace_id}/{project_id}/-/archive/{id}.{format}{/}",
        // Route 1356
        "/{*namespace_id}/{project_id}/-/security/configuration/sast{/}",
        // Route 1357
        "/{*namespace_id}/{project_id}/-/security/configuration{/}",
        // Route 1358
        "/{*namespace_id}/{project_id}/-/artifacts{/}",
        // Route 1359
        "/{*namespace_id}/{project_id}/-/artifacts/{id}{/}",
        // Route 1360
        "/{*namespace_id}/{project_id}/-/packages{/}",
        // Route 1361
        "/{*namespace_id}/{project_id}/-/packages/{id}{/}",
        // Route 1362
        // "/{*namespace_id}/{project_id}/-/packages/{id}{/}",
        // Route 1363
        "/{*namespace_id}/{project_id}/-/package_files/{id}/download{/}",
        // Route 1364
        "/{*namespace_id}/{project_id}/-/terraform_module_registry{/}",
        // Route 1365
        "/{*namespace_id}/{project_id}/-/terraform_module_registry/{id}{/}",
        // Route 1366
        "/{*namespace_id}/{project_id}/-/infrastructure_registry{/}",
        // Route 1367
        "/{*namespace_id}/{project_id}/-/jobs/artifacts/{*ref_name_and_path}{/}",
        // Route 1368
        "/{*namespace_id}/{project_id}/-/jobs/{id}/status{/}",
        // Route 1369
        "/{*namespace_id}/{project_id}/-/jobs/{id}/cancel{/}",
        // Route 1370
        "/{*namespace_id}/{project_id}/-/jobs/{id}/unschedule{/}",
        // Route 1371
        "/{*namespace_id}/{project_id}/-/jobs/{id}/retry{/}",
        // Route 1372
        "/{*namespace_id}/{project_id}/-/jobs/{id}/play{/}",
        // Route 1373
        "/{*namespace_id}/{project_id}/-/jobs/{id}/erase{/}",
        // Route 1374
        "/{*namespace_id}/{project_id}/-/jobs/{id}/trace.{format?}{/}",
        // Route 1375
        "/{*namespace_id}/{project_id}/-/jobs/{id}/raw{/}",
        // Route 1376
        "/{*namespace_id}/{project_id}/-/jobs/{id}/viewer{/}",
        // Route 1377
        "/{*namespace_id}/{project_id}/-/jobs/{id}/terminal{/}",
        // Route 1378
        "/{*namespace_id}/{project_id}/-/jobs/{id}/proxy{/}",
        // Route 1379
        "/{*namespace_id}/{project_id}/-/jobs/{id}/test_report_summary{/}",
        // Route 1380
        "/{*namespace_id}/{project_id}/-/jobs/{id}/terminal.ws/authorize{/}",
        // Route 1381
        "/{*namespace_id}/{project_id}/-/jobs/{id}/proxy.ws/authorize{/}",
        // Route 1382
        "/{*namespace_id}/{project_id}/-/jobs/{job_id}/artifacts/download{/}",
        // Route 1383
        "/{*namespace_id}/{project_id}/-/jobs/{job_id}/artifacts/browse/{*path?}{/}",
        // Route 1384
        "/{*namespace_id}/{project_id}/-/jobs/{job_id}/artifacts/file/{*path}{/}",
        // Route 1385
        "/{*namespace_id}/{project_id}/-/jobs/{job_id}/artifacts/external_file/{*path}{/}",
        // Route 1386
        "/{*namespace_id}/{project_id}/-/jobs/{job_id}/artifacts/raw/{*path}{/}",
        // Route 1387
        "/{*namespace_id}/{project_id}/-/jobs/{job_id}/artifacts/keep{/}",
        // Route 1388
        "/{*namespace_id}/{project_id}/-/jobs{/}",
        // Route 1389
        "/{*namespace_id}/{project_id}/-/jobs/{id}{/}",
        // Route 1390
        "/{*namespace_id}/{project_id}/-/ci/lint{/}",
        // Route 1391
        // "/{*namespace_id}/{project_id}/-/ci/lint{/}",
        // Route 1392
        "/{*namespace_id}/{project_id}/-/ci/editor{/}",
        // Route 1393
        "/{*namespace_id}/{project_id}/-/ci/daily_build_group_report_results.{format?}{/}",
        // Route 1394
        "/{*namespace_id}/{project_id}/-/ci/prometheus_metrics/histograms.{format?}{/}",
        // Route 1395
        "/{*namespace_id}/{project_id}/-/runners/{id}/register{/}",
        // Route 1396
        "/{*namespace_id}/{project_id}/-/runners/{id}/resume{/}",
        // Route 1397
        "/{*namespace_id}/{project_id}/-/runners/{id}/pause{/}",
        // Route 1398
        "/{*namespace_id}/{project_id}/-/runners/toggle_shared_runners{/}",
        // Route 1399
        "/{*namespace_id}/{project_id}/-/runners/toggle_group_runners{/}",
        // Route 1400
        "/{*namespace_id}/{project_id}/-/runners{/}",
        // Route 1401
        "/{*namespace_id}/{project_id}/-/runners/new{/}",
        // Route 1402
        "/{*namespace_id}/{project_id}/-/runners/{id}/edit{/}",
        // Route 1403
        "/{*namespace_id}/{project_id}/-/runners/{id}{/}",
        // Route 1404
        // "/{*namespace_id}/{project_id}/-/runners/{id}{/}",
        // Route 1405
        // "/{*namespace_id}/{project_id}/-/runners/{id}{/}",
        // Route 1406
        // "/{*namespace_id}/{project_id}/-/runners/{id}{/}",
        // Route 1407
        "/{*namespace_id}/{project_id}/-/settings/ci_cd/reset_cache{/}",
        // Route 1408
        "/{*namespace_id}/{project_id}/-/settings/ci_cd/reset_registration_token{/}",
        // Route 1409
        "/{*namespace_id}/{project_id}/-/settings/ci_cd/deploy_token/create{/}",
        // Route 1410
        "/{*namespace_id}/{project_id}/-/settings/ci_cd/runner_setup_scripts{/}",
        // Route 1411
        "/{*namespace_id}/{project_id}/-/settings/ci_cd{/}",
        // Route 1412
        // "/{*namespace_id}/{project_id}/-/settings/ci_cd{/}",
        // Route 1413
        // "/{*namespace_id}/{project_id}/-/settings/ci_cd{/}",
        // Route 1414
        "/{*namespace_id}/{project_id}/-/settings/operations/reset_alerting_token{/}",
        // Route 1415
        "/{*namespace_id}/{project_id}/-/settings/operations/reset_pagerduty_token{/}",
        // Route 1416
        "/{*namespace_id}/{project_id}/-/settings/operations{/}",
        // Route 1417
        // "/{*namespace_id}/{project_id}/-/settings/operations{/}",
        // Route 1418
        // "/{*namespace_id}/{project_id}/-/settings/operations{/}",
        // Route 1419
        "/{*namespace_id}/{project_id}/-/settings/integrations/{id}/test{/}",
        // Route 1420
        "/{*namespace_id}/{project_id}/-/settings/integrations/{integration_id}/hook_logs/{id}/retry{/}",
        // Route 1421
        "/{*namespace_id}/{project_id}/-/settings/integrations/{integration_id}/hook_logs/{id}{/}",
        // Route 1422
        "/{*namespace_id}/{project_id}/-/settings/integrations{/}",
        // Route 1423
        "/{*namespace_id}/{project_id}/-/settings/integrations/{id}/edit{/}",
        // Route 1424
        "/{*namespace_id}/{project_id}/-/settings/integrations/{id}{/}",
        // Route 1425
        // "/{*namespace_id}/{project_id}/-/settings/integrations/{id}{/}",
        // Route 1426
        "/{*namespace_id}/{project_id}/-/settings/slack/slack_auth{/}",
        // Route 1427
        "/{*namespace_id}/{project_id}/-/settings/slack/edit{/}",
        // Route 1428
        "/{*namespace_id}/{project_id}/-/settings/slack{/}",
        // Route 1429
        // "/{*namespace_id}/{project_id}/-/settings/slack{/}",
        // Route 1430
        // "/{*namespace_id}/{project_id}/-/settings/slack{/}",
        // Route 1431
        "/{*namespace_id}/{project_id}/-/settings/repository/deploy_token/create{/}",
        // Route 1432
        "/{*namespace_id}/{project_id}/-/settings/repository/cleanup{/}",
        // Route 1433
        "/{*namespace_id}/{project_id}/-/settings/repository/branch_rules{/}",
        // Route 1434
        "/{*namespace_id}/{project_id}/-/settings/repository{/}",
        // Route 1435
        // "/{*namespace_id}/{project_id}/-/settings/repository{/}",
        // Route 1436
        // "/{*namespace_id}/{project_id}/-/settings/repository{/}",
        // Route 1437
        "/{*namespace_id}/{project_id}/-/settings/access_tokens/{id}/revoke{/}",
        // Route 1438
        "/{*namespace_id}/{project_id}/-/settings/access_tokens{/}",
        // Route 1439
        // "/{*namespace_id}/{project_id}/-/settings/access_tokens{/}",
        // Route 1440
        "/{*namespace_id}/{project_id}/-/settings/packages_and_registries/cleanup_image_tags{/}",
        // Route 1441
        "/{*namespace_id}/{project_id}/-/settings/packages_and_registries{/}",
        // Route 1442
        "/{*namespace_id}/{project_id}/-/settings/merge_requests{/}",
        // Route 1443
        // "/{*namespace_id}/{project_id}/-/settings/merge_requests{/}",
        // Route 1444
        // "/{*namespace_id}/{project_id}/-/settings/merge_requests{/}",
        // Route 1445
        "/{*namespace_id}/{project_id}/-/usage_quotas{/}",
        // Route 1446
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/members{/}",
        // Route 1447
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/issues{/}",
        // Route 1448
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/merge_requests{/}",
        // Route 1449
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/labels{/}",
        // Route 1450
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/milestones{/}",
        // Route 1451
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/commands{/}",
        // Route 1452
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/snippets{/}",
        // Route 1453
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/contacts{/}",
        // Route 1454
        "/{*namespace_id}/{project_id}/-/autocomplete_sources/wikis{/}",
        // Route 1455
        "/{*namespace_id}/{project_id}/-/project_members/leave{/}",
        // Route 1456
        "/{*namespace_id}/{project_id}/-/project_members/{id}/resend_invite{/}",
        // Route 1457
        "/{*namespace_id}/{project_id}/-/project_members/request_access{/}",
        // Route 1458
        // "/{*namespace_id}/{project_id}/-/project_members/request_access{/}",
        // Route 1459
        "/{*namespace_id}/{project_id}/-/project_members/{id}/approve_access_request{/}",
        // Route 1460
        "/{*namespace_id}/{project_id}/-/project_members{/}",
        // Route 1461
        "/{*namespace_id}/{project_id}/-/project_members/{id}{/}",
        // Route 1462
        // "/{*namespace_id}/{project_id}/-/project_members/{id}{/}",
        // Route 1463
        // "/{*namespace_id}/{project_id}/-/project_members/{id}{/}",
        // Route 1464
        "/{*namespace_id}/{project_id}/-/deploy_keys/enabled_keys{/}",
        // Route 1465
        "/{*namespace_id}/{project_id}/-/deploy_keys/available_project_keys{/}",
        // Route 1466
        "/{*namespace_id}/{project_id}/-/deploy_keys/available_public_keys{/}",
        // Route 1467
        "/{*namespace_id}/{project_id}/-/deploy_keys/{id}/enable{/}",
        // Route 1468
        "/{*namespace_id}/{project_id}/-/deploy_keys/{id}/disable{/}",
        // Route 1469
        "/{*namespace_id}/{project_id}/-/deploy_keys{/}",
        // Route 1470
        // "/{*namespace_id}/{project_id}/-/deploy_keys{/}",
        // Route 1471
        "/{*namespace_id}/{project_id}/-/deploy_keys/new{/}",
        // Route 1472
        "/{*namespace_id}/{project_id}/-/deploy_keys/{id}/edit{/}",
        // Route 1473
        "/{*namespace_id}/{project_id}/-/deploy_keys/{id}{/}",
        // Route 1474
        // "/{*namespace_id}/{project_id}/-/deploy_keys/{id}{/}",
        // Route 1475
        "/{*namespace_id}/{project_id}/-/deploy_tokens/{id}/revoke{/}",
        // Route 1476
        "/{*namespace_id}/{project_id}/-/milestones/{id}/promote{/}",
        // Route 1477
        "/{*namespace_id}/{project_id}/-/milestones/{id}/issues{/}",
        // Route 1478
        "/{*namespace_id}/{project_id}/-/milestones/{id}/merge_requests{/}",
        // Route 1479
        "/{*namespace_id}/{project_id}/-/milestones/{id}/participants{/}",
        // Route 1480
        "/{*namespace_id}/{project_id}/-/milestones/{id}/labels{/}",
        // Route 1481
        "/{*namespace_id}/{project_id}/-/milestones{/}",
        // Route 1482
        // "/{*namespace_id}/{project_id}/-/milestones{/}",
        // Route 1483
        "/{*namespace_id}/{project_id}/-/milestones/new{/}",
        // Route 1484
        "/{*namespace_id}/{project_id}/-/milestones/{id}/edit{/}",
        // Route 1485
        "/{*namespace_id}/{project_id}/-/milestones/{id}{/}",
        // Route 1486
        // "/{*namespace_id}/{project_id}/-/milestones/{id}{/}",
        // Route 1487
        // "/{*namespace_id}/{project_id}/-/milestones/{id}{/}",
        // Route 1488
        // "/{*namespace_id}/{project_id}/-/milestones/{id}{/}",
        // Route 1489
        "/{*namespace_id}/{project_id}/-/labels/generate{/}",
        // Route 1490
        "/{*namespace_id}/{project_id}/-/labels/set_priorities{/}",
        // Route 1491
        "/{*namespace_id}/{project_id}/-/labels/{id}/promote{/}",
        // Route 1492
        "/{*namespace_id}/{project_id}/-/labels/{id}/toggle_subscription{/}",
        // Route 1493
        "/{*namespace_id}/{project_id}/-/labels/{id}/remove_priority{/}",
        // Route 1494
        "/{*namespace_id}/{project_id}/-/labels{/}",
        // Route 1495
        // "/{*namespace_id}/{project_id}/-/labels{/}",
        // Route 1496
        "/{*namespace_id}/{project_id}/-/labels/new{/}",
        // Route 1497
        "/{*namespace_id}/{project_id}/-/labels/{id}/edit{/}",
        // Route 1498
        "/{*namespace_id}/{project_id}/-/labels/{id}{/}",
        // Route 1499
        // "/{*namespace_id}/{project_id}/-/labels/{id}{/}",
        // Route 1500
        // "/{*namespace_id}/{project_id}/-/labels/{id}{/}",
        // Route 1501
        "/{*namespace_id}/{project_id}/-/boards{/}",
        // Route 1502
        "/{*namespace_id}/{project_id}/-/boards/{id}{/}",
        // Route 1503
        "/{*namespace_id}/{project_id}/-/releases/permalink/latest/{*suffix_path?}{/}",
        // Route 1504
        "/{*namespace_id}/{project_id}/-/releases/{tag}/downloads/{*filepath}{/}",
        // Route 1505
        "/{*namespace_id}/{project_id}/-/releases/{tag}/evidences/{id}{/}",
        // Route 1506
        // "/{*namespace_id}/{project_id}/-/releases{/}",
        // Route 1507
        "/{*namespace_id}/{project_id}/-/releases/new{/}",
        // Route 1508
        "/{*namespace_id}/{project_id}/-/releases/{tag}/edit{/}",
        // Route 1509
        "/{*namespace_id}/{project_id}/-/releases/{tag}{/}",
        // Route 1510
        "/{*namespace_id}/{project_id}/-/starrers{/}",
        // Route 1511
        "/{*namespace_id}/{project_id}/-/forks{/}",
        // Route 1512
        // "/{*namespace_id}/{project_id}/-/forks{/}",
        // Route 1513
        "/{*namespace_id}/{project_id}/-/forks/new{/}",
        // Route 1514
        "/{*namespace_id}/{project_id}/-/group_links/{id}{/}",
        // Route 1515
        // "/{*namespace_id}/{project_id}/-/group_links/{id}{/}",
        // Route 1516
        // "/{*namespace_id}/{project_id}/-/group_links/{id}{/}",
        // Route 1517
        "/{*namespace_id}/{project_id}/-/import/new{/}",
        // Route 1518
        "/{*namespace_id}/{project_id}/-/import{/}",
        // Route 1519
        // "/{*namespace_id}/{project_id}/-/import{/}",
        // Route 1520
        "/{*namespace_id}/{project_id}/-/avatar{/}",
        // Route 1521
        // "/{*namespace_id}/{project_id}/-/avatar{/}",
        // Route 1522
        "/{*namespace_id}/{project_id}/-/mattermost/new{/}",
        // Route 1523
        "/{*namespace_id}/{project_id}/-/mattermost{/}",
        // Route 1524
        "/{*namespace_id}/{project_id}/-/variables{/}",
        // Route 1525
        // "/{*namespace_id}/{project_id}/-/variables{/}",
        // Route 1526
        // "/{*namespace_id}/{project_id}/-/variables{/}",
        // Route 1527
        "/{*namespace_id}/{project_id}/-/triggers{/}",
        // Route 1528
        // "/{*namespace_id}/{project_id}/-/triggers{/}",
        // Route 1529
        "/{*namespace_id}/{project_id}/-/triggers/{id}{/}",
        // Route 1530
        // "/{*namespace_id}/{project_id}/-/triggers/{id}{/}",
        // Route 1531
        // "/{*namespace_id}/{project_id}/-/triggers/{id}{/}",
        // Route 1532
        "/{*namespace_id}/{project_id}/-/mirror/ssh_host_keys.{format?}{/}",
        // Route 1533
        "/{*namespace_id}/{project_id}/-/mirror/update_now{/}",
        // Route 1534
        "/{*namespace_id}/{project_id}/-/mirror{/}",
        // Route 1535
        // "/{*namespace_id}/{project_id}/-/mirror{/}",
        // Route 1536
        // "/{*namespace_id}/{project_id}/-/mirror{/}",
        // Route 1537
        "/{*namespace_id}/{project_id}/-/value_stream_analytics{/}",
        // Route 1538
        "/{*namespace_id}/{project_id}/-/value_stream_analytics/events/issue{/}",
        // Route 1539
        "/{*namespace_id}/{project_id}/-/value_stream_analytics/events/plan{/}",
        // Route 1540
        "/{*namespace_id}/{project_id}/-/value_stream_analytics/events/code{/}",
        // Route 1541
        "/{*namespace_id}/{project_id}/-/value_stream_analytics/events/test{/}",
        // Route 1542
        "/{*namespace_id}/{project_id}/-/value_stream_analytics/events/review{/}",
        // Route 1543
        "/{*namespace_id}/{project_id}/-/value_stream_analytics/events/staging{/}",
        // Route 1544
        "/{*namespace_id}/{project_id}/-/value_stream_analytics/events/production{/}",
        // Route 1545
        "/{*namespace_id}/{project_id}/-/cycle_analytics{/}",
        // Route 1546
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics{/}",
        // Route 1547
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/median{/}",
        // Route 1548
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/average{/}",
        // Route 1549
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/records{/}",
        // Route 1550
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/count{/}",
        // Route 1551
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages{/}",
        // Route 1552
        // "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/value_streams{/}",
        // Route 1553
        "/{*namespace_id}/{project_id}/-/analytics/value_stream_analytics/summary{/}",
        // Route 1554
        "/{*namespace_id}/{project_id}/-/cluster_agents/{name}{/}",
        // Route 1555
        "/{*namespace_id}/{project_id}/-/clusters/connect{/}",
        // Route 1556
        "/{*namespace_id}/{project_id}/-/clusters/new_cluster_docs{/}",
        // Route 1557
        "/{*namespace_id}/{project_id}/-/clusters/create_user{/}",
        // Route 1558
        "/{*namespace_id}/{project_id}/-/clusters/{cluster_id}/integration/create_or_update{/}",
        // Route 1559
        "/{*namespace_id}/{project_id}/-/clusters/{id}/metrics{/}",
        // Route 1560
        "/{*namespace_id}/{project_id}/-/clusters/{id}/environments{/}",
        // Route 1561
        "/{*namespace_id}/{project_id}/-/clusters/{id}/metrics_dashboard{/}",
        // Route 1562
        "/{*namespace_id}/{project_id}/-/clusters/{id}/cluster_status{/}",
        // Route 1563
        "/{*namespace_id}/{project_id}/-/clusters/{id}/clear_cache{/}",
        // Route 1564
        "/{*namespace_id}/{project_id}/-/clusters{/}",
        // Route 1565
        "/{*namespace_id}/{project_id}/-/clusters/{id}{/}",
        // Route 1566
        // "/{*namespace_id}/{project_id}/-/clusters/{id}{/}",
        // Route 1567
        // "/{*namespace_id}/{project_id}/-/clusters/{id}{/}",
        // Route 1568
        // "/{*namespace_id}/{project_id}/-/clusters/{id}{/}",
        // Route 1569
        "/{*namespace_id}/{project_id}/-/terraform{/}",
        // Route 1570
        "/{*namespace_id}/{project_id}/-/google_cloud{/}",
        // Route 1571
        "/{*namespace_id}/{project_id}/-/google_cloud/configuration{/}",
        // Route 1572
        "/{*namespace_id}/{project_id}/-/google_cloud/revoke_oauth{/}",
        // Route 1573
        "/{*namespace_id}/{project_id}/-/google_cloud/service_accounts{/}",
        // Route 1574
        // "/{*namespace_id}/{project_id}/-/google_cloud/service_accounts{/}",
        // Route 1575
        "/{*namespace_id}/{project_id}/-/google_cloud/gcp_regions{/}",
        // Route 1576
        // "/{*namespace_id}/{project_id}/-/google_cloud/gcp_regions{/}",
        // Route 1577
        "/{*namespace_id}/{project_id}/-/google_cloud/deployments{/}",
        // Route 1578
        "/{*namespace_id}/{project_id}/-/google_cloud/deployments/cloud_run{/}",
        // Route 1579
        "/{*namespace_id}/{project_id}/-/google_cloud/deployments/cloud_storage{/}",
        // Route 1580
        "/{*namespace_id}/{project_id}/-/google_cloud/databases{/}",
        // Route 1581
        // "/{*namespace_id}/{project_id}/-/google_cloud/databases{/}",
        // Route 1582
        "/{*namespace_id}/{project_id}/-/google_cloud/databases/new/{product}{/}",
        // Route 1583
        "/{*namespace_id}/{project_id}/-/aws{/}",
        // Route 1584
        "/{*namespace_id}/{project_id}/-/aws/configuration{/}",
        // Route 1585
        "/{*namespace_id}/{project_id}/-/environments/{id}/stop{/}",
        // Route 1586
        "/{*namespace_id}/{project_id}/-/environments/{id}/cancel_auto_stop{/}",
        // Route 1587
        "/{*namespace_id}/{project_id}/-/environments/{id}/terminal{/}",
        // Route 1588
        "/{*namespace_id}/{project_id}/-/environments/{id}/k8s/{*vueroute?}{/}",
        // Route 1589
        "/{*namespace_id}/{project_id}/-/environments/{id}/terminal.ws/authorize{/}",
        // Route 1590
        "/{*namespace_id}/{project_id}/-/environments/{id}/prometheus/api/v1/{*proxy_path}{/}",
        // Route 1591
        "/{*namespace_id}/{project_id}/-/environments/folders/{*id?}/{name}.{format?}{/}",
        // Route 1592
        "/{*namespace_id}/{project_id}/-/environments/search{/}",
        // Route 1593
        "/{*namespace_id}/{project_id}/-/environments/{environment_id}/deployments/{id}/metrics{/}",
        // Route 1594
        "/{*namespace_id}/{project_id}/-/environments/{environment_id}/deployments/{id}/additional_metrics{/}",
        // Route 1595
        "/{*namespace_id}/{project_id}/-/environments/{environment_id}/deployments{/}",
        // Route 1596
        "/{*namespace_id}/{project_id}/-/environments/{environment_id}/deployments/{id}{/}",
        // Route 1597
        "/{*namespace_id}/{project_id}/-/environments{/}",
        // Route 1598
        // "/{*namespace_id}/{project_id}/-/environments{/}",
        // Route 1599
        "/{*namespace_id}/{project_id}/-/environments/new{/}",
        // Route 1600
        "/{*namespace_id}/{project_id}/-/environments/{id}/edit{/}",
        // Route 1601
        "/{*namespace_id}/{project_id}/-/environments/{id}{/}",
        // Route 1602
        // "/{*namespace_id}/{project_id}/-/environments/{id}{/}",
        // Route 1603
        // "/{*namespace_id}/{project_id}/-/environments/{id}{/}",
        // Route 1604
        "/{*namespace_id}/{project_id}/-/alert_management/{id}/details/{*page?}{/}",
        // Route 1605
        "/{*namespace_id}/{project_id}/-/alert_management{/}",
        // Route 1606
        "/{*namespace_id}/{project_id}/-/alert_management/{id}{/}",
        // Route 1607
        "/{*namespace_id}/{project_id}/-/work_items/import_csv{/}",
        // Route 1608
        "/{*namespace_id}/{project_id}/-/work_items/import_csv/authorize{/}",
        // Route 1609
        "/{*namespace_id}/{project_id}/-/work_items/{iid}/designs/{*vueroute?}{/}",
        // Route 1610
        "/{*namespace_id}/{project_id}/-/work_items/{iid}{/}",
        // Route 1611
        "/{*namespace_id}/{project_id}/-/incidents/integrations/pagerduty{/}",
        // Route 1612
        "/{*namespace_id}/{project_id}/-/incidents{/}",
        // Route 1613
        "/{*namespace_id}/{project_id}/-/incident_management/timeline_events/preview_markdown{/}",
        // Route 1614
        "/{*namespace_id}/{project_id}/-/error_tracking/projects{/}",
        // Route 1615
        "/{*namespace_id}/{project_id}/-/error_tracking/{issue_id}/details{/}",
        // Route 1616
        "/{*namespace_id}/{project_id}/-/error_tracking/{issue_id}/stack_trace{/}",
        // Route 1617
        "/{*namespace_id}/{project_id}/-/error_tracking/{issue_id}{/}",
        // Route 1618
        "/{*namespace_id}/{project_id}/-/error_tracking{/}",
        // Route 1619
        "/{*namespace_id}/{project_id}/-/design_management/designs/{design_id}/{sha?}/raw_image{/}",
        // Route 1620
        "/{*namespace_id}/{project_id}/-/design_management/designs/{design_id}/{sha?}/resized_image/{id}{/}",
        // Route 1621
        "/{*namespace_id}/{project_id}/-/snippets/{snippet_id}/raw/{ref}/{*path}{/}",
        // Route 1622
        "/{*namespace_id}/{project_id}/-/issues/{id}/descriptions/{version_id}/diff{/}",
        // Route 1623
        "/{*namespace_id}/{project_id}/-/issues/{id}/descriptions/{version_id}{/}",
        // Route 1624
        "/{*namespace_id}/{project_id}/-/issues/{issue_id}/feature_flags{/}",
        // Route 1625
        "/{*namespace_id}/{project_id}/-/issues/{issue_id}/feature_flags/{id}{/}",
        // Route 1626
        "/{*namespace_id}/{project_id}/-/issues{/}",
        // Route 1627
        "/{*namespace_id}/{project_id}/-/issues/{id}/toggle_subscription{/}",
        // Route 1628
        "/{*namespace_id}/{project_id}/-/issues/{id}/mark_as_spam{/}",
        // Route 1629
        "/{*namespace_id}/{project_id}/-/issues/{id}/move{/}",
        // Route 1630
        "/{*namespace_id}/{project_id}/-/issues/{id}/reorder{/}",
        // Route 1631
        "/{*namespace_id}/{project_id}/-/issues/{id}/related_branches{/}",
        // Route 1632
        "/{*namespace_id}/{project_id}/-/issues/{id}/can_create_branch{/}",
        // Route 1633
        "/{*namespace_id}/{project_id}/-/issues/{id}/realtime_changes{/}",
        // Route 1634
        "/{*namespace_id}/{project_id}/-/issues/{id}/create_merge_request{/}",
        // Route 1635
        "/{*namespace_id}/{project_id}/-/issues/{id}/discussions{/}",
        // Route 1636
        "/{*namespace_id}/{project_id}/-/issues/{id}/designs/{*vueroute?}{/}",
        // Route 1637
        "/{*namespace_id}/{project_id}/-/issues/{id}/{incident_tab}{/}",
        // Route 1638
        "/{*namespace_id}/{project_id}/-/issues/service_desk{/}",
        // Route 1639
        "/{*namespace_id}/{project_id}/-/issues/bulk_update{/}",
        // Route 1640
        "/{*namespace_id}/{project_id}/-/issues/import_csv{/}",
        // Route 1641
        "/{*namespace_id}/{project_id}/-/issues/export_csv{/}",
        // Route 1642
        "/{*namespace_id}/{project_id}/-/issues/incident/{id}/{incident_tab?}{/}",
        // Route 1643
        "/{*namespace_id}/{project_id}/-/issues/{issue_id}/links{/}",
        // Route 1644
        // "/{*namespace_id}/{project_id}/-/issues/{issue_id}/links{/}",
        // Route 1645
        "/{*namespace_id}/{project_id}/-/issues/{issue_id}/links/{id}{/}",
        // Route 1646
        "/{*namespace_id}/{project_id}/-/issues/{id}/toggle_award_emoji{/}",
        // Route 1647
        // "/{*namespace_id}/{project_id}/-/issues{/}",
        // Route 1648
        // "/{*namespace_id}/{project_id}/-/issues{/}",
        // Route 1649
        "/{*namespace_id}/{project_id}/-/issues/new{/}",
        // Route 1650
        "/{*namespace_id}/{project_id}/-/issues/{id}/edit{/}",
        // Route 1651
        "/{*namespace_id}/{project_id}/-/issues/{id}{/}",
        // Route 1652
        // "/{*namespace_id}/{project_id}/-/issues/{id}{/}",
        // Route 1653
        // "/{*namespace_id}/{project_id}/-/issues/{id}{/}",
        // Route 1654
        // "/{*namespace_id}/{project_id}/-/issues/{id}{/}",
        // Route 1655
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/descriptions/{version_id}/diff{/}",
        // Route 1656
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/descriptions/{version_id}{/}",
        // Route 1657
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/metrics_reports{/}",
        // Route 1658
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/license_scanning_reports{/}",
        // Route 1659
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/license_scanning_reports_collapsed{/}",
        // Route 1660
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/container_scanning_reports{/}",
        // Route 1661
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/dependency_scanning_reports{/}",
        // Route 1662
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/sast_reports{/}",
        // Route 1663
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/secret_detection_reports{/}",
        // Route 1664
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/dast_reports{/}",
        // Route 1665
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/coverage_fuzzing_reports{/}",
        // Route 1666
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/api_fuzzing_reports{/}",
        // Route 1667
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/security_reports{/}",
        // Route 1668
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/saml_approval{/}",
        // Route 1669
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/rebase{/}",
        // Route 1670
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/reports{/}",
        // Route 1671
        "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/approvers/{id}{/}",
        // Route 1672
        "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/approvers{/}",
        // Route 1673
        "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/approver_groups/{id}{/}",
        // Route 1674
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}{/}",
        // Route 1675
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/commit_change_content{/}",
        // Route 1676
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/merge{/}",
        // Route 1677
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/cancel_auto_merge{/}",
        // Route 1678
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/pipeline_status{/}",
        // Route 1679
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/ci_environments_status{/}",
        // Route 1680
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/toggle_subscription{/}",
        // Route 1681
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/remove_wip{/}",
        // Route 1682
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/assign_related_issues{/}",
        // Route 1683
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/discussions{/}",
        // Route 1684
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}/rebase{/}",
        // Route 1685
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/test_reports{/}",
        // Route 1686
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/exposed_artifacts{/}",
        // Route 1687
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/accessibility_reports{/}",
        // Route 1688
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/coverage_reports{/}",
        // Route 1689
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/terraform_reports{/}",
        // Route 1690
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/codequality_reports{/}",
        // Route 1691
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/codequality_mr_diff_reports{/}",
        // Route 1692
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/commits{/}",
        // Route 1693
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/pipelines{/}",
        // Route 1694
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/context_commits{/}",
        // Route 1695
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diffs{/}",
        // Route 1696
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diffs_batch{/}",
        // Route 1697
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diffs_metadata{/}",
        // Route 1698
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/widget{/}",
        // Route 1699
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/cached_widget{/}",
        // Route 1700
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}/commits{/}",
        // Route 1701
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}/pipelines{/}",
        // Route 1702
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diffs{/}",
        // Route 1703
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diffs{/}",
        // Route 1704
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diff_for_path{/}",
        // Route 1705
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diff_by_file_hash/{file_hash}{/}",
        // Route 1706
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diffs_stream{/}",
        // Route 1707
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}/diff_for_path{/}",
        // Route 1708
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/conflicts{/}",
        // Route 1709
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/conflict_for_path{/}",
        // Route 1710
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/resolve_conflicts{/}",
        // Route 1711
        "/{*namespace_id}/{project_id}/-/merge_requests/diff_for_path{/}",
        // Route 1712
        "/{*namespace_id}/{project_id}/-/merge_requests/bulk_update{/}",
        // Route 1713
        "/{*namespace_id}/{project_id}/-/merge_requests/export_csv{/}",
        // Route 1714
        "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/drafts/publish{/}",
        // Route 1715
        "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/drafts/discard{/}",
        // Route 1716
        "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/drafts{/}",
        // Route 1717
        // "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/drafts{/}",
        // Route 1718
        "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/drafts/{id}{/}",
        // Route 1719
        // "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/drafts/{id}{/}",
        // Route 1720
        // "/{*namespace_id}/{project_id}/-/merge_requests/{merge_request_id}/drafts/{id}{/}",
        // Route 1721
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/toggle_award_emoji{/}",
        // Route 1722
        "/{*namespace_id}/{project_id}/-/merge_requests{/}",
        // Route 1723
        "/{*namespace_id}/{project_id}/-/merge_requests/{id}/edit{/}",
        // Route 1724
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}{/}",
        // Route 1725
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}{/}",
        // Route 1726
        // "/{*namespace_id}/{project_id}/-/merge_requests/{id}{/}",
        // Route 1727
        // "/{*namespace_id}/{project_id}/-/merge_requests{/}",
        // Route 1728
        "/{*namespace_id}/{project_id}/-/merge_requests/new{/}",
        // Route 1729
        "/{*namespace_id}/{project_id}/-/merge_requests/new/diffs{/}",
        // Route 1730
        "/{*namespace_id}/{project_id}/-/merge_requests/new/pipelines{/}",
        // Route 1731
        "/{*namespace_id}/{project_id}/-/merge_requests/new/target_projects{/}",
        // Route 1732
        // "/{*namespace_id}/{project_id}/-/merge_requests/new/diffs{/}",
        // Route 1733
        // "/{*namespace_id}/{project_id}/-/merge_requests/new/pipelines{/}",
        // Route 1734
        "/{*namespace_id}/{project_id}/-/merge_requests/new/diff_for_path{/}",
        // Route 1735
        "/{*namespace_id}/{project_id}/-/merge_requests/new/branch_from{/}",
        // Route 1736
        "/{*namespace_id}/{project_id}/-/merge_requests/new/branch_to{/}",
        // Route 1737
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/security{/}",
        // Route 1738
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/licenses{/}",
        // Route 1739
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/license_count{/}",
        // Route 1740
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/codequality_report{/}",
        // Route 1741
        "/{*namespace_id}/{project_id}/-/pipelines/{pipeline_id}/validate_account{/}",
        // Route 1742
        "/{*namespace_id}/{project_id}/-/pipelines/settings{/}",
        // Route 1743
        // "/{*namespace_id}/{project_id}/-/pipelines/settings{/}",
        // Route 1744
        // "/{*namespace_id}/{project_id}/-/pipelines/settings{/}",
        // Route 1745
        "/{*namespace_id}/{project_id}/-/pipelines/charts{/}",
        // Route 1746
        "/{*namespace_id}/{project_id}/-/pipelines/{*ref?}/latest{/}",
        // Route 1747
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/stage{/}",
        // Route 1748
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/cancel{/}",
        // Route 1749
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/retry{/}",
        // Route 1750
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/builds{/}",
        // Route 1751
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/dag{/}",
        // Route 1752
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/failures{/}",
        // Route 1753
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/status{/}",
        // Route 1754
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/test_report{/}",
        // Route 1755
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/manual_variables{/}",
        // Route 1756
        "/{*namespace_id}/{project_id}/-/pipelines/{id}/downloadable_artifacts{/}",
        // Route 1757
        "/{*namespace_id}/{project_id}/-/pipelines/{pipeline_id}/stages/{stage_name}/play_manual{/}",
        // Route 1758
        "/{*namespace_id}/{project_id}/-/pipelines/{pipeline_id}/tests/summary{/}",
        // Route 1759
        "/{*namespace_id}/{project_id}/-/pipelines/{pipeline_id}/tests/{suite_name}{/}",
        // Route 1760
        "/{*namespace_id}/{project_id}/-/pipelines{/}",
        // Route 1761
        // "/{*namespace_id}/{project_id}/-/pipelines{/}",
        // Route 1762
        "/{*namespace_id}/{project_id}/-/pipelines/new{/}",
        // Route 1763
        "/{*namespace_id}/{project_id}/-/pipelines/{id}{/}",
        // Route 1764
        // "/{*namespace_id}/{project_id}/-/pipelines/{id}{/}",
        // Route 1765
        "/{*namespace_id}/{project_id}/-/pipeline_schedules/{id}/play{/}",
        // Route 1766
        "/{*namespace_id}/{project_id}/-/pipeline_schedules/{id}/take_ownership{/}",
        // Route 1767
        "/{*namespace_id}/{project_id}/-/pipeline_schedules{/}",
        // Route 1768
        // "/{*namespace_id}/{project_id}/-/pipeline_schedules{/}",
        // Route 1769
        "/{*namespace_id}/{project_id}/-/pipeline_schedules/new{/}",
        // Route 1770
        "/{*namespace_id}/{project_id}/-/pipeline_schedules/{id}/edit{/}",
        // Route 1771
        "/{*namespace_id}/{project_id}/-/pipeline_schedules/{id}{/}",
        // Route 1772
        // "/{*namespace_id}/{project_id}/-/pipeline_schedules/{id}{/}",
        // Route 1773
        // "/{*namespace_id}/{project_id}/-/pipeline_schedules/{id}{/}",
        // Route 1774
        "/{*namespace_id}/{project_id}/-/compare/{from}...{to}{/}",
        // Route 1775
        "/{*namespace_id}/{project_id}/-/compare/diff_for_path{/}",
        // Route 1776
        "/{*namespace_id}/{project_id}/-/compare/signatures{/}",
        // Route 1777
        "/{*namespace_id}/{project_id}/-/compare{/}",
        // Route 1778
        // "/{*namespace_id}/{project_id}/-/compare{/}",
        // Route 1779
        "/{*namespace_id}/{project_id}/-/refs/switch{/}",
        // Route 1780
        "/{*namespace_id}/{project_id}/-/refs/{id}/logs_tree{/}",
        // Route 1781
        "/{*namespace_id}/{project_id}/-/refs/{id}/logs_tree/{*path}{/}",
        // Route 1782
        "/{*namespace_id}/{project_id}/-/network/{id}{/}",
        // Route 1783
        "/{*namespace_id}/{project_id}/-/graphs/{id}/charts{/}",
        // Route 1784
        "/{*namespace_id}/{project_id}/-/graphs/{id}/commits{/}",
        // Route 1785
        "/{*namespace_id}/{project_id}/-/graphs/{id}/ci{/}",
        // Route 1786
        "/{*namespace_id}/{project_id}/-/graphs/{id}/languages{/}",
        // Route 1787
        "/{*namespace_id}/{project_id}/-/graphs/{id}{/}",
        // Route 1788
        "/{*namespace_id}/{project_id}/-/branches/{state}{/}",
        // Route 1789
        "/{*namespace_id}/{project_id}/-/branches/diverging_commit_counts{/}",
        // Route 1790
        "/{*namespace_id}/{project_id}/-/branches{/}",
        // Route 1791
        // "/{*namespace_id}/{project_id}/-/branches{/}",
        // Route 1792
        "/{*namespace_id}/{project_id}/-/branches/new{/}",
        // Route 1793
        "/{*namespace_id}/{project_id}/-/branches/{id}{/}",
        // Route 1794
        "/{*namespace_id}/{project_id}/-/merged_branches{/}",
        // Route 1795
        "/{*namespace_id}/{project_id}/-/tags{/}",
        // Route 1796
        // "/{*namespace_id}/{project_id}/-/tags{/}",
        // Route 1797
        "/{*namespace_id}/{project_id}/-/tags/new{/}",
        // Route 1798
        "/{*namespace_id}/{project_id}/-/tags/{id}{/}",
        // Route 1799
        // "/{*namespace_id}/{project_id}/-/tags/{id}{/}",
        // Route 1800
        "/{*namespace_id}/{project_id}/-/protected_branches{/}",
        // Route 1801
        // "/{*namespace_id}/{project_id}/-/protected_branches{/}",
        // Route 1802
        "/{*namespace_id}/{project_id}/-/protected_branches/{id}{/}",
        // Route 1803
        // "/{*namespace_id}/{project_id}/-/protected_branches/{id}{/}",
        // Route 1804
        // "/{*namespace_id}/{project_id}/-/protected_branches/{id}{/}",
        // Route 1805
        // "/{*namespace_id}/{project_id}/-/protected_branches/{id}{/}",
        // Route 1806
        "/{*namespace_id}/{project_id}/-/protected_tags{/}",
        // Route 1807
        // "/{*namespace_id}/{project_id}/-/protected_tags{/}",
        // Route 1808
        "/{*namespace_id}/{project_id}/-/protected_tags/{id}{/}",
        // Route 1809
        // "/{*namespace_id}/{project_id}/-/protected_tags/{id}{/}",
        // Route 1810
        // "/{*namespace_id}/{project_id}/-/protected_tags/{id}{/}",
        // Route 1811
        // "/{*namespace_id}/{project_id}/-/protected_tags/{id}{/}",
        // Route 1812
        "/{*namespace_id}/{project_id}/-/new/{*id}{/}",
        // Route 1813
        "/{*namespace_id}/{project_id}/-/create/{*id}{/}",
        // Route 1814
        "/{*namespace_id}/{project_id}/-/edit/{*id}{/}",
        // Route 1815
        "/{*namespace_id}/{project_id}/-/update/{*id}{/}",
        // Route 1816
        "/{*namespace_id}/{project_id}/-/preview/{*id}{/}",
        // Route 1817
        "/{*namespace_id}/{project_id}/-/blob/{*id}/diff{/}",
        // Route 1818
        "/{*namespace_id}/{project_id}/-/blob/{*id}{/}",
        // Route 1819
        // "/{*namespace_id}/{project_id}/-/blob/{*id}{/}",
        // Route 1820
        // "/{*namespace_id}/{project_id}/-/blob/{*id}{/}",
        // Route 1821
        // "/{*namespace_id}/{project_id}/-/blob/{*id}{/}",
        // Route 1822
        "/{*namespace_id}/{project_id}/-/tree/{*id}{/}",
        // Route 1823
        "/{*namespace_id}/{project_id}/-/raw/{*id}{/}",
        // Route 1824
        "/{*namespace_id}/{project_id}/-/blame_page/{*id}{/}",
        // Route 1825
        "/{*namespace_id}/{project_id}/-/blame/{*id}/streaming{/}",
        // Route 1826
        "/{*namespace_id}/{project_id}/-/blame/{*id}{/}",
        // Route 1827
        "/{*namespace_id}/{project_id}/-/commits{/}",
        // Route 1828
        "/{*namespace_id}/{project_id}/-/commits/{*id}/signatures{/}",
        // Route 1829
        "/{*namespace_id}/{project_id}/-/commits/{*id}{/}",
        // Route 1830
        "/{*namespace_id}/{project_id}/-/create_dir/{*id}{/}",
        // Route 1831
        "/{*namespace_id}/{project_id}/-/find_file/{*id}{/}",
        // Route 1832
        "/{*namespace_id}/{project_id}/-/files/{*id}{/}",
        // Route 1833
        "/{*namespace_id}/{project_id}/-/commit/{id}{/}",
        // Route 1834
        "/{*namespace_id}/{project_id}/-/commit/{id}/branches{/}",
        // Route 1835
        "/{*namespace_id}/{project_id}/-/commit/{id}/pipelines{/}",
        // Route 1836
        "/{*namespace_id}/{project_id}/-/commit/{id}/revert{/}",
        // Route 1837
        "/{*namespace_id}/{project_id}/-/commit/{id}/cherry_pick{/}",
        // Route 1838
        "/{*namespace_id}/{project_id}/-/commit/{id}/diff_for_path{/}",
        // Route 1839
        "/{*namespace_id}/{project_id}/-/commit/{id}/diff_files{/}",
        // Route 1840
        "/{*namespace_id}/{project_id}/-/commit/{id}/merge_requests{/}",
        // Route 1841
        // "/{*namespace_id}/{project_id}/-/commit/{id}{/}",
        // Route 1842
        "/{*namespace_id}/{project_id}/-/repository{/}",
        // Route 1843
        "/{*namespace_id}/{project_id}/-/wikis/git_access{/}",
        // Route 1844
        "/{*namespace_id}/{project_id}/-/wikis/pages{/}",
        // Route 1845
        "/{*namespace_id}/{project_id}/-/wikis/templates{/}",
        // Route 1846
        "/{*namespace_id}/{project_id}/-/wikis/new{/}",
        // Route 1847
        "/{*namespace_id}/{project_id}/-/wikis{/}",
        // Route 1848
        // "/{*namespace_id}/{project_id}/-/wikis{/}",
        // Route 1849
        "/{*namespace_id}/{project_id}/-/wikis/-/confluence{/}",
        // Route 1850
        "/{*namespace_id}/{project_id}/-/wikis/{*id}/edit{/}",
        // Route 1851
        "/{*namespace_id}/{project_id}/-/wikis/{*id}/history{/}",
        // Route 1852
        "/{*namespace_id}/{project_id}/-/wikis/{*id}/diff{/}",
        // Route 1853
        "/{*namespace_id}/{project_id}/-/wikis/{*id}/raw{/}",
        // Route 1854
        "/{*namespace_id}/{project_id}/-/wikis/{*id}/preview_markdown{/}",
        // Route 1855
        "/{*namespace_id}/{project_id}/-/wikis/{*id}{/}",
        // Route 1856
        // "/{*namespace_id}/{project_id}/-/wikis/{*id}{/}",
        // Route 1857
        // "/{*namespace_id}/{project_id}/-/wikis/{*id}{/}",
        // Route 1858
        "/{*namespace_id}/{project_id}/-/import/jira{/}",
        // Route 1859
        "/{*namespace_id}/{project_id}/-/snippets/{id}/raw{/}",
        // Route 1860
        "/{*namespace_id}/{project_id}/-/snippets/{id}/mark_as_spam{/}",
        // Route 1861
        "/{*namespace_id}/{project_id}/-/snippets/{id}/toggle_award_emoji{/}",
        // Route 1862
        "/{*namespace_id}/{project_id}/-/snippets{/}",
        // Route 1863
        "/{*namespace_id}/{project_id}/-/snippets/new{/}",
        // Route 1864
        "/{*namespace_id}/{project_id}/-/snippets/{id}/edit{/}",
        // Route 1865
        "/{*namespace_id}/{project_id}/-/snippets/{id}{/}",
        // Route 1866
        // "/{*namespace_id}/{project_id}/-/feature_flags{/}",
        // Route 1867
        // "/{*namespace_id}/{project_id}/-/feature_flags{/}",
        // Route 1868
        // "/{*namespace_id}/{project_id}/-/feature_flags/new{/}",
        // Route 1869
        // "/{*namespace_id}/{project_id}/-/feature_flags/{iid}/edit{/}",
        // Route 1870
        // "/{*namespace_id}/{project_id}/-/feature_flags/{iid}{/}",
        // Route 1871
        // "/{*namespace_id}/{project_id}/-/feature_flags/{iid}{/}",
        // Route 1872
        // "/{*namespace_id}/{project_id}/-/feature_flags/{iid}{/}",
        // Route 1873
        // "/{*namespace_id}/{project_id}/-/feature_flags/{iid}{/}",
        // Route 1874
        "/{*namespace_id}/{project_id}/-/feature_flags_client/reset_token{/}",
        // Route 1875
        "/{*namespace_id}/{project_id}/-/feature_flags_user_lists{/}",
        // Route 1876
        "/{*namespace_id}/{project_id}/-/feature_flags_user_lists/new{/}",
        // Route 1877
        "/{*namespace_id}/{project_id}/-/feature_flags_user_lists/{iid}/edit{/}",
        // Route 1878
        "/{*namespace_id}/{project_id}/-/feature_flags_user_lists/{iid}{/}",
        // Route 1879
        "/{*namespace_id}/{project_id}/-/schema/{branch}/{*filename}{/}",
        // Route 1880
        "/{*namespace_id}/{project_id}/-/hooks/{id}/test{/}",
        // Route 1881
        "/{*namespace_id}/{project_id}/-/hooks/{hook_id}/hook_logs/{id}/retry{/}",
        // Route 1882
        "/{*namespace_id}/{project_id}/-/hooks/{hook_id}/hook_logs/{id}{/}",
        // Route 1883
        "/{*namespace_id}/{project_id}/-/hooks{/}",
        // Route 1884
        // "/{*namespace_id}/{project_id}/-/hooks{/}",
        // Route 1885
        "/{*namespace_id}/{project_id}/-/hooks/{id}/edit{/}",
        // Route 1886
        "/{*namespace_id}/{project_id}/-/hooks/{id}{/}",
        // Route 1887
        // "/{*namespace_id}/{project_id}/-/hooks/{id}{/}",
        // Route 1888
        // "/{*namespace_id}/{project_id}/-/hooks/{id}{/}",
        // Route 1889
        "/{*namespace_id}/{project_id}/-/integrations/slash_commands/confirm{/}",
        // Route 1890
        "/{*namespace_id}/{project_id}/-/integrations/slash_commands{/}",
        // Route 1891
        "/{*namespace_id}/{project_id}/-/badges/release.{format?}{/}",
        // Route 1892
        "/{*namespace_id}/{project_id}/-/harbor/repositories/{repository_id}/artifacts/{artifact_id}/tags{/}",
        // Route 1893
        "/{*namespace_id}/{project_id}/-/harbor/repositories/{repository_id}/artifacts{/}",
        // Route 1894
        "/{*namespace_id}/{project_id}/-/harbor/repositories{/}",
        // Route 1895
        "/{*namespace_id}/{project_id}/-/harbor/repositories/{id}{/}",
        // Route 1896
        "/{*namespace_id}/{project_id}/-/ml/experiments{/}",
        // Route 1897
        "/{*namespace_id}/{project_id}/-/ml/experiments/{iid}{/}",
        // Route 1898
        // "/{*namespace_id}/{project_id}/-/ml/experiments/{iid}{/}",
        // Route 1899
        "/{*namespace_id}/{project_id}/-/ml/candidates/{iid}{/}",
        // Route 1900
        // "/{*namespace_id}/{project_id}/-/ml/candidates/{iid}{/}",
        // Route 1901
        "/{*namespace_id}/{project_id}/-/ml/models/{model_model_id}/versions/{model_version_id}{/}",
        // Route 1902
        "/{*namespace_id}/{project_id}/-/ml/models{/}",
        // Route 1903
        "/{*namespace_id}/{project_id}/-/ml/models/new{/}",
        // Route 1904
        "/{*namespace_id}/{project_id}/-/ml/models/{model_id}{/}",
        // Route 1905
        // "/{*namespace_id}/{project_id}/-/ml/models/{model_id}{/}",
        // Route 1906
        "/{*namespace_id}/{project_id}/-/ml/preview_markdown{/}",
        // Route 1907
        "/{*namespace_id}/{project_id}/-/service_desk/custom_email{/}",
        // Route 1908
        // "/{*namespace_id}/{project_id}/-/service_desk/custom_email{/}",
        // Route 1909
        // "/{*namespace_id}/{project_id}/-/service_desk/custom_email{/}",
        // Route 1910
        // "/{*namespace_id}/{project_id}/-/service_desk/custom_email{/}",
        // Route 1911
        // "/{*namespace_id}/{project_id}/-/service_desk/custom_email{/}",
        // Route 1912
        "/{*namespace_id}/{project_id}/-/{noteable_type}/{noteable_id}/discussions/{id}/resolve{/}",
        // Route 1913
        // "/{*namespace_id}/{project_id}/-/{noteable_type}/{noteable_id}/discussions/{id}/resolve{/}",
        // Route 1914
        "/{*namespace_id}/{project_id}/-/{noteable_type}/{noteable_id}/discussions/{id}{/}",
        // Route 1915
        "/{*namespace_id}/{project_id}/service_desk{/}",
        // Route 1916
        // "/{*namespace_id}/{project_id}/service_desk{/}",
        // Route 1917
        "/{*namespace_id}/{project_id}/templates/{template_type}.{format?}{/}",
        // Route 1918
        "/{*namespace_id}/{project_id}/templates/{template_type}/{key}.{format?}{/}",
        // Route 1919
        "/{*namespace_id}/{project_id}/description_templates/names/{template_type}.{format?}{/}",
        // Route 1920
        "/{*namespace_id}/{project_id}/pages/domains/{id}/verify{/}",
        // Route 1921
        "/{*namespace_id}/{project_id}/pages/domains/{id}/retry_auto_ssl{/}",
        // Route 1922
        "/{*namespace_id}/{project_id}/pages/domains/{id}/clean_certificate{/}",
        // Route 1923
        "/{*namespace_id}/{project_id}/pages/domains{/}",
        // Route 1924
        "/{*namespace_id}/{project_id}/pages/domains/new{/}",
        // Route 1925
        "/{*namespace_id}/{project_id}/pages/domains/{id}/edit{/}",
        // Route 1926
        "/{*namespace_id}/{project_id}/pages/domains/{id}{/}",
        // Route 1927
        // "/{*namespace_id}/{project_id}/pages/domains/{id}{/}",
        // Route 1928
        // "/{*namespace_id}/{project_id}/pages/domains/{id}{/}",
        // Route 1929
        // "/{*namespace_id}/{project_id}/pages/domains/{id}{/}",
        // Route 1930
        "/{*namespace_id}/{project_id}/pages/new{/}",
        // Route 1931
        "/{*namespace_id}/{project_id}/pages{/}",
        // Route 1932
        // "/{*namespace_id}/{project_id}/pages{/}",
        // Route 1933
        // "/{*namespace_id}/{project_id}/pages{/}",
        // Route 1934
        // "/{*namespace_id}/{project_id}/pages{/}",
        // Route 1935
        "/{*namespace_id}/{project_id}/prometheus/metrics/active_common{/}",
        // Route 1936
        "/{*namespace_id}/{project_id}/prometheus/metrics/validate_query{/}",
        // Route 1937
        "/{*namespace_id}/{project_id}/prometheus/metrics{/}",
        // Route 1938
        // "/{*namespace_id}/{project_id}/prometheus/metrics{/}",
        // Route 1939
        "/{*namespace_id}/{project_id}/prometheus/metrics/new{/}",
        // Route 1940
        "/{*namespace_id}/{project_id}/prometheus/metrics/{id}/edit{/}",
        // Route 1941
        "/{*namespace_id}/{project_id}/prometheus/metrics/{id}{/}",
        // Route 1942
        // "/{*namespace_id}/{project_id}/prometheus/metrics/{id}{/}",
        // Route 1943
        // "/{*namespace_id}/{project_id}/prometheus/metrics/{id}{/}",
        // Route 1944
        "/{*namespace_id}/{project_id}/prometheus/alerts/notify{/}",
        // Route 1945
        "/{*namespace_id}/{project_id}/prometheus/alerts/{id}/metrics_dashboard{/}",
        // Route 1946
        "/{*namespace_id}/{project_id}/alerts/notify{/}",
        // Route 1947
        "/{*namespace_id}/{project_id}/alerts/notify/{name}/{endpoint_identifier}{/}",
        // Route 1948
        "/{*namespace_id}/{project_id}/builds/artifacts/{*ref_name_and_path}{/}",
        // Route 1949
        "/{*namespace_id}/{project_id}/builds/{id}/raw{/}",
        // Route 1950
        "/{*namespace_id}/{project_id}/builds/{build_id}/artifacts/download{/}",
        // Route 1951
        "/{*namespace_id}/{project_id}/builds/{build_id}/artifacts/browse/{*path?}{/}",
        // Route 1952
        "/{*namespace_id}/{project_id}/builds/{build_id}/artifacts/file/{*path}{/}",
        // Route 1953
        "/{*namespace_id}/{project_id}/builds/{build_id}/artifacts/raw/{*path}{/}",
        // Route 1954
        "/{*namespace_id}/{project_id}/builds{/}",
        // Route 1955
        "/{*namespace_id}/{project_id}/builds/{id}{/}",
        // Route 1956
        "/{*namespace_id}/{project_id}/container_registry{/}",
        // Route 1957
        "/{*namespace_id}/{project_id}/container_registry/{id}{/}",
        // Route 1958
        // "/{*namespace_id}/{project_id}/container_registry/{id}{/}",
        // Route 1959
        "/{*namespace_id}/{project_id}/registry/repository/{repository_id}/tags/bulk_destroy{/}",
        // Route 1960
        "/{*namespace_id}/{project_id}/registry/repository/{repository_id}/tags{/}",
        // Route 1961
        "/{*namespace_id}/{project_id}/registry/repository/{repository_id}/tags/{id}{/}",
        // Route 1962
        "/{*namespace_id}/{project_id}/notes/{id}/delete_attachment{/}",
        // Route 1963
        "/{*namespace_id}/{project_id}/notes/{id}/resolve{/}",
        // Route 1964
        // "/{*namespace_id}/{project_id}/notes/{id}/resolve{/}",
        // Route 1965
        "/{*namespace_id}/{project_id}/notes/{id}/outdated_line_change{/}",
        // Route 1966
        "/{*namespace_id}/{project_id}/notes/{id}/toggle_award_emoji{/}",
        // Route 1967
        "/{*namespace_id}/{project_id}/notes{/}",
        // Route 1968
        "/{*namespace_id}/{project_id}/notes/{id}{/}",
        // Route 1969
        // "/{*namespace_id}/{project_id}/notes/{id}{/}",
        // Route 1970
        // "/{*namespace_id}/{project_id}/notes/{id}{/}",
        // Route 1971
        "/{*namespace_id}/{project_id}/noteable/{target_type}/{target_id}/notes{/}",
        // Route 1972
        "/{*namespace_id}/{project_id}/todos{/}",
        // Route 1973
        "/{*namespace_id}/{project_id}/uploads/{secret}/{filename}{/}",
        // Route 1974
        "/{*namespace_id}/{project_id}/uploads/authorize{/}",
        // Route 1975
        "/{*namespace_id}/{project_id}/uploads{/}",
        // Route 1976
        "/{*namespace_id}/{project_id}/runner_projects{/}",
        // Route 1977
        "/{*namespace_id}/{project_id}/runner_projects/{id}{/}",
        // Route 1978
        "/{*namespace_id}/{project_id}/badges/{*ref}/pipeline.{format?}{/}",
        // Route 1979
        "/{*namespace_id}/{project_id}/badges/{*ref}/coverage.{format?}{/}",
        // Route 1980
        "/{*namespace_id}/{project_id}/badges{/}",
        // Route 1981
        "/{*namespace_id}/{project_id}/service_ping/web_ide_pipelines_count{/}",
        // Route 1982
        "/{*namespace_id}/{project_id}/ide_terminals/{id}/cancel.{format?}{/}",
        // Route 1983
        "/{*namespace_id}/{project_id}/ide_terminals/{id}/retry.{format?}{/}",
        // Route 1984
        "/{*namespace_id}/{project_id}/ide_terminals/check_config.{format?}{/}",
        // Route 1985
        "/{*namespace_id}/{project_id}/ide_terminals.{format?}{/}",
        // Route 1986
        "/{*namespace_id}/{project_id}/ide_terminals/{id}.{format?}{/}",
        // Route 1987
        "/{*namespace_id}/{project_id}/repository{/}",
        // Route 1988
        "/{*namespace_id}/{project_id}/refs/switch{/}",
        // Route 1989
        "/{*namespace_id}/{project_id}/refs/{id}/logs_tree{/}",
        // Route 1990
        "/{*namespace_id}/{project_id}/refs/{id}/logs_tree/{*path}{/}",
        // Route 1991
        "/{*namespace_id}/{project_id}/tree/{*id}{/}",
        // Route 1992
        "/{*namespace_id}/{project_id}/blob/{*id}{/}",
        // Route 1993
        "/{*namespace_id}/{project_id}/raw/{*id}{/}",
        // Route 1994
        "/{*namespace_id}/{project_id}/blame/{*id}{/}",
        // Route 1995
        "/{*namespace_id}/{project_id}/new/{*id}{/}",
        // Route 1996
        "/{*namespace_id}/{project_id}/edit/{*id}{/}",
        // Route 1997
        "/{*namespace_id}/{project_id}/snippets/{id}/raw{/}",
        // Route 1998
        "/{*namespace_id}/{project_id}/mirror/{*rest?}{/}",
        // Route 1999
        "/{*namespace_id}/{project_id}/tags/{*rest?}{/}",
        // Route 2000
        "/{*namespace_id}/{project_id}/hooks/{*rest?}{/}",
        // Route 2001
        "/{*namespace_id}/{project_id}/commits/{*rest?}{/}",
        // Route 2002
        "/{*namespace_id}/{project_id}/commit/{*rest?}{/}",
        // Route 2003
        "/{*namespace_id}/{project_id}/find_file/{*rest?}{/}",
        // Route 2004
        "/{*namespace_id}/{project_id}/files/{*rest?}{/}",
        // Route 2005
        "/{*namespace_id}/{project_id}/compare/{*rest?}{/}",
        // Route 2006
        "/{*namespace_id}/{project_id}/cycle_analytics/{*rest?}{/}",
        // Route 2007
        "/{*namespace_id}/{project_id}/mattermost/{*rest?}{/}",
        // Route 2008
        "/{*namespace_id}/{project_id}/variables/{*rest?}{/}",
        // Route 2009
        "/{*namespace_id}/{project_id}/triggers/{*rest?}{/}",
        // Route 2010
        "/{*namespace_id}/{project_id}/environments/{*rest?}{/}",
        // Route 2011
        "/{*namespace_id}/{project_id}/protected_environments/{*rest?}{/}",
        // Route 2012
        "/{*namespace_id}/{project_id}/error_tracking/{*rest?}{/}",
        // Route 2013
        "/{*namespace_id}/{project_id}/alert_management/{*rest?}{/}",
        // Route 2014
        "/{*namespace_id}/{project_id}/serverless/{*rest?}{/}",
        // Route 2015
        "/{*namespace_id}/{project_id}/clusters/{*rest?}{/}",
        // Route 2016
        "/{*namespace_id}/{project_id}/audit_events/{*rest?}{/}",
        // Route 2017
        "/{*namespace_id}/{project_id}/wikis/{*rest?}{/}",
        // Route 2018
        "/{*namespace_id}/{project_id}/merge_requests/{*rest?}{/}",
        // Route 2019
        "/{*namespace_id}/{project_id}/vulnerability_feedback/{*rest?}{/}",
        // Route 2020
        "/{*namespace_id}/{project_id}/security/{*rest?}{/}",
        // Route 2021
        "/{*namespace_id}/{project_id}/dependencies/{*rest?}{/}",
        // Route 2022
        "/{*namespace_id}/{project_id}/issues/{*rest?}{/}",
        // Route 2023
        "/{*namespace_id}/{project_id}/pipelines/{*rest?}{/}",
        // Route 2024
        "/{*namespace_id}/{project_id}/pipeline_schedules/{*rest?}{/}",
        // Route 2025
        "/{*namespace_id}/{project_id}/runners/{*rest?}{/}",
        // Route 2026
        "/{*namespace_id}/{project_id}/snippets/{*rest?}{/}",
        // Route 2027
        "/{*namespace_id}/{id}/transfer{/}",
        // Route 2028
        "/{*namespace_id}/{id}/remove_fork{/}",
        // Route 2029
        "/{*namespace_id}/{id}/archive{/}",
        // Route 2030
        "/{*namespace_id}/{id}/unarchive{/}",
        // Route 2031
        "/{*namespace_id}/{id}/housekeeping{/}",
        // Route 2032
        "/{*namespace_id}/{id}/toggle_star{/}",
        // Route 2033
        "/{*namespace_id}/{id}/export{/}",
        // Route 2034
        "/{*namespace_id}/{id}/remove_export{/}",
        // Route 2035
        "/{*namespace_id}/{id}/generate_new_export{/}",
        // Route 2036
        "/{*namespace_id}/{id}/download_export{/}",
        // Route 2037
        "/{*namespace_id}/{id}/activity{/}",
        // Route 2038
        "/{*namespace_id}/{id}/refs{/}",
        // Route 2039
        "/{*namespace_id}/{id}/new_issuable_address{/}",
        // Route 2040
        "/{*namespace_id}/{id}/unfoldered_environment_names{/}",
        // Route 2041
        "/{*namespace_id}/{id}/edit{/}",
        // Route 2042
        "/{*namespace_id}/{id}{/}",
        // Route 2043
        // "/{*namespace_id}/{id}{/}",
        // Route 2044
        // "/{*namespace_id}/{id}{/}",
        // Route 2045
        // "/{*namespace_id}/{id}{/}",
        // Route 2046
        // NOTE: We don't support optionals spanning across sections
        "/-/jira/{*namespace_id}/{project_id}{/}",
        // "/{*namespace_id}/{project_id}{/}",
        // Route 2047
        // NOTE: We don't support optionals spanning across sections
        "/-/jira/{*namespace_id}/{project_id}/commit/{id}{/}",
        "/{*namespace_id}/{project_id}/commit/{id}{/}",
        // Route 2048
        // NOTE: We don't support optionals spanning across sections
        "/-/jira/{*namespace_id}/{project_id}/tree/{*id}{/}",
        // "/{*namespace_id}/{project_id}/tree/{*id}{/}",
        // Route 2049
        "/{*namespace_id}/{project_id}/{*all}{/}",
        // Route 2050
        // "/{*namespace_id}/{project_id}/{*all}{/}",
        // Route 2051
        // "/{*namespace_id}/{project_id}/{*all}{/}",
        // Route 2052
        // "/{*namespace_id}/{project_id}/{*all}{/}",
        // Route 2053
        "/{*namespace_id}/{project_id}{/}",
        // Route 2054
        // "/{*namespace_id}/{project_id}{/}",
        // Route 2055
        // "/{*namespace_id}/{project_id}{/}",
        // Route 2056
        // "/{*namespace_id}/{project_id}{/}",
        // Route 2057
        "/.well-known/change-password{/}",
        // Route 2058
        "/.well-known/security.txt{/}",
        // Route 2059
        "/snippets/{id}/raw{/}",
        // Route 2060
        "/snippets/{*rest?}{/}",
        // Route 2061
        "/sitemap{/}",
        // Route 2062
        // "/",
        // Route 2063
        "/{*unmatched_route}{/}",
        // Route 2064
        "/health_check/{checks?}/{check}.{format?}{/}",
        // Route 2065
        // NOTE: See Gitlab::Experiment::Engine
        // "/-/experiment{/}",

        // Routes for LetterOpenerWeb::Engine

        // Route 1
        "/rails/letter_opener{/}",
        // Route 2
        "/rails/letter_opener/clear{/}",
        // Route 3
        "/rails/letter_opener/{id}/{style?}{/}",
        // Route 4
        "/rails/letter_opener/{id}/delete{/}",
        // Route 5
        "/rails/letter_opener/{id}/attachments/{file}{/}",

        // Routes for Lookbook::Engine

        // Route 1
        "/rails/lookbook/cable{/}",
        // Route 2
        "/rails/lookbook",
        // Route 3
        "/rails/lookbook/pages{/}",
        // Route 4
        "/rails/lookbook/pages/{*path}{/}",
        // Route 5
        "/rails/lookbook/previews{/}",
        // Route 6
        "/rails/lookbook/preview/{*path}{/}",
        // Route 7
        "/rails/lookbook/inspect/{*path}{/}",
        // Route 8
        "/rails/lookbook/embed{/}",
        // Route 9
        "/rails/lookbook/embed/{*path}{/}",
        // Route 10
        "/rails/lookbook/{*path}{/}",

        // Routes for Toogle::Engine

        // Route 1
        "/rails/features/definitions{/}",
        // Route 2
        "/rails/features",
        // Route 3
        "/rails/features/{id}{/}",
        // Route 4
        // "/rails/features/{id}{/}",
        // Route 5
        // "/rails/features/{id}{/}",
        // Route 6
        // "/rails/features/{id}{/}",

        // Routes for Peek::Railtie

        // Route 1
        "/-/peek/results{/}",

        // Routes for GraphiQL::Rails::Engine

        // Route 1
        "/-/graphql-explorer{/}",

        // Routes for Gitlab::Experiment::Engine

        // Route 1
        "/-/experiment/{id}{/}",
    ]
}

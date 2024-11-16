# `rails`

A tool to convert Rails routes to Wayfind routes.

```sh
cargo run --release -- input/routes.txt | tee output/routes.json
```

## routes.txt

This was extracted via running `rails routes --expanded` from within the [GitLab Gitpod Env](https://docs.gitlab.com/ee/development/contributing/first_contribution/configure-dev-env-gitpod.html).

Some manual changes are needed, since our router is stricter than Rails.

### Calendar

The following routes are only meant to handle `ics`.

- Route 558
- Route 1121
- Route 1635

### JSON

The following routes are only meant to handle `json`.

- Route 752
- Route 1701
- Route 1702
- Route 1704
- Route 1713
- Route 1738
- Route 1739

### Feature Flags

These routes are alternative implementations that are only usable via a feature flag.
Delete them.

- Route 1711
- Route 1844

### Route 2079

This route is provided by the `health_check` gem.
It conflicts with GitLab's own internal health check endpoint (Route 66).

Rails allows conflicting routes, we don't.
Delete this route.

```
--[ Route 2079 ]----------------------------------------------------------------
Prefix            |
Verb              | GET|POST
URI               | /health_check(/:checks)(.:format)
Controller#Action | health_check/health_check#index {:format=>"txt"}
```

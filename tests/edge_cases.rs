use std::error::Error;
use wayfind::Router;

#[path = "./utils.rs"]
mod utils;

#[test]
fn test_specific_matching_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{file}", 1)?;
    router.insert("/{file}.{extension}", 1)?;

    insta::assert_snapshot!(router, @r#"
    /
    ╰─ {file} [*]
       ╰─ .
          ╰─ {extension} [*]
    "#);

    assert_router_matches!(router, {
        "/readme" => {
            route: "/{file}",
            data: 1,
            params: {
                "file" => "readme"
            }
        }
        "/report.pdf" => {
            route: "/{file}.{extension}",
            data: 1,
            params: {
                "file" => "report",
                "extension" => "pdf"
            }
        }
    });

    Ok(())
}

#[test]
fn test_specific_matching_complex() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{year}", 1)?;
    router.insert("/{year}-{month}", 1)?;
    router.insert("/{year}-{month}-{day}", 1)?;

    insta::assert_snapshot!(router, @r#"
    /
    ╰─ {year} [*]
       ╰─ -
          ╰─ {month} [*]
             ╰─ -
                ╰─ {day} [*]
    "#);

    assert_router_matches!(router, {
        "/2000" => {
            route: "/{year}",
            data: 1,
            params: {
                "year" => "2000"
            }
        }
        "/2000-01" => {
            route: "/{year}-{month}",
            data: 1,
            params: {
                "year" => "2000",
                "month" => "01"
            }
        }
        "/2000-01-01" => {
            route: "/{year}-{month}-{day}",
            data: 1,
            params: {
                "year" => "2000",
                "month" => "01",
                "day" => "01"
            }
        }
        "/2000-01-01-01" => {
            route: "/{year}-{month}-{day}",
            data: 1,
            params: {
                "year" => "2000-01",
                "month" => "01",
                "day" => "01"
            }
        }
    });

    Ok(())
}

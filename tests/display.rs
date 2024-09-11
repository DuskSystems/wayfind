use std::error::Error;
use wayfind::Router;

#[test]
fn test_display_multibyte() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert("/👨‍👩‍👧", 1)?; // Family: Man, Woman, Girl
    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /👨‍👩‍👧 ○
    "#);

    router.insert("/👨‍👩‍👦", 1)?; // Family: Man, Woman, Boy
    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /👨‍👩‍�
                        ├─ � ○
                        ╰─ � ○
    "#);

    router.insert("/👩‍👩‍👧", 1)?; // Family: Woman, Woman, Girl
    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /�
          ├─ �‍👩‍�
          │                 ├─ � ○
          │                 ╰─ � ○
          ╰─ �‍👩‍👧 ○
    "#);

    router.insert("/👩‍👩‍👦", 1)?; // Family: Woman, Woman, Boy
    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /�
          ├─ �‍👩‍�
          │                 ├─ � ○
          │                 ╰─ � ○
          ╰─ �‍👩‍�
                            ├─ � ○
                            ╰─ � ○
    "#);

    router.insert("/👨‍👨‍👧", 1)?; // Family: Man, Man, Girl
    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /�
          ├─ �‍�
          │          ├─ �‍👧 ○
          │          ╰─ �‍�
          │                     ├─ � ○
          │                     ╰─ � ○
          ╰─ �‍👩‍�
                            ├─ � ○
                            ╰─ � ○
    "#);

    router.insert("/👨‍👨‍👦", 1)?; // Family: Man, Man, Boy
    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /�
          ├─ �‍�
          │          ├─ �‍�
          │          │          ├─ � ○
          │          │          ╰─ � ○
          │          ╰─ �‍�
          │                     ├─ � ○
          │                     ╰─ � ○
          ╰─ �‍👩‍�
                            ├─ � ○
                            ╰─ � ○
    "#);

    Ok(())
}

use std::error::Error;
use wayfind::Router;

#[test]
#[allow(clippy::cognitive_complexity)]
fn test_example() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    // Static
    router.insert("/", 1)?;
    router.insert("/health", 2)?;

    {
        let search = router.search("/").unwrap();
        assert_eq!(search.data, &1);

        let search = router.search("/health").unwrap();
        assert_eq!(search.data, &2);

        let search = router.search("/heal");
        assert_eq!(search, None);
    }

    // Dynamic
    router.insert("/users/<id>", 3)?;
    router.insert("/users/<id>/message", 4)?;

    {
        let search = router.search("/users/123").unwrap();
        assert_eq!(search.data, &3);
        assert_eq!(search.parameters[0], ("id", "123"));

        let search = router.search("/users/123/message").unwrap();
        assert_eq!(search.data, &4);
        assert_eq!(search.parameters[0], ("id", "123"));

        let search = router.search("/users/");
        assert_eq!(search, None);
    }

    // Dynamic Inline
    router.insert("/images/<name>.png", 5)?;
    router.insert("/images/<name>.<ext>", 6)?;

    {
        let search = router.search("/images/avatar.final.png").unwrap();
        assert_eq!(search.data, &5);
        assert_eq!(search.parameters[0], ("name", "avatar.final"));

        let search = router.search("/images/photo.jpg").unwrap();
        assert_eq!(search.data, &6);
        assert_eq!(search.parameters[0], ("name", "photo"));
        assert_eq!(search.parameters[1], ("ext", "jpg"));

        let search = router.search("/images/.png");
        assert_eq!(search, None);
    }

    // Wildcard
    router.insert("/files/<*path>", 7)?;
    router.insert("/files/<*path>/delete", 8)?;

    {
        let search = router.search("/files/documents").unwrap();
        assert_eq!(search.data, &7);
        assert_eq!(search.parameters[0], ("path", "documents"));

        let search = router.search("/files/documents/my-project/delete").unwrap();
        assert_eq!(search.data, &8);
        assert_eq!(search.parameters[0], ("path", "documents/my-project"));

        let search = router.search("/files");
        assert_eq!(search, None);
    }

    // Wildcard Inline
    router.insert("/backups/<*path>.tar.gz", 9)?;
    router.insert("/backups/<*path>.<ext>", 10)?;

    {
        let search = router
            .search("/backups/production/database.tar.gz")
            .unwrap();
        assert_eq!(search.data, &9);
        assert_eq!(search.parameters[0], ("path", "production/database"));

        let search = router.search("/backups/dev/application.log.bak").unwrap();
        assert_eq!(search.data, &10);
        assert_eq!(search.parameters[0], ("path", "dev/application.log"));
        assert_eq!(search.parameters[1], ("ext", "bak"));

        let search = router.search("/backups/.bak");
        assert_eq!(search, None);
    }

    insta::assert_snapshot!(router, @r"
    /
    ├─ backups/
    │  ╰─ <*path>
    │     ╰─ .
    │        ├─ tar.gz
    │        ╰─ <ext>
    ├─ files/
    │  ├─ <*path>
    │  │  ╰─ /delete
    │  ╰─ <*path>
    ├─ health
    ├─ images/
    │  ╰─ <name>
    │     ╰─ .
    │        ├─ png
    │        ╰─ <ext>
    ╰─ users/
       ╰─ <id>
          ╰─ /message
    ");

    Ok(())
}

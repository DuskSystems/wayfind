use std::error::Error;
use wayfind::Router;

#[test]
fn test_readme() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/pet(/)", 1)?;
    router.insert("/pet/findByStatus(/)", 2)?;
    router.insert("/pet/findByTags(/)", 3)?;
    router.insert("/pet/{pet}(/)", 4)?;
    router.insert("/pet/{petId:u16}/uploadImage(/)", 5)?;
    router.insert("/store/inventory(/)", 6)?;
    router.insert("/store/order(/{orderId:u16})(/)", 7)?;
    router.insert("/user(/)", 8)?;
    router.insert("/user/createWithList(/)", 9)?;
    router.insert("/user/login(/)", 10)?;
    router.insert("/user/logout(/)", 11)?;
    router.insert("/user/{username}(/)", 12)?;
    router.insert("/{*catch_all}", 13)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ pet [*]
    │  ╰─ / [*]
    │     ├─ findBy
    │     │  ├─ Status [*]
    │     │  │  ╰─ / [*]
    │     │  ╰─ Tags [*]
    │     │     ╰─ / [*]
    │     ├─ {petId:u16}
    │     │  ╰─ /uploadImage [*]
    │     │     ╰─ / [*]
    │     ╰─ {pet} [*]
    │        ╰─ / [*]
    ├─ store/
    │  ├─ inventory [*]
    │  │  ╰─ / [*]
    │  ╰─ order [*]
    │     ╰─ / [*]
    │        ╰─ {orderId:u16} [*]
    │           ╰─ / [*]
    ├─ user [*]
    │  ╰─ / [*]
    │     ├─ createWithList [*]
    │     │  ╰─ / [*]
    │     ├─ log
    │     │  ├─ in [*]
    │     │  │  ╰─ / [*]
    │     │  ╰─ out [*]
    │     │     ╰─ / [*]
    │     ╰─ {username} [*]
    │        ╰─ / [*]
    ╰─ {*catch_all} [*]
    ");

    Ok(())
}

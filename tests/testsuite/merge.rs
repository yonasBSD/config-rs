use config::{Config, File, FileFormat, Map};

#[test]
#[cfg(feature = "json")]
fn test_merge() {
    let c = Config::builder()
        .add_source(File::from_str(
            r#"
{
  "debug": true,
  "production": false,
  "place": {
    "rating": 4.5,
    "creator": {
      "name": "John Smith",
      "username": "jsmith",
      "email": "jsmith@localhost"
    }
  }
}
"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"
{
  "debug": false,
  "production": true,
  "place": {
    "rating": 4.9,
    "creator": {
      "name": "Somebody New"
    }
  }
}
"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some(true));
    assert_eq!(c.get("place.rating").ok(), Some(4.9));

    if cfg!(feature = "preserve_order") {
        let m: Map<String, String> = c.get("place.creator").unwrap();
        assert_eq!(
            m.into_iter().collect::<Vec<(String, String)>>(),
            vec![
                ("name".to_owned(), "Somebody New".to_owned()),
                ("username".to_owned(), "jsmith".to_owned()),
                ("email".to_owned(), "jsmith@localhost".to_owned()),
            ]
        );
    } else {
        assert_eq!(
            c.get("place.creator.name").ok(),
            Some("Somebody New".to_owned())
        );
    }
}

#[test]
fn test_merge_whole_config() {
    let builder1 = Config::builder().set_override("x", 10).unwrap();
    let builder2 = Config::builder().set_override("y", 25).unwrap();

    let config1 = builder1.build_cloned().unwrap();
    let config2 = builder2.build_cloned().unwrap();

    assert_eq!(config1.get("x").ok(), Some(10));
    assert_eq!(config2.get::<()>("x").ok(), None);

    assert_eq!(config2.get("y").ok(), Some(25));
    assert_eq!(config1.get::<()>("y").ok(), None);

    let config3 = builder1.add_source(config2).build().unwrap();

    assert_eq!(config3.get("x").ok(), Some(10));
    assert_eq!(config3.get("y").ok(), Some(25));
}
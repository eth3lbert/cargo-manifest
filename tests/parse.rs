use cargo_manifest as lib;
use cargo_manifest::Manifest;
use std::fs::read;
use toml;

#[test]
fn own() {
    let m = Manifest::from_slice(&read("Cargo.toml").unwrap()).unwrap();
    let package = m.package.as_ref().unwrap();
    assert_eq!("cargo-manifest", package.name);
    let m = Manifest::<toml::Value>::from_slice_with_metadata(&read("Cargo.toml").unwrap()).unwrap();
    let package = m.package.as_ref().unwrap();
    assert_eq!("cargo-manifest", package.name);
    assert_eq!(lib::Edition::E2018, package.edition);
}

#[test]
fn opt_level() {
    let m = Manifest::from_slice(&read("tests/opt_level.toml").unwrap()).unwrap();
    let package = m.package.as_ref().unwrap();
    assert_eq!("byteorder", package.name);
    assert_eq!(3, m.profile.unwrap().bench.unwrap().opt_level.unwrap().as_integer().unwrap());
    assert_eq!(false, m.lib.unwrap().bench);
    assert_eq!(lib::Edition::E2015, package.edition);
    assert_eq!(1, m.patch.unwrap().len());
}

#[test]
fn autobin() {
    let m = Manifest::from_path("tests/autobin/Cargo.toml").expect("load autobin");
    let package = m.package.as_ref().unwrap();
    assert_eq!("auto-bin", package.name);
    assert_eq!(lib::Edition::E2018, package.edition);
    assert!(package.autobins);
    assert!(m.lib.is_none());
    assert_eq!(1, m.bin.as_ref().unwrap().len());
    assert_eq!(Some("auto-bin"), m.bin.unwrap()[0].name.as_ref().map(|s| s.as_str()));
}

#[test]
fn autolib() {
    let m = Manifest::from_path("tests/autolib/Cargo.toml").expect("load autolib");
    let package = m.package.as_ref().unwrap();
    assert_eq!("auto-lib", package.name);
    assert_eq!(false, package.publish);
    assert_eq!(lib::Edition::E2015, package.edition);
    assert!(package.autobins);
    assert!(!package.autoexamples);
    assert!(m.lib.is_some());
    assert_eq!("auto_lib", m.lib.unwrap().name.unwrap());
    assert_eq!(0, m.bin.unwrap().len());
}

#[test]
fn metadata() {
    let m = Manifest::from_path("tests/metadata/Cargo.toml").expect("load metadata");
    let package = m.package.as_ref().unwrap();
    assert_eq!("metadata", package.name);
}

#[test]
fn legacy() {
    let m = Manifest::from_slice(br#"[project]
                name = "foo"
                version = "1"
                "#).expect("parse old");
    let package = m.package.as_ref().unwrap();
    assert_eq!("foo", package.name);
    let m = Manifest::from_str("name = \"foo\"\nversion=\"1\"").expect("parse bare");
    let package = m.package.as_ref().unwrap();
    assert_eq!("foo", package.name);
}

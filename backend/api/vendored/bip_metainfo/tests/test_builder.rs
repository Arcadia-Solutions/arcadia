extern crate bip_metainfo;

use bip_metainfo::MetainfoBuilder;

const TRACKER: &'static str = "udp://foo.bar.baz:6969";
const DATE: i64 = 1517651523851;
const COMMENT: &'static str = "Foo bar baz";
const CREATED_BY: &'static str = "Fridge";

#[test]
fn positive_set_trackers() {
    let trackers = vec![
        vec![TRACKER.to_string()]
    ];

    let builder = MetainfoBuilder::new()
        .set_trackers(Some(&trackers));

    assert_eq!(builder.get_trackers(), Some(trackers.clone()));
}

#[test]
fn positive_set_main_tracker() {
    let builder = MetainfoBuilder::new()
        .set_main_tracker(Some(TRACKER));

    assert_eq!(builder.get_main_tracker(), Some(TRACKER.to_string()));
}

#[test]
fn positive_set_creation_date() {
    let builder = MetainfoBuilder::new()
        .set_creation_date(Some(DATE));

    assert_eq!(builder.get_creation_date(), Some(DATE));
}

#[test]
fn positive_set_comment() {
    let builder = MetainfoBuilder::new()
        .set_comment(Some(COMMENT));

    assert_eq!(builder.get_comment(), Some(COMMENT.to_string()));
}

#[test]
fn positive_set_created_by() {
    let builder = MetainfoBuilder::new()
        .set_created_by(Some(CREATED_BY));

    assert_eq!(builder.get_created_by(), Some(CREATED_BY.to_string()));
}

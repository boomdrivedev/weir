//! The people list is read whole and refused naming the field (119; C.9;
//! record 0011).

use weir_policy::{About, Org, People, Person};

const C9: &str = include_str!("c9.toml");

fn refused(settings: &str) -> String {
    match People::read(settings) {
        Ok(read) => panic!("{settings} was read: {read:?}"),
        Err(error) => error.field.expect("a field"),
    }
}

#[test]
fn a_people_list_read_from_the_settings() {
    let people = People::read(C9).expect("C.9's people list");
    assert_eq!(
        people.persons,
        [Person {
            name: "Dana Reyes".into(),
            addresses: vec!["dana@acme.com".into()],
            names: vec!["Dana".into(), "Dana Reyes".into()],
            role: Some("executive".into()),
            teams: vec!["web".into()],
        }]
    );
    assert_eq!(
        people.orgs,
        [Org {
            name: "Globex".into(),
            domains: vec!["globex.com".into()],
            client: true,
        }]
    );
    assert_eq!(
        people.about,
        About {
            teams: vec!["web".into(), "platform".into()],
            repositories: vec!["acme/web-app".into(), "acme/infra".into()],
        }
    );
}

#[test]
fn a_person_with_no_name() {
    assert_eq!(
        refused("[[person]]\naddresses = [\"dana@acme.com\"]\n"),
        "person[0].name"
    );
    assert_eq!(
        refused("[[person]]\nname = \"Dana\"\n\n[[org]]\nclient = true\n"),
        "org[0].name"
    );
}

#[test]
fn a_field_the_people_list_does_not_name() {
    assert_eq!(
        refused("[[person]]\nname = \"Dana Reyes\"\nadresses = [\"dana@acme.com\"]\n"),
        "person[0].adresses"
    );
    // A field of the wrong shape is refused naming it, and every other
    // field takes its default.
    assert_eq!(
        refused("[[person]]\nname = \"Dana\"\naddresses = \"dana@acme.com\"\n"),
        "person[0].addresses"
    );
    assert_eq!(
        refused("[[org]]\nname = \"Globex\"\nclient = \"yes\"\n"),
        "org[0].client"
    );
    let read = People::read("[[person]]\nname = \"Kim\"\n[[org]]\nname = \"Initech\"\n")
        .expect("a people list");
    assert_eq!(read.persons[0].addresses, Vec::<String>::new());
    assert_eq!(read.persons[0].role, None);
    assert!(!read.orgs[0].client);
    assert_eq!(read.about, About::default());
}

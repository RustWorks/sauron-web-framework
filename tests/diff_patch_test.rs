#![deny(warnings)]
use sauron::{html::{attributes::*,
                    events::*,
                    *},
             Node};
use sauron_vdom::{diff,
                  Patch,
                  Text,
                  Value};

use maplit::btreemap;

#[test]
fn truncate_children() {
    let old: Node<()> = div([],
                            [div([class("class1")], []),
                             div([class("class2")], []),
                             div([class("class3")], []),
                             div([class("class4")], []),
                             div([class("class5")], []),
                             div([class("class6")], []),
                             div([class("class7")], [])]);

    let new = div([],
                  [div([class("class1")], []),
                   div([class("class2")], []),
                   div([class("class3")], [])]);
    assert_eq!(diff(&old, &new),
               vec![Patch::TruncateChildren(0, 3)],
               "Should truncate children");
}

#[test]
fn truncate_children_different_attributes() {
    let old: Node<()> = div([],
                            [div([class("class1")], []),
                             div([class("class2")], []),
                             div([class("class3")], []),
                             div([class("class4")], []),
                             div([class("class5")], []),
                             div([class("class6")], []),
                             div([class("class7")], [])]);

    let new = div([],
                  [div([class("class5")], []),
                   div([class("class6")], []),
                   div([class("class7")], [])]);
    let class5 = "class5".into();
    let class6 = "class6".into();
    let class7 = "class7".into();
    assert_eq!(diff(&old, &new),
               vec![Patch::TruncateChildren(0, 3),
                    Patch::AddAttributes(1, btreemap! { "class" => &class5}),
                    Patch::AddAttributes(2, btreemap! { "class"=> &class6}),
                    Patch::AddAttributes(3, btreemap! { "class"=> &class7}),],
               "Should truncate children");
}

#[test]
fn replace_node() {
    let old: Node<()> = div([], []);
    let new = span([], []);
    assert_eq!(diff(&old, &new),
               vec![Patch::Replace(0, &span([], []))],
               "Replace the root if the tag changed");

    let old: Node<()> = div([], [b([], [])]);
    let new = div([], [strong([], [])]);
    assert_eq!(diff(&old, &new),
               vec![Patch::Replace(1, &strong([], []))],
               "Replace a child node");

    let old: Node<()> = div([], [b([], [text("1")]), b([], [])]);
    let new = div([], [i([], [text("1")]), i([], [])]);
    assert_eq!(diff(&old, &new),
               vec![Patch::Replace(1, &i([], [text("1")])),
                    Patch::Replace(3, &i([], [])),],
               "Replace node with a child",)
}

#[test]
fn add_children() {
    let old: Node<()> = div([], [b([], [])]); //{ <div> <b></b> </div> },
    let new = div([], [b([], []), html_element("new", [], [])]); //{ <div> <b></b> <new></new> </div> },
    assert_eq!(diff(&old, &new),
               vec![Patch::AppendChildren(0,
                                          vec![&html_element("new", [], [])])],
               "Added a new node to the root node",)
}

#[test]
fn remove_nodes() {
    let old: Node<()> = div([], [b([], []), span([], [])]); //{ <div> <b></b> <span></span> </div> },
    let new = div([], []); //{ <div> </div> },

    assert_eq!(diff(&old, &new),
               vec![Patch::TruncateChildren(0, 0)],
               "Remove all child nodes at and after child sibling index 1",);

    let old: Node<()> = div([],
                            [span([],
                                  [b([], []),
                                   // This `i` tag will get removed
                                   i([], [])]),
                             // This `strong` tag will get removed
                             strong([], [])]);

    let new: Node<()> = div([], [span([], [b([], [])])]);

    assert_eq!(diff(&old, &new),
               vec![Patch::TruncateChildren(0, 1),
                    Patch::TruncateChildren(1, 1)],
               "Remove a child and a grandchild node",);

    let old: Node<()> = div([], [b([], [i([], []), i([], [])]), b([], [])]); //{ <div> <b> <i></i> <i></i> </b> <b></b> </div> },
    let new = div([], [b([], [i([], [])]), i([], [])]); //{ <div> <b> <i></i> </b> <i></i> </div>},
    assert_eq!(diff(&old, &new),
               vec![Patch::TruncateChildren(1, 1),
                    Patch::Replace(4, &i([], [])),],
               "Removing child and change next node after parent",)
}

#[test]
fn add_attributes() {
    let hello: Value = "hello".into();
    let attributes = btreemap! {
    "id" => &hello,
    };

    let old: Node<()> = div([], []); //{ <div> </div> },
    let new = div([id("hello")], []); //{ <div id="hello"> </div> },
    assert_eq!(diff(&old, &new),
               vec![Patch::AddAttributes(0, attributes.clone())],
               "Add attributes",);

    let old: Node<()> = div([id("foobar")], []); //{ <div id="foobar"> </div> },
    let new = div([id("hello")], []); //{ <div id="hello"> </div> },

    assert_eq!(diff(&old, &new),
               vec![Patch::AddAttributes(0, attributes)],
               "Change attribute",);
}

/*
#[test]
fn no_replacing_of_events() {
    let func = |_| {
        println!("hello");
    };
    let hello: Callback<sauron::MouseEvent, ()> = func.into();
    let events = btreemap! {
    "click" => &hello,
    };

    let old: Node<()> = div([], []);
    let new = div([onclick(hello.clone())], []);
    assert_eq!(diff(&old, &new),
               vec![Patch::AddEventListener(0, events.clone())],
               "Should add event listener",);

    let old: Node<()> = div([onclick(hello.clone())], []);
    let new = div([onclick(|_| {
                      sauron::log("Can't be able to replace the first callback")
                  })],
                  []);

    assert_eq!(diff(&old, &new),
               vec![],
               "Should not replace the old callback that was set",);
}
*/

#[test]
fn remove_attributes() {
    let old: Node<()> = div([id("hey-there")], []); //{ <div id="hey-there"></div> },
    let new = div([], []); //{ <div> </div> },
    assert_eq!(diff(&old, &new),
               vec![Patch::RemoveAttributes(0, vec!["id"])],
               "Remove attributes",);
}

#[test]
fn remove_events() {
    let old: Node<()> = div([onclick(|_| println!("hi"))], []);
    let new = div([], []);
    assert_eq!(diff(&old, &new),
               vec![Patch::RemoveEventListener(0, vec!["click"])],
               "Remove events",);
}

#[test]
fn change_attribute() {
    let changed: Value = "changed".into();
    let attributes = btreemap! {
    "id" => &changed,
    };

    let old: Node<()> = div([id("hey-there")], []); //{ <div id="hey-there"></div> },
    let new = div([id("changed")], []); //{ <div id="changed"> </div> },

    assert_eq!(diff(&old, &new),
               vec![Patch::AddAttributes(0, attributes)],
               "Add attributes",);
}

#[test]
fn replace_text_node() {
    let old: Node<()> = text("Old"); //{ Old },
    let new: Node<()> = text("New"); //{ New },

    assert_eq!(diff(&old, &new),
               vec![Patch::ChangeText(0, &Text::new("New"))],
               "Replace text node",);
}

// Initially motivated by having two elements where all that changed was an event listener
// because right now we don't patch event listeners. So.. until we have a solution
// for that we can just give them different keys to force a replace.
#[test]
fn replace_if_different_keys() {
    let old: Node<()> = div([key(1)], []); //{ <div key="1"> </div> },
    let new = div([key(2)], []); //{ <div key="2"> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(0, &div([key(2)], []))],
        "If two nodes have different keys always generate a full replace.",
    );
}

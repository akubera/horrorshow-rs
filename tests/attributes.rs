#[macro_use]
extern crate horrorshow;

use horrorshow::Template;


#[test]
fn test_dash() {
    assert_eq!(html! {
        my_tag {
            inner(data-test="abcde");
        }
    }.into_string().unwrap(), "<my_tag><inner data-test=\"abcde\" /></my_tag>");
}

#[test]
fn test_no_value() {
    assert_eq!(html! {
        my_tag {
            inner(a="test", some_tag, other="1");
        }
    }.into_string().unwrap(), "<my_tag><inner a=\"test\" some_tag other=\"1\" /></my_tag>");
}

#[test]
fn test_boolean() {
    assert_eq!(html! {
        tag(flag?=true);
    }.into_string().unwrap(), "<tag flag />");

    assert_eq!(html! {
        tag(flag?=false);
    }.into_string().unwrap(), "<tag />");
}

#[test]
fn test_option() {
    assert_eq!(html! {
        tag(flag?=Some("value"));
    }.into_string().unwrap(), "<tag flag=\"value\" />");

    assert_eq!(html! {
        tag(flag?=None::<&'static str>);
    }.into_string().unwrap(), "<tag />");
}

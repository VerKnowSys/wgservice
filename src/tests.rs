use crate::utils::find_last_ipv4;
use crate::utils::next_instance_ipv4;
use crate::utils::next_workstation_ipv4;


#[test]
fn check_v4_instance_increment() {
    assert_eq!(next_instance_ipv4("0"), None);
    assert_eq!(
        next_instance_ipv4("127.0.0.0"),
        Some("127.0.0.2".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.0.1"),
        Some("127.0.0.2".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.0.2"),
        Some("127.0.0.3".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.0.253"),
        Some("127.0.0.254".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.0.254"),
        Some("127.0.2.2".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.252.254"),
        Some("127.0.254.2".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.254.253"),
        Some("127.0.254.254".to_string())
    );
    assert_eq!(next_instance_ipv4("127.0.254.254"), None);
}


#[test]
fn check_v4_workstation_increment() {
    assert_eq!(
        next_workstation_ipv4("127.0.0.0"),
        Some("127.0.0.2".to_string())
    );
    assert_eq!(
        next_workstation_ipv4("127.0.0.1"),
        Some("127.0.0.2".to_string())
    );
    assert_eq!(
        next_workstation_ipv4("127.0.0.2"),
        Some("127.0.0.3".to_string())
    );
    assert_eq!(
        next_workstation_ipv4("127.0.0.253"),
        Some("127.0.0.254".to_string())
    );
    assert_eq!(next_workstation_ipv4("127.0.0.254"), None);
    assert_eq!(next_workstation_ipv4("0"), None);
}


#[test]
fn check_find_last_ipv4() {
    let list = vec![
        "127.0.0.150".to_string(),
        "127.0.0.3".to_string(),
        "127.0.0.10".to_string(),
        "127.0.0.120".to_string(),
        "127.0.0.100".to_string(),
    ];
    assert_eq!(find_last_ipv4(list), Some("127.0.0.150".to_string()));

    let list2 = vec![
        "127.0.0.150".to_string(),
        "127.0.0.3".to_string(),
        "127.0.0.10".to_string(),
        "127.0.0.120".to_string(),
        "127.0.2.100".to_string(),
    ];
    assert_eq!(find_last_ipv4(list2), Some("127.0.2.100".to_string()));

    let list2 = vec![
        "127.0.8.150".to_string(),
        "127.0.224.3".to_string(),
        "127.0.0.10".to_string(),
        "127.0.118.120".to_string(),
        "127.0.2.100".to_string(),
    ];
    assert_eq!(find_last_ipv4(list2), Some("127.0.224.3".to_string()));
}

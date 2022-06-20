use super::*;

#[test]
#[cfg(feature = "list_builtin")]
fn hash_stability() {
	assert_eq!("TastefulRapidCassowary".to_string(), gfyhash(&"".to_string(), None, None, None).unwrap());
}

#[test]
#[cfg(not(feature = "list_builtin"))]
fn no_list_breaks() {
	assert!(gfyhash(&"".to_string(), None, None, None).is_err());
}
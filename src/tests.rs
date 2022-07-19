use super::*;

#[test]
#[cfg(feature = "list_builtin")]
fn hash_stability() {
	assert_eq!("TastefulRapidCassowary".to_string(), gfyhash(&"".to_string(), None, None, None).unwrap());
}

#[test]
#[cfg(feature = "list_builtin")]
fn u8_slice_impl() {
	assert!(gfyhash(&std::slice::from_ref(&254u8), None, None, None).is_ok());
}

#[test]
#[cfg(feature = "list_builtin")]
fn u8_impl() {
	assert!(gfyhash(&254u8, None, None, None).is_ok());
}

#[test]
#[cfg(feature = "list_builtin")]
fn usize_impl() {
	assert!(gfyhash(&42usize, None, None, None).is_ok());
}

#[test]
#[cfg(not(feature = "list_builtin"))]
fn no_list_breaks() {
	assert!(gfyhash(&"".to_string(), None, None, None).is_err());
}
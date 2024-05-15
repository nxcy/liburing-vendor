#[test]
fn test() {
    unsafe {
        assert_eq!(liburing_vendor::io_uring_major_version(), 2);
        assert_eq!(liburing_vendor::io_uring_minor_version(), 6);
    }
}

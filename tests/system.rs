extern crate psutil;

#[test]
fn uptime() {
    assert!(psutil::system::uptime() > 0);
}

#[test]
fn virtual_memory() {
    let vm = psutil::system::virtual_memory().unwrap();
    // simple sanity checking
    assert!(vm.total != 0);
    assert!(vm.free <= vm.total);
    assert!(vm.percent > 0.0);
}

#[test]
fn swap_memory() {
    let sm = psutil::system::swap_memory().unwrap();
    // simple sanity checking
    assert!(sm.total != 0);
    assert!(sm.free <= sm.total);
    assert!(sm.percent >= 0.0);
}

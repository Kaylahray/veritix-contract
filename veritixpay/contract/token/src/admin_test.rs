#[test]
fn test_transfer_admin() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let new_admin = Address::generate(&e);

    // Initialize with first admin
    write_admin(&e, &admin);
    
    // Perform transfer (requires admin's mock auth in test environment)
    e.mock_all_auths();
    transfer_admin(&e, new_admin.clone());

    assert_eq!(read_admin(&e), new_admin);
}

#[test]
#[should_panic]
fn test_transfer_admin_unauthorized_panics() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let hacker = Address::generate(&e);
    let new_admin = Address::generate(&e);

    write_admin(&e, &admin);

    // This should panic because hacker is calling it, not the current admin
    e.set_auths(&[]); // Ensure no mock auths bypass the check
    transfer_admin(&e, new_admin);
}
use crate::shell_contract::public_shell_contract;

#[test]
fn public_shell_omits_app_only_controls() {
    let contract = public_shell_contract();
    assert!(!contract.project_switcher);
    assert!(!contract.login_control);
    assert!(!contract.install_action);
    assert!(!contract.initial_backend_fetch);
}

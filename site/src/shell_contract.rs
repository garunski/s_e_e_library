/// Declares what the public library shell intentionally omits (no app chrome).
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicShellContract {
    pub project_switcher: bool,
    pub login_control: bool,
    pub install_action: bool,
    pub initial_backend_fetch: bool,
}

#[must_use]
pub fn public_shell_contract() -> PublicShellContract {
    PublicShellContract {
        project_switcher: false,
        login_control: false,
        install_action: false,
        initial_backend_fetch: false,
    }
}

#[cfg(test)]
#[path = "shell_contract_tests.rs"]
mod shell_contract_tests;

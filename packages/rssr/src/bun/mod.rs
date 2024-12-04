use std::process::Command;

use crate::logger::debugger_lite::RsDebugger;

pub fn check_for_bun_in_env() -> Option<String> {
    let mut bun_logger = RsDebugger::new(String::from("rssr:bun:utilities"));
    bun_logger.write("Checking environment for `bun` executable... :rocket:".to_string());

    let mut base_command = Command::new("bun");
    let command = base_command.arg("--version");
    let output = command.output();
    match output {
        Ok(output) => {
            if output.status.success() {
                Some(
                    String::from_utf8_lossy(&output.stdout)
                        .to_string()
                        .replace("\n", ""),
                )
            } else {
                None
            }
        }
        Err(error) => {
            let mut warn = bun_logger.extend(String::from("warn"));
            warn.write(String::from("You do not have `bun` installed!"));
            warn.write(String::from(
                "rssr relies on `bun` for server side 'just in time' rendering!",
            ));
            warn.write(format!("{:#?}", error));
            None
        }
    }
}

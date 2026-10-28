use orbit_system_manager::preview::DemoScenario;

const USAGE: &str = "Usage: orbit-system-manager [--demo-state full|empty|single|unavailable]";

#[derive(Debug)]
pub struct DemoOptions {
    pub scenario: DemoScenario,
}

pub fn parse_demo_args(args: impl IntoIterator<Item = String>) -> Result<DemoOptions, String> {
    let mut scenario = DemoScenario::Full;
    let mut args = args.into_iter();

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--demo-state" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("Missing value for --demo-state.\n{USAGE}"))?;
                scenario = DemoScenario::parse(&value)
                    .ok_or_else(|| format!("Invalid demo state '{value}'.\n{USAGE}"))?;
            }
            _ => return Err(format!("Unknown argument '{argument}'.\n{USAGE}")),
        }
    }

    Ok(DemoOptions { scenario })
}

#[cfg(test)]
mod tests {
    use super::parse_demo_args;
    use orbit_system_manager::preview::DemoScenario;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn demo_state_defaults_to_full_and_accepts_supported_scenarios() {
        assert_eq!(
            parse_demo_args(args(&[])).unwrap().scenario,
            DemoScenario::Full
        );
        for (value, expected) in [
            ("full", DemoScenario::Full),
            ("empty", DemoScenario::Empty),
            ("single", DemoScenario::Single),
            ("unavailable", DemoScenario::Unavailable),
        ] {
            assert_eq!(
                parse_demo_args(args(&["--demo-state", value]))
                    .unwrap()
                    .scenario,
                expected
            );
        }
    }

    #[test]
    fn invalid_demo_state_and_unknown_arguments_explain_usage() {
        assert!(parse_demo_args(args(&["--demo-state", "bogus"]))
            .unwrap_err()
            .contains(USAGE));
        assert!(parse_demo_args(args(&["--demo-state"]))
            .unwrap_err()
            .contains(USAGE));
        assert!(parse_demo_args(args(&["--other"]))
            .unwrap_err()
            .contains(USAGE));
    }

    const USAGE: &str = "Usage: orbit-system-manager";
}

use orbit_system_manager::preview::DemoScenario;

const USAGE: &str = "Usage: orbit-system-manager [--demo-state full|empty|single|unavailable] [--demo-cores 0|1|4|16|32|64]";

#[derive(Debug)]
pub struct DemoOptions {
    pub scenario: DemoScenario,
    pub core_count: usize,
}

pub fn parse_demo_args(args: impl IntoIterator<Item = String>) -> Result<DemoOptions, String> {
    let mut scenario = DemoScenario::Full;
    let mut core_count = 16;
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
            "--demo-cores" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("Missing value for --demo-cores.\n{USAGE}"))?;
                core_count = match value.as_str() {
                    "0" => 0,
                    "1" => 1,
                    "4" => 4,
                    "16" => 16,
                    "32" => 32,
                    "64" => 64,
                    _ => return Err(format!("Invalid demo core count '{value}'.\n{USAGE}")),
                };
            }
            _ => return Err(format!("Unknown argument '{argument}'.\n{USAGE}")),
        }
    }

    Ok(DemoOptions {
        scenario,
        core_count,
    })
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
        let defaults = parse_demo_args(args(&[])).unwrap();
        assert_eq!(defaults.scenario, DemoScenario::Full);
        assert_eq!(defaults.core_count, 16);
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
    fn demo_core_count_accepts_only_supported_values() {
        for count in [0, 1, 4, 16, 32, 64] {
            let parsed = parse_demo_args(args(&["--demo-cores", &count.to_string()])).unwrap();
            assert_eq!(parsed.core_count, count);
        }
        assert!(parse_demo_args(args(&["--demo-cores", "8"]))
            .unwrap_err()
            .contains(USAGE));
        assert!(parse_demo_args(args(&["--demo-cores"]))
            .unwrap_err()
            .contains(USAGE));
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

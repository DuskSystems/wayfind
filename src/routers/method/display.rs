use super::MethodRouter;
use std::{
    collections::BTreeSet,
    fmt::{Display, Write},
};

impl Display for MethodRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let route_keys: BTreeSet<_> = self.map.keys().collect();
        let last_key = route_keys.last().copied();

        for key in route_keys {
            writeln!(output, "[{}, {}]", key.0, key.1)?;

            let method_map = &self.map[key];
            let mut methods: Vec<(&str, String)> = method_map
                .iter()
                .map(|(method, id)| (method.as_str(), format!("[{id}]")))
                .collect();
            methods.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

            for (i, (method, id)) in methods.iter().enumerate() {
                let branch = if i == methods.len() - 1 {
                    "╰─"
                } else {
                    "├─"
                };
                writeln!(output, "{branch} {method} {id}")?;
            }

            if Some(key) != last_key {
                writeln!(output)?;
            }
        }

        write!(f, "{}", output.trim_end())
    }
}

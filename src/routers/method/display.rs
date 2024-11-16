use super::MethodRouter;
use std::{
    collections::BTreeSet,
    fmt::{Display, Write},
};

impl Display for MethodRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let paths: BTreeSet<_> = self.map.keys().collect();
        let last_path = paths.last().copied();

        for path in paths {
            writeln!(output, "{path}")?;

            let mut methods: Vec<(&str, String)> = self.map[path]
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

            if Some(path) != last_path {
                writeln!(output)?;
            }
        }

        write!(f, "{}", output.trim_end())
    }
}

use super::MethodRouter;
use std::fmt::{Display, Write};

impl Display for MethodRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let last_key = self.map.keys().last();

        for key in self.map.keys() {
            writeln!(output, "[{}]", key.0)?;

            let method_map = &self.map[key];
            for (i, (method, id)) in method_map.iter().enumerate() {
                let branch = if i == method_map.len() - 1 {
                    "╰─"
                } else {
                    "├─"
                };
                writeln!(output, "{branch} {method} [{id}]")?;
            }

            if Some(key) != last_key {
                writeln!(output)?;
            }
        }

        write!(f, "{}", output.trim_end())
    }
}

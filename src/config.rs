// struct Cycle {
//     name: String,
//     steps: Vec<String>,
// }

enum Command {
    // Cycle(Cycle),
    DefaultCommand(String),
}

enum ConfigError {
    InvalidSyntax,
}

pub struct ExtendedConfig {
    content: String,
    commands: Vec<Command>,
}

impl ExtendedConfig {
    pub fn new(content: String) -> Self {
        Self {
            content,
            commands: Vec::new(),
        }
    }

    pub fn parse(self: &mut Self) {
        let lines = self.content.split('\n');

        let commands: Vec<Result<Command, ConfigError>> = lines
            .into_iter()
            .map(|line| {
                // comment
                if line.starts_with("//") {
                    // TODO: Ignore comments more elegantly
                    return Err(ConfigError::InvalidSyntax);
                }

                // extention
                if line.starts_with("$") {
                    // TODO: impl extended command
                    return Err(ConfigError::InvalidSyntax);
                }

                // default command
                return Ok(Command::DefaultCommand(line.to_string()));
            })
            .collect();

        self.commands = commands
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<Command>>();
    }

    pub fn export(self: Self) -> String {
        self.commands
            .into_iter()
            .map(|command| match command {
                Command::DefaultCommand(line) => line,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::ExtendedConfig;

    #[test]
    fn comment() {
        // TODO: ignore comments
        let content = String::from("// This is a comment");
        let mut config = ExtendedConfig::new(content);
        config.parse();

        assert_eq!(config.commands.len(), 0);
    }
}

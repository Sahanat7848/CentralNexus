use anyhow::Result;
use std::fmt;
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Dev",
            Stage::Production => "Prod",
        };
        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn try_from_str(stage: &str) -> Result<Self> {
        match stage {
            "Local" => Ok(Self::Local),
            "Dev" | "Development" => Ok(Self::Development),
            "Prod" | "Production" => Ok(Self::Production),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}

impl std::str::FromStr for Stage {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

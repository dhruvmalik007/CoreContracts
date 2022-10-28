use create_type_spec_derive::CreateTypeSpec;
use read_write_state_derive::ReadWriteState;

/// ## Description
/// This structure describes contract version base state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug, Default)]
pub struct ContractVersionBase {
    pub name: String,
    pub version: String,
}

impl ContractVersionBase {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    pub fn set_contract_version(&mut self, name: &str, version: &str) {
        self.name = name.to_string();
        self.version = version.to_string();
    }

    pub fn get_contract_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_contract_version(&self) -> String {
        self.version.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_version_base() {
        let mut state = ContractVersionBase::new("name1", "1.0.0");
        assert_eq!(state.get_contract_name(), "name1".to_string());
        assert_eq!(state.get_contract_version(), "1.0.0".to_string());

        state.set_contract_version("name2", "1.1.0");
        assert_eq!(state.get_contract_name(), "name2".to_string());
        assert_eq!(state.get_contract_version(), "1.1.0".to_string());
    }
}

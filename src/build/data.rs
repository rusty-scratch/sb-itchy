use crate::{name::Name, uid::Uid};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableBuilder {
    pub value: sb_sbity::value::Value,
    /// Cloud variable can only store number. Becareful!
    pub is_cloud_variable: bool,
}

impl VariableBuilder {
    pub fn new(starting_value: sb_sbity::value::Value) -> VariableBuilder {
        VariableBuilder {
            value: starting_value,
            is_cloud_variable: false,
        }
    }

    pub fn new_cloud_variable(starting_value: sb_sbity::value::Value) -> VariableBuilder {
        debug_assert!(matches!(starting_value, sb_sbity::value::Value::Number(_)));
        VariableBuilder {
            value: starting_value,
            is_cloud_variable: true,
        }
    }

    pub fn build(self, name_for_this_var: Name) -> sb_sbity::variable::Variable {
        let VariableBuilder {
            value,
            is_cloud_variable,
        } = self;
        let var = sb_sbity::variable::Variable {
            name: name_for_this_var.to_string(),
            value,
            is_cloud_variable,
        };
        var
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListBuilder {
    pub values: Vec<sb_sbity::value::Value>,
}

impl ListBuilder {
    pub fn new(values: Vec<sb_sbity::value::Value>) -> ListBuilder {
        ListBuilder { values }
    }

    pub fn build(self, name_for_this_list: Name) -> sb_sbity::list::List {
        let ListBuilder { values } = self;
        let my_uid = Uid::generate();
        let list = sb_sbity::list::List {
            name: name_for_this_list.to_string(),
            values,
        };
        list
    }
}

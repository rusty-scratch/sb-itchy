use std::collections::HashMap;

use crate::uid::Uid;

struct TargetContextData {
    variables: HashMap<String, Uid>,
    lists: HashMap<String, Uid>,
}

struct ProjectContextData {
    global_variables: HashMap<String, Uid>,
    global_lists: HashMap<String, Uid>,
    broadcasts: HashMap<String, Uid>,
}

use crate::helper_functions::{Definition,ApiConnection};

pub struct  ApiConnections{
    pub list: Vec<ApiConnection>
}

impl ApiConnections {
    pub fn new() -> Self { Self { list: <Vec<ApiConnection>>::new() } }
    pub fn find_by_api_id(&self, api_definition: Definition) -> Option<&ApiConnection> {
        self.list.iter().find(|item| item.definition.id == api_definition.id)
    }

}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: Option<String>,
    pub name: Option<String>,
}

rbatis::crud!(Company {});

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub repo_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariable {
    pub id: String,
    pub profile_id: String,
    pub key: String,
    pub value: String, // If is_secret == true, value is stored as mask or empty in JSON, read from Keychain
    pub is_secret: bool,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreData {
    pub workspaces: Vec<Workspace>,
    pub projects: Vec<Project>,
    pub profiles: Vec<Profile>,
    pub variables: Vec<EnvVariable>,
    pub active_profile_id: Option<String>,
    pub injected_keys: Vec<String>,
}

impl Default for StoreData {
    fn default() -> Self {
        let ws_id = "ws_general".to_string();
        let proj_id = "proj_demo".to_string();
        let prof_local_id = "prof_local".to_string();
        let prof_prod_id = "prof_prod".to_string();

        StoreData {
            workspaces: vec![Workspace {
                id: ws_id.clone(),
                name: "通用全局".to_string(),
                icon: "globe".to_string(),
            }],
            projects: vec![Project {
                id: proj_id.clone(),
                workspace_id: ws_id,
                name: "演示项目".to_string(),
                repo_path: None,
            }],
            profiles: vec![
                Profile {
                    id: prof_local_id.clone(),
                    project_id: proj_id.clone(),
                    name: "Local".to_string(),
                    description: "本地开发联调环境".to_string(),
                    is_active: true,
                },
                Profile {
                    id: prof_prod_id.clone(),
                    project_id: proj_id,
                    name: "Prod".to_string(),
                    description: "生产部署环境".to_string(),
                    is_active: false,
                },
            ],
            variables: vec![
                EnvVariable {
                    id: "var_node_env".to_string(),
                    profile_id: prof_local_id.clone(),
                    key: "NODE_ENV".to_string(),
                    value: "development".to_string(),
                    is_secret: false,
                    description: "运行环境模式".to_string(),
                    enabled: true,
                },
                EnvVariable {
                    id: "var_api_url".to_string(),
                    profile_id: prof_local_id,
                    key: "API_BASE_URL".to_string(),
                    value: "http://localhost:8000".to_string(),
                    is_secret: false,
                    description: "后端服务地址".to_string(),
                    enabled: true,
                },
                EnvVariable {
                    id: "var_api_url_prod".to_string(),
                    profile_id: prof_prod_id.clone(),
                    key: "API_BASE_URL".to_string(),
                    value: "https://api.example.com".to_string(),
                    is_secret: false,
                    description: "生产后端网关".to_string(),
                    enabled: true,
                },
                EnvVariable {
                    id: "var_prod_secret".to_string(),
                    profile_id: prof_prod_id,
                    key: "APP_SECRET_TOKEN".to_string(),
                    value: "".to_string(),
                    is_secret: true,
                    description: "生产签名私钥（钥匙串加密）".to_string(),
                    enabled: true,
                },
            ],
            active_profile_id: Some("prof_local".to_string()),
            injected_keys: Vec::new(),
        }
    }
}

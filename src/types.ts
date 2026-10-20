export interface Workspace {
  id: string;
  name: string;
  icon?: string;
  created_at: string;
}

export interface Project {
  id: string;
  workspace_id: string;
  name: string;
  description: string;
  created_at: string;
}

export interface Profile {
  id: string;
  project_id: string;
  name: string;
  color: string;
  is_active: boolean;
  created_at: string;
}

export interface EnvVariable {
  id: string;
  profile_id: string;
  key: string;
  value: string;
  is_secret: boolean;
  enabled: boolean;
  description: string;
  updated_at: string;
}

export interface StoreData {
  version: number;
  workspaces: Workspace[];
  projects: Project[];
  profiles: Profile[];
  variables: EnvVariable[];
  active_profile_id?: string;
  applied_keys: string[];
}

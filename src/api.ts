import { invoke } from "@tauri-apps/api/core";
import type { StoreData } from "./types";

export const api = {
  async getStore(): Promise<StoreData> {
    return await invoke<StoreData>("get_store");
  },

  async saveStoreData(data: StoreData): Promise<void> {
    return await invoke<void>("save_store_data", { newData: data });
  },

  async applyProfile(profileId: string): Promise<string[]> {
    return await invoke<string[]>("apply_profile", { profileId });
  },

  async clearSystemEnv(): Promise<void> {
    return await invoke<void>("clear_system_env");
  },

  async setSecretVal(profileId: string, key: string, secret: string): Promise<void> {
    return await invoke<void>("set_secret_val", { profileId, key, secret });
  },

  async getSecretVal(profileId: string, key: string): Promise<string> {
    return await invoke<string>("get_secret_val", { profileId, key });
  },

  async parseEnvText(content: string): Promise<[string, string][]> {
    return await invoke<[string, string][]>("parse_env_text", { content });
  },

  async exportProfileDotenv(profileId: string): Promise<string> {
    return await invoke<string>("export_profile_dotenv", { profileId });
  },

  async copyToClipboard(text: string): Promise<void> {
    return await invoke<void>("copy_to_clipboard", { text });
  },
};

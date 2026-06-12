import { create } from 'zustand'
import { AppSettings, api } from '../lib/tauri'

const DEFAULTS: AppSettings = {
  ollama_url: 'http://localhost:11434',
  text_model: 'llama3',
  auto_scan_on_startup: true,
  scan_interval_minutes: 60,
  show_safe_processes: false,
  keep_history_days: 30,
  cpu_spike_threshold: 50,
  memory_high_threshold: 80,
  disk_warning_threshold: 85,
  temp_warning_celsius: 80,
}

interface SettingsStore {
  settings: AppSettings
  setSettings: (s: AppSettings) => void
  load: () => Promise<void>
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  settings: DEFAULTS,
  setSettings: s => set({ settings: s }),
  load: async () => {
    try { set({ settings: await api.getSettings() }) } catch {}
  },
}))

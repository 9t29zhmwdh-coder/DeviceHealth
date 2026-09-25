import { create } from 'zustand'
import { HealthSnapshot, Finding, ProcessEntry, Recommendation, HardwareReport, AutostartEntry, api } from '../lib/tauri'

interface AnalysisStore {
  snapshot: HealthSnapshot | null
  findings: Finding[]
  processes: ProcessEntry[]
  recommendations: Recommendation[]
  autostart: AutostartEntry[]
  hardware: HardwareReport | null
  history: HealthSnapshot[]
  running: boolean
  ollamaOnline: boolean
  showSafe: boolean
  setSnapshot: (s: HealthSnapshot) => void
  setRunning: (v: boolean) => void
  setOllamaOnline: (v: boolean) => void
  setShowSafe: (v: boolean) => void
  loadAll: () => Promise<void>
  /** One entry point for the button, the start-up scan and the interval. */
  runScan: () => Promise<void>
  loadHistory: () => Promise<void>
}

export const useAnalysisStore = create<AnalysisStore>((set, get) => ({
  snapshot: null,
  findings: [],
  processes: [],
  recommendations: [],
  autostart: [],
  hardware: null,
  history: [],
  running: false,
  ollamaOnline: false,
  showSafe: false,

  setSnapshot: s => set({ snapshot: s }),
  setRunning: v => set({ running: v }),
  setOllamaOnline: v => set({ ollamaOnline: v }),
  setShowSafe: v => {
    set({ showSafe: v })
    api.getProcesses(v).then(processes => set({ processes })).catch(() => {})
  },

  loadAll: async () => {
    try {
      const [snapshot, findings, processes, recommendations, hardware, autostart] = await Promise.all([
        api.getLastSnapshot(),
        api.getFindings(),
        api.getProcesses(get().showSafe),
        api.getRecommendations(),
        api.getHardware(),
        api.getAutostart(),
      ])
      set({ snapshot, findings, processes, recommendations, autostart, hardware: hardware ?? null })
    } catch {}
  },

  runScan: async () => {
    if (get().running) return
    set({ running: true })
    try {
      const snapshot = await api.runAnalysis()
      set({ snapshot })
      await Promise.all([get().loadAll(), get().loadHistory()])
    } finally {
      set({ running: false })
    }
    api.checkOllama().then(ok => set({ ollamaOnline: ok })).catch(() => {})
  },

  loadHistory: async () => {
    try {
      const history = await api.getHistory(30)
      set({ history })
    } catch {}
  },
}))

import { useEffect, useState } from 'react'
import { useAnalysisStore } from './stores/analysisStore'
import { useSettingsStore } from './stores/settingsStore'
import { api, events } from './lib/tauri'
import { Dashboard } from './components/Dashboard/Dashboard'
import { ProcessList } from './components/ProcessList/ProcessList'
import { FindingsView } from './components/Findings/FindingsView'
import { HardwareView } from './components/Hardware/HardwareView'
import { HistoryView } from './components/History/HistoryView'
import { SettingsView } from './components/Settings/SettingsView'

type Tab = 'dashboard' | 'processes' | 'findings' | 'hardware' | 'history' | 'settings'

export default function App() {
  const [tab, setTab] = useState<Tab>('dashboard')
  const { setRunning, setSnapshot, setOllamaOnline, loadAll, loadHistory, findings } = useAnalysisStore()
  const { load: loadSettings } = useSettingsStore()

  useEffect(() => {
    loadSettings()
    api.checkOllama().then(setOllamaOnline).catch(() => {})

    const cleanup: Array<() => void> = []
    events.onAnalysisStarted(() => setRunning(true)).then(u => cleanup.push(u))
    events.onAnalysisDone(snap => {
      setSnapshot(snap)
      setRunning(false)
      loadAll()
      loadHistory()
    }).then(u => cleanup.push(u))

    loadAll()
    loadHistory()
    return () => cleanup.forEach(u => u())
  }, [])

  const nav = (id: Tab, icon: string, label: string, badge?: number) => (
    <button
      key={id}
      onClick={() => setTab(id)}
      className={`flex items-center gap-2.5 w-full px-3 py-2 rounded-md text-sm transition-colors relative
        ${tab === id ? 'bg-[#21262d] text-[#e6edf3]' : 'text-[#8b949e] hover:bg-[#161b22] hover:text-[#e6edf3]'}`}
    >
      <span className="text-base">{icon}</span>
      <span className="flex-1 text-left">{label}</span>
      {badge != null && badge > 0 && (
        <span className="text-xs bg-[#f85149] text-white px-1.5 py-0.5 rounded-full min-w-[20px] text-center leading-none">
          {badge > 99 ? '99+' : badge}
        </span>
      )}
    </button>
  )

  const criticalCount = findings.filter(f => f.severity === 'Critical' || f.severity === 'High').length

  return (
    <div className="flex h-screen bg-[#0d1117] text-[#e6edf3] overflow-hidden">
      {/* Sidebar */}
      <div className="w-52 flex-shrink-0 border-r border-[#30363d] flex flex-col">
        <div className="p-4 border-b border-[#30363d]">
          <div className="flex items-center gap-2 mb-1">
            <span className="text-xl">🩺</span>
            <span className="font-semibold">DeviceHealth</span>
          </div>
        </div>
        <nav className="flex-1 p-2 space-y-0.5">
          {nav('dashboard', '📊', 'Übersicht')}
          {nav('findings', '🔍', 'Befunde', criticalCount)}
          {nav('processes', '⚙️', 'Prozesse')}
          {nav('hardware', '💾', 'Hardware')}
          {nav('history', '📈', 'Verlauf')}
          {nav('settings', '⚙️', 'Einstellungen')}
        </nav>
      </div>

      {/* Main */}
      <div className="flex-1 overflow-hidden">
        {tab === 'dashboard'  && <Dashboard onNavigate={setTab} />}
        {tab === 'findings'   && <FindingsView />}
        {tab === 'processes'  && <ProcessList />}
        {tab === 'hardware'   && <HardwareView />}
        {tab === 'history'    && <HistoryView />}
        {tab === 'settings'   && <SettingsView />}
      </div>
    </div>
  )
}

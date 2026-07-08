import { useState } from 'react'
import { useSettingsStore } from '../../stores/settingsStore'
import { api, type AppSettings } from '../../lib/tauri'
import { useT } from '../../lib/i18n'

export function SettingsView() {
  const { settings, setSettings } = useSettingsStore()
  const [draft, setDraft] = useState<AppSettings>(settings)
  const [saved, setSaved] = useState(false)
  const [testing, setTesting] = useState(false)
  const [ollamaStatus, setOllamaStatus] = useState<'idle' | 'ok' | 'fail'>('idle')
  const t = useT()

  const set = <K extends keyof AppSettings>(key: K, val: AppSettings[K]) =>
    setDraft(d => ({ ...d, [key]: val }))

  const handleSave = async () => {
    await api.saveSettings(draft)
    setSettings(draft)
    setSaved(true)
    setTimeout(() => setSaved(false), 2000)
  }

  const handleTestOllama = async () => {
    setTesting(true)
    setOllamaStatus('idle')
    try {
      const ok = await api.checkOllama()
      setOllamaStatus(ok ? 'ok' : 'fail')
    } catch {
      setOllamaStatus('fail')
    } finally {
      setTesting(false)
    }
  }

  return (
    <div className="h-full overflow-y-auto p-6">
      <div className="max-w-xl space-y-8">

        {/* KI */}
        <section>
          <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-4">{t('settings.aiBackend')}</h2>
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 space-y-4">
            <Field label={t('settings.ollamaUrl')}>
              <input value={draft.ollama_url} onChange={e => set('ollama_url', e.target.value)}
                className="w-full bg-[#0d1117] border border-[#30363d] rounded-md px-3 py-2 text-sm text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]" />
            </Field>
            <Field label={t('settings.model')}>
              <input value={draft.text_model} onChange={e => set('text_model', e.target.value)}
                placeholder={t('settings.modelPlaceholder')}
                className="w-full bg-[#0d1117] border border-[#30363d] rounded-md px-3 py-2 text-sm text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]" />
            </Field>
            <div className="flex items-center gap-3">
              <button onClick={handleTestOllama} disabled={testing}
                className="px-3 py-1.5 text-xs bg-[#21262d] hover:bg-[#30363d] text-[#e6edf3] rounded-md transition-colors disabled:opacity-50">
                {testing ? t('settings.testing') : t('settings.testConnection')}
              </button>
              {ollamaStatus === 'ok' && <span className="text-xs text-[#3fb950]">✓ {t('settings.connected')}</span>}
              {ollamaStatus === 'fail' && <span className="text-xs text-[#f85149]">✗ {t('settings.unreachable')}</span>}
            </div>
          </div>
        </section>

        {/* Analyse */}
        <section>
          <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-4">{t('settings.analysis')}</h2>
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 space-y-4">
            <Toggle label={t('settings.autoScanOnStartup')}
              checked={draft.auto_scan_on_startup}
              onChange={v => set('auto_scan_on_startup', v)} />
            <Toggle label={t('settings.showSafeProcesses')}
              checked={draft.show_safe_processes}
              onChange={v => set('show_safe_processes', v)} />
            <Field label={t('settings.scanInterval')}>
              <input type="number" min={5} max={1440} value={draft.scan_interval_minutes}
                onChange={e => set('scan_interval_minutes', Number(e.target.value))}
                className="w-32 bg-[#0d1117] border border-[#30363d] rounded-md px-3 py-2 text-sm text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]" />
            </Field>
          </div>
        </section>

        {/* Schwellenwerte */}
        <section>
          <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-4">{t('settings.thresholds')}</h2>
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 space-y-4">
            <SliderField label={t('settings.cpuWarning')} value={draft.cpu_spike_threshold} min={20} max={95}
              onChange={v => set('cpu_spike_threshold', v)} />
            <SliderField label={t('settings.memWarning')} value={draft.memory_high_threshold} min={50} max={98}
              onChange={v => set('memory_high_threshold', v)} />
            <SliderField label={t('settings.diskWarning')} value={draft.disk_warning_threshold} min={50} max={98}
              onChange={v => set('disk_warning_threshold', v)} />
            <SliderField label={t('settings.tempWarning')} value={draft.temp_warning_celsius} min={50} max={100}
              onChange={v => set('temp_warning_celsius', v)} />
          </div>
        </section>

        {/* Verlauf */}
        <section>
          <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-4">{t('settings.history')}</h2>
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 space-y-4">
            <Field label={t('settings.keepHistoryDays')}>
              <input type="number" min={1} max={365} value={draft.keep_history_days}
                onChange={e => set('keep_history_days', Number(e.target.value))}
                className="w-32 bg-[#0d1117] border border-[#30363d] rounded-md px-3 py-2 text-sm text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]" />
            </Field>
            <button onClick={() => api.cleanupHistory()}
              className="text-xs text-[#f85149] hover:underline">
              {t('settings.cleanupNow')}
            </button>
          </div>
        </section>

        <div className="flex items-center gap-3 pb-6">
          <button onClick={handleSave}
            className="px-5 py-2 bg-[#238636] hover:bg-[#2ea043] text-white text-sm rounded-lg transition-colors">
            {t('settings.save')}
          </button>
          {saved && <span className="text-xs text-[#3fb950]">✓ {t('settings.saved')}</span>}
        </div>
      </div>
    </div>
  )
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div>
      <label className="block text-xs text-[#8b949e] mb-1.5">{label}</label>
      {children}
    </div>
  )
}

function Toggle({ label, checked, onChange }: { label: string; checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <label className="flex items-center justify-between cursor-pointer select-none">
      <span className="text-sm text-[#e6edf3]">{label}</span>
      <div onClick={() => onChange(!checked)}
        className={`w-10 h-5 rounded-full transition-colors relative ${checked ? 'bg-[#238636]' : 'bg-[#30363d]'}`}>
        <div className={`absolute top-0.5 w-4 h-4 bg-white rounded-full shadow transition-transform ${checked ? 'translate-x-5' : 'translate-x-0.5'}`} />
      </div>
    </label>
  )
}

function SliderField({ label, value, min, max, onChange }: {
  label: string; value: number; min: number; max: number; onChange: (v: number) => void
}) {
  return (
    <div>
      <div className="flex justify-between mb-1.5">
        <span className="text-xs text-[#8b949e]">{label}</span>
        <span className="text-xs text-[#e6edf3] font-medium">{value}</span>
      </div>
      <input type="range" min={min} max={max} value={value}
        onChange={e => onChange(Number(e.target.value))}
        className="w-full accent-[#58a6ff]" />
    </div>
  )
}

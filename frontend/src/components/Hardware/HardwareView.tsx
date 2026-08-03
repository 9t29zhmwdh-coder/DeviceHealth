import { useAnalysisStore } from '../../stores/analysisStore'
import { formatBytes } from '../../lib/tauri'
import { useT } from '../../lib/i18n'

export function HardwareView() {
  const { hardware } = useAnalysisStore()
  const t = useT()

  if (!hardware) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <div className="text-4xl mb-2">💾</div>
          <div className="text-[#8b949e] text-sm">{t('hardware.noAnalysisYet')}</div>
        </div>
      </div>
    )
  }

  const sys = hardware.system
  const memPct = sys.total_memory_bytes > 0
    ? (sys.used_memory_bytes / sys.total_memory_bytes) * 100 : 0
  const swapPct = sys.swap_total_bytes > 0
    ? (sys.swap_used_bytes / sys.swap_total_bytes) * 100 : 0

  return (
    <div className="h-full overflow-y-auto p-6 space-y-6">
      {/* System */}
      <section>
        <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-3">{t('hardware.system')}</h2>
        <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 grid grid-cols-2 gap-x-8 gap-y-3">
          <InfoRow label={t('hardware.os')} value={`${sys.os_name} ${sys.os_version}`} />
          <InfoRow label={t('hardware.hostname')} value={sys.hostname} />
          <InfoRow label={t('hardware.cpu')} value={sys.cpu_brand} />
          <InfoRow label={t('hardware.cpuCores')} value={String(sys.cpu_cores)} />
          <InfoRow label={t('hardware.cpuFreq')} value={`${(sys.cpu_freq_mhz / 1000).toFixed(2)} GHz`} />
          <InfoRow label={t('hardware.cpuUsage')} value={`${sys.cpu_usage_global.toFixed(1)}%`} />
        </div>
      </section>

      {/* Memory */}
      <section>
        <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-3">{t('hardware.memory')}</h2>
        <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 space-y-4">
          <BarStat label="RAM" used={sys.used_memory_bytes} total={sys.total_memory_bytes} pct={memPct}
            color={memPct > 85 ? '#f0883e' : '#58a6ff'} />
          {sys.swap_total_bytes > 0 && (
            <BarStat label="Swap" used={sys.swap_used_bytes} total={sys.swap_total_bytes} pct={swapPct}
              color={swapPct > 70 ? '#d29922' : '#8b949e'} />
          )}
        </div>
      </section>

      {/* Disks */}
      {hardware.disks.length > 0 && (
        <section>
          <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-3">{t('hardware.disks')}</h2>
          <div className="space-y-2">
            {hardware.disks.map((d, i) => {
              const used = d.total_bytes - d.available_bytes
              const pct = d.total_bytes > 0 ? (used / d.total_bytes) * 100 : 0
              return (
                <div key={i} className="bg-[#161b22] border border-[#30363d] rounded-xl p-4">
                  <div className="flex items-center justify-between mb-2">
                    <div>
                      <span className="text-sm text-[#e6edf3]">{d.name}</span>
                      <span className="ml-2 text-xs text-[#8b949e]">{d.mount_point}</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <span className="text-xs bg-[#30363d] text-[#8b949e] px-2 py-0.5 rounded-sm">{d.kind}</span>
                      <span className="text-xs text-[#8b949e]">{d.file_system}</span>
                    </div>
                  </div>
                  <BarStat
                    label={`${formatBytes(used)} / ${formatBytes(d.total_bytes)}`}
                    used={used} total={d.total_bytes} pct={pct}
                    color={pct > 90 ? '#f85149' : pct > 75 ? '#f0883e' : '#3fb950'}
                    showLabel={false}
                  />
                </div>
              )
            })}
          </div>
        </section>
      )}

      {/* Temperatures */}
      {hardware.temperatures.length > 0 && (
        <section>
          <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-3">{t('hardware.temperatures')}</h2>
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 grid grid-cols-2 gap-3">
            {hardware.temperatures.map((t, i) => {
              const hot = t.temperature_celsius > (t.critical_threshold ?? 90)
              const warm = t.temperature_celsius > 70
              return (
                <div key={i} className="flex items-center justify-between">
                  <span className="text-xs text-[#8b949e] truncate mr-2">{t.label}</span>
                  <span className={`text-sm font-medium ${hot ? 'text-[#f85149]' : warm ? 'text-[#f0883e]' : 'text-[#3fb950]'}`}>
                    {t.temperature_celsius.toFixed(0)}°C
                  </span>
                </div>
              )
            })}
          </div>
        </section>
      )}

      {/* Network */}
      {hardware.network.length > 0 && (
        <section>
          <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-3">{t('hardware.network')}</h2>
          <div className="space-y-2">
            {hardware.network.filter(n => n.bytes_received + n.bytes_transmitted > 0).map((n, i) => (
              <div key={i} className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 grid grid-cols-3 gap-4">
                <div>
                  <div className="text-xs text-[#8b949e] mb-1">{t('hardware.interface')}</div>
                  <div className="text-sm text-[#e6edf3]">{n.interface}</div>
                </div>
                <div>
                  <div className="text-xs text-[#8b949e] mb-1">{t('hardware.received')}</div>
                  <div className="text-sm text-[#3fb950]">↓ {formatBytes(n.bytes_received)}</div>
                </div>
                <div>
                  <div className="text-xs text-[#8b949e] mb-1">{t('hardware.sent')}</div>
                  <div className="text-sm text-[#58a6ff]">↑ {formatBytes(n.bytes_transmitted)}</div>
                </div>
              </div>
            ))}
          </div>
        </section>
      )}
    </div>
  )
}

function InfoRow({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <div className="text-xs text-[#8b949e] mb-0.5">{label}</div>
      <div className="text-sm text-[#e6edf3]">{value}</div>
    </div>
  )
}

function BarStat({ label, used, total, pct, color, showLabel = true }: {
  label: string; used: number; total: number; pct: number; color: string; showLabel?: boolean
}) {
  return (
    <div>
      {showLabel && (
        <div className="flex justify-between text-xs mb-1">
          <span className="text-[#8b949e]">{label}</span>
          <span className="text-[#e6edf3]">{formatBytes(used)} / {formatBytes(total)} ({pct.toFixed(1)}%)</span>
        </div>
      )}
      {!showLabel && (
        <div className="flex justify-between text-xs mb-1">
          <span className="text-[#8b949e]">{label}</span>
          <span className="text-[#e6edf3]">{pct.toFixed(1)}%</span>
        </div>
      )}
      <div className="h-2 bg-[#21262d] rounded-full overflow-hidden">
        <div className="h-full rounded-full transition-all" style={{ width: `${Math.min(pct, 100)}%`, background: color }} />
      </div>
    </div>
  )
}

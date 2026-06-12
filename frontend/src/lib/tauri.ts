import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// ─── Types ──────────────────────────────────────────────────────────────────

export type RiskLevel = 'Safe' | 'Low' | 'Medium' | 'High' | 'Critical' | 'Unknown'
export type ProcessCategory = 'System' | 'Security' | 'Browser' | 'Utility' | 'Telemetry' |
  'Bloatware' | 'Gaming' | 'Development' | 'Media' | 'Network' | 'Zombie' | 'Unknown'
export type Severity = 'Critical' | 'High' | 'Medium' | 'Low' | 'Info'
export type HealthGrade = 'Excellent' | 'Good' | 'Fair' | 'Poor' | 'Critical'

export interface ProcessEntry {
  pid: number; name: string; exe_path?: string
  cpu_usage: number; memory_bytes: number; status: string
  is_zombie: boolean; user?: string
  risk: RiskLevel; category: ProcessCategory
  description?: string; vendor?: string
  can_disable: boolean; is_telemetry: boolean; flags: string[]
}

export interface Finding {
  id: string; kind: string; severity: Severity
  title: string; description: string
  affected_item: string; recommendation: string
  can_auto_fix: boolean; timestamp: string
}

export interface FindingCounts {
  critical: number; high: number; medium: number; low: number; info: number
}

export interface HealthSnapshot {
  id: string; score: number; grade: HealthGrade
  cpu_usage: number; memory_used_pct: number
  process_count: number; finding_counts: FindingCounts
  uptime_seconds: number; timestamp: string
}

export interface SystemInfo {
  os_name: string; os_version: string; hostname: string
  cpu_brand: string; cpu_cores: number; cpu_freq_mhz: number
  total_memory_bytes: number; used_memory_bytes: number
  swap_total_bytes: number; swap_used_bytes: number
  uptime_seconds: number; boot_time: number
  cpu_usage_global: number; cpu_per_core: number[]
}

export interface DiskInfo {
  name: string; mount_point: string; kind: string
  total_bytes: number; available_bytes: number; file_system: string
}

export interface ThermalInfo {
  label: string; temperature_celsius: number; critical_threshold?: number
}

export interface NetworkStat {
  interface: string; bytes_received: number; bytes_transmitted: number
  packets_received: number; packets_transmitted: number
  errors_in: number; errors_out: number
}

export interface HardwareReport {
  system: SystemInfo; disks: DiskInfo[]
  temperatures: ThermalInfo[]; network: NetworkStat[]
}

export interface Recommendation {
  id: string; title: string; description: string
  action_kind: string; target: string
  risk_to_system: RiskLevel; effort: number; impact: number
  confirmed: boolean
}

export interface AppSettings {
  ollama_url: string; text_model: string
  auto_scan_on_startup: boolean; scan_interval_minutes: number
  show_safe_processes: boolean; keep_history_days: number
  cpu_spike_threshold: number; memory_high_threshold: number
  disk_warning_threshold: number; temp_warning_celsius: number
}

// ─── API ────────────────────────────────────────────────────────────────────

export const api = {
  runAnalysis:        ()                           => invoke<HealthSnapshot>('run_analysis'),
  getProcesses:       (showSafe: boolean)          => invoke<ProcessEntry[]>('get_processes', { showSafe }),
  getFindings:        ()                           => invoke<Finding[]>('get_findings'),
  getRecommendations: ()                           => invoke<Recommendation[]>('get_recommendations'),
  explainProcess:     (name: string, description: string | null, cpu: number, memoryMb: number) =>
    invoke<string>('explain_process', { name, description, cpu, memoryMb }),
  checkOllama:        ()                           => invoke<boolean>('check_ollama'),
  getLastSnapshot:    ()                           => invoke<HealthSnapshot | null>('get_last_snapshot'),
  getHardware:        ()                           => invoke<HardwareReport | null>('get_hardware'),
  getHistory:         (days?: number)              => invoke<HealthSnapshot[]>('get_history', { days }),
  cleanupHistory:     ()                           => invoke<number>('cleanup_history'),
  getSettings:        ()                           => invoke<AppSettings>('get_settings'),
  saveSettings:       (settings: AppSettings)      => invoke<void>('save_settings', { settings }),
}

// ─── Events ─────────────────────────────────────────────────────────────────

export const events = {
  onAnalysisStarted: (cb: () => void) =>
    listen('analysis://started', () => cb()),
  onAnalysisDone: (cb: (s: HealthSnapshot) => void) =>
    listen<HealthSnapshot>('analysis://done', e => cb(e.payload)),
}

// ─── Helpers ────────────────────────────────────────────────────────────────

export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 ** 2) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 ** 3) return `${(bytes / 1024 ** 2).toFixed(1)} MB`
  return `${(bytes / 1024 ** 3).toFixed(2)} GB`
}

export function severityColor(s: Severity): string {
  const m: Record<Severity, string> = {
    Critical: '#f85149', High: '#f0883e', Medium: '#d29922', Low: '#58a6ff', Info: '#8b949e',
  }
  return m[s]
}

export function riskColor(r: RiskLevel): string {
  const m: Record<RiskLevel, string> = {
    Safe: '#3fb950', Low: '#79c0ff', Medium: '#d29922', High: '#f0883e', Critical: '#f85149', Unknown: '#8b949e',
  }
  return m[r]
}

export function gradeColor(g: HealthGrade): string {
  const m: Record<HealthGrade, string> = {
    Excellent: '#3fb950', Good: '#79c0ff', Fair: '#d29922', Poor: '#f0883e', Critical: '#f85149',
  }
  return m[g]
}

export function gradeLabel(g: HealthGrade): string {
  const m: Record<HealthGrade, string> = {
    Excellent: 'Ausgezeichnet', Good: 'Gut', Fair: 'Akzeptabel', Poor: 'Schlecht', Critical: 'Kritisch',
  }
  return m[g]
}

export function uptimeHuman(seconds: number): string {
  const d = Math.floor(seconds / 86400)
  const h = Math.floor((seconds % 86400) / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  if (d > 0) return `${d}d ${h}h ${m}m`
  if (h > 0) return `${h}h ${m}m`
  return `${m}m`
}

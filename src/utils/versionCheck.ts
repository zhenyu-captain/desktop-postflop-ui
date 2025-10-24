import { invoke } from '@tauri-apps/api/tauri';

export interface VersionControlResponse {
  free: string;
}

export interface VersionCheckResult {
  is_shutdown: boolean;
  error?: string;
  response?: VersionControlResponse;
}

/**
 * 通过Rust后端检查版本控制信息
 * @returns Promise<VersionCheckResult>
 */
export async function checkVersionControl(): Promise<VersionCheckResult> {
  try {
    const result = await invoke<VersionCheckResult>('check_version_control_status');
    console.log('版本检查结果:', result);
    return result;
  } catch (error) {
    console.error('版本检查失败:', error);
    return {
      is_shutdown: false,
      error: error instanceof Error ? error.message : 'Unknown error'
    };
  }
}

/**
 * 检查是否应该显示停用提醒
 * 只有在成功获取到版本信息且free值为'shutdown'时才显示
 * @returns Promise<boolean>
 */
export async function shouldShowShutdownWarning(): Promise<boolean> {
  const result = await checkVersionControl();
  return result.is_shutdown && !result.error;
}

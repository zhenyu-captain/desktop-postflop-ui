import { invoke } from '@tauri-apps/api/tauri';

export interface NetworkCheckResult {
  is_online: boolean;
  error?: string;
  gtosim_com_status?: boolean;
  www_gtosim_com_status?: boolean;
}

/**
 * 通过Rust后端检查网络连接状态
 * @returns Promise<NetworkCheckResult>
 */
export async function checkNetworkConnectivity(): Promise<NetworkCheckResult> {
  try {
    const result = await invoke<NetworkCheckResult>('check_network_connectivity');
    console.log('网络检查结果:', result);
    return result;
  } catch (error) {
    console.error('网络检查失败:', error);
    return {
      is_online: false,
      error: error instanceof Error ? error.message : 'Unknown error',
      gtosim_com_status: false,
      www_gtosim_com_status: false,
    };
  }
}

/**
 * 检查是否应该显示网络检测弹窗
 * @returns Promise<boolean>
 */
export async function shouldShowNetworkCheck(): Promise<boolean> {
  const result = await checkNetworkConnectivity();
  return !result.is_online;
}

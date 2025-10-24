<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-300 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition duration-200 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="show"
        class="fixed inset-0 z-[10000] flex items-center justify-center bg-black bg-opacity-50 backdrop-blur-sm"
      >
        <Transition
          enter-active-class="transition duration-300 ease-out"
          enter-from-class="opacity-0 scale-95 translate-y-4"
          enter-to-class="opacity-100 scale-100 translate-y-0"
          leave-active-class="transition duration-200 ease-in"
          leave-from-class="opacity-100 scale-100 translate-y-0"
          leave-to-class="opacity-0 scale-95 translate-y-4"
        >
          <div
            v-if="show"
            class="bg-white rounded-xl shadow-2xl max-w-lg w-full mx-4 p-6 transform"
          >
            <!-- Header -->
            <div class="flex items-center mb-6">
              <div class="flex-shrink-0">
                <div class="w-12 h-12 bg-orange-100 rounded-full flex items-center justify-center">
                  <svg class="h-6 w-6 text-orange-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.236-3.905 14.141 0M1.394 9.393c5.857-5.857 15.355-5.857 21.213 0" />
                  </svg>
                </div>
              </div>
              <div class="ml-4">
                <h3 class="text-xl font-semibold text-gray-900">
                  Network Connection Check
                </h3>
                <p class="text-sm text-gray-500 mt-1">
                  Checking connectivity to gtosim.com
                </p>
              </div>
            </div>

            <!-- Content -->
            <div class="mb-8">
              <p class="text-gray-700 mb-6 leading-relaxed">
                Unable to connect to gtosim.com servers. Please check your internet connection and try again. The application will remain blocked until a successful connection is established.
              </p>
              
              <!-- Connection Status -->
              <div class="bg-gray-50 border border-gray-200 rounded-lg p-4 mb-4">
                <h4 class="text-sm font-medium text-gray-900 mb-3">Connection Status</h4>
                <div class="space-y-2">
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">gtosim.com</span>
                    <div class="flex items-center">
                      <div :class="[
                        'w-2 h-2 rounded-full mr-2',
                        gtosimComStatus ? 'bg-green-500' : 'bg-red-500'
                      ]"></div>
                      <span :class="[
                        'text-sm font-medium',
                        gtosimComStatus ? 'text-green-700' : 'text-red-700'
                      ]">
                        {{ gtosimComStatus ? 'Connected' : 'Failed' }}
                      </span>
                    </div>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">www.gtosim.com</span>
                    <div class="flex items-center">
                      <div :class="[
                        'w-2 h-2 rounded-full mr-2',
                        wwwGtosimComStatus ? 'bg-green-500' : 'bg-red-500'
                      ]"></div>
                      <span :class="[
                        'text-sm font-medium',
                        wwwGtosimComStatus ? 'text-green-700' : 'text-red-700'
                      ]">
                        {{ wwwGtosimComStatus ? 'Connected' : 'Failed' }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Error Details -->
              <div v-if="error" class="bg-red-50 border border-red-200 rounded-lg p-4">
                <div class="flex items-start">
                  <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-red-400 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                    </svg>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm font-medium text-red-800">
                      Connection Error
                    </p>
                    <p class="text-sm text-red-700 mt-1">
                      {{ error }}
                    </p>
                  </div>
                </div>
              </div>

              <!-- Instructions -->
              <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
                <div class="flex items-start">
                  <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-blue-400 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                    </svg>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm font-medium text-blue-800">
                      Troubleshooting Tips
                    </p>
                    <ul class="text-sm text-blue-700 mt-1 list-disc list-inside space-y-1">
                      <li>Check your internet connection</li>
                      <li>Verify firewall settings</li>
                      <li>Try using a different network</li>
                      <li>Contact your network administrator</li>
                    </ul>
                  </div>
                </div>
              </div>
            </div>

            <!-- Buttons -->
            <div class="flex justify-end space-x-3">
              <button
                @click="retryConnection"
                :disabled="isRetrying"
                class="px-6 py-2.5 text-sm font-medium text-white bg-gradient-to-r from-blue-600 to-blue-700 border border-transparent rounded-lg hover:from-blue-700 hover:to-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-all duration-200 shadow-lg hover:shadow-xl disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <span v-if="isRetrying" class="flex items-center">
                  <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  Retrying...
                </span>
                <span v-else>Retry Connection</span>
              </button>
              <button
                @click="exitApplication"
                class="px-6 py-2.5 text-sm font-medium text-white bg-gradient-to-r from-red-600 to-red-700 border border-transparent rounded-lg hover:from-red-700 hover:to-red-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 transition-all duration-200 shadow-lg hover:shadow-xl"
              >
                Exit Application
              </button>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const props = defineProps<{
  show: boolean;
  gtosimComStatus?: boolean;
  wwwGtosimComStatus?: boolean;
  error?: string;
}>();

const emit = defineEmits<{
  (event: 'retry'): void;
}>();

const isRetrying = ref(false);

const retryConnection = async () => {
  isRetrying.value = true;
  emit('retry');
  // Reset retrying state after a delay
  setTimeout(() => {
    isRetrying.value = false;
  }, 3000); // 增加延迟时间，给用户更好的反馈
};

const exitApplication = async () => {
  try {
    await invoke('exit_application');
  } catch (error) {
    console.error('Exit application failed:', error);
    // 退出失败时不做任何操作，保持弹窗显示
  }
};
</script>

<style scoped>
/* Ensure modal is on top */
.fixed {
  z-index: 10000;
}
</style>

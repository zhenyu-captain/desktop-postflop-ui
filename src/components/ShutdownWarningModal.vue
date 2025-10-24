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
        class="fixed inset-0 z-[9999] flex items-center justify-center bg-black bg-opacity-50 backdrop-blur-sm"
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
            class="bg-white rounded-xl shadow-2xl max-w-md w-full mx-4 p-6 transform"
          >
            <!-- 标题 -->
            <div class="flex items-center mb-6">
              <div class="flex-shrink-0">
                <div class="w-12 h-12 bg-red-100 rounded-full flex items-center justify-center">
                  <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                  </svg>
                </div>
              </div>
              <div class="ml-4">
                <h3 class="text-xl font-semibold text-gray-900">
                  Free Version Disabled
                </h3>
                <p class="text-sm text-gray-500 mt-1">
                  Application will exit, please upgrade to the latest version
                </p>
              </div>
            </div>

            <!-- Content -->
            <div class="mb-8">
              <p class="text-gray-700 mb-6 leading-relaxed">
                The current free version has been disabled and cannot be used. Please visit the official website to download the latest version, or click "Exit Application" to close the program.
              </p>
              
              <div class="bg-gradient-to-r from-yellow-50 to-orange-50 border border-yellow-200 rounded-lg p-4">
                <div class="flex items-start">
                  <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-yellow-500 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                    </svg>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm font-medium text-yellow-800">
                      Important Notice
                    </p>
                    <p class="text-sm text-yellow-700 mt-1">
                      The free version has been disabled. You must upgrade to the latest version to continue using the application.
                    </p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Buttons -->
            <div class="flex justify-end space-x-3">
              <button
                @click="openOfficialWebsite"
                class="px-6 py-2.5 text-sm font-medium text-white bg-gradient-to-r from-blue-600 to-blue-700 border border-transparent rounded-lg hover:from-blue-700 hover:to-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-all duration-200 shadow-lg hover:shadow-xl"
              >
                Visit Website
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
import { invoke } from '@tauri-apps/api/tauri';

const props = defineProps<{
  show: boolean;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
}>();

const closeModal = () => {
  emit('close');
};

const openOfficialWebsite = () => {
  // 打开官方网站
  window.open('https://gtosim.com', '_blank');
  // 不关闭弹窗，因为用户必须退出应用
};

const exitApplication = async () => {
  try {
    await invoke('exit_application');
  } catch (error) {
    console.error('退出应用失败:', error);
    // 如果退出失败，至少关闭弹窗
    closeModal();
  }
};
</script>

<style scoped>
/* 确保弹窗在最顶层 */
.fixed {
  z-index: 9999;
}
</style>

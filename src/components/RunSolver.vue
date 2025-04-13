<template>
  <div class="solver-container p-5 rounded-xl bg-gray-50 dark:bg-gray-800 shadow-md transition-all duration-300">
    <!-- Configuration Card -->
    <div class="config-card bg-white dark:bg-gray-700 rounded-lg shadow p-4 mb-4 transition-all hover:shadow-md">
      <h3 class="text-lg font-semibold mb-3 text-gray-800 dark:text-gray-200 flex items-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-500" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
        </svg>
        Configuration
      </h3>
      
      <div class="flex my-2 items-center">
        <div class="font-medium text-gray-700 dark:text-gray-300">Number of threads:</div>
        <input
          v-model="numThreads"
          type="number"
          :class="
            'w-20 ml-3 px-3 py-2 rounded-lg text-sm text-center transition-all border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 focus:outline-none dark:bg-gray-800 dark:text-gray-200 ' +
            (numThreads < 1 || numThreads > 64 || numThreads % 1 !== 0
              ? 'border-red-500 focus:ring-red-500 focus:border-red-500'
              : '')
          "
          min="1"
          max="64"
        />
        <button
          class="ml-4 inline-flex items-center px-4 py-2 rounded-lg text-sm font-medium transition-all shadow-sm 
            bg-blue-600 text-white hover:bg-blue-700 hover:shadow focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
            disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:shadow-none"
          :disabled="isTreeBuilding || store.isSolverRunning || store.isFinalizing"
          @click="buildTree"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1.5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M3.293 6.293a1 1 0 011.414 0L10 11.586l5.293-5.293a1 1 0 111.414 1.414l-6 6a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
          Build New Tree
        </button>
        
        <!-- 添加调试按钮 -->
        <button
          class="ml-4 inline-flex items-center px-4 py-2 rounded-lg text-sm font-medium transition-all shadow-sm 
            bg-yellow-500 text-white hover:bg-yellow-600 hover:shadow focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-yellow-500"
          @click="showDebugInfo"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1.5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M11 3a1 1 0 10-2 0v1a1 1 0 102 0V3zM6.293 6.707a1 1 0 010-1.414l.7-.7a1 1 0 111.414 1.414l-.7.7a1 1 0 01-1.414 0zM13.707 6.707a1 1 0 010-1.414l.7-.7a1 1 0 111.414 1.414l-.7.7a1 1 0 01-1.414 0zM4 10a1 1 0 100 2h1a1 1 0 100-2H4zM15 10a1 1 0 100 2h1a1 1 0 100-2h-1zM6.293 13.293a1 1 0 011.414 0l.7.7a1 1 0 11-1.414 1.414l-.7-.7a1 1 0 010-1.414zM13.707 13.293a1 1 0 011.414 0l.7.7a1 1 0 11-1.414 1.414l-.7-.7a1 1 0 010-1.414zM10 16a1 1 0 00-1 1v1a1 1 0 102 0v-1a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
          Debug
        </button>
      </div>

      <!-- 调试信息区域 -->
      <div v-if="showDebug" class="debug-info mt-4 p-4 rounded-lg bg-gray-100 dark:bg-gray-800 overflow-auto max-h-[500px]">
        <h4 class="font-semibold text-lg mb-2 text-gray-800 dark:text-gray-200">Debug Information</h4>
        <div class="space-y-2">
          <div class="border-b pb-2 dark:border-gray-700">
            <p class="font-medium text-gray-800 dark:text-gray-200">Board Configuration:</p>
            <pre class="bg-white dark:bg-gray-900 p-2 rounded mt-1 text-xs overflow-auto">{{ debugBoardInfo }}</pre>
          </div>
          <div class="border-b pb-2 dark:border-gray-700">
            <p class="font-medium text-gray-800 dark:text-gray-200">Range Configuration:</p>
            <pre class="bg-white dark:bg-gray-900 p-2 rounded mt-1 text-xs overflow-auto">{{ debugRangeInfo }}</pre>
          </div>
          <div class="border-b pb-2 dark:border-gray-700">
            <p class="font-medium text-gray-800 dark:text-gray-200">Betting Configuration:</p>
            <pre class="bg-white dark:bg-gray-900 p-2 rounded mt-1 text-xs overflow-auto">{{ debugBettingInfo }}</pre>
          </div>
          <div class="border-b pb-2 dark:border-gray-700">
            <p class="font-medium text-gray-800 dark:text-gray-200">Tree Configuration:</p>
            <pre class="bg-white dark:bg-gray-900 p-2 rounded mt-1 text-xs overflow-auto">{{ debugTreeInfo }}</pre>
          </div>
          <div>
            <p class="font-medium text-gray-800 dark:text-gray-200">Validation Result:</p>
            <pre class="bg-white dark:bg-gray-900 p-2 rounded mt-1 text-xs overflow-auto" :class="{'text-red-500': debugValidationError}">{{ debugValidationResult }}</pre>
          </div>
        </div>
        <button 
          class="mt-4 px-4 py-2 rounded-lg bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
          @click="showDebug = false"
        >
          Close Debug Info
        </button>
      </div>

      <div class="status-indicator my-2 py-1.5 px-3 rounded-lg bg-gray-100 dark:bg-gray-600 text-gray-700 dark:text-gray-300 flex items-center">
        <div class="w-2 h-2 rounded-full mr-2" :class="{
          'bg-green-500': isTreeBuilt && !store.isSolverError,
          'bg-yellow-500': isTreeBuilding,
          'bg-red-500': !isTreeBuilt || store.isSolverError
        }"></div>
        Status: <span class="font-medium ml-1" :class="{'text-green-600 dark:text-green-400': isTreeBuilt && !store.isSolverError}">{{ treeStatus }}</span>
      </div>
    </div>

    <div v-if="isTreeBuilt" class="tree-settings-card bg-white dark:bg-gray-700 rounded-lg shadow p-4 mb-4 transition-all hover:shadow-md">
      <h3 class="text-lg font-semibold mb-3 text-gray-800 dark:text-gray-200 flex items-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-500" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd" />
        </svg>
        Calculation Settings
      </h3>
      
      <div class="precision-section mb-5 bg-gray-50 dark:bg-gray-800 p-3 rounded-lg">
        <div class="flex items-center mb-2">
          <div class="font-medium text-gray-700 dark:text-gray-300">Precision mode:</div>
          <Tippy
            class="inline-block cursor-help ml-2"
            max-width="500px"
            placement="bottom"
            trigger="mouseenter click"
            :delay="[200, 0]"
            :interactive="true"
          >
            <QuestionMarkCircleIcon class="w-5 h-5 text-blue-500 dark:text-blue-400 hover:text-blue-600 transition-colors" />
            <template #content>
              <div class="px-3 py-2 text-justify bg-white dark:bg-gray-800 shadow-xl rounded-lg border border-gray-200 dark:border-gray-700">
                <div class="font-medium text-gray-800 dark:text-gray-200 mb-2">Precision mode mainly affects memory usage, but it also has several other effects:</div>
                <ul class="pl-6 list-disc">
                  <li class="mt-1 text-gray-700 dark:text-gray-300">
                    <span class="font-medium">Memory efficiency:</span> 16-bit integer > 32-bit FP. The 16-bit integer mode uses only about half the memory of the 32-bit FP mode.
                  </li>
                  <li class="mt-1 text-gray-700 dark:text-gray-300">
                    <span class="font-medium">Performance:</span> Depends on the environment. The 32-bit FP mode requires fewer execution instructions than the 16-bit integer mode. However, memory reads and writes can become a bottleneck in highly multithreaded environments, and in such situations the 16-bit integer mode may provide better performance.
                  </li>
                  <li class="mt-1 text-gray-700 dark:text-gray-300">
                    <span class="font-medium">Significant digits:</span> 32-bit FP (about 7 digits) > 16-bit integer (about 4 digits). The 16-bit integer mode is not suitable for satisfying an exploitability target of less than 0.1%.
                  </li>
                </ul>
              </div>
            </template>
          </Tippy>
        </div>
        
        <div class="precision-options space-y-2 mt-3">
          <label :class="{ 'cursor-pointer': !store.hasSolverRun, 'opacity-75': store.hasSolverRun }" class="flex items-center p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
            <input
              v-model="isCompressionEnabled"
              class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 cursor-pointer disabled:cursor-default"
              type="radio"
              name="compression"
              :value="false"
              :disabled="store.hasSolverRun"
            />
            <div class="ml-3 flex items-center">
              <span class="font-medium text-gray-800 dark:text-gray-200">32-bit FP:</span>
              <span class="ml-2 text-gray-600 dark:text-gray-400">
                needs
                <span class="font-medium">
                {{
                  memoryUsageRaw >= 1023.5 * 1024 * 1024
                    ? (memoryUsageRaw / (1024 * 1024 * 1024)).toFixed(2) + "GB"
                    : (memoryUsageRaw / (1024 * 1024)).toFixed(0) + "MB"
                }}
                </span>
                RAM
              </span>
              <span v-if="memoryUsageRaw > maxMemoryUsage" class="ml-2 text-red-500 font-medium inline-flex items-center">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                (out of memory)
              </span>
            </div>
          </label>
          
          <label :class="{ 'cursor-pointer': !store.hasSolverRun, 'opacity-75': store.hasSolverRun }" class="flex items-center p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
            <input
              v-model="isCompressionEnabled"
              class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 cursor-pointer disabled:cursor-default"
              type="radio"
              name="compression"
              :value="true"
              :disabled="store.hasSolverRun"
            />
            <div class="ml-3 flex items-center">
              <span class="font-medium text-gray-800 dark:text-gray-200">16-bit integer:</span>
              <span class="ml-2 text-gray-600 dark:text-gray-400">
                needs
                <span class="font-medium">
                {{
                  memoryUsageRawCompressed >= 1023.5 * 1024 * 1024
                    ? (memoryUsageRawCompressed / (1024 * 1024 * 1024)).toFixed(2) + "GB"
                    : (memoryUsageRawCompressed / (1024 * 1024)).toFixed(0) + "MB"
                }}
                </span>
                RAM
              </span>
              <span v-if="memoryUsageRawCompressed > maxMemoryUsage" class="ml-2 text-red-500 font-medium inline-flex items-center">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                (out of memory)
              </span>
            </div>
          </label>
        </div>
        
        <div
          v-if="store.isBunchingEnabled && store.bunchingFlop.length > 0"
          class="mt-3 text-gray-600 dark:text-gray-400 p-2 border-t border-gray-200 dark:border-gray-600"
        >
          Additional RAM usage for bunching effect:
          <span class="font-medium">
            {{
              memoryUsageBunching >= 1023.5 * 1024 * 1024
                ? (memoryUsageBunching / (1024 * 1024 * 1024)).toFixed(2) + "GB"
                : (memoryUsageBunching / (1024 * 1024)).toFixed(0) + "MB"
            }}
          </span>
        </div>
        
        <div
          v-if="memoryUsage > maxMemoryUsage && osName !== 'macos'"
          class="mt-3 text-gray-600 dark:text-gray-400 p-2 border-t border-gray-200 dark:border-gray-600"
        >
          Available RAM:
          <span class="font-medium">{{ (availableMemory / (1024 * 1024 * 1024)).toFixed(2) }}GB</span> /
          <span>{{ (totalMemory / (1024 * 1024 * 1024)).toFixed() }}GB total</span>
        </div>
        
        <div
          v-if="memoryUsage > maxMemoryUsage && osName === 'macos'"
          class="mt-3 text-gray-600 dark:text-gray-400 p-2 border-t border-gray-200 dark:border-gray-600"
        >
          RAM limit: <span class="font-medium">{{ (maxMemoryUsage / (1024 * 1024 * 1024)).toFixed(1) }}GB</span> 
          (= 70% * {{ (totalMemory / (1024 * 1024 * 1024)).toFixed() }}GB total)
        </div>
      </div>

      <div class="solver-params grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="param-group bg-gray-50 dark:bg-gray-800 p-3 rounded-lg">
          <div class="flex items-center">
            <div class="font-medium text-gray-700 dark:text-gray-300">Target exploitability:</div>
            <Tippy
              class="inline-block cursor-help ml-2"
              max-width="500px"
              placement="bottom"
              trigger="mouseenter click"
              :delay="[200, 0]"
              :interactive="true"
            >
              <QuestionMarkCircleIcon class="w-5 h-5 text-blue-500 dark:text-blue-400 hover:text-blue-600 transition-colors" />
              <template #content>
                <div class="px-3 py-2 text-justify bg-white dark:bg-gray-800 shadow-xl rounded-lg border border-gray-200 dark:border-gray-700">
                  <div class="text-gray-700 dark:text-gray-300">
                    Specifies the acceptable distance to the Nash equilibrium. A lower value gives more accurate results, but also requires more computation time.
                  </div>
                  <div class="mt-3 text-gray-700 dark:text-gray-300">
                    <span class="font-medium underline text-gray-800 dark:text-gray-200">A more detailed description:</span>
                    When a Nash equilibrium solution is obtained, the strategies of both players become MESs (Maximally Exploitative Strategies) to each other. Using this property, we define the distance to the Nash equilibrium of the obtained strategy as follows:
                  </div>
                  <div class="my-2 text-center font-medium text-gray-800 dark:text-gray-200">
                    Distance = (Opponent's MES EV) - (Opponent's obtained EV)
                  </div>
                  <div class="text-gray-700 dark:text-gray-300">
                    This distance is always non-negative and is zero if and only if the obtained strategy is a part of a particular Nash equilibrium. Exploitability is defined as the average distance of both players.
                  </div>
                </div>
              </template>
            </Tippy>
          </div>
          <div class="mt-2 flex items-center">
            <input
              v-model="targetExploitability"
              type="number"
              :class="
                'w-24 px-3 py-2 rounded-lg text-sm text-center transition-all border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 focus:outline-none dark:bg-gray-800 dark:text-gray-200 ' +
                (targetExploitability <= 0 ? 'border-red-500 focus:ring-red-500 focus:border-red-500' : '')
              "
              :disabled="store.hasSolverRun && !store.isSolverPaused"
              min="0"
              step="0.05"
            />
            <span class="ml-2 text-gray-700 dark:text-gray-300">%</span>
          </div>
        </div>

        <div class="param-group bg-gray-50 dark:bg-gray-800 p-3 rounded-lg">
          <div class="font-medium text-gray-700 dark:text-gray-300">Maximum number of iterations:</div>
          <div class="mt-2">
            <input
              v-model="maxIterations"
              type="number"
              :class="
                'w-full px-3 py-2 rounded-lg text-sm text-center transition-all border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 focus:outline-none dark:bg-gray-800 dark:text-gray-200 ' +
                (maxIterations < 0 ||
                maxIterations % 1 !== 0 ||
                maxIterations > 100000
                  ? 'border-red-500 focus:ring-red-500 focus:border-red-500'
                  : '')
              "
              :disabled="store.hasSolverRun && !store.isSolverPaused"
              min="0"
              max="100000"
            />
          </div>
        </div>
      </div>

      <div
        v-if="!areFlopMatching && !store.hasSolverRun"
        class="mt-4 p-3 bg-red-100 text-red-700 rounded-lg font-medium flex items-center"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
        </svg>
        Error: The input flop and the bunching data do not match.
      </div>

      <div
        v-if="
          store.isBunchingEnabled &&
          store.bunchingFlop.length === 0 &&
          !store.hasSolverRun
        "
        class="mt-4 p-3 bg-yellow-100 text-yellow-700 rounded-lg font-medium flex items-center"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
        </svg>
        Warning: Bunching effect is disabled because the data is not ready.
      </div>

      <div class="flex mt-6 gap-3 flex-wrap">
        <button
          class="flex items-center justify-center px-4 py-2 min-w-[120px] rounded-lg font-medium transition-all shadow-sm hover:shadow
            bg-blue-600 text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
            disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:shadow-none"
          :disabled="
            store.hasSolverRun ||
            store.isBunchingRunning ||
            memoryUsageSelected > maxMemoryUsage ||
            !areFlopMatching ||
            numThreads < 1 ||
            numThreads > 64 ||
            numThreads % 1 !== 0 ||
            targetExploitability <= 0 ||
            maxIterations < 0 ||
            maxIterations % 1 !== 0 ||
            maxIterations > 100000
          "
          @click="runSolver"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
          </svg>
          Run Solver
        </button>
        <button
          class="flex items-center justify-center px-4 py-2 min-w-[120px] rounded-lg font-medium transition-all shadow-sm hover:shadow
            bg-red-600 text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500
            disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:shadow-none"
          :disabled="!store.isSolverRunning"
          @click="() => (terminateFlag = true)"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 001 1h4a1 1 0 001-1V8a1 1 0 00-1-1H8z" clip-rule="evenodd" />
          </svg>
          Stop
        </button>
        <button
          v-if="!store.isSolverPaused"
          class="flex items-center justify-center px-4 py-2 min-w-[120px] rounded-lg font-medium transition-all shadow-sm hover:shadow
            bg-green-600 text-white hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500
            disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:shadow-none"
          :disabled="!store.isSolverRunning"
          @click="() => (pauseFlag = true)"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
          Pause
        </button>
        <button
          v-else
          class="flex items-center justify-center px-4 py-2 min-w-[120px] rounded-lg font-medium transition-all shadow-sm hover:shadow
            bg-green-600 text-white hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500
            disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:shadow-none"
          :disabled="
            store.isBunchingRunning ||
            numThreads < 1 ||
            numThreads > 64 ||
            numThreads % 1 !== 0 ||
            targetExploitability <= 0 ||
            maxIterations < 0 ||
            maxIterations % 1 !== 0 ||
            maxIterations > 100000
          "
          @click="resumeSolver"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
          </svg>
          Resume
        </button>
      </div>

      <div
        v-if="memoryUsage > maxMemoryUsage && !store.hasSolverRun"
        class="mt-6"
      >
        <button
          class="flex items-center justify-center px-4 py-2 rounded-lg font-medium transition-all shadow-sm hover:shadow
            bg-red-100 text-red-700 hover:bg-red-200 border border-red-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
          @click="maxMemoryUsage = Number.POSITIVE_INFINITY"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
          Ignore RAM Limit (not recommended)
        </button>
      </div>
    </div>

    <div v-if="store.hasSolverRun" class="solver-status-card bg-white dark:bg-gray-700 rounded-lg shadow p-4 transition-all hover:shadow-md">
      <h3 class="text-lg font-semibold mb-3 text-gray-800 dark:text-gray-200 flex items-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-500" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
        </svg>
        Solver Status
      </h3>
      
      <div class="status-section p-4 rounded-lg transition-all duration-300" :class="{
        'bg-blue-50 dark:bg-blue-900/30': store.isSolverRunning || store.isFinalizing,
        'bg-yellow-50 dark:bg-yellow-900/30': store.isSolverPaused, 
        'bg-red-50 dark:bg-red-900/30': store.isSolverError,
        'bg-green-50 dark:bg-green-900/30': !store.isSolverRunning && !store.isFinalizing && !store.isSolverPaused && !store.isSolverError
      }">
        <div class="flex items-center mb-3">
          <span
            v-if="store.isSolverRunning || store.isFinalizing"
            class="spinner inline-block mr-3"
          ></span>
          <svg v-else-if="store.isSolverPaused" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-3 text-yellow-500" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
          <svg v-else-if="store.isSolverError" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-3 text-red-500" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
          <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-3 text-green-500" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
          <span class="font-medium text-lg" :class="{
            'text-blue-700 dark:text-blue-300': store.isSolverRunning || store.isFinalizing,
            'text-yellow-700 dark:text-yellow-300': store.isSolverPaused,
            'text-red-700 dark:text-red-300': store.isSolverError,
            'text-green-700 dark:text-green-300': !store.isSolverRunning && !store.isFinalizing && !store.isSolverPaused && !store.isSolverError
          }">
            {{
              store.isSolverRunning
                ? "Solver running..."
                : store.isFinalizing
                ? "Finalizing..."
                : store.isSolverPaused
                ? "Solver paused."
                : store.isSolverError
                ? solverErrorText
                : "Solver finished."
            }}
          </span>
        </div>
        
        <div class="solver-info space-y-2 pl-8">
          <div v-if="currentIteration.value !== -1" class="text-gray-700 dark:text-gray-300 bg-white/50 dark:bg-black/20 p-2 rounded">
            <span class="font-medium">{{ iterationText }}</span>
          </div>
          <div v-if="Number.isFinite(exploitability.value)" class="text-gray-700 dark:text-gray-300 bg-white/50 dark:bg-black/20 p-2 rounded">
            <span class="font-medium">{{ exploitabilityText }}</span>
          </div>
          <div v-if="elapsedTimeMs.value !== -1 && store.isSolverFinished" class="text-gray-700 dark:text-gray-300 bg-white/50 dark:bg-black/20 p-2 rounded">
            <span class="font-medium">{{ timeText }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from "vue";
import {
  useStore,
  useConfigStore,
  useTmpConfigStore,
  useSavedConfigStore,
  saveConfigTmp,
  saveConfig,
} from "../store";
import {
  MAX_AMOUNT,
  convertBetString,
  ROOT_LINE_STRING,
  INVALID_LINE_STRING,
  readableLineString,
} from "../utils";
import * as invokes from "../invokes";

import { Tippy } from "vue-tippy";
import { QuestionMarkCircleIcon } from "@heroicons/vue/20/solid";

const checkConfig = (
  config: ReturnType<typeof useConfigStore>
): string | null => {
  if (config.board.length < 3) {
    return "Board must consist of at least three cards";
  }

  if (config.startingPot <= 0) {
    return "Starting pot must be positive";
  }

  if (config.startingPot > MAX_AMOUNT) {
    return "Starting pot is too large";
  }

  if (config.startingPot % 1 !== 0) {
    return "Starting pot must be an integer";
  }

  if (config.effectiveStack <= 0) {
    return "Effective stack must be positive";
  }

  if (config.effectiveStack > MAX_AMOUNT) {
    return "Effective stack is too large";
  }

  if (config.effectiveStack % 1 !== 0) {
    return "Effective stack is be an integer";
  }

  const betConfig = [
    { s: config.oopFlopBetSanitized, kind: "OOP flop bet" },
    { s: config.oopFlopRaiseSanitized, kind: "OOP flop raise" },
    { s: config.oopTurnBetSanitized, kind: "OOP turn bet" },
    { s: config.oopTurnRaiseSanitized, kind: "OOP turn raise" },
    { s: config.oopRiverBetSanitized, kind: "OOP river bet" },
    { s: config.oopRiverRaiseSanitized, kind: "OOP river raise" },
    { s: config.ipFlopBetSanitized, kind: "IP flop bet" },
    { s: config.ipFlopRaiseSanitized, kind: "IP flop raise" },
    { s: config.ipTurnBetSanitized, kind: "IP turn bet" },
    { s: config.ipTurnRaiseSanitized, kind: "IP turn raise" },
    { s: config.ipRiverBetSanitized, kind: "IP river bet" },
    { s: config.ipRiverRaiseSanitized, kind: "IP river raise" },
  ];

  for (const { s, kind } of betConfig) {
    if (!s.valid) {
      return `${kind}: ${s.s}`;
    }
  }

  if (config.donkOption) {
    if (!config.oopTurnDonkSanitized.valid) {
      return `OOP turn donk: ${config.oopTurnDonkSanitized.s}`;
    }
    if (!config.oopRiverDonkSanitized.valid) {
      return `OOP river donk: ${config.oopRiverDonkSanitized.s}`;
    }
  }

  if (config.addAllInThreshold < 0) {
    return "Invalid add all-in threshold";
  }

  if (config.forceAllInThreshold < 0) {
    return "Invalid force all-in threshold";
  }

  if (config.mergingThreshold < 0) {
    return "Invalid merging threshold";
  }

  if (
    config.expectedBoardLength > 0 &&
    config.board.length !== config.expectedBoardLength
  ) {
    return `Invalid board (expected ${config.expectedBoardLength} cards)`;
  }

  const addedLinesArray =
    config.addedLines === ""
      ? []
      : config.addedLines.split(",").map(readableLineString);

  const removedLinesArray =
    config.removedLines === ""
      ? []
      : config.removedLines.split(",").map(readableLineString);

  if (
    addedLinesArray.includes(ROOT_LINE_STRING) ||
    addedLinesArray.includes(INVALID_LINE_STRING) ||
    removedLinesArray.includes(ROOT_LINE_STRING) ||
    removedLinesArray.includes(INVALID_LINE_STRING)
  ) {
    return "Invalid line found (loaded broken configurations?)";
  }

  if (
    ![0, 3, 4, 5].includes(config.expectedBoardLength) ||
    (config.expectedBoardLength === 0 &&
      (addedLinesArray.length > 0 || removedLinesArray.length > 0)) ||
    (config.expectedBoardLength > 0 &&
      addedLinesArray.length === 0 &&
      removedLinesArray.length === 0)
  ) {
    return "Invalid configurations (loaded broken configurations?)";
  }

  return null;
};

const store = useStore();
const config = useConfigStore();
const tmpConfig = useTmpConfigStore();
const savedConfig = useSavedConfigStore();

// 添加调试相关的响应式变量
const showDebug = ref(false);
const debugBoardInfo = ref('');
const debugRangeInfo = ref('');
const debugBettingInfo = ref('');
const debugTreeInfo = ref('');
const debugValidationResult = ref('');
const debugValidationError = ref(false);

const numThreads = ref(navigator.hardwareConcurrency || 1);
const targetExploitability = ref(0.3);
const maxIterations = ref(1000);

const isTreeBuilding = ref(false);
const isTreeBuilt = ref(false);
const treeStatus = ref("Module not loaded");
const memoryUsageRaw = ref(0);
const memoryUsageRawCompressed = ref(0);
const memoryUsageBunching = ref(0);
const osName = ref<Awaited<ReturnType<typeof invokes.osName>> | null>(null);
const maxMemoryUsage = ref(0);
const availableMemory = ref(0);
const totalMemory = ref(0);
const isCompressionEnabled = ref(false);
const solverErrorText = ref("");
const terminateFlag = ref(false);
const pauseFlag = ref(false);
const currentIteration = ref(-1);
const exploitability = ref(Number.POSITIVE_INFINITY);
const elapsedTimeMs = ref(-1);

let startTime = 0;
let exploitabilityUpdated = false;

const memoryUsage = computed(() => {
  if (store.isBunchingEnabled && store.bunchingFlop.length > 0) {
    return memoryUsageRaw.value + memoryUsageBunching.value;
  } else {
    return memoryUsageRaw.value;
  }
});

const memoryUsageCompressed = computed(() => {
  if (store.isBunchingEnabled && store.bunchingFlop.length > 0) {
    return memoryUsageRawCompressed.value + memoryUsageBunching.value;
  } else {
    return memoryUsageRawCompressed.value;
  }
});

const memoryUsageSelected = computed(() => {
  if (isCompressionEnabled.value) {
    return memoryUsageCompressed.value;
  } else {
    return memoryUsage.value;
  }
});

const areFlopMatching = computed(() => {
  const flop = savedConfig.board.slice(0, 3);
  const bunchingFlop = store.bunchingFlop;
  return (
    !store.isBunchingEnabled ||
    bunchingFlop.length === 0 ||
    (flop[0] === bunchingFlop[0] &&
      flop[1] === bunchingFlop[1] &&
      flop[2] === bunchingFlop[2])
  );
});

const iterationText = computed(() => {
  if (currentIteration.value === -1) {
    return "Allocating memory...";
  } else if (currentIteration.value === -2) {
    return "Collecting bunching data...";
  } else {
    return `Iteration: ${currentIteration.value}`;
  }
});

const exploitabilityText = computed(() => {
  if (!Number.isFinite(exploitability.value)) {
    return "";
  } else {
    const valueText = exploitability.value.toFixed(2);
    const percent = (exploitability.value * 100) / config.startingPot;
    const percentText = `${percent.toFixed(2)}%`;
    return `Exploitability: ${valueText} (${percentText})`;
  }
});

const timeText = computed(() => {
  if (elapsedTimeMs.value === -1 || !store.isSolverFinished) {
    return "";
  } else {
    return `Time: ${(elapsedTimeMs.value / 1000).toFixed(2)}s`;
  }
});

const buildTree = async () => {
  isTreeBuilt.value = false;

  const configError = checkConfig(config);
  if (configError !== null) {
    treeStatus.value = `Error: ${configError}`;
    return;
  }

  saveConfigTmp();
  isTreeBuilding.value = true;
  treeStatus.value = "Building tree...";

  const errorString = await invokes.gameInit(
    tmpConfig.board,
    tmpConfig.startingPot,
    tmpConfig.effectiveStack,
    tmpConfig.rakePercent / 100,
    tmpConfig.rakeCap,
    tmpConfig.donkOption,
    convertBetString(tmpConfig.oopFlopBet),
    convertBetString(tmpConfig.oopFlopRaise),
    convertBetString(tmpConfig.oopTurnBet),
    convertBetString(tmpConfig.oopTurnRaise),
    tmpConfig.donkOption ? convertBetString(tmpConfig.oopTurnDonk) : "",
    convertBetString(tmpConfig.oopRiverBet),
    convertBetString(tmpConfig.oopRiverRaise),
    tmpConfig.donkOption ? convertBetString(tmpConfig.oopRiverDonk) : "",
    convertBetString(tmpConfig.ipFlopBet),
    convertBetString(tmpConfig.ipFlopRaise),
    convertBetString(tmpConfig.ipTurnBet),
    convertBetString(tmpConfig.ipTurnRaise),
    convertBetString(tmpConfig.ipRiverBet),
    convertBetString(tmpConfig.ipRiverRaise),
    tmpConfig.addAllInThreshold / 100,
    tmpConfig.forceAllInThreshold / 100,
    tmpConfig.mergingThreshold / 100,
    tmpConfig.addedLines,
    tmpConfig.removedLines
  );

  if (errorString) {
    isTreeBuilding.value = false;
    treeStatus.value = "Error: " + errorString;
    return;
  }

  saveConfig();

  [memoryUsageRaw.value, memoryUsageRawCompressed.value] =
    await invokes.gameMemoryUsage();
  memoryUsageBunching.value = await invokes.gameMemoryUsageBunching();

  osName.value = await invokes.osName();
  [availableMemory.value, totalMemory.value] = await invokes.memory();

  if (osName.value === "macos") {
    // available memory is not useful on macOS
    maxMemoryUsage.value = totalMemory.value * 0.7;
  } else {
    maxMemoryUsage.value = availableMemory.value;
  }

  if (
    memoryUsage.value > maxMemoryUsage.value &&
    memoryUsageCompressed.value <= maxMemoryUsage.value
  ) {
    isCompressionEnabled.value = true;
  }

  isTreeBuilding.value = false;
  isTreeBuilt.value = true;
  treeStatus.value = "Successfully built tree";

  store.isSolverRunning = false;
  store.isSolverPaused = false;
  store.isSolverFinished = false;
  store.isSolverError = false;
};

const runSolver = async () => {
  terminateFlag.value = false;
  pauseFlag.value = false;
  currentIteration.value = -1;
  exploitability.value = Number.POSITIVE_INFINITY;
  elapsedTimeMs.value = -1;

  store.isSolverRunning = true;

  startTime = performance.now();

  await invokes.setNumThreads(numThreads.value);
  await invokes.gameAllocateMemory(isCompressionEnabled.value);

  if (store.isBunchingEnabled && store.bunchingFlop.length > 0) {
    currentIteration.value = -2;
    const errorString = await invokes.gameSetBunching();
    if (errorString) {
      solverErrorText.value = "Error: " + errorString;
      store.isSolverRunning = false;
      store.isSolverError = true;
      return;
    }
  }

  currentIteration.value = 0;
  exploitability.value = Math.max(await invokes.gameExploitability(), 0);
  exploitabilityUpdated = true;

  await resumeSolver();
};

const resumeSolver = async () => {
  store.isSolverRunning = true;
  store.isSolverPaused = false;

  if (startTime === 0) {
    startTime = performance.now();
    await invokes.setNumThreads(numThreads.value);
  }

  const target = (config.startingPot * targetExploitability.value) / 100;

  while (
    !terminateFlag.value &&
    currentIteration.value < maxIterations.value &&
    exploitability.value > target
  ) {
    if (pauseFlag.value) {
      const end = performance.now();
      elapsedTimeMs.value += end - startTime;
      startTime = 0;
      pauseFlag.value = false;
      store.isSolverRunning = false;
      store.isSolverPaused = true;
      return;
    }

    await invokes.gameSolveStep(currentIteration.value);
    ++currentIteration.value;
    exploitabilityUpdated = false;

    if (currentIteration.value % 10 === 0) {
      exploitability.value = Math.max(await invokes.gameExploitability(), 0);
      exploitabilityUpdated = true;
    }
  }

  if (!exploitabilityUpdated) {
    exploitability.value = Math.max(await invokes.gameExploitability(), 0);
  }

  store.isSolverRunning = false;
  store.isFinalizing = true;

  await invokes.gameFinalize();

  store.isFinalizing = false;
  store.isSolverFinished = true;

  const end = performance.now();
  elapsedTimeMs.value += end - startTime;
};

// 显示调试信息的函数
const showDebugInfo = async () => {
  showDebug.value = true;
  
  // 收集牌局配置信息
  debugBoardInfo.value = JSON.stringify({
    board: config.board.map(card => {
      const rank = Math.floor(card / 4);
      const suit = card % 4;
      const ranks = "23456789TJQKA";
      const suits = "cdhs";
      return ranks[rank] + suits[suit];
    }),
    startingPot: config.startingPot,
    effectiveStack: config.effectiveStack,
    rakePercent: config.rakePercent,
    rakeCap: config.rakeCap,
    donkOption: config.donkOption
  }, null, 2);
  
  // 收集范围信息
  try {
    const oopRangeInfo = await invokes.rangeInfo(0);
    const ipRangeInfo = await invokes.rangeInfo(1);
    debugRangeInfo.value = JSON.stringify({
      oopRangeInfo,
      ipRangeInfo,
      storeRanges: store.ranges
    }, null, 2);
  } catch (error) {
    debugRangeInfo.value = `Error fetching range info: ${error}`;
  }
  
  // 收集下注配置信息
  debugBettingInfo.value = JSON.stringify({
    oopFlopBet: config.oopFlopBet,
    oopFlopRaise: config.oopFlopRaise,
    oopTurnBet: config.oopTurnBet,
    oopTurnRaise: config.oopTurnRaise,
    oopTurnDonk: config.oopTurnDonk,
    oopRiverBet: config.oopRiverBet,
    oopRiverRaise: config.oopRiverRaise,
    oopRiverDonk: config.oopRiverDonk,
    ipFlopBet: config.ipFlopBet,
    ipFlopRaise: config.ipFlopRaise,
    ipTurnBet: config.ipTurnBet,
    ipTurnRaise: config.ipTurnRaise,
    ipRiverBet: config.ipRiverBet,
    ipRiverRaise: config.ipRiverRaise,
    
    // 添加sanitized版本，检查是否有错误
    oopFlopBetSanitized: config.oopFlopBetSanitized,
    oopFlopRaiseSanitized: config.oopFlopRaiseSanitized,
    oopTurnBetSanitized: config.oopTurnBetSanitized,
    oopTurnRaiseSanitized: config.oopTurnRaiseSanitized,
    oopTurnDonkSanitized: config.oopTurnDonkSanitized,
    oopRiverBetSanitized: config.oopRiverBetSanitized,
    oopRiverRaiseSanitized: config.oopRiverRaiseSanitized,
    oopRiverDonkSanitized: config.oopRiverDonkSanitized,
    ipFlopBetSanitized: config.ipFlopBetSanitized,
    ipFlopRaiseSanitized: config.ipFlopRaiseSanitized,
    ipTurnBetSanitized: config.ipTurnBetSanitized,
    ipTurnRaiseSanitized: config.ipTurnRaiseSanitized,
    ipRiverBetSanitized: config.ipRiverBetSanitized,
    ipRiverRaiseSanitized: config.ipRiverRaiseSanitized
  }, null, 2);
  
  // 收集树配置信息
  debugTreeInfo.value = JSON.stringify({
    addAllInThreshold: config.addAllInThreshold,
    forceAllInThreshold: config.forceAllInThreshold,
    mergingThreshold: config.mergingThreshold,
    expectedBoardLength: config.expectedBoardLength,
    addedLines: config.addedLines,
    removedLines: config.removedLines,
    
    // 添加bunching信息
    isBunchingEnabled: store.isBunchingEnabled,
    bunchingFlop: store.bunchingFlop
  }, null, 2);
  
  // 检查配置是否有效
  const validationResult = checkConfig(config);
  if (validationResult === null) {
    debugValidationResult.value = "配置验证通过√";
    debugValidationError.value = false;
  } else {
    debugValidationResult.value = `配置验证失败: ${validationResult}`;
    debugValidationError.value = true;
  }
};

// 处理自动构建树事件
const handleAutoBuildTree = (event) => {
  console.log("RunSolver收到自动构建树事件:", event.detail);
  if (!isTreeBuilding.value && !store.isSolverRunning && !store.isFinalizing) {
    buildTree();
  }
};

// 添加生命周期钩子和事件监听器
onMounted(() => {
  // 监听自动构建树事件
  document.addEventListener('auto-build-tree', handleAutoBuildTree);
});

onUnmounted(() => {
  // 移除事件监听器
  document.removeEventListener('auto-build-tree', handleAutoBuildTree);
});
</script>

<style scoped>
.spinner {
  width: 1.25rem;
  height: 1.25rem;
  border: 3px solid rgba(59, 130, 246, 0.3);
  border-radius: 50%;
  border-top-color: rgb(59, 130, 246);
  animation: spin 1s ease-in-out infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>

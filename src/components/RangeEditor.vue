<template>
  <div class="flex gap-8 flex-col">
    <!-- 上部：主要内容区域 -->
    <div class="flex gap-8">
      <!-- 左侧：矩阵和控件 -->
      <div class="shrink-0">
        <div class="bg-white rounded-lg shadow-lg p-5">
          <!-- 矩阵表格 -->
          <table 
            class="shadow-md select-none rounded-md overflow-hidden" 
            @mouseleave="dragEnd"
            style="border-collapse: separate; border-spacing: 0.5px;"
          >
            <tr v-for="row in 13" :key="row" class="h-9">
              <td
                v-for="col in 13"
                :key="col"
                class="relative w-[2.625rem] border border-gray-300 transition-all duration-150"
                :class="{
                  'hover:border-blue-500': !cellValue(row, col),
                  'hover:border-blue-600': cellValue(row, col) > 0
                }"
                @mousedown="dragStart(row, col)"
                @mouseup="dragEnd"
                @mouseenter="mouseEnter(row, col)"
              >
                <div
                  :class="[
                    'absolute w-full h-full left-0 top-0',
                    row === col ? 'bg-blue-50' : (row > col ? 'bg-fuchsia-50' : 'bg-emerald-50')
                  ]"
                >
                  <div
                    class="absolute w-full h-full left-0 top-0 bg-bottom bg-no-repeat"
                    :style="{
                      'background-image': `linear-gradient(${row === col ? colors.pocketPairs : (row > col ? colors.suited : colors.offsuit)} 0% 100%)`,
                      'background-size': `100% ${cellValue(row, col)}%`
                    }"
                  ></div>
                </div>
                <div
                  :class="[
                    'absolute -top-px left-[0.1875rem] z-10 font-medium',
                    cellValue(row, col) > 0 
                      ? 'text-white' 
                      : (row === col ? 'text-blue-600' : (row > col ? 'text-fuchsia-600' : 'text-emerald-600'))
                  ]"
                  style="text-shadow: 0 1px 1px rgba(0, 0, 0, 0.2);"
                >
                  {{ cellText(row, col) }}
                </div>
                <div
                  v-if="cellValue(row, col) > 0 && cellValue(row, col) < 100"
                  class="absolute bottom-px right-1 z-10 text-xs font-medium"
                  :class="cellValue(row, col) > 70 ? 'text-white' : 'text-gray-700'"
                  style="text-shadow: 0 1px 1px rgba(0, 0, 0, 0.2);"
                >
                  {{ cellValue(row, col).toFixed(1) }}
                </div>
              </td>
            </tr>
          </table>

          <!-- 权重和动作控件 -->
          <div class="mt-5 space-y-4">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <span class="text-sm font-medium text-gray-700">Weight:</span>
                <input
                  v-model="weight"
                  type="range"
                  class="w-32 align-middle accent-blue-500"
                  min="0"
                  max="100"
                  step="5"
                  @change="onWeightChange"
                />
                <input
                  v-model="weight"
                  type="number"
                  :class="[
                    'w-16 px-2 py-1 rounded text-sm text-center border transition-colors duration-150',
                    weight < 0 || weight > 100 
                      ? 'border-red-500 bg-red-50' 
                      : 'border-gray-300 focus:border-blue-400 focus:ring-1 focus:ring-blue-400 focus:outline-none'
                  ]"
                  min="0"
                  max="100"
                  step="5"
                  @change="onWeightChange"
                />
                <span class="text-sm text-gray-700">%</span>
              </div>
            </div>

            <!-- 范围文本输入 -->
            <div class="flex items-center gap-2 mt-3">
              <input
                v-model="rangeText"
                type="text"
                :class="[
                  'flex-grow px-3 py-2 rounded-lg text-sm border transition-colors duration-150 focus:outline-none focus:ring-1',
                  rangeTextError 
                    ? 'border-red-500 bg-red-50 focus:ring-red-400' 
                    : 'border-gray-300 focus:border-blue-400 focus:ring-blue-400'
                ]"
                placeholder="Enter range notation (e.g., 'AKs, QQ+')"
                @focus="($event.target as HTMLInputElement).select()"
                @change="onRangeTextChange"
              />

              <div class="flex gap-2">
                <button
                  v-if="player >= 2"
                  class="px-3 py-1.5 rounded-lg bg-slate-100 hover:bg-slate-200 text-slate-700 text-sm font-medium transition-colors duration-150"
                  @click="invertRange"
                >
                  Invert
                </button>

                <button 
                  class="px-3 py-1.5 rounded-lg bg-slate-100 hover:bg-slate-200 text-slate-700 text-sm font-medium transition-colors duration-150"
                  @click="clearRange"
                >
                  Clear
                </button>
              </div>
            </div>

            <div v-if="rangeTextError" class="text-sm text-red-500 flex items-center mt-1">
              <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
              </svg>
              {{ rangeTextError }}
            </div>

            <div class="flex items-center justify-between text-sm text-gray-600 mt-2">
              <span>Combos: <span class="font-medium">{{ numCombos.toFixed(1) }}</span></span>
              <span>{{ ((numCombos * 100) / ((52 * 51) / 2)).toFixed(1) }}% of total</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 中间：范围分析 -->
      <div class="w-[240px]">
        <div class="bg-white rounded-lg shadow-lg p-5">
          <h3 class="text-base font-medium text-gray-800 mb-4">Range Analysis</h3>
          
          <!-- 手牌类型分布 -->
          <div class="space-y-3 mb-5">
            <div class="flex justify-between text-sm">
              <span class="text-gray-700">Pocket Pairs</span>
              <span class="font-medium text-blue-600">{{ calculatePocketPairs() }}%</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-gray-700">Suited</span>
              <span class="font-medium text-purple-600">{{ calculateSuited() }}%</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-gray-700">Offsuit</span>
              <span class="font-medium text-emerald-600">{{ calculateOffsuit() }}%</span>
            </div>
            <!-- 新增指标 -->
            <div class="border-t border-gray-200 pt-3 mt-2"></div>
            <div class="flex justify-between text-sm">
              <span class="text-gray-700">Broadway Cards</span>
              <span class="font-medium text-amber-600">{{ calculateBroadwayHands() }}%</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-gray-700">Connected Cards</span>
              <span class="font-medium text-red-600">{{ calculateConnected() }}%</span>
            </div>
          </div>

          <!-- 新增的Advanced Analysis面板 -->
          <div class="border-t border-gray-200 pt-4 mb-5">
            <h4 class="text-sm font-medium text-gray-800 mb-3">Advanced Analysis</h4>
            
            <!-- 牌力分层 -->
            <div class="mb-4">
              <h5 class="text-xs font-medium text-gray-600 mb-2">Hand Strength</h5>
              <div class="space-y-2">
                <div>
                  <div class="flex justify-between text-xs mb-1">
                    <span class="text-gray-600">Premium</span>
                    <span class="font-medium text-blue-600">{{ calculatePremiumHands() }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-1.5">
                    <div class="bg-blue-600 h-1.5 rounded-full" :style="{ width: calculatePremiumHands() + '%' }"></div>
                  </div>
                </div>
                
                <div>
                  <div class="flex justify-between text-xs mb-1">
                    <span class="text-gray-600">Strong</span>
                    <span class="font-medium text-indigo-600">{{ calculateStrongHands() }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-1.5">
                    <div class="bg-indigo-600 h-1.5 rounded-full" :style="{ width: calculateStrongHands() + '%' }"></div>
                  </div>
                </div>
                
                <div>
                  <div class="flex justify-between text-xs mb-1">
                    <span class="text-gray-600">Medium</span>
                    <span class="font-medium text-purple-600">{{ calculateMediumHands() }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-1.5">
                    <div class="bg-purple-600 h-1.5 rounded-full" :style="{ width: calculateMediumHands() + '%' }"></div>
                  </div>
                </div>
                
                <div>
                  <div class="flex justify-between text-xs mb-1">
                    <span class="text-gray-600">Marginal</span>
                    <span class="font-medium text-amber-600">{{ calculateMarginalHands() }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-1.5">
                    <div class="bg-amber-600 h-1.5 rounded-full" :style="{ width: calculateMarginalHands() + '%' }"></div>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- 高牌分析 -->
            <div class="mb-4">
              <h5 class="text-xs font-medium text-gray-600 mb-2">Card Distribution</h5>
              <div class="grid grid-cols-3 gap-2">
                <div class="bg-gray-100 p-2 rounded text-center">
                  <div class="text-sm font-bold text-red-600">{{ calculateAceHands() }}%</div>
                  <div class="text-xs text-gray-500">Ace</div>
                </div>
                <div class="bg-gray-100 p-2 rounded text-center">
                  <div class="text-sm font-bold text-red-600">{{ calculateKingHands() }}%</div>
                  <div class="text-xs text-gray-500">King</div>
                </div>
                <div class="bg-gray-100 p-2 rounded text-center">
                  <div class="text-sm font-bold text-red-600">{{ calculateQueenHands() }}%</div>
                  <div class="text-xs text-gray-500">Queen</div>
                </div>
              </div>
            </div>
            
            <!-- 连接性分析 -->
            <div>
              <h5 class="text-xs font-medium text-gray-600 mb-2">Connectivity</h5>
              <div class="grid grid-cols-2 gap-x-3 gap-y-1.5">
                <div class="text-xs text-gray-600 flex justify-between">
                  <span>Connected</span>
                  <span class="font-medium text-indigo-600">{{ calculateConnected() }}%</span>
                </div>
                <div class="text-xs text-gray-600 flex justify-between">
                  <span>1-Gap</span>
                  <span class="font-medium text-indigo-600">{{ calculateOneGap() }}%</span>
                </div>
                <div class="text-xs text-gray-600 flex justify-between">
                  <span>2-Gap</span>
                  <span class="font-medium text-indigo-600">{{ calculateTwoGap() }}%</span>
                </div>
                <div class="text-xs text-gray-600 flex justify-between">
                  <span>Unconnected</span>
                  <span class="font-medium text-indigo-600">{{ calculateUnconnected() }}%</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 快速模板 - 已注释 -->
          <!--
          <div class="border-t border-gray-200 pt-4">
            <h4 class="text-sm font-medium text-gray-800 mb-3">Quick Templates</h4>
            <div class="grid grid-cols-2 gap-2">
              <button 
                class="px-3 py-2 text-sm rounded-lg bg-blue-50 hover:bg-blue-100 text-blue-700 transition-colors duration-150"
                @click="loadTemplate('RFI')"
              >
                RFI Range
              </button>
              <button 
                class="px-3 py-2 text-sm rounded-lg bg-purple-50 hover:bg-purple-100 text-purple-700 transition-colors duration-150"
                @click="loadTemplate('3BET')"
              >
                3Bet Range
              </button>
              <button 
                class="px-3 py-2 text-sm rounded-lg bg-emerald-50 hover:bg-emerald-100 text-emerald-700 transition-colors duration-150"
                @click="loadTemplate('4BET')"
              >
                4Bet Range
              </button>
              <button 
                class="px-3 py-2 text-sm rounded-lg bg-amber-50 hover:bg-amber-100 text-amber-700 transition-colors duration-150"
                @click="loadTemplate('CALL')"
              >
                Call Range
              </button>
            </div>
          </div>
          -->
        </div>
      </div>

      <!-- 右侧：数据库 -->
      <div class="flex-1">
        <div class="bg-white rounded-lg shadow-lg p-5">
          <DbItemPicker
            store-name="ranges"
            :index="player"
            :value="rangeText"
            :allow-save="rangeText !== '' && rangeTextError === ''"
            :hide-import-export="player >= 2"
            @load-item="loadRange"
          />

          <!-- 操作按钮 -->
          <div v-if="player >= 2" class="mt-6 flex justify-end gap-3">
            <button 
              class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium transition-colors duration-150"
              @click="emit('cancel')"
            >
              Cancel
            </button>
            <button 
              class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium transition-colors duration-150"
              @click="emit('save')"
            >
              Save
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 下方：新增的指标面板 -->
    <div class="w-full">
      <div class="bg-white rounded-lg shadow-lg p-5">
        <h3 class="text-base font-medium text-gray-800 mb-4 flex items-center">
          <svg class="w-5 h-5 mr-2 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
          </svg>
          Extended Range Analysis
        </h3>
        
        <div class="grid grid-cols-4 gap-6">
          <!-- 翻牌命中率分析 -->
          <div class="col-span-2">
            <h4 class="text-sm font-medium text-gray-700 mb-3 flex items-center">
              Flop Hit Potential
              <span class="ml-1 text-xs text-gray-500 inline-flex items-center" title="估计在翻牌面上命中不同牌型的概率">
                (?)
              </span>
            </h4>
            <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
              <div class="space-y-3">
                <div>
                  <div class="flex justify-between mb-1">
                    <span class="text-sm font-medium text-gray-700">Pair+ on Flop</span>
                    <span class="text-sm font-medium text-emerald-600">{{ calculatePairPlusOnFlop() }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-2">
                    <div class="bg-emerald-600 rounded-full h-2" :style="{ width: Math.min(100, calculatePairPlusOnFlop()) + '%' }"></div>
                  </div>
                </div>
                
                <div>
                  <div class="flex justify-between mb-1">
                    <span class="text-sm font-medium text-gray-700">Flush Draw</span>
                    <span class="text-sm font-medium text-blue-600">{{ calculateFlushDrawOnFlop() }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-2">
                    <div class="bg-blue-600 rounded-full h-2" :style="{ width: Math.min(100, calculateFlushDrawOnFlop()) + '%' }"></div>
                  </div>
                </div>
                
                <div>
                  <div class="flex justify-between mb-1">
                    <span class="text-sm font-medium text-gray-700">Open-Ended Straight Draw</span>
                    <span class="text-sm font-medium text-purple-600">{{ calculateStraightDrawOnFlop() }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-2">
                    <div class="bg-purple-600 rounded-full h-2" :style="{ width: Math.min(100, calculateStraightDrawOnFlop()) + '%' }"></div>
                  </div>
                </div>
                
                <div>
                  <div class="flex justify-between mb-1">
                    <span class="text-sm font-medium text-gray-700">Overcards</span>
                    <span class="text-sm font-medium text-amber-600">{{ calculateOvercardsOnFlop() }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-2">
                    <div class="bg-amber-600 rounded-full h-2" :style="{ width: Math.min(100, calculateOvercardsOnFlop()) + '%' }"></div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <!-- 位置强度评估 -->
          <div>
            <h4 class="text-sm font-medium text-gray-700 mb-3 flex items-center">
              Position Strength
              <span class="ml-1 text-xs text-gray-500 inline-flex items-center" title="不同位置下范围的相对强度">
                (?)
              </span>
            </h4>
            <div class="bg-gray-50 rounded-lg p-4 border border-gray-200 h-full">
              <div class="grid grid-cols-1 gap-3">
                <div class="bg-white p-3 rounded shadow-sm">
                  <div class="text-xs text-gray-500">Early Position</div>
                  <div class="flex items-center mt-1">
                    <div class="w-full bg-gray-200 rounded-full h-2">
                      <div class="bg-red-500 rounded-full h-2" :style="{ width: Math.min(100, calculateEarlyPositionStrength()) + '%' }"></div>
                    </div>
                    <span class="ml-2 text-sm font-medium text-gray-700">{{ calculateEarlyPositionStrength() }}%</span>
                  </div>
                </div>
                <div class="bg-white p-3 rounded shadow-sm">
                  <div class="text-xs text-gray-500">Middle Position</div>
                  <div class="flex items-center mt-1">
                    <div class="w-full bg-gray-200 rounded-full h-2">
                      <div class="bg-yellow-500 rounded-full h-2" :style="{ width: Math.min(100, calculateMiddlePositionStrength()) + '%' }"></div>
                    </div>
                    <span class="ml-2 text-sm font-medium text-gray-700">{{ calculateMiddlePositionStrength() }}%</span>
                  </div>
                </div>
                <div class="bg-white p-3 rounded shadow-sm">
                  <div class="text-xs text-gray-500">Late Position</div>
                  <div class="flex items-center mt-1">
                    <div class="w-full bg-gray-200 rounded-full h-2">
                      <div class="bg-green-500 rounded-full h-2" :style="{ width: Math.min(100, calculateLatePositionStrength()) + '%' }"></div>
                    </div>
                    <span class="ml-2 text-sm font-medium text-gray-700">{{ calculateLatePositionStrength() }}%</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <!-- 特殊牌型分析 -->
          <div>
            <h4 class="text-sm font-medium text-gray-700 mb-3 flex items-center">
              Special Hand Groups
              <span class="ml-1 text-xs text-gray-500 inline-flex items-center" title="特殊价值牌型在范围中的分布">
                (?)
              </span>
            </h4>
            <div class="bg-gray-50 rounded-lg p-4 border border-gray-200 h-full">
              <div class="space-y-3">
                <div class="flex justify-between items-center">
                  <span class="text-sm text-gray-700">Suited Aces</span>
                  <div class="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-medium">
                    {{ calculateSuitedAces() }}%
                  </div>
                </div>
                <div class="flex justify-between items-center">
                  <span class="text-sm text-gray-700">Suited Connectors</span>
                  <div class="px-2 py-1 bg-green-100 text-green-800 rounded-full text-xs font-medium">
                    {{ calculateSuitedConnectors() }}%
                  </div>
                </div>
                <div class="flex justify-between items-center">
                  <span class="text-sm text-gray-700">Big Pairs (TT+)</span>
                  <div class="px-2 py-1 bg-purple-100 text-purple-800 rounded-full text-xs font-medium">
                    {{ calculateBigPairs() }}%
                  </div>
                </div>
                <div class="flex justify-between items-center">
                  <span class="text-sm text-gray-700">Axo Hands</span>
                  <div class="px-2 py-1 bg-amber-100 text-amber-800 rounded-full text-xs font-medium">
                    {{ calculateAxoHands() }}%
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 均衡性指标 -->
        <div class="mt-6">
          <h4 class="text-sm font-medium text-gray-700 mb-3 flex items-center">
            Range Balance Assessment
            <span class="ml-1 text-xs text-gray-500 inline-flex items-center" title="评估范围的平衡性和策略价值">
              (?)
            </span>
            <!-- 添加Debug按钮 -->
            <button 
              class="ml-auto px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 text-gray-700 rounded"
              @click="showDebugInfo"
            >
              Debug
            </button>
          </h4>
          <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
            <div class="grid grid-cols-2 gap-6">
              <!-- 左侧指标 -->
              <div>
                <div class="mb-4">
                  <div class="flex justify-between items-center mb-1">
                    <span class="text-sm font-medium text-gray-700">Range Polarization</span>
                    <div class="flex items-center">
                      <span class="mr-2 text-sm font-medium" :class="rangePolarization() < 30 ? 'text-green-600' : (rangePolarization() < 60 ? 'text-amber-600' : 'text-red-600')">
                        {{ rangePolarization() }}%
                      </span>
                      <div class="w-16 h-2 bg-gradient-to-r from-green-400 via-amber-400 to-red-400 rounded-full"></div>
                    </div>
                  </div>
                  <p class="text-xs text-gray-500">Measures how polarized your range is between strong and weak hands.</p>
                </div>
                
                <div>
                  <div class="flex justify-between items-center mb-1">
                    <span class="text-sm font-medium text-gray-700">Playability Score</span>
                    <div class="flex items-center">
                      <span class="mr-2 text-sm font-medium" :class="playabilityScore() > 70 ? 'text-green-600' : (playabilityScore() > 40 ? 'text-amber-600' : 'text-red-600')">
                        {{ playabilityScore() }}%
                      </span>
                      <div class="w-16 h-2 bg-gradient-to-r from-red-400 via-amber-400 to-green-400 rounded-full"></div>
                    </div>
                  </div>
                  <p class="text-xs text-gray-500">Estimates how well this range plays on later streets.</p>
                </div>
              </div>
              
              <!-- 右侧指标 -->
              <div>
                <div class="mb-4">
                  <div class="flex justify-between items-center mb-1">
                    <span class="text-sm font-medium text-gray-700">Nut Potential</span>
                    <div class="flex items-center">
                      <span class="mr-2 text-sm font-medium" :class="nutPotential() > 60 ? 'text-green-600' : (nutPotential() > 30 ? 'text-amber-600' : 'text-red-600')">
                        {{ nutPotential() }}%
                      </span>
                      <div class="w-16 h-2 bg-gradient-to-r from-red-400 via-amber-400 to-green-400 rounded-full"></div>
                    </div>
                  </div>
                  <p class="text-xs text-gray-500">Likelihood of making the strongest possible hands.</p>
                </div>
                
                <div>
                  <div class="flex justify-between items-center mb-1">
                    <span class="text-sm font-medium text-gray-700">Bluff Ratio</span>
                    <div class="flex items-center">
                      <span class="mr-2 text-sm font-medium" :class="bluffRatio() < 35 || bluffRatio() > 65 ? 'text-red-600' : (Math.abs(parseFloat(bluffRatio()) - 50) < 10 ? 'text-green-600' : 'text-amber-600')">
                        {{ bluffRatio() }}%
                      </span>
                      <div class="w-16 h-2 bg-gradient-to-r from-red-400 via-green-400 to-red-400 rounded-full"></div>
                    </div>
                  </div>
                  <p class="text-xs text-gray-500">Proportion of bluff candidates to value hands in your range.</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useStore } from "../store";
import { ranks, trimRegex, rangeRegex } from "../utils";
import * as invokes from "../invokes";

import DbItemPicker from "./DbItemPicker.vue";

// 颜色方案
const colors = {
  pocketPairs: "#4f46e5",   // 靛蓝色 - 口袋对子 (indigo-600)
  suited: "#c026d3",        // 洋红色 - 同花 (fuchsia-600)
  offsuit: "#059669"         // 绿色 - 非同花 (emerald-600)
};

type DraggingMode = "none" | "enabling" | "disabling";

const props = withDefaults(
  defineProps<{ player: number; defaultText?: string }>(),
  { defaultText: "" }
);

const emit = defineEmits<{
  (event: "save"): void;
  (event: "cancel"): void;
}>();

const store = useStore();
const rangeText = ref(props.defaultText);
const rangeTextError = ref("");
const weight = ref(100);
const numCombos = ref(0);

let draggingMode: DraggingMode = "none";

const cellText = (row: number, col: number) => {
  const r1 = 13 - Math.min(row, col);
  const r2 = 13 - Math.max(row, col);
  return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
};

const cellIndex = (row: number, col: number) => {
  return 13 * (row - 1) + col - 1;
};

const cellValue = (row: number, col: number) => {
  return store.ranges[props.player][cellIndex(row, col)];
};

const onUpdate = async () => {
  rangeText.value = await invokes.rangeToString(props.player);
  numCombos.value = await invokes.rangeNumCombos(props.player);
  rangeTextError.value = "";
};

const update = async (row: number, col: number, weight: number) => {
  const idx = 13 * (row - 1) + col - 1;
  await invokes.rangeUpdate(props.player, row, col, weight / 100);
  store.ranges[props.player][idx] = weight;
  await onUpdate();
};

const onRangeTextChange = async () => {
  const trimmed = rangeText.value.replace(trimRegex, "$1").trim();
  const ranges = trimmed.split(",");

  if (ranges[ranges.length - 1] === "") {
    ranges.pop();
  }

  for (const range of ranges) {
    if (!rangeRegex.test(range)) {
      rangeTextError.value = `Failed to parse range: ${
        range || "(empty string)"
      }`;
      return;
    }
  }

  const errorString = await invokes.rangeFromString(props.player, trimmed);

  if (errorString) {
    rangeTextError.value = errorString;
  } else {
    const weights = await invokes.rangeGetWeights(props.player);
    for (let i = 0; i < 13 * 13; ++i) {
      store.ranges[props.player][i] = weights[i] * 100;
    }
    await onUpdate();
  }
};

const dragStart = (row: number, col: number) => {
  const idx = 13 * (row - 1) + col - 1;

  if (store.ranges[props.player][idx] !== weight.value) {
    draggingMode = "enabling";
    update(row, col, weight.value);
  } else {
    draggingMode = "disabling";
    update(row, col, 0);
  }
};

const dragEnd = () => {
  draggingMode = "none";
};

const mouseEnter = (row: number, col: number) => {
  if (draggingMode === "enabling") {
    update(row, col, weight.value);
  } else if (draggingMode === "disabling") {
    update(row, col, 0);
  }
};

const onWeightChange = () => {
  weight.value = Math.round(Math.max(0, Math.min(100, weight.value)));
};

const clearRange = async () => {
  await invokes.rangeClear(props.player);
  store.ranges[props.player].fill(0);
  rangeText.value = "";
  rangeTextError.value = "";
  weight.value = 100;
  numCombos.value = 0;
};

const invertRange = async () => {
  await invokes.rangeInvert(props.player);
  for (let i = 0; i < 13 * 13; ++i) {
    store.ranges[props.player][i] = 100 - store.ranges[props.player][i];
  }
  await onUpdate();
};

const loadRange = (rangeStr: unknown) => {
  rangeText.value = String(rangeStr);
  onRangeTextChange();
};

// 范围分析功能
const calculatePocketPairs = () => {
  let total = 0;
  let pairCount = 0;
  for (let i = 0; i < 13; i++) {
    if (store.ranges[props.player][i * 13 + i] > 0) {
      pairCount++;
      total += store.ranges[props.player][i * 13 + i];
    }
  }
  // 如果只有口袋对，应返回100%
  if (pairCount > 0 && sumOfRange() === total) return "100.0";
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateSuited = () => {
  let total = 0;
  for (let i = 0; i < 13; i++) {
    for (let j = i + 1; j < 13; j++) {
      total += store.ranges[props.player][i * 13 + j];
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateOffsuit = () => {
  let total = 0;
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j < i; j++) {
      total += store.ranges[props.player][i * 13 + j];
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

// 计算百分比时的总和函数
const sumOfRange = () => {
  let sum = 0;
  for (let i = 0; i < 13 * 13; i++) {
    sum += store.ranges[props.player][i];
  }
  return sum || 1; // 防止除以0
};

// Broadway牌型占比
const calculateBroadwayHands = () => {
  let total = 0;
  // Broadway牌型:AKQJT相关
  for (let i = 0; i < 5; i++) {
    for (let j = 0; j < 5; j++) {
      if (i !== j) { // 排除对子
        total += store.ranges[props.player][Math.min(i, j) * 13 + Math.max(i, j)];
      }
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

// 连牌占比
const calculateConnected = () => {
  let total = 0;
  // 检查相邻牌
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j < 13; j++) {
      if (Math.abs(i - j) === 1) {
        total += store.ranges[props.player][Math.min(i, j) * 13 + Math.max(i, j)];
      }
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

// 单缺口牌占比
const calculateOneGap = () => {
  let total = 0;
  // 检查单缺口
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j < 13; j++) {
      if (Math.abs(i - j) === 2) {
        total += store.ranges[props.player][Math.min(i, j) * 13 + Math.max(i, j)];
      }
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

// 双缺口牌占比
const calculateTwoGap = () => {
  let total = 0;
  // 检查双缺口
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j < 13; j++) {
      if (Math.abs(i - j) === 3) {
        total += store.ranges[props.player][Math.min(i, j) * 13 + Math.max(i, j)];
      }
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

// 未连接牌占比
const calculateUnconnected = () => {
  let total = 0;
  for (let i = 0; i < 13; i++) {
    for (let j = 0; j < 13; j++) {
      if (Math.abs(i - j) > 3 || i === j) { // 间隔大于3或者对子
        total += store.ranges[props.player][Math.min(i, j) * 13 + Math.max(i, j)];
      }
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

// 高牌系列分析
const calculateAceHands = () => {
  let total = 0;
  // A开头的牌（排除AA）
  for (let i = 1; i < 13; i++) {
    // A与其他牌的组合
    total += store.ranges[props.player][Math.min(0, i) * 13 + Math.max(0, i)];
  }
  // 加上AA
  total += store.ranges[props.player][0 * 13 + 0];
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateKingHands = () => {
  let total = 0;
  // K开头的牌，包括K与其他牌的组合（除A）
  for (let i = 2; i < 13; i++) { // 从2开始，跳过A
    total += store.ranges[props.player][Math.min(1, i) * 13 + Math.max(1, i)];
  }
  // 加上AK组合和KK
  total += store.ranges[props.player][0 * 13 + 1]; // AKs
  total += store.ranges[props.player][1 * 13 + 0]; // AKo
  total += store.ranges[props.player][1 * 13 + 1]; // KK
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateQueenHands = () => {
  let total = 0;
  // Q开头的牌，包括Q与其他牌的组合（除A和K）
  for (let i = 3; i < 13; i++) { // 从3开始，跳过A和K
    total += store.ranges[props.player][Math.min(2, i) * 13 + Math.max(2, i)];
  }
  // 加上AQ、KQ组合和QQ
  total += store.ranges[props.player][0 * 13 + 2]; // AQs
  total += store.ranges[props.player][1 * 13 + 2]; // KQs
  total += store.ranges[props.player][2 * 13 + 0]; // AQo
  total += store.ranges[props.player][2 * 13 + 1]; // KQo
  total += store.ranges[props.player][2 * 13 + 2]; // QQ
  return (total / sumOfRange() * 100).toFixed(1);
};

// 牌力分层计算
const calculatePremiumHands = () => {
  let total = 0;
  // 检查AA-QQ
  for (let i = 0; i < 3; i++) {
    total += store.ranges[props.player][i * 13 + i];
  }
  // 检查AKs
  total += store.ranges[props.player][0 * 13 + 1];
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateStrongHands = () => {
  let total = 0;
  // JJ-TT
  for (let i = 3; i < 5; i++) {
    total += store.ranges[props.player][i * 13 + i];
  }
  // AQs-AJs, AKo
  total += store.ranges[props.player][0 * 13 + 2];
  total += store.ranges[props.player][0 * 13 + 3];
  total += store.ranges[props.player][1 * 13 + 0]; // AKo
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateMediumHands = () => {
  let total = 0;
  // 99-77
  for (let i = 5; i < 8; i++) {
    total += store.ranges[props.player][i * 13 + i];
  }
  // ATs-A5s, KQs-KTs
  for (let i = 4; i < 10; i++) {
    total += store.ranges[props.player][0 * 13 + i]; // Axs
  }
  for (let i = 2; i < 5; i++) {
    total += store.ranges[props.player][1 * 13 + i]; // Kxs
  }
  // AQo-ATo
  for (let i = 2; i < 5; i++) {
    total += store.ranges[props.player][i * 13 + 0]; // Axo
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateMarginalHands = () => {
  let total = 0;
  // 66-22
  for (let i = 8; i < 13; i++) {
    total += store.ranges[props.player][i * 13 + i];
  }
  // A4s-A2s, KTs以下的中高同花连接
  for (let i = 10; i < 13; i++) {
    total += store.ranges[props.player][0 * 13 + i]; // Axs
  }
  // 其他连接同花
  for (let i = 1; i < 10; i++) {
    for (let j = i + 1; j < 11; j++) {
      if (j - i === 1) { // 相邻
        total += store.ranges[props.player][i * 13 + j]; // 只计算同花
      }
    }
  }
  // 添加中小同花非对牌型
  for (let i = 5; i < 13; i++) {
    for (let j = i + 1; j < 13; j++) {
      if (j - i > 1 && j >= 5) { // 非连接且非高牌
        total += store.ranges[props.player][i * 13 + j]; // 同花
      }
    }
  }
  // 添加中小非同花牌型
  for (let i = 5; i < 13; i++) {
    for (let j = 0; j < i; j++) {
      if (j >= 5 || (i >= 8 && j < 5)) { // 中小非同花组合
        total += store.ranges[props.player][i * 13 + j]; // 非同花
      }
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

// 快速模板加载
const loadTemplate = (type: string) => {
  let rangeStr = "";
  switch (type) {
    case 'RFI':
      rangeStr = "22+, A2s+, K9s+, QTs+, JTs, T9s, 98s, 87s, 76s, 65s, ATo+, KJo+, QJo";
      break;
    case '3BET':
      rangeStr = "JJ+, AQs+, AKo";
      break;
    case '4BET':
      rangeStr = "QQ+, AKs";
      break;
    case 'CALL':
      rangeStr = "TT-22, AJs-A2s, KQs-K9s, QJs-QTs, JTs, T9s, 98s, 87s, 76s, AQo-ATo, KQo-KJo";
      break;
  }
  rangeText.value = rangeStr;
  onRangeTextChange();
};

// 新增的指标计算方法
const calculateSuitedAces = () => {
  let total = 0;
  // A开头的同花牌
  for (let i = 1; i < 13; i++) {
    total += store.ranges[props.player][0 * 13 + i]; // A和其他牌的同花组合
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateSuitedConnectors = () => {
  let total = 0;
  // 相邻且同花的牌
  for (let i = 0; i < 12; i++) {
    for (let j = i+1; j < 13; j++) {
      if (j - i === 1) { // 确保是相邻的牌
        total += store.ranges[props.player][i * 13 + j]; // 同花连牌
      }
    }
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateBigPairs = () => {
  let total = 0;
  // TT+对子 (AA, KK, QQ, JJ, TT)
  for (let i = 0; i < 5; i++) {
    total += store.ranges[props.player][i * 13 + i]; 
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

const calculateAxoHands = () => {
  let total = 0;
  // A开头的非同花牌
  for (let i = 1; i < 13; i++) {
    total += store.ranges[props.player][i * 13 + 0]; // A和其他牌的非同花组合
  }
  return (total / sumOfRange() * 100).toFixed(1);
};

// 范围极化度计算
const rangePolarization = () => {
  // 基础牌力指标
  const premium = parseFloat(calculatePremiumHands());
  const strong = parseFloat(calculateStrongHands());
  const medium = parseFloat(calculateMediumHands());
  const marginal = parseFloat(calculateMarginalHands());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  const bigPairs = parseFloat(calculateBigPairs());
  
  // 如果是纯口袋对范围
  if (pocketPairRatio >= 99.9) {
    // 计算大对子比例
    const bigPairRatio = bigPairs / pocketPairRatio;
    // 计算小对子比例
    const smallPairRatio = 1 - bigPairRatio;
    
    // 如果对子大小分布均匀，极化程度低
    // 如果全是大对子或全是小对子，极化程度高
    const pairPolarization = Math.abs(bigPairRatio - smallPairRatio) * 100;
    return pairPolarization.toFixed(1);
  }
  
  // 计算强牌组占比 (Premium + Strong)
  const strongGroup = premium + strong;
  
  // 计算弱牌组占比 (Medium + Marginal)
  const weakGroup = medium + marginal;
  
  // 总和
  const total = strongGroup + weakGroup;
  
  if (total === 0) return "0.0";
  
  // 计算极化指数 - 强弱牌比例差异
  const polarizationIndex = Math.abs(strongGroup - weakGroup) / total * 100;
  return Math.min(100, polarizationIndex).toFixed(1);
};

// 可玩性评分计算
const playabilityScore = () => {
  // 获取牌力分布数据
  const premium = parseFloat(calculatePremiumHands());
  const strong = parseFloat(calculateStrongHands());
  const medium = parseFloat(calculateMediumHands());
  const marginal = parseFloat(calculateMarginalHands());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  const suitedRatio = parseFloat(calculateSuited());
  const connectedRatio = parseFloat(calculateConnected());
  const bigPairs = parseFloat(calculateBigPairs());
  
  // 口袋对专用处理逻辑
  if (pocketPairRatio >= 99.9) {
    // 大对子(TT+)的比例 - 大对子在翻牌后更容易做决策
    const bigPairRatio = bigPairs / pocketPairRatio;
    // 根据对子大小调整可玩性 - 大对子可玩性更高
    const premiumPairWeight = 0.7;  // AA, KK, QQ的可玩性权重
    const strongPairWeight = 0.6;   // JJ, TT的可玩性权重
    const mediumPairWeight = 0.4;   // 99, 88, 77的可玩性权重 
    const marginalPairWeight = 0.3; // 66以下小对子的可玩性权重
    
    // 综合计算所有对子的加权可玩性
    const playabilityValue = (
      (premium / pocketPairRatio) * premiumPairWeight +
      (strong / pocketPairRatio) * strongPairWeight +
      (medium / pocketPairRatio) * mediumPairWeight +
      (marginal / pocketPairRatio) * marginalPairWeight
    ) * 100;
    
    // 对子整体可玩性调整 - 考虑到翻牌圈后的难度
    return Math.min(100, playabilityValue).toFixed(1);
  }
  
  // 检查分母，避免除以零
  const totalHandsRatio = premium + strong + medium + marginal;
  if (totalHandsRatio === 0) return "50.0"; // 默认中等可玩性
  
  // 非纯口袋对范围的计算
  // 基础牌力评分
  const strengthScore = (
    premium * 0.9 +      // 顶级牌力
    strong * 0.8 +       // 强牌
    medium * 0.6 +       // 中等牌力
    marginal * 0.3       // 边缘牌力
  ) / totalHandsRatio * 60;  // 牌力贡献60%的可玩性
  
  // 牌型评分 - 同花和连牌在后续街道更容易游戏
  const shapeScore = (
    pocketPairRatio * 0.7 +  // 口袋对后续街道相对容易游戏
    suitedRatio * 0.6 +      // 同花牌有同花听牌潜力
    connectedRatio * 0.6     // 连牌有顺子听牌潜力
  ) / (pocketPairRatio + suitedRatio + connectedRatio || 1) * 40;  // 牌型贡献40%的可玩性
  
  // 综合评分
  return Math.min(100, strengthScore + shapeScore).toFixed(1);
};

// 坚果潜力计算
const nutPotential = () => {
  // 获取基础数据
  const premium = parseFloat(calculatePremiumHands());
  const strong = parseFloat(calculateStrongHands());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  const aceRatio = parseFloat(calculateAceHands());
  const bigPairs = parseFloat(calculateBigPairs());
  const suitedRatio = parseFloat(calculateSuited());
  const connectedRatio = parseFloat(calculateConnected());
  
  // 如果是纯口袋对范围
  if (pocketPairRatio >= 99.9) {
    // 分析大对子在范围中的占比
    const bigPairRatio = bigPairs / pocketPairRatio;
    
    // KK+的权重 (这些手牌有顶对/超对子潜力)
    let topPairPotential = 0;
    
    // 检查范围中是否有AA和KK
    if (premium > 0) {
      // AA, KK占比
      const aaKkRatio = premium / pocketPairRatio;
      topPairPotential = aaKkRatio * 70; // 顶对牌力加权
    }
    
    // JJ-TT的权重 (这些手牌有超对子或暗三条潜力)
    let setTripsOverpairPotential = 0;
    
    // 检查范围中是否有JJ-TT
    if (strong > 0) {
      // JJ, TT占比
      const jjTtRatio = strong / pocketPairRatio;
      setTripsOverpairPotential = jjTtRatio * 40; // 超对/三条牌力加权
    }
    
    // 中小对子的权重 (主要依靠命中暗三条)
    const smallPairRatio = 1 - bigPairRatio;
    const setTripsPotential = smallPairRatio * 20; // 中小对子主要靠命中三条
    
    // 综合口袋对坚果牌潜力
    return Math.min(100, topPairPotential + setTripsOverpairPotential + setTripsPotential).toFixed(1);
  }
  
  // 检查总手牌数，避免除以零
  const totalHands = premium + strong + pocketPairRatio + suitedRatio + connectedRatio;
  if (totalHands === 0) return "0.0";
  
  // 常规范围的计算
  // 高牌带来的坚果牌潜力 (同花、顺子、高对)
  const highCardPotential = aceRatio * 0.5;
  
  // 大对子的坚果牌潜力 (大对，暗三条)
  const bigPairPotential = bigPairs * 0.4;
  
  // 顶级牌力的坚果牌潜力 (AA, KK, QQ, AKs等)
  const premiumPotential = premium * 0.7;
  
  // 同花牌的坚果牌潜力
  const suitedPotential = suitedRatio * 0.3;
  
  // 连牌的坚果牌潜力
  const connectedPotential = connectedRatio * 0.2;
  
  // 综合计算坚果牌潜力，限制最小值为0，最大值为100
  const potential = premiumPotential + bigPairPotential * 0.5 + highCardPotential * 0.3 + 
                    suitedPotential * 0.2 + connectedPotential * 0.1;
                    
  return Math.min(100, Math.max(0, potential)).toFixed(1);
};

// 诈唬比例计算
const bluffRatio = () => {
  // 获取基础数据
  const premium = parseFloat(calculatePremiumHands());
  const strong = parseFloat(calculateStrongHands());
  const medium = parseFloat(calculateMediumHands());
  const marginal = parseFloat(calculateMarginalHands());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  const bigPairs = parseFloat(calculateBigPairs());
  
  // 如果是纯口袋对范围
  if (pocketPairRatio >= 99.9) {
    // 大对子通常作为价值牌使用
    const valueHands = premium + strong * 0.8;
    
    // 中小对子在多数翻牌面下往往作为诈唬牌使用
    const bluffCandidates = strong * 0.2 + medium * 0.7 + marginal * 0.9;
    
    const total = valueHands + bluffCandidates;
    if (total === 0) return "50.0";
    
    // 返回诈唬牌在整个范围中的百分比
    return ((bluffCandidates / total) * 100).toFixed(1);
  }
  
  // 价值牌：顶级手牌和高强度手牌
  const valueHands = premium * 1.0 + strong * 0.7 + medium * 0.3;
  
  // 诈唬牌：边缘手牌和低强度连牌
  const connectedRatio = parseFloat(calculateConnected());
  const bluffCandidates = marginal * 0.8 + medium * 0.4 + strong * 0.2 + connectedRatio * 0.2;
  
  const total = valueHands + bluffCandidates;
  if (total === 0) return "50.0";
  
  // 返回诈唬牌在整个范围中的百分比，限制在0-100之间
  return Math.min(100, Math.max(0, (bluffCandidates / total) * 100)).toFixed(1);
};

// 翻牌命中率计算方法
const calculatePairPlusOnFlop = () => {
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  // 如果只有口袋对，命中率应接近100%
  if (pocketPairRatio >= 99.9) return "100.0";
  
  // 计算非对子牌在翻牌面命中对子的概率
  // 非口袋对在翻牌面命中对子的概率大约是32.4%
  const nonPairRatio = 100 - pocketPairRatio;
  
  // 口袋对100%命中至少一对，非口袋对约32.4%命中至少一对
  const pairPlusProbability = (pocketPairRatio + nonPairRatio * 0.324) / 100;
  
  return (pairPlusProbability * 100).toFixed(1);
};

const calculateFlushDrawOnFlop = () => {
  // 计算同花牌在范围中的比例
  const suitedRatio = parseFloat(calculateSuited());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  
  // 纯口袋对范围不可能有同花听牌
  if (pocketPairRatio >= 99.9) return "0.0";
  
  // 同花牌在翻牌面形成同花听牌的概率约为10.9%
  const flushDrawProbability = (suitedRatio * 0.109) / 100;
  
  return (flushDrawProbability * 100).toFixed(1);
};

const calculateStraightDrawOnFlop = () => {
  // 计算连牌和单缺口牌在范围中的比例
  const connectedRatio = parseFloat(calculateConnected());
  const oneGapRatio = parseFloat(calculateOneGap());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  
  // 纯口袋对范围不可能有顺子听牌
  if (pocketPairRatio >= 99.9) return "0.0";
  
  // 连牌在翻牌面形成顺子听牌的概率约为9.6%
  // 单缺口牌形成顺子听牌的概率约为6.2%
  const straightDrawProbability = (connectedRatio * 0.096 + oneGapRatio * 0.062) / 100;
  
  return (straightDrawProbability * 100).toFixed(1);
};

const calculateOvercardsOnFlop = () => {
  // 计算高牌在范围中的比例
  const broadwayRatio = parseFloat(calculateBroadwayHands());
  const aceRatio = parseFloat(calculateAceHands());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  
  // 纯口袋对范围不可能有超牌
  if (pocketPairRatio >= 99.9) return "0.0";
  
  // Broadway牌形成超牌的概率约为23.5%
  // Ace牌增加额外的超牌几率
  const overcardsDrawProbability = (broadwayRatio * 0.235 + (aceRatio - broadwayRatio * 0.4) * 0.1) / 100;
  
  return (overcardsDrawProbability * 100).toFixed(1);
};

const calculateEarlyPositionStrength = () => {
  // 获取牌力分布数据
  const premium = parseFloat(calculatePremiumHands());
  const strong = parseFloat(calculateStrongHands());
  const medium = parseFloat(calculateMediumHands());
  const marginal = parseFloat(calculateMarginalHands());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  const bigPairs = parseFloat(calculateBigPairs());
  
  // 计算分母，确保不会出现除以0的情况
  const denominator = premium + strong + medium + marginal;
  if (denominator === 0) return "0.0";
  
  // 早期位置权重 - 大对子和高牌在早期位置较强
  const premiumWeight = 1.0;  // AA, KK, QQ, AKs 在早期很强
  const strongWeight = 0.8;   // JJ, TT, AQs, AJs, AKo 在早期较强
  const mediumWeight = 0.5;   // 99-77, ATs-A5s 等在早期中等
  const marginalWeight = 0.2; // 小对子和低价值牌在早期较弱
  
  // 计算加权分数
  let score = (premium * premiumWeight + 
              strong * strongWeight + 
              medium * mediumWeight + 
              marginal * marginalWeight) / denominator;
  
  // 口袋对范围的特殊调整
  if (pocketPairRatio >= 99.9) {
    // 大对子比例 (AA-TT)
    const bigPairRatio = bigPairs / 100;
    // 早期位置对大对子友好
    score = score * (0.7 + 0.3 * bigPairRatio); 
  }
  
  // 应用位置调整系数并格式化
  return Math.min(100, score * 70).toFixed(1);
};

const calculateMiddlePositionStrength = () => {
  // 获取牌力分布数据
  const premium = parseFloat(calculatePremiumHands());
  const strong = parseFloat(calculateStrongHands());
  const medium = parseFloat(calculateMediumHands());
  const marginal = parseFloat(calculateMarginalHands());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  const bigPairs = parseFloat(calculateBigPairs());
  
  // 计算分母，确保不会出现除以0的情况
  const denominator = premium + strong + medium + marginal;
  if (denominator === 0) return "0.0";
  
  // 中期位置权重 - 更加平衡
  const premiumWeight = 1.1;  // AA, KK, QQ, AKs 在中期很强
  const strongWeight = 0.9;   // JJ, TT, AQs, AJs, AKo 在中期很强
  const mediumWeight = 0.7;   // 99-77, ATs-A5s 等在中期较强
  const marginalWeight = 0.4; // 小对子和低价值牌在中期有一定价值
  
  // 计算加权分数
  let score = (premium * premiumWeight + 
              strong * strongWeight + 
              medium * mediumWeight + 
              marginal * marginalWeight) / denominator;
  
  // 口袋对范围的特殊调整
  if (pocketPairRatio >= 99.9) {
    // 大对子比例 (AA-TT)
    const bigPairRatio = bigPairs / 100;
    // 中期位置对大中对子都较为友好
    score = score * (0.75 + 0.25 * bigPairRatio);
  }
  
  // 应用位置调整系数并格式化
  return Math.min(100, score * 75).toFixed(1);
};

const calculateLatePositionStrength = () => {
  // 获取牌力分布数据
  const premium = parseFloat(calculatePremiumHands());
  const strong = parseFloat(calculateStrongHands());
  const medium = parseFloat(calculateMediumHands());
  const marginal = parseFloat(calculateMarginalHands());
  const pocketPairRatio = parseFloat(calculatePocketPairs());
  const bigPairs = parseFloat(calculateBigPairs());
  
  // 计算分母，确保不会出现除以0的情况
  const denominator = premium + strong + medium + marginal;
  if (denominator === 0) return "0.0";
  
  // 晚期位置权重 - 更偏好投机性的牌
  const premiumWeight = 1.2;  // AA, KK, QQ, AKs 在晚期仍然很强
  const strongWeight = 1.0;   // JJ, TT, AQs, AJs, AKo 在晚期很强
  const mediumWeight = 0.9;   // 99-77, ATs-A5s 等在晚期很有价值
  const marginalWeight = 0.7; // 小对子和低价值牌在晚期也有不错价值
  
  // 计算加权分数
  let score = (premium * premiumWeight + 
              strong * strongWeight + 
              medium * mediumWeight + 
              marginal * marginalWeight) / denominator;
  
  // 口袋对范围的特殊调整
  if (pocketPairRatio >= 99.9) {
    // 大对子比例 (AA-TT)
    const bigPairRatio = bigPairs / 100;
    // 晚期位置对所有对子都友好
    score = score * (0.8 + 0.2 * bigPairRatio);
  }
  
  // 应用位置调整系数并格式化
  return Math.min(100, score * 80).toFixed(1);
};

const showDebugInfo = async () => {
  // 收集当前范围的所有指标
  const metrics = {
    "Pocket Pairs": calculatePocketPairs(),
    "Suited": calculateSuited(),
    "Offsuit": calculateOffsuit(),
    "Broadway Cards": calculateBroadwayHands(),
    "Connected Cards": calculateConnected(),
    "Premium": calculatePremiumHands(),
    "Strong": calculateStrongHands(),
    "Medium": calculateMediumHands(),
    "Marginal": calculateMarginalHands(),
    "Ace": calculateAceHands(),
    "King": calculateKingHands(),
    "Queen": calculateQueenHands(),
    "Connected": calculateConnected(),
    "1-Gap": calculateOneGap(),
    "2-Gap": calculateTwoGap(),
    "Unconnected": calculateUnconnected(),
    "Pair+ on Flop": calculatePairPlusOnFlop(),
    "Flush Draw": calculateFlushDrawOnFlop(),
    "Open-Ended Straight Draw": calculateStraightDrawOnFlop(),
    "Overcards": calculateOvercardsOnFlop(),
    "Early Position": calculateEarlyPositionStrength(),
    "Middle Position": calculateMiddlePositionStrength(),
    "Late Position": calculateLatePositionStrength(),
    "Suited Aces": calculateSuitedAces(),
    "Suited Connectors": calculateSuitedConnectors(),
    "Big Pairs (TT+)": calculateBigPairs(),
    "Axo Hands": calculateAxoHands(),
    "Range Polarization": rangePolarization(),
    "Nut Potential": nutPotential(),
    "Playability Score": playabilityScore(),
    "Bluff Ratio": bluffRatio()
  };
  
  // 创建格式化的文本输出，用于显示和复制
  const textOutput = Object.entries(metrics)
    .map(([key, value]) => `${key}: ${value}%`)
    .join('\n');
  
  // 创建JSON格式
  const jsonFormat = JSON.stringify(metrics, null, 2);
  
  try {
    // 使用现代剪贴板API
    await navigator.clipboard.writeText(jsonFormat);
    
    // 创建自定义对话框
    const dialog = document.createElement('div');
    dialog.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 80%;
      max-width: 600px;
      max-height: 80vh;
      background: white;
      border-radius: 8px;
      box-shadow: 0 4px 15px rgba(0,0,0,0.2);
      z-index: 9999;
      padding: 20px;
      display: flex;
      flex-direction: column;
    `;
    
    const header = document.createElement('div');
    header.style.cssText = `
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 15px;
      border-bottom: 1px solid #eee;
      padding-bottom: 10px;
    `;
    header.innerHTML = `
      <h3 style="margin: 0; font-size: 16px; font-weight: 600;">当前范围的指标值（JSON已复制到剪贴板）</h3>
      <button id="closeDialog" style="background: none; border: none; cursor: pointer; font-size: 16px;">×</button>
    `;
    
    const content = document.createElement('div');
    content.style.cssText = `
      overflow-y: auto;
      font-family: monospace;
      white-space: pre;
      background: #f5f5f5;
      padding: 15px;
      border-radius: 5px;
      flex-grow: 1;
      font-size: 14px;
    `;
    content.textContent = jsonFormat;
    
    const actions = document.createElement('div');
    actions.style.cssText = `
      display: flex;
      justify-content: flex-end;
      gap: 10px;
      margin-top: 15px;
    `;
    
    const copyJsonBtn = document.createElement('button');
    copyJsonBtn.textContent = "复制JSON";
    copyJsonBtn.style.cssText = `
      padding: 6px 12px;
      background: #e6f7ff;
      border: 1px solid #91d5ff;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
      color: #1890ff;
    `;
    copyJsonBtn.addEventListener('click', async () => {
      await navigator.clipboard.writeText(jsonFormat);
      copyJsonBtn.textContent = "✓ 已复制";
      setTimeout(() => {
        copyJsonBtn.textContent = "复制JSON";
      }, 2000);
    });
    
    const copyTextBtn = document.createElement('button');
    copyTextBtn.textContent = "复制文本格式";
    copyTextBtn.style.cssText = `
      padding: 6px 12px;
      background: #f0f0f0;
      border: 1px solid #ddd;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
    `;
    copyTextBtn.addEventListener('click', async () => {
      await navigator.clipboard.writeText(textOutput);
      copyTextBtn.textContent = "✓ 已复制";
      setTimeout(() => {
        copyTextBtn.textContent = "复制文本格式";
      }, 2000);
    });
    
    const toggleFormatBtn = document.createElement('button');
    toggleFormatBtn.textContent = "切换为文本格式";
    toggleFormatBtn.style.cssText = `
      padding: 6px 12px;
      background: #fff;
      border: 1px solid #ddd;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
    `;
    
    let showingTextFormat = false;
    toggleFormatBtn.addEventListener('click', () => {
      showingTextFormat = !showingTextFormat;
      if (showingTextFormat) {
        content.textContent = textOutput;
        toggleFormatBtn.textContent = "切换为JSON格式";
      } else {
        content.textContent = jsonFormat;
        toggleFormatBtn.textContent = "切换为文本格式";
      }
    });
    
    actions.appendChild(copyJsonBtn);
    actions.appendChild(copyTextBtn);
    actions.appendChild(toggleFormatBtn);
    
    dialog.appendChild(header);
    dialog.appendChild(content);
    dialog.appendChild(actions);
    document.body.appendChild(dialog);
    
    // 添加关闭事件
    document.getElementById('closeDialog').addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
    });
    
    // 点击外部关闭
    const overlay = document.createElement('div');
    overlay.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0,0,0,0.5);
      z-index: 9998;
    `;
    document.body.appendChild(overlay);
    
    overlay.addEventListener('click', () => {
      document.body.removeChild(dialog);
      document.body.removeChild(overlay);
    });
  } catch (err) {
    // 如果剪贴板API失败，退回到alert
    alert("当前范围的指标值:\n\n" + textOutput);
  }
};
</script>
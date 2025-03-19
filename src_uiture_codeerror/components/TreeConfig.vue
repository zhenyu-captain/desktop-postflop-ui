<template>
  <div v-if="!isEditMode" class="tree-config">
    <div class="flex">
      <div class="config-container shrink-0">
        <!-- 基础配置部分 -->
        <div class="config-panel">
          <h2 class="panel-title">Basic Settings</h2>
          <div class="grid grid-cols-2 gap-x-8 gap-y-3">
            <div class="form-group">
              <label for="starting-pot">Starting pot:</label>
              <input
                id="starting-pot"
                v-model="config.startingPot"
                type="number"
                :class="[
                  'form-input',
                  (config.startingPot <= 0 ||
                  config.startingPot > MAX_AMOUNT ||
                  config.startingPot % 1 !== 0)
                    ? 'input-error'
                    : ''
                ]"
                :disabled="hasEdit"
                min="0"
                :max="MAX_AMOUNT"
              />
            </div>

            <div class="form-group">
              <label for="effective-stack">Effective stack:</label>
              <input
                id="effective-stack"
                v-model="config.effectiveStack"
                type="number"
                :class="[
                  'form-input',
                  (config.effectiveStack <= 0 ||
                  config.effectiveStack > MAX_AMOUNT ||
                  config.effectiveStack % 1 !== 0)
                    ? 'input-error'
                    : ''
                ]"
                :disabled="hasEdit"
                min="0"
                :max="MAX_AMOUNT"
              />
            </div>

            <div class="form-group">
              <label for="rake-percent">Rake:</label>
              <div class="flex items-center">
                <input
                  id="rake-percent"
                  v-model="config.rakePercent"
                  type="number"
                  :class="[
                    'form-input',
                    (config.rakePercent < 0 || config.rakePercent > 100)
                      ? 'input-error'
                      : ''
                  ]"
                  :disabled="hasEdit"
                  min="0"
                  max="100"
                  step="0.5"
                />
                
              </div>
            </div>

            <div class="form-group">
              <label for="rake-cap">Rake cap:</label>
              <input
                id="rake-cap"
                v-model="config.rakeCap"
                type="number"
                :class="[
                  'form-input',
                  (config.rakeCap < 0 || config.rakeCap > 3 * MAX_AMOUNT)
                    ? 'input-error'
                    : ''
                ]"
                :disabled="hasEdit"
                min="0"
                :max="3 * MAX_AMOUNT"
              />
            </div>
          </div>

          <div
            v-if="errorBasics.length > 0"
            class="error-message mt-2"
          >
            <div class="font-semibold underline">Error:</div>
            <div class="ml-2">
              <div v-for="error in errorBasics" :key="error">
                {{ error }}
              </div>
            </div>
          </div>
        </div>

        <!-- OOP Bet Sizes 部分 -->
        <div class="config-panel mt-5">
          <div class="flex justify-between items-center">
            <h2 class="panel-title">OOP Bet Sizes</h2>
            <label class="flex items-center">
              <input
                v-model="config.donkOption"
                type="checkbox"
                class="mr-2 h-4 w-4 rounded cursor-pointer peer"
                :disabled="hasEdit"
              />
              <span class="cursor-pointer text-sm peer-disabled:cursor-not-allowed">
                Use different sizes for donk bets
              </span>
            </label>
          </div>
          
          <div class="mt-2 text-right">
            <Tippy
              class="inline-block cursor-help text-gray-600"
              max-width="500px"
              trigger="mouseenter click"
              placement="bottom"
              :delay="[200, 0]"
              :interactive="true"
            >
              <QuestionMarkCircleIcon class="inline w-5 h-5" />
              <span class="text-sm underline ml-1">How to input</span>
              <template #content>
                <div class="px-1 py-0.5 text-justify">
                  Multiple bet sizes can be input, separated by commas or
                  spaces, in any of the following sizes. If left blank, no bet
                  or raise will be made.
                  <ul class="pl-6 list-disc">
                    <li class="mt-1">
                      A number representing a percentage of the pot (e.g.,
                      "50"). For raises, the size is calculated by first
                      calling and then combining a bet of the specified
                      percentage. For example, if the pot is 100 before the
                      bet, and the opponent bets 75, a raise of 50% would
                      result in a bet of 75 + (100 + 75 + 75) * 50% = 200.
                    </li>
                    <li class="mt-1">
                      A multiple of the previous bet size (e.g., "2.5x"). Only
                      valid for raises.
                    </li>
                    <li class="mt-1">All-in (e.g., "a").</li>
                    <li class="mt-1">
                      Constant addition (e.g., "100c"). For raises, you can
                      also specify a cap on the number of raises (e.g.,
                      "20c3r").
                    </li>
                    <li class="mt-1">
                      Geometric size, i.e., dividing the remaining stack into
                      a specified number of equal percentage bets (e.g.,
                      "3e"). For example, if the current pot is 100 and the
                      effective stack is 400, "2e" would result in a bet of
                      100. If the number before the "e" is omitted, the number
                      of remaining streets is used (flop=3, turn=2, river=1).
                      For raises, the number of previous raises is subtracted
                      from the specified number. For example, "3e" will change
                      to "2e" on re-raises. You can also specify a maximum
                      percentage limit by adding a number after the "e" (e.g.,
                      "2e200").
                    </li>
                  </ul>
                </div>
              </template>
            </Tippy>
          </div>

          <div class="grid grid-cols-3 gap-4 mt-3">
            <!-- Flop Column -->
            <div class="street-column">
              <h3 class="street-title">Flop</h3>
              <div class="form-group">
                <label for="oop-flop-bet">Bet:</label>
                <div class="flex items-center">
                  <input
                    id="oop-flop-bet"
                    v-model="config.oopFlopBet"
                    type="text"
                    :class="[
                      'form-input',
                      !config.oopFlopBetSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.oopFlopBetSanitized.valid &&
                        (config.oopFlopBet = config.oopFlopBetSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
              <div class="form-group">
                <label for="oop-flop-raise">Raise:</label>
                <div class="flex items-center">
                  <input
                    id="oop-flop-raise"
                    v-model="config.oopFlopRaise"
                    type="text"
                    :class="[
                      'form-input',
                      !config.oopFlopRaiseSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.oopFlopRaiseSanitized.valid &&
                        (config.oopFlopRaise = config.oopFlopRaiseSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
            </div>

            <!-- Turn Column -->
            <div class="street-column">
              <h3 class="street-title">Turn</h3>
              <div class="form-group">
                <label for="oop-turn-bet">Bet:</label>
                <div class="flex items-center">
                  <input
                    id="oop-turn-bet"
                    v-model="config.oopTurnBet"
                    type="text"
                    :class="[
                      'form-input',
                      !config.oopTurnBetSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.oopTurnBetSanitized.valid &&
                        (config.oopTurnBet = config.oopTurnBetSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
              <div class="form-group">
                <label for="oop-turn-raise">Raise:</label>
                <div class="flex items-center">
                  <input
                    id="oop-turn-raise"
                    v-model="config.oopTurnRaise"
                    type="text"
                    :class="[
                      'form-input',
                      !config.oopTurnRaiseSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.oopTurnRaiseSanitized.valid &&
                        (config.oopTurnRaise = config.oopTurnRaiseSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
              <div v-if="config.donkOption" class="form-group">
                <label for="oop-turn-donk">Donk:</label>
                <div class="flex items-center">
                  <input
                    id="oop-turn-donk"
                    v-model="config.oopTurnDonk"
                    type="text"
                    :class="[
                      'form-input',
                      !config.oopTurnDonkSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.oopTurnDonkSanitized.valid &&
                        (config.oopTurnDonk = config.oopTurnDonkSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
            </div>

            <!-- River Column -->
            <div class="street-column">
              <h3 class="street-title">River</h3>
              <div class="form-group">
                <label for="oop-river-bet">Bet:</label>
                <div class="flex items-center">
                  <input
                    id="oop-river-bet"
                    v-model="config.oopRiverBet"
                    type="text"
                    :class="[
                      'form-input',
                      !config.oopRiverBetSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.oopRiverBetSanitized.valid &&
                        (config.oopRiverBet = config.oopRiverBetSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
              <div class="form-group">
                <label for="oop-river-raise">Raise:</label>
                <div class="flex items-center">
                  <input
                    id="oop-river-raise"
                    v-model="config.oopRiverRaise"
                    type="text"
                    :class="[
                      'form-input',
                      !config.oopRiverRaiseSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.oopRiverRaiseSanitized.valid &&
                        (config.oopRiverRaise = config.oopRiverRaiseSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
              <div v-if="config.donkOption" class="form-group">
                <label for="oop-river-donk">Donk:</label>
                <div class="flex items-center">
                  <input
                    id="oop-river-donk"
                    v-model="config.oopRiverDonk"
                    type="text"
                    :class="[
                      'form-input',
                      !config.oopRiverDonkSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.oopRiverDonkSanitized.valid &&
                        (config.oopRiverDonk = config.oopRiverDonkSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- IP Bet Sizes 部分 -->
        <div class="config-panel mt-5">
          <div class="flex justify-between items-center">
            <h2 class="panel-title">IP Bet Sizes</h2>
            <div class="flex items-center gap-2">
              <button
                class="button-action"
                :disabled="errorIp.length > 0 || hasEdit"
                @click="ipToOop"
                title="Copy IP sizes to OOP"
              >
                <span>↑</span>
              </button>
              <button
                class="button-action"
                :disabled="errorOop.length > 0 || hasEdit"
                @click="oopToIp"
                title="Copy OOP sizes to IP"
              >
                <span>↓</span>
              </button>
            </div>
          </div>

          <div class="grid grid-cols-3 gap-4 mt-3">
            <!-- Flop Column -->
            <div class="street-column">
              <h3 class="street-title">Flop</h3>
              <div class="form-group">
                <label for="ip-flop-bet">Bet:</label>
                <div class="flex items-center">
                  <input
                    id="ip-flop-bet"
                    v-model="config.ipFlopBet"
                    type="text"
                    :class="[
                      'form-input',
                      !config.ipFlopBetSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.ipFlopBetSanitized.valid &&
                        (config.ipFlopBet = config.ipFlopBetSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
              <div class="form-group">
                <label for="ip-flop-raise">Raise:</label>
                <div class="flex items-center">
                  <input
                    id="ip-flop-raise"
                    v-model="config.ipFlopRaise"
                    type="text"
                    :class="[
                      'form-input',
                      !config.ipFlopRaiseSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.ipFlopRaiseSanitized.valid &&
                        (config.ipFlopRaise = config.ipFlopRaiseSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
            </div>

            <!-- Turn Column -->
            <div class="street-column">
              <h3 class="street-title">Turn</h3>
              <div class="form-group">
                <label for="ip-turn-bet">Bet:</label>
                <div class="flex items-center">
                  <input
                    id="ip-turn-bet"
                    v-model="config.ipTurnBet"
                    type="text"
                    :class="[
                      'form-input',
                      !config.ipTurnBetSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.ipTurnBetSanitized.valid &&
                        (config.ipTurnBet = config.ipTurnBetSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
              <div class="form-group">
                <label for="ip-turn-raise">Raise:</label>
                <div class="flex items-center">
                  <input
                    id="ip-turn-raise"
                    v-model="config.ipTurnRaise"
                    type="text"
                    :class="[
                      'form-input',
                      !config.ipTurnRaiseSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.ipTurnRaiseSanitized.valid &&
                        (config.ipTurnRaise = config.ipTurnRaiseSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
            </div>

            <!-- River Column -->
            <div class="street-column">
              <h3 class="street-title">River</h3>
              <div class="form-group">
                <label for="ip-river-bet">Bet:</label>
                <div class="flex items-center">
                  <input
                    id="ip-river-bet"
                    v-model="config.ipRiverBet"
                    type="text"
                    :class="[
                      'form-input',
                      !config.ipRiverBetSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.ipRiverBetSanitized.valid &&
                        (config.ipRiverBet = config.ipRiverBetSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
              <div class="form-group">
                <label for="ip-river-raise">Raise:</label>
                <div class="flex items-center">
                  <input
                    id="ip-river-raise"
                    v-model="config.ipRiverRaise"
                    type="text"
                    :class="[
                      'form-input',
                      !config.ipRiverRaiseSanitized.valid ? 'input-error' : ''
                    ]"
                    :disabled="hasEdit"
                    @change="
                      config.ipRiverRaiseSanitized.valid &&
                        (config.ipRiverRaise = config.ipRiverRaiseSanitized.s)
                    "
                  />
                  <span class="ml-2">%</span>
                </div>
              </div>
            </div>
          </div>

          <div
            v-if="errorOop.length > 0 || errorIp.length > 0"
            class="error-message mt-2"
          >
            <div class="font-semibold underline">Error:</div>
            <div class="ml-2">
              <div v-for="error in errorOop" :key="error">
                {{ error }}
              </div>
              <div v-for="error in errorIp" :key="error">
                {{ error }}
              </div>
            </div>
          </div>
        </div>

        <!-- Advanced Options -->
        <div class="config-panel mt-5">
          <h2 class="panel-title">Advanced Settings</h2>
          <div class="grid grid-cols-3 gap-4">
            <div class="form-group">
              <label for="all-in-threshold" class="flex items-start">
                <span>Add all-in threshold:</span>
                <Tippy
                  class="ml-1 cursor-help text-gray-600"
                  max-width="420px"
                  trigger="mouseenter click"
                  :delay="[200, 0]"
                  :interactive="true"
                >
                  <QuestionMarkCircleIcon class="w-4 h-4" />
                  <template #content>
                    <div class="px-1 py-0.5 text-justify">
                      Add all-in action to all spots where the ratio of the
                      maximum possible betting size to the pot is less than this
                      threshold.
                    </div>
                  </template>
                </Tippy>
              </label>
              <div class="flex items-center">
                <input
                  id="all-in-threshold"
                  v-model="config.addAllInThreshold"
                  type="number"
                  :class="[
                    'form-input',
                    config.addAllInThreshold < 0 ? 'input-error' : ''
                  ]"
                  :disabled="hasEdit"
                  min="0"
                  :max="MAX_AMOUNT * 100"
                />
                <span class="ml-2">%</span>
              </div>
            </div>

            <div class="form-group">
              <label for="force-all-in-threshold" class="flex items-start">
                <span>Force all-in threshold:</span>
                <Tippy
                  class="ml-1 cursor-help text-gray-600"
                  max-width="500px"
                  trigger="mouseenter click"
                  :delay="[200, 0]"
                  :interactive="true"
                >
                  <QuestionMarkCircleIcon class="w-4 h-4" />
                  <template #content>
                    <div class="px-1 py-0.5 text-justify">
                      <div>
                        Replace the betting action with an all-in action if the
                        SPR (Stack/Pot Ratio) is below this threshold after the
                        betting action is called by the opponent. The
                        recommended value is around 15% - 20%.
                      </div>
                      <div class="mt-3">
                        This option is similar to the "all-in threshold" in
                        PioSOLVER. PioSOLVER will replace the betting action
                        with an all-in if the percentage of one's pot commitment
                        to the initial stack exceeds the threshold.
                      </div>
                      <div class="mt-3">
                        The conversion formula, when ignoring decimal rounding,
                        is as follows (s = initial SPR, r = PioSOLVER's
                        threshold):
                      </div>
                      <div class="mt-1 text-center">
                        Threshold = s * (1 - r) / (1 + 2 * s * r).
                      </div>
                    </div>
                  </template>
                </Tippy>
              </label>
              <div class="flex items-center">
                <input
                  id="force-all-in-threshold"
                  v-model="config.forceAllInThreshold"
                  type="number"
                  :class="[
                    'form-input',
                    config.forceAllInThreshold < 0 ? 'input-error' : ''
                  ]"
                  :disabled="hasEdit"
                  min="0"
                  :max="MAX_AMOUNT * 100"
                />
                <span class="ml-2">%</span>
              </div>
            </div>

            <div class="form-group">
              <label for="merging-threshold" class="flex items-start">
                <span>Merging threshold:</span>
                <Tippy
                  class="ml-1 cursor-help text-gray-600"
                  max-width="500px"
                  trigger="mouseenter click"
                  :delay="[200, 0]"
                  :interactive="true"
                >
                  <QuestionMarkCircleIcon class="w-4 h-4" />
                  <template #content>
                    <div class="px-1 py-0.5 text-justify">
                      <div>
                        Merge betting actions if there are betting actions of
                        similar size.
                      </div>
                      <div class="mt-3">
                        The algorithm is the same as in PioSOLVER. That is,
                        select the largest bet size (= X% of the pot) and remove
                        all other betting actions with sizes (= Y% of the pot)
                        that satisfy the following inequality:
                      </div>
                      <div class="my-1 text-center">
                        (100 + X) / (100 + Y) &lt; 1.0 + Threshold.
                      </div>
                      <div>
                        Repeat this process with the next largest bet size among
                        the remaining bets.
                      </div>
                    </div>
                  </template>
                </Tippy>
              </label>
              <div class="flex items-center">
                <input
                  id="merging-threshold"
                  v-model="config.mergingThreshold"
                  type="number"
                  :class="[
                    'form-input',
                    config.mergingThreshold < 0 ? 'input-error' : ''
                  ]"
                  :disabled="hasEdit"
                  min="0"
                  :max="MAX_AMOUNT * 100"
                />
                <span class="ml-2">%</span>
              </div>
            </div>
          </div>

          <div
            v-if="errorMisc.length > 0"
            class="error-message mt-2"
          >
            <div class="font-semibold underline">Error:</div>
            <div class="ml-2">
              <div v-for="error in errorMisc" :key="error">
                {{ error }}
              </div>
            </div>
          </div>

          <div
            v-if="warningMisc.length > 0"
            class="warning-message mt-2"
          >
            <div class="font-semibold underline">Warning:</div>
            <div class="ml-2">
              <div
                v-for="warning in warningMisc"
                :key="warning"
                class="whitespace-pre-line"
              >
                {{ warning }}
              </div>
            </div>
          </div>
        </div>

        <!-- Actions Section -->
        <div class="flex justify-between items-center mt-5">
          <div class="flex gap-2">
            <button
              class="button-secondary"
              @click="clearConfig"
            >
              Clear Configuration
            </button>
            <button 
              @click="toggleDebugInfo"
              class="px-3 py-1 text-sm bg-yellow-500 hover:bg-yellow-600 text-white rounded">
              Debug
            </button>
          </div>

          <div class="flex gap-3">
            <button
              v-if="hasEdit"
              class="button-danger"
              @click="clearEdit"
            >
              Clear Edit & Unlock
            </button>
            
            <button
              class="button-primary"
              :disabled="!isInputValid"
              @click="startEdit"
            >
              Preview & Edit Tree
            </button>
          </div>
        </div>

        <!-- Debug信息框 -->
        <div v-if="showDebugInfo" class="p-4 bg-gray-100 rounded-lg text-xs overflow-auto max-h-80 mt-4">
          <h3 class="font-bold mb-2">Tree Config Debug信息:</h3>
          <div class="grid grid-cols-1 gap-2">
            <div>
              <h4 class="font-semibold">config状态:</h4>
              <pre>{{ config }}</pre>
            </div>
            <div>
              <h4 class="font-semibold">savedConfig状态:</h4>
              <pre>{{ savedConfig }}</pre>
            </div>
            <div>
              <h4 class="font-semibold">tmpConfig状态:</h4>
              <pre>{{ tmpConfig }}</pre>
            </div>
            <div>
              <h4 class="font-semibold">编辑状态:</h4>
              <pre>{{ {
                isEditMode,
                hasEdit,
                expectedBoardLength: config.expectedBoardLength,
                addedLines: config.addedLines,
                removedLines: config.removedLines
              } }}</pre>
            </div>
          </div>
          <div class="mt-2">
            <button class="px-3 py-1 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded" @click="refreshDebugInfo">刷新调试信息</button>
            <button class="px-3 py-1 text-xs bg-green-500 hover:bg-green-600 text-white rounded ml-2" @click="syncConfigData">同步配置数据</button>
            <button class="px-3 py-1 text-xs bg-red-500 hover:bg-red-600 text-white rounded ml-2" @click="logDebugInfo">控制台输出</button>
          </div>
        </div>

        <div
          v-if="errorLines.length > 0"
          class="error-message mt-2"
        >
          <div class="font-semibold underline">Error:</div>
          <div class="ml-2">
            <div v-for="error in errorLines" :key="error">
              {{ error }}
            </div>
          </div>
        </div>
      </div>

      <div class="config-sidebar ml-6">
        <DbItemPicker
          store-name="configurations"
          :value="dbValue"
          :allow-save="isInputValid"
          @load-item="loadConfig"
        />
      </div>
    </div>
  </div>

  <div v-else>
    <Suspense>
      <template #default>
        <TreeEditor @save="saveEdit" @cancel="cancelEdit" />
      </template>
      <template #fallback>
        <div class="flex items-center justify-center h-40">
          <div class="text-lg">Loading...</div>
        </div>
      </template>
    </Suspense>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useStore, useConfigStore, useSavedConfigStore, useTmpConfigStore } from "../store";
import {
  MAX_AMOUNT,
  sanitizeBetString,
  ROOT_LINE_STRING,
  INVALID_LINE_STRING,
  readableLineString,
} from "../utils";

import DbItemPicker from "./DbItemPicker.vue";
import TreeEditor from "./TreeEditor.vue";
import { Tippy } from "vue-tippy";
import { QuestionMarkCircleIcon } from "@heroicons/vue/20/solid";

type ConfigValue = {
  startingPot: number;
  effectiveStack: number;
  rakePercent: number;
  rakeCap: number;
  donkOption: number;
  oopFlopBet: string;
  oopFlopRaise: string;
  oopTurnBet: string;
  oopTurnRaise: string;
  oopTurnDonk: string;
  oopRiverBet: string;
  oopRiverRaise: string;
  oopRiverDonk: string;
  ipFlopBet: string;
  ipFlopRaise: string;
  ipTurnBet: string;
  ipTurnRaise: string;
  ipRiverBet: string;
  ipRiverRaise: string;
  addAllInThreshold: number;
  forceAllInThreshold: number;
  mergingThreshold: number;
  expectedBoardLength: number;
  addedLines: string;
  removedLines: string;
};

const store = useStore();
const config = useConfigStore();
const savedConfig = useSavedConfigStore();
const tmpConfig = useTmpConfigStore();

const isEditMode = ref(false);

const hasEdit = computed(
  () => config.addedLines.length > 0 || config.removedLines.length > 0
);

const addedLinesArray = computed(() =>
  config.addedLines === ""
    ? []
    : config.addedLines.split(",").map(readableLineString)
);

const removedLinesArray = computed(() =>
  config.removedLines === ""
    ? []
    : config.removedLines.split(",").map(readableLineString)
);

const errorBasics = computed(() => {
  const errors: string[] = [];
  if (config.startingPot <= 0) {
    errors.push("Starting pot must be positive");
  }
  if (config.startingPot > MAX_AMOUNT) {
    errors.push(`Starting pot must not exceed ${MAX_AMOUNT}`);
  }
  if (config.startingPot % 1 !== 0) {
    errors.push("Starting pot must be an integer");
  }
  if (config.effectiveStack <= 0) {
    errors.push("Effective stack must be positive");
  }
  if (config.effectiveStack > MAX_AMOUNT) {
    errors.push(`Effective stack must not exceed ${MAX_AMOUNT}`);
  }
  if (config.effectiveStack % 1 !== 0) {
    errors.push("Effective stack must be an integer");
  }
  if (config.rakePercent < 0) {
    errors.push("Rake must be non-negative");
  }
  if (config.rakePercent > 100) {
    errors.push("Rake must not exceed 100%");
  }
  if (config.rakeCap < 0) {
    errors.push("Rake cap must be non-negative");
  }
  if (config.rakeCap > 3 * MAX_AMOUNT) {
    errors.push(`Rake cap must not exceed ${3 * MAX_AMOUNT}`);
  }
  return errors;
});

const errorOop = computed(() => {
  const errors: string[] = [];
  const betConfig = [
    { name: "OOP flop bet", res: config.oopFlopBetSanitized },
    { name: "OOP flop raise", res: config.oopFlopRaiseSanitized },
    { name: "OOP turn bet", res: config.oopTurnBetSanitized },
    { name: "OOP turn raise", res: config.oopTurnRaiseSanitized },
    { name: "OOP turn donk", res: config.oopTurnDonkSanitized, isDonk: true },
    { name: "OOP river bet", res: config.oopRiverBetSanitized },
    { name: "OOP river raise", res: config.oopRiverRaiseSanitized },
    { name: "OOP river donk", res: config.oopRiverDonkSanitized, isDonk: true },
  ];
  for (const { name, res, isDonk } of betConfig) {
    if (!res.valid && (!isDonk || config.donkOption)) {
      errors.push(`${name}: ${res.s}`);
    }
  }
  return errors;
});

const errorIp = computed(() => {
  const errors: string[] = [];
  const betConfig = [
    { name: "IP flop bet", res: config.ipFlopBetSanitized },
    { name: "IP flop raise", res: config.ipFlopRaiseSanitized },
    { name: "IP turn bet", res: config.ipTurnBetSanitized },
    { name: "IP turn raise", res: config.ipTurnRaiseSanitized },
    { name: "IP river bet", res: config.ipRiverBetSanitized },
    { name: "IP river raise", res: config.ipRiverRaiseSanitized },
  ];
  for (const { name, res } of betConfig) {
    if (!res.valid) {
      errors.push(`${name}: ${res.s}`);
    }
  }
  return errors;
});

const errorMisc = computed(() => {
  const errors: string[] = [];
  if (config.addAllInThreshold < 0) {
    errors.push("Add all-in threshold must be non-negative");
  }
  if (config.forceAllInThreshold < 0) {
    errors.push("Force all-in threshold must be non-negative");
  }
  if (config.mergingThreshold < 0) {
    errors.push("Merging threshold must be non-negative");
  }
  return errors;
});

const warningMisc = computed(() => {
  const warnings: string[] = [];
  if (config.forceAllInThreshold > 30) {
    warnings.push(
      "Force all-in threshold higher than 30% is not recommended.\nPlease see help to confirm the meaning."
    );
  }
  return warnings;
});

const errorLines = computed(() => {
  const errors: string[] = [];
  if (
    addedLinesArray.value.includes(ROOT_LINE_STRING) ||
    addedLinesArray.value.includes(INVALID_LINE_STRING) ||
    removedLinesArray.value.includes(ROOT_LINE_STRING) ||
    removedLinesArray.value.includes(INVALID_LINE_STRING)
  ) {
    errors.push("Invalid line found (loaded broken configurations?)");
  }
  if (
    ![0, 3, 4, 5].includes(config.expectedBoardLength) ||
    (config.expectedBoardLength === 0 &&
      (addedLinesArray.value.length > 0 ||
        removedLinesArray.value.length > 0)) ||
    (config.expectedBoardLength > 0 &&
      addedLinesArray.value.length === 0 &&
      removedLinesArray.value.length === 0)
  ) {
    errors.push("Invalid configurations (loaded broken configurations?)");
  }
  return errors;
});

const isInputValid = computed(
  () =>
    errorBasics.value.length === 0 &&
    errorOop.value.length === 0 &&
    errorIp.value.length === 0 &&
    errorMisc.value.length === 0 &&
    errorLines.value.length === 0
);

const clearConfig = () => {
  config.startingPot = 0;
  config.effectiveStack = 0;
  config.rakePercent = 0;
  config.rakeCap = 0;
  config.donkOption = false;
  config.oopFlopBet = "";
  config.oopFlopRaise = "";
  config.oopTurnBet = "";
  config.oopTurnRaise = "";
  config.oopTurnDonk = "";
  config.oopRiverBet = "";
  config.oopRiverRaise = "";
  config.oopRiverDonk = "";
  config.ipFlopBet = "";
  config.ipFlopRaise = "";
  config.ipTurnBet = "";
  config.ipTurnRaise = "";
  config.ipRiverBet = "";
  config.ipRiverRaise = "";
  config.addAllInThreshold = 0;
  config.forceAllInThreshold = 0;
  config.mergingThreshold = 0;
  config.expectedBoardLength = 0;
  config.addedLines = "";
  config.removedLines = "";
};

const oopToIp = () => {
  config.ipFlopBet = config.oopFlopBet;
  config.ipFlopRaise = config.oopFlopRaise;
  config.ipTurnBet = config.oopTurnBet;
  config.ipTurnRaise = config.oopTurnRaise;
  config.ipRiverBet = config.oopRiverBet;
  config.ipRiverRaise = config.oopRiverRaise;
};

const ipToOop = () => {
  config.oopFlopBet = config.ipFlopBet;
  config.oopFlopRaise = config.ipFlopRaise;
  config.oopTurnBet = config.ipTurnBet;
  config.oopTurnRaise = config.ipTurnRaise;
  config.oopRiverBet = config.ipRiverBet;
  config.oopRiverRaise = config.ipRiverRaise;
};

const dbValue = computed(
  (): ConfigValue => ({
    startingPot: config.startingPot,
    effectiveStack: config.effectiveStack,
    rakePercent: config.rakePercent,
    rakeCap: config.rakeCap,
    donkOption: Number(config.donkOption),
    oopFlopBet: config.oopFlopBet,
    oopFlopRaise: config.oopFlopRaise,
    oopTurnBet: config.oopTurnBet,
    oopTurnRaise: config.oopTurnRaise,
    oopTurnDonk: config.donkOption ? config.oopTurnDonk : "",
    oopRiverBet: config.oopRiverBet,
    oopRiverRaise: config.oopRiverRaise,
    oopRiverDonk: config.donkOption ? config.oopRiverDonk : "",
    ipFlopBet: config.ipFlopBet,
    ipFlopRaise: config.ipFlopRaise,
    ipTurnBet: config.ipTurnBet,
    ipTurnRaise: config.ipTurnRaise,
    ipRiverBet: config.ipRiverBet,
    ipRiverRaise: config.ipRiverRaise,
    addAllInThreshold: config.addAllInThreshold,
    forceAllInThreshold: config.forceAllInThreshold,
    mergingThreshold: config.mergingThreshold,
    expectedBoardLength: config.expectedBoardLength,
    addedLines: config.addedLines,
    removedLines: config.removedLines,
  })
);

const loadConfig = (value: unknown) => {
  const configValue = value as ConfigValue;
  config.startingPot = Number(configValue.startingPot);
  config.effectiveStack = Number(configValue.effectiveStack);
  config.rakePercent = Number(configValue.rakePercent);
  config.rakeCap = Number(configValue.rakeCap);
  config.donkOption = Boolean(configValue.donkOption);
  config.addAllInThreshold = Number(configValue.addAllInThreshold);
  config.forceAllInThreshold = Number(configValue.forceAllInThreshold);
  config.mergingThreshold = Number(configValue.mergingThreshold);
  config.expectedBoardLength = Number(configValue.expectedBoardLength);
  config.addedLines = String(configValue.addedLines);
  config.removedLines = String(configValue.removedLines);

  const betMembers = [
    "oopFlopBet",
    "oopFlopRaise",
    "oopTurnBet",
    "oopTurnRaise",
    "oopTurnDonk",
    "oopRiverBet",
    "oopRiverRaise",
    "oopRiverDonk",
    "ipFlopBet",
    "ipFlopRaise",
    "ipTurnBet",
    "ipTurnRaise",
    "ipRiverBet",
    "ipRiverRaise",
  ] as const;

  for (const member of betMembers) {
    const str = String(configValue[member]);
    const sanitized = sanitizeBetString(str, member.endsWith("Raise"));
    config[member] = sanitized.valid ? sanitized.s : str;
  }
};

const startEdit = () => {
  isEditMode.value = true;
  if (config.expectedBoardLength === 0) {
    config.expectedBoardLength = Math.max(config.board.length, 3);
  }
  store.headers["tree-config"].push("Tree Preview & Edit");
};

const clearEdit = () => {
  config.expectedBoardLength = 0;
  config.addedLines = "";
  config.removedLines = "";
};

const saveEdit = (addedLines: string, removedLines: string) => {
  isEditMode.value = false;
  config.addedLines = addedLines;
  config.removedLines = removedLines;
  if (config.addedLines === "" && config.removedLines === "") {
    config.expectedBoardLength = 0;
  }
  store.headers["tree-config"].pop();
};

const cancelEdit = () => {
  isEditMode.value = false;
  if (config.addedLines === "" && config.removedLines === "") {
    config.expectedBoardLength = 0;
  }
  store.headers["tree-config"].pop();
};

// 调试相关状态
const showDebugInfo = ref(false)
const debugInfo = ref({})

// 切换调试信息显示状态
const toggleDebugInfo = () => {
  showDebugInfo.value = !showDebugInfo.value
  if (showDebugInfo.value) {
    refreshDebugInfo()
  }
}

// 刷新调试信息
const refreshDebugInfo = () => {
  debugInfo.value = {
    config: { ...config },
    savedConfig: { ...savedConfig },
    tmpConfig: { ...tmpConfig },
    editState: {
      isEditMode: isEditMode.value,
      hasEdit: hasEdit.value,
      expectedBoardLength: config.expectedBoardLength,
      addedLines: config.addedLines,
      removedLines: config.removedLines
    }
  }
}

// 同步配置数据
const syncConfigData = () => {
  // 同步三个store的配置数据
  const configData = {
    startingPot: config.startingPot,
    effectiveStack: config.effectiveStack,
    rakePercent: config.rakePercent,
    rakeCap: config.rakeCap,
    donkOption: config.donkOption,
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
    addAllInThreshold: config.addAllInThreshold,
    forceAllInThreshold: config.forceAllInThreshold,
    mergingThreshold: config.mergingThreshold,
    expectedBoardLength: config.expectedBoardLength,
    addedLines: config.addedLines,
    removedLines: config.removedLines
  }
  
  // 更新savedConfig和tmpConfig
  Object.assign(savedConfig, configData)
  Object.assign(tmpConfig, configData)
  
  refreshDebugInfo()
  console.log('已同步配置数据:', {
    config: { ...config },
    savedConfig: { ...savedConfig },
    tmpConfig: { ...tmpConfig }
  })
}

// 在控制台输出更详细的调试信息
const logDebugInfo = () => {
  console.log("=== TREE CONFIG DEBUG INFO ===")
  console.log("config:", { ...config })
  console.log("savedConfig:", { ...savedConfig })
  console.log("tmpConfig:", { ...tmpConfig })
  console.log("编辑状态:", {
    isEditMode: isEditMode.value,
    hasEdit: hasEdit.value,
    expectedBoardLength: config.expectedBoardLength,
    addedLines: config.addedLines,
    removedLines: config.removedLines
  })
  console.log("=======================")
}
</script>

<style scoped>
.tree-config {
  @apply text-gray-800;
}

.config-container {
  @apply w-3/4;
}

.config-sidebar {
  @apply w-1/4 flex-grow max-w-xs;
}

.config-panel {
  @apply bg-white rounded-md p-5 border border-gray-200 shadow-sm mb-6;
}

.panel-title {
  @apply text-base font-semibold mb-4 text-gray-800 pb-2 border-b border-gray-200;
  letter-spacing: 0.01em;
}

.street-title {
  @apply font-medium text-sm mb-2 text-gray-700 text-center pb-1 border-b border-gray-200;
}

.street-column {
  @apply bg-gray-50 rounded-md p-3 border border-gray-200;
}

.form-group {
  @apply mb-3;
}

.form-group label {
  @apply block text-sm font-medium text-gray-600 mb-1;
}

.form-input {
  @apply w-full px-2.5 py-1.5 rounded-md text-sm border border-gray-300 
         focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500 
         disabled:bg-gray-100 disabled:text-gray-500 disabled:cursor-not-allowed;
}

.input-error {
  @apply border-red-500 focus:ring-red-300 focus:border-red-500;
  background-color: rgba(254, 226, 226, 0.1);
}

.error-message {
  @apply flex text-red-600 text-sm bg-red-50 p-2.5 rounded-md border border-red-200;
}

.warning-message {
  @apply flex text-amber-600 text-sm bg-amber-50 p-2.5 rounded-md border border-amber-200;
}

/* 主按钮 - 使用品牌主色 */
.button-primary {
  @apply px-4 py-2 bg-indigo-600 text-white font-medium rounded-md 
         hover:bg-indigo-700
         focus:outline-none focus:ring-2 focus:ring-indigo-400 focus:ring-offset-1
         disabled:opacity-50 disabled:cursor-not-allowed;
}

/* 次要按钮 - 轮廓风格 */
.button-secondary {
  @apply px-4 py-2 bg-white text-gray-700 font-medium rounded-md 
         hover:bg-gray-50 
         focus:outline-none focus:ring-2 focus:ring-gray-300 
         disabled:opacity-50 disabled:cursor-not-allowed
         border border-gray-300;
}

/* 危险按钮 */
.button-danger {
  @apply px-4 py-2 bg-red-600 text-white font-medium rounded-md 
         hover:bg-red-700
         focus:outline-none focus:ring-2 focus:ring-red-400 focus:ring-offset-1 
         disabled:opacity-50 disabled:cursor-not-allowed;
}

/* 操作按钮 */
.button-action {
  @apply flex items-center justify-center w-8 h-8 bg-white text-gray-600 rounded-md 
         hover:bg-gray-100 hover:text-gray-900 
         focus:outline-none focus:ring-2 focus:ring-gray-400 
         disabled:opacity-50 disabled:cursor-not-allowed 
         border border-gray-300;
}

/* 添加/删除行显示区域 */
.lines-display {
  @apply mt-4 p-3 bg-gray-50 border border-gray-200 rounded-md;
}

/* 创建平衡的间距和边距 */
.mt-5 {
  @apply mt-4;
}

.gap-4 {
  @apply gap-3;
}

/* 提升提示气泡的视觉效果 */
:deep(.tippy-box) {
  @apply shadow-md rounded-md border border-gray-200;
}

:deep(.tippy-content) {
  @apply p-2.5 text-gray-700;
}

/* tooltip图标样式 */
.w-4.h-4, .w-5.h-5 {
  @apply text-gray-500 hover:text-indigo-600;
}

/* 强调当前操作的表单区域 */
.form-input:focus {
  @apply bg-white;
  box-shadow: 0 0 0 2px rgba(79, 70, 229, 0.1);
}

/* 街道列标题样式改进 */
.street-title {
  @apply text-indigo-700 font-medium;
}

/* 改善面板间的视觉区分 */
.config-panel + .config-panel {
  @apply mt-5;
}

/* 针对按钮组的样式 */
.flex.justify-between.items-center.mt-5 {
  @apply pt-2 border-t border-gray-200;
}

/* 增强表单组的可用性 */
.form-group:hover:not(:has(input:disabled)) label {
  @apply text-indigo-600;
}
</style>

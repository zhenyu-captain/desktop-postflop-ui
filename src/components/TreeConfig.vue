<template>
  <div v-if="!store.treeEditMode" class="tree-config">
    <div class="flex">
      <div class="config-container shrink-0 w-full">
        <!-- Main Two Column Layout -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-5 mb-5">
          <!-- Left Column (2/3) -->
          <div class="md:col-span-2 space-y-5">
            <!-- Basic Settings Card -->
            <div class="bg-white rounded-lg shadow-lg p-5">
              <h2 class="text-lg font-medium mb-4">Basic Settings</h2>
              <div class="grid grid-cols-2 gap-x-8 gap-y-3">
                <div class="form-group">
                  <label for="starting-pot" class="text-sm text-gray-600 mb-1 block">Starting pot:</label>
                  <input
                    id="starting-pot"
                    v-model="config.startingPot"
                    type="number"
                    :class="[
                      'w-full px-3 py-2 rounded-lg text-sm border',
                      (config.startingPot <= 0 ||
                      config.startingPot > MAX_AMOUNT ||
                      config.startingPot % 1 !== 0)
                        ? 'border-red-500 bg-red-50' 
                        : 'border-gray-300'
                    ]"
                    :disabled="hasEdit"
                    min="0"
                    :max="MAX_AMOUNT"
                  />
                </div>

                <div class="form-group">
                  <label for="effective-stack" class="text-sm text-gray-600 mb-1 block">Effective stack:</label>
                  <input
                    id="effective-stack"
                    v-model="config.effectiveStack"
                    type="number"
                    :class="[
                      'w-full px-3 py-2 rounded-lg text-sm border',
                      (config.effectiveStack <= 0 ||
                      config.effectiveStack > MAX_AMOUNT ||
                      config.effectiveStack % 1 !== 0)
                        ? 'border-red-500 bg-red-50' 
                        : 'border-gray-300'
                    ]"
                    :disabled="hasEdit"
                    min="0"
                    :max="MAX_AMOUNT"
                  />
                </div>

                <div class="form-group">
                  <label for="rake-percent" class="text-sm text-gray-600 mb-1 block">Rake (%):</label>
                  <div class="flex items-center">
                    <input
                      id="rake-percent"
                      v-model="config.rakePercent"
                      type="number"
                      :class="[
                        'w-full px-3 py-2 rounded-lg text-sm border',
                        (config.rakePercent < 0 || config.rakePercent > 100)
                          ? 'border-red-500 bg-red-50' 
                          : 'border-gray-300'
                      ]"
                      :disabled="hasEdit"
                      min="0"
                      max="100"
                      step="0.5"
                    />
                  </div>
                </div>

                <div class="form-group">
                  <label for="rake-cap" class="text-sm text-gray-600 mb-1 block">Rake cap:</label>
                  <input
                    id="rake-cap"
                    v-model="config.rakeCap"
                    type="number"
                    :class="[
                      'w-full px-3 py-2 rounded-lg text-sm border',
                      (config.rakeCap < 0 || config.rakeCap > 3 * MAX_AMOUNT)
                        ? 'border-red-500 bg-red-50' 
                        : 'border-gray-300'
                    ]"
                    :disabled="hasEdit"
                    min="0"
                    :max="3 * MAX_AMOUNT"
                  />
                </div>
              </div>

              <div class="flex justify-end mt-3">
                <button class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium" @click="clearConfig">
                  Clear
                </button>
              </div>

              <div
                v-if="errorBasics.length > 0"
                class="mt-3 p-3 bg-red-50 border border-red-300 rounded-lg text-red-700"
              >
                <div class="font-semibold underline">Error:</div>
                <div class="ml-2">
                  <div v-for="error in errorBasics" :key="error">
                    {{ error }}
                  </div>
                </div>
              </div>
            </div>

            <!-- OOP Bet Sizes Section -->
            <div class="bg-white rounded-lg shadow-lg p-5">
              <div class="flex justify-between items-center mb-4">
                <h2 class="text-lg font-medium">OOP Bet Sizes</h2>
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
              
              <div class="grid grid-cols-3 gap-4">
                <!-- Flop Column -->
                <div class="street-column">
                  <h3 class="text-base font-medium mb-3 border-b pb-1">Flop</h3>
                  <div class="mb-3">
                    <label for="oop-flop-bet" class="text-sm text-gray-600 mb-1 block">Bet:</label>
                    <div class="flex items-center">
                      <input
                        id="oop-flop-bet"
                        v-model="config.oopFlopBet"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.oopFlopBetSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <div class="mb-3">
                    <label for="oop-flop-raise" class="text-sm text-gray-600 mb-1 block">Raise:</label>
                    <div class="flex items-center">
                      <input
                        id="oop-flop-raise"
                        v-model="config.oopFlopRaise"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.oopFlopRaiseSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <h3 class="text-base font-medium mb-3 border-b pb-1">Turn</h3>
                  <div class="mb-3">
                    <label for="oop-turn-bet" class="text-sm text-gray-600 mb-1 block">Bet:</label>
                    <div class="flex items-center">
                      <input
                        id="oop-turn-bet"
                        v-model="config.oopTurnBet"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.oopTurnBetSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <div class="mb-3">
                    <label for="oop-turn-raise" class="text-sm text-gray-600 mb-1 block">Raise:</label>
                    <div class="flex items-center">
                      <input
                        id="oop-turn-raise"
                        v-model="config.oopTurnRaise"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.oopTurnRaiseSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <div v-if="config.donkOption" class="mb-3">
                    <label for="oop-turn-donk" class="text-sm text-gray-600 mb-1 block">Donk:</label>
                    <div class="flex items-center">
                      <input
                        id="oop-turn-donk"
                        v-model="config.oopTurnDonk"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.oopTurnDonkSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <h3 class="text-base font-medium mb-3 border-b pb-1">River</h3>
                  <div class="mb-3">
                    <label for="oop-river-bet" class="text-sm text-gray-600 mb-1 block">Bet:</label>
                    <div class="flex items-center">
                      <input
                        id="oop-river-bet"
                        v-model="config.oopRiverBet"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.oopRiverBetSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <div class="mb-3">
                    <label for="oop-river-raise" class="text-sm text-gray-600 mb-1 block">Raise:</label>
                    <div class="flex items-center">
                      <input
                        id="oop-river-raise"
                        v-model="config.oopRiverRaise"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.oopRiverRaiseSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <div v-if="config.donkOption" class="mb-3">
                    <label for="oop-river-donk" class="text-sm text-gray-600 mb-1 block">Donk:</label>
                    <div class="flex items-center">
                      <input
                        id="oop-river-donk"
                        v-model="config.oopRiverDonk"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.oopRiverDonkSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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

              <div
                v-if="errorOop.length > 0"
                class="mt-3 p-3 bg-red-50 border border-red-300 rounded-lg text-red-700"
              >
                <div class="font-semibold underline">Error:</div>
                <div class="ml-2">
                  <div v-for="error in errorOop" :key="error">
                    {{ error }}
                  </div>
                </div>
              </div>
            </div>

            <!-- IP Bet Sizes Section -->
            <div class="bg-white rounded-lg shadow-lg p-5">
              <div class="flex justify-between items-center mb-4">
                <h2 class="text-lg font-medium">IP Bet Sizes</h2>
                
                <div class="flex items-center space-x-2">
                  <button
                    class="px-3 py-1.5 rounded bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm font-medium flex items-center"
                    :disabled="errorIp.length > 0 || hasEdit"
                    @click="ipToOop"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" />
                    </svg>
                    Copy to OOP
                  </button>
                  
                  <button
                    class="px-3 py-1.5 rounded bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm font-medium flex items-center"
                    :disabled="errorOop.length > 0 || hasEdit"
                    @click="oopToIp"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
                    </svg>
                    Copy from OOP
                  </button>
                </div>

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
                          percentage.
                        </li>
                        <li class="mt-1">
                          A multiple of the previous bet size (e.g., "2.5x"). Only
                          valid for raises.
                        </li>
                        <li class="mt-1">All-in (e.g., "a").</li>
                      </ul>
                    </div>
                  </template>
                </Tippy>
              </div>
              
              <div class="grid grid-cols-3 gap-4">
                <!-- Flop Column -->
                <div class="street-column">
                  <h3 class="text-base font-medium mb-3 border-b pb-1">Flop</h3>
                  <div class="mb-3">
                    <label for="ip-flop-bet" class="text-sm text-gray-600 mb-1 block">Bet:</label>
                    <div class="flex items-center">
                      <input
                        id="ip-flop-bet"
                        v-model="config.ipFlopBet"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.ipFlopBetSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <div class="mb-3">
                    <label for="ip-flop-raise" class="text-sm text-gray-600 mb-1 block">Raise:</label>
                    <div class="flex items-center">
                      <input
                        id="ip-flop-raise"
                        v-model="config.ipFlopRaise"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.ipFlopRaiseSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <h3 class="text-base font-medium mb-3 border-b pb-1">Turn</h3>
                  <div class="mb-3">
                    <label for="ip-turn-bet" class="text-sm text-gray-600 mb-1 block">Bet:</label>
                    <div class="flex items-center">
                      <input
                        id="ip-turn-bet"
                        v-model="config.ipTurnBet"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.ipTurnBetSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <div class="mb-3">
                    <label for="ip-turn-raise" class="text-sm text-gray-600 mb-1 block">Raise:</label>
                    <div class="flex items-center">
                      <input
                        id="ip-turn-raise"
                        v-model="config.ipTurnRaise"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.ipTurnRaiseSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <h3 class="text-base font-medium mb-3 border-b pb-1">River</h3>
                  <div class="mb-3">
                    <label for="ip-river-bet" class="text-sm text-gray-600 mb-1 block">Bet:</label>
                    <div class="flex items-center">
                      <input
                        id="ip-river-bet"
                        v-model="config.ipRiverBet"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.ipRiverBetSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                  <div class="mb-3">
                    <label for="ip-river-raise" class="text-sm text-gray-600 mb-1 block">Raise:</label>
                    <div class="flex items-center">
                      <input
                        id="ip-river-raise"
                        v-model="config.ipRiverRaise"
                        type="text"
                        :class="[
                          'w-full px-3 py-2 rounded-lg text-sm border',
                          !config.ipRiverRaiseSanitized.valid 
                            ? 'border-red-500 bg-red-50' 
                            : 'border-gray-300'
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
                v-if="errorIp.length > 0"
                class="mt-3 p-3 bg-red-50 border border-red-300 rounded-lg text-red-700"
              >
                <div class="font-semibold underline">Error:</div>
                <div class="ml-2">
                  <div v-for="error in errorIp" :key="error">
                    {{ error }}
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Right Column (1/3) -->
          <div class="h-full">
            <!-- Saved Configurations Card -->
            <div class="bg-white rounded-lg shadow-lg p-5 h-full">
              <DbItemPicker
                store-name="configurations"
                :value="dbValue"
                :allow-save="isInputValid"
                @load-item="loadConfig"
              />
            </div>
          </div>
        </div>

        <!-- Bottom Section for Thresholds and Tree Actions -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-5 mb-5">
          <!-- Thresholds Card -->
          <div class="md:col-span-2 bg-white rounded-lg shadow-lg p-5">
            <h2 class="text-lg font-medium mb-4">Thresholds</h2>
            
            <div class="space-y-4">
              <div class="form-group">
                <div class="flex justify-between items-start">
                  <label for="add-allin-threshold" class="text-sm text-gray-600 mb-1 flex items-center">
                    Add all-in threshold:
                    <Tippy
                      class="ml-1 cursor-help"
                      max-width="420px"
                      trigger="mouseenter click"
                      :delay="[200, 0]"
                      :interactive="true"
                    >
                      <QuestionMarkCircleIcon class="w-4 h-4 text-gray-500" />
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
                      id="add-allin-threshold"
                      v-model="config.addAllInThreshold"
                      type="number"
                      :class="[
                        'w-24 px-3 py-2 rounded-lg text-sm text-right border',
                        config.addAllInThreshold < 0 
                          ? 'border-red-500 bg-red-50' 
                          : 'border-gray-300'
                      ]"
                      :disabled="hasEdit"
                      min="0"
                      :max="MAX_AMOUNT * 100"
                    />
                    <span class="ml-2">%</span>
                  </div>
                </div>
              </div>
              
              <div class="form-group">
                <div class="flex justify-between items-start">
                  <label for="force-allin-threshold" class="text-sm text-gray-600 mb-1 flex items-center">
                    Force all-in threshold:
                    <Tippy
                      class="ml-1 cursor-help"
                      max-width="500px"
                      trigger="mouseenter click"
                      :delay="[200, 0]"
                      :interactive="true"
                    >
                      <QuestionMarkCircleIcon class="w-4 h-4 text-gray-500" />
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
                      id="force-allin-threshold"
                      v-model="config.forceAllInThreshold"
                      type="number"
                      :class="[
                        'w-24 px-3 py-2 rounded-lg text-sm text-right border',
                        config.forceAllInThreshold < 0 
                          ? 'border-red-500 bg-red-50' 
                          : 'border-gray-300'
                      ]"
                      :disabled="hasEdit"
                      min="0"
                      :max="MAX_AMOUNT * 100"
                    />
                    <span class="ml-2">%</span>
                  </div>
                </div>
              </div>
              
              <div class="form-group">
                <div class="flex justify-between items-start">
                  <label for="merging-threshold" class="text-sm text-gray-600 mb-1 flex items-center">
                    Merging threshold:
                    <Tippy
                      class="ml-1 cursor-help"
                      max-width="500px"
                      trigger="mouseenter click"
                      :delay="[200, 0]"
                      :interactive="true"
                    >
                      <QuestionMarkCircleIcon class="w-4 h-4 text-gray-500" />
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
                        'w-24 px-3 py-2 rounded-lg text-sm text-right border',
                        config.mergingThreshold < 0 
                          ? 'border-red-500 bg-red-50' 
                          : 'border-gray-300'
                      ]"
                      :disabled="hasEdit"
                      min="0"
                      :max="MAX_AMOUNT * 100"
                    />
                    <span class="ml-2">%</span>
                  </div>
                </div>
              </div>
            </div>

            <div
              v-if="errorMisc.length > 0"
              class="mt-3 p-3 bg-red-50 border border-red-300 rounded-lg text-red-700"
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
              class="mt-3 p-3 bg-orange-50 border border-orange-300 rounded-lg text-orange-700"
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

          <!-- Tree Actions Card -->
          <div class="bg-white rounded-lg shadow-lg p-5">
            <h2 class="text-lg font-medium mb-4">Tree Actions</h2>
            
            <div class="flex items-center justify-center">
              <button
                class="w-full px-4 py-4 rounded-lg bg-blue-600 hover:bg-blue-700 text-white font-medium text-lg disabled:bg-blue-300 disabled:cursor-not-allowed"
                :disabled="!isInputValid"
                @click="startEdit"
              >
                Preview & Edit Tree
              </button>
            </div>

            <div class="mt-4" v-if="hasEdit">
              <button
                class="w-full px-4 py-3 rounded-lg bg-red-600 hover:bg-red-700 text-white font-medium"
                @click="clearEdit"
              >
                Clear Edit & Unlock
              </button>
            </div>
          </div>
        </div>
        
        <!-- Tree Edit Information Section -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-5 mb-5" v-if="addedLinesArray.length > 0 || removedLinesArray.length > 0">
          <div class="md:col-span-2 bg-white rounded-lg shadow-lg p-5">
            <h2 class="text-lg font-medium mb-4">Tree Edit Information</h2>
            
            <div v-if="addedLinesArray.length > 0" class="mb-3">
              <h3 class="text-base font-medium mb-2">Added Lines:</h3>
              <div class="p-3 bg-green-50 border border-green-200 rounded-lg">
                <div v-for="addedLine in addedLinesArray" :key="addedLine" class="text-sm">
                  {{ addedLine }}
                </div>
              </div>
            </div>

            <div v-if="removedLinesArray.length > 0" class="mb-3">
              <h3 class="text-base font-medium mb-2">Removed Lines:</h3>
              <div class="p-3 bg-red-50 border border-red-200 rounded-lg">
                <div v-for="removedLine in removedLinesArray" :key="removedLine" class="text-sm">
                  {{ removedLine }}
                </div>
              </div>
            </div>

            <div
              v-if="
                config.board.length >= 3 &&
                config.expectedBoardLength > 0 &&
                config.board.length !== config.expectedBoardLength
              "
              class="mt-4 p-3 bg-orange-50 border border-orange-300 rounded-lg text-orange-700"
            >
              <div class="font-semibold">Warning:</div>
              <div class="mt-1">
                The edited tree assumes a {{ config.expectedBoardLength }}-card board,
                but the current board consists of {{ config.board.length }} cards.
                <br />
                To reset the edited tree, click the "Clear Edit & Unlock" button.
              </div>
            </div>
          </div>
        </div>

        <!-- Error Lines Section -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-5 mb-5" v-if="errorLines.length > 0">
          <div class="md:col-span-2 p-3 bg-red-50 border border-red-300 rounded-lg text-red-700">
            <div class="font-semibold underline">Error:</div>
            <div class="ml-2">
              <div v-for="error in errorLines" :key="error">
                {{ error }}
              </div>
            </div>
          </div>
        </div>

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
import { useStore, useConfigStore } from "../store";
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
  store.treeEditMode = true;
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
  store.treeEditMode = false;
  config.addedLines = addedLines;
  config.removedLines = removedLines;
  if (config.addedLines === "" && config.removedLines === "") {
    config.expectedBoardLength = 0;
  }
  store.headers["tree-config"].pop();
};

const cancelEdit = () => {
  store.treeEditMode = false;
  if (config.addedLines === "" && config.removedLines === "") {
    config.expectedBoardLength = 0;
  }
  store.headers["tree-config"].pop();
};
</script>

<style scoped>
/* Add any additional styles as needed */
</style>

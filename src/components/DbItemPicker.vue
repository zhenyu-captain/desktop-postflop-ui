<template>
  <div class="db-picker-container">
    <!-- 内容区域，使用渐变色边框包围 -->
    <div 
      class="picker-content-wrapper"
    >
      <div
        class="picker-content-area"
        @click="unselect"
        @keydown.f2="renameItem()"
        @keydown.delete="deleteItem(true)"
      >
        <!-- No saved items -->
        <div v-if="data.length === 0" class="empty-state">
          <div class="empty-icon">📂</div>
          <p>No saved {{ storeName }}</p>
        </div>

        <!-- Items Tree -->
        <div v-else class="items-wrapper">
          <!-- Depth 0 -->
          <div
            v-for="item0 in data"
            :key="item0.pathStr"
            :class="['item-card', !item0.isGroup ? 'item-card-leaf' : 'item-card-group']"
          >
            <template v-if="item0.isGroup">
              <!-- Group header -->
              <div class="group-header">
                <!-- Toggle button -->
                <button
                  class="toggle-btn"
                  @click="toggleGroup(item0)"
                >
                  <span class="toggle-icon" :class="{ 'open': item0.opened }">
                    ▶
                  </span>
                </button>

                <!-- Name edit -->
                <input
                  v-if="item0.isEditing"
                  ref="nameInput"
                  v-model="editingName"
                  type="text"
                  :class="['edit-input', !isNameValid ? 'edit-input-error' : '']"
                  @blur="tryRename(item0)"
                  @keydown.enter="tryRename(item0)"
                  @keydown.escape="cancelRename(item0)"
                  @keydown.delete.stop
                />

                <!-- Group name -->
                <label
                  v-else
                  class="group-name-label"
                  @dblclick="toggleGroup(item0)"
                  @keydown.enter="toggleGroup(item0)"
                >
                  <input
                    v-model="selectedValue"
                    type="radio"
                    :name="`item-picker-${storeName}-${index}`"
                    class="sr-only"
                    :value="item0.pathStr"
                  />
                  <span
                    class="group-name"
                    :class="{ 'selected': selectedValue === item0.pathStr }"
                  >
                    {{ item0.path[0] }}
                  </span>
                </label>
              </div>

              <!-- Group items -->
              <div
                v-if="item0.opened"
                class="group-children"
              >
                <!-- Empty group -->
                <div v-if="item0.items.length === 0" class="empty-group">
                  <span>Empty folder</span>
                </div>

                <!-- Depth 1 -->
                <div
                  v-for="item1 in item0.items"
                  :key="item1.pathStr"
                  :class="['child-item', item1.isGroup ? 'child-group' : 'child-leaf']"
                >
                  <template v-if="item1.isGroup">
                    <!-- Group header -->
                    <div class="group-header">
                      <!-- Toggle button -->
                      <button
                        class="toggle-btn"
                        @click="toggleGroup(item1)"
                      >
                        <span class="toggle-icon" :class="{ 'open': item1.opened }">
                          ▶
                        </span>
                      </button>

                      <!-- Name edit -->
                      <input
                        v-if="item1.isEditing"
                        ref="nameInput"
                        v-model="editingName"
                        type="text"
                        :class="['edit-input', !isNameValid ? 'edit-input-error' : '']"
                        @blur="tryRename(item1)"
                        @keydown.enter="tryRename(item1)"
                        @keydown.escape="cancelRename(item1)"
                        @keydown.delete.stop
                      />

                      <!-- Group name -->
                      <label
                        v-else
                        class="group-name-label"
                        @dblclick="toggleGroup(item1)"
                        @keydown.enter="toggleGroup(item1)"
                      >
                        <input
                          v-model="selectedValue"
                          type="radio"
                          :name="`item-picker-${storeName}-${index}`"
                          class="sr-only"
                          :value="item1.pathStr"
                        />
                        <span
                          class="group-name"
                          :class="{ 'selected': selectedValue === item1.pathStr }"
                        >
                          {{ item1.path[1] }}
                        </span>
                      </label>
                    </div>

                    <!-- Group items -->
                    <div
                      v-if="item1.opened"
                      class="group-children"
                    >
                      <!-- Empty group -->
                      <div v-if="item1.items.length === 0" class="empty-group">
                        <span>Empty folder</span>
                      </div>

                      <!-- Depth 2 -->
                      <div
                        v-for="item2 in item1.items"
                        :key="item2.pathStr"
                        :class="['child-item', item2.isGroup ? 'child-group' : 'child-leaf']"
                      >
                        <template v-if="item2.isGroup">
                          <!-- Group header -->
                          <div class="group-header">
                            <!-- Toggle button -->
                            <button
                              class="toggle-btn"
                              @click="toggleGroup(item2)"
                            >
                              <span class="toggle-icon" :class="{ 'open': item2.opened }">
                                ▶
                              </span>
                            </button>

                            <!-- Name edit -->
                            <input
                              v-if="item2.isEditing"
                              ref="nameInput"
                              v-model="editingName"
                              type="text"
                              :class="['edit-input', !isNameValid ? 'edit-input-error' : '']"
                              @blur="tryRename(item2)"
                              @keydown.enter="tryRename(item2)"
                              @keydown.escape="cancelRename(item2)"
                              @keydown.delete.stop
                            />

                            <!-- Group name -->
                            <label
                              v-else
                              class="group-name-label"
                              @dblclick="toggleGroup(item2)"
                              @keydown.enter="toggleGroup(item2)"
                            >
                              <input
                                v-model="selectedValue"
                                type="radio"
                                :name="`item-picker-${storeName}-${index}`"
                                class="sr-only"
                                :value="item2.pathStr"
                              />
                              <span
                                class="group-name"
                                :class="{ 'selected': selectedValue === item2.pathStr }"
                              >
                                {{ item2.path[2] }}
                              </span>
                            </label>
                          </div>

                          <!-- Group items -->
                          <div
                            v-if="item2.opened"
                            class="group-children"
                          >
                            <!-- Empty group -->
                            <div v-if="item2.items.length === 0" class="empty-group">
                              <span>Empty folder</span>
                            </div>

                            <!-- Depth 3 -->
                            <div
                              v-for="item3 in item2.items"
                              :key="item3.pathStr"
                              class="child-item child-leaf last-level"
                            >
                              <!-- Name edit -->
                              <input
                                v-if="item3.isEditing"
                                ref="nameInput"
                                v-model="editingName"
                                type="text"
                                :class="['edit-input', !isNameValid ? 'edit-input-error' : '']"
                                @blur="tryRename(item3)"
                                @keydown.enter="tryRename(item3)"
                                @keydown.escape="cancelRename(item3)"
                                @keydown.delete.stop
                              />

                              <!-- Item name -->
                              <label
                                v-else
                                class="item-name-label"
                                @dblclick="loadItem()"
                                @keydown.enter="loadItem()"
                              >
                                <input
                                  v-model="selectedValue"
                                  type="radio"
                                  :name="`item-picker-${storeName}-${index}`"
                                  class="sr-only"
                                  :value="item3.pathStr"
                                />
                                <span
                                  class="item-name"
                                  :class="{ 'selected': selectedValue === item3.pathStr }"
                                >
                                  {{ item3.path[3] }}
                                </span>
                              </label>
                            </div>
                          </div>
                        </template>

                        <template v-else>
                          <!-- Name edit -->
                          <input
                            v-if="item2.isEditing"
                            ref="nameInput"
                            v-model="editingName"
                            type="text"
                            :class="['edit-input', !isNameValid ? 'edit-input-error' : '']"
                            @blur="tryRename(item2)"
                            @keydown.enter="tryRename(item2)"
                            @keydown.escape="cancelRename(item2)"
                            @keydown.delete.stop
                          />

                          <!-- Item name -->
                          <label
                            v-else
                            class="item-name-label"
                            @dblclick="loadItem()"
                            @keydown.enter="loadItem()"
                          >
                            <input
                              v-model="selectedValue"
                              type="radio"
                              :name="`item-picker-${storeName}-${index}`"
                              class="sr-only"
                              :value="item2.pathStr"
                            />
                            <span
                              class="item-name"
                              :class="{ 'selected': selectedValue === item2.pathStr }"
                            >
                              {{ item2.path[2] }}
                            </span>
                          </label>
                        </template>
                      </div>
                    </div>
                  </template>

                  <template v-else>
                    <!-- Name edit -->
                    <input
                      v-if="item1.isEditing"
                      ref="nameInput"
                      v-model="editingName"
                      type="text"
                      :class="['edit-input', !isNameValid ? 'edit-input-error' : '']"
                      @blur="tryRename(item1)"
                      @keydown.enter="tryRename(item1)"
                      @keydown.escape="cancelRename(item1)"
                      @keydown.delete.stop
                    />

                    <!-- Item name -->
                    <label
                      v-else
                      class="item-name-label"
                      @dblclick="loadItem()"
                      @keydown.enter="loadItem()"
                    >
                      <input
                        v-model="selectedValue"
                        type="radio"
                        :name="`item-picker-${storeName}-${index}`"
                        class="sr-only"
                        :value="item1.pathStr"
                      />
                      <span
                        class="item-name"
                        :class="{ 'selected': selectedValue === item1.pathStr }"
                      >
                        {{ item1.path[1] }}
                      </span>
                    </label>
                  </template>
                </div>
              </div>
            </template>

            <template v-else>
              <!-- Name edit -->
              <input
                v-if="item0.isEditing"
                ref="nameInput"
                v-model="editingName"
                type="text"
                :class="['edit-input', !isNameValid ? 'edit-input-error' : '']"
                @blur="tryRename(item0)"
                @keydown.enter="tryRename(item0)"
                @keydown.escape="cancelRename(item0)"
                @keydown.delete.stop
              />

              <!-- Item name -->
              <label
                v-else
                class="item-name-label"
                @dblclick="loadItem()"
                @keydown.enter="loadItem()"
              >
                <input
                  v-model="selectedValue"
                  type="radio"
                  :name="`item-picker-${storeName}-${index}`"
                  class="sr-only"
                  :value="item0.pathStr"
                />
                <span
                  class="item-name"
                  :class="{ 'selected': selectedValue === item0.pathStr }"
                >
                  {{ item0.path[0] }}
                </span>
              </label>
            </template>
          </div>
        </div>
      </div>
    </div>

    <div v-if="errorOccured" class="error-message">
      <div class="error-icon">⚠️</div>
      <div class="error-text">Something went wrong. Please relaunch the app.</div>
    </div>

    <!-- 恢复原始按钮布局结构 -->
    <div class="control-panel">
      <!-- 第一行：3个主要按钮 -->
      <div class="grid-row-3">
        <button
          class="action-btn primary-btn"
          :disabled="selectedItem?.item?.isGroup ?? true"
          @click="loadItem()"
        >
          <span class="btn-icon">✓</span>
          <span class="btn-text">Load</span>
        </button>

        <button
          class="action-btn primary-btn"
          :disabled="errorOccured || !allowSave || isEditing"
          @click="addOrOverwriteItem()"
        >
          <span class="btn-icon">💾</span>
          <span class="btn-text">{{ selectedItem?.item?.isGroup === false ? "Overwrite" : "Save" }}</span>
        </button>

        <button
          class="action-btn primary-btn"
          :disabled="errorOccured || selectedValue === false"
          @click="renameItem()"
        >
          <span class="btn-icon">✏️</span>
          <span class="btn-text">Rename</span>
        </button>
      </div>

      <!-- 第二行：分为2列 -->
      <div class="grid-row-2">
        <button
          class="action-btn secondary-btn"
          :disabled="
            errorOccured ||
            (selectedItem !== null &&
              selectedItem.item.path.length +
                (selectedItem.item.isGroup ? 1 : 0) >=
                4)
          "
          @click="addGroup()"
        >
          <span class="btn-icon">📁</span>
          <span class="btn-text">Add Group</span>
        </button>

        <button
          class="action-btn danger-btn"
          :disabled="errorOccured || selectedValue === false"
          @click="deleteItem(true)"
        >
          <span class="btn-icon">🗑️</span>
          <span class="btn-text">Delete</span>
        </button>

        <button
          v-if="!hideImportExport"
          class="action-btn import-btn"
          :disabled="errorOccured || isEditing"
          @click="importJson"
        >
          <span class="btn-icon">📥</span>
          <span class="btn-text">Import JSON</span>
        </button>

        <button
          v-if="!hideImportExport"
          class="action-btn export-btn"
          :disabled="errorOccured || isEditing"
          @click="exportJson"
        >
          <span class="btn-icon">📤</span>
          <span class="btn-text">Export JSON</span>
        </button>
      </div>
    </div>

    <div
      v-if="importError !== ''"
      class="import-error"
    >
      <div class="error-header">
        <span class="error-icon">⚠️</span>
        <span>Error: Import failed</span>
        <button
          class="close-error-btn"
          @click="importError = ''"
        >
          <XMarkIcon class="close-icon" />
        </button>
      </div>
      <div class="error-details">- {{ importError }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref } from "vue";
import * as Db from "../db";

import { open, save } from "@tauri-apps/api/dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";

import { XMarkIcon } from "@heroicons/vue/20/solid";

type Item = {
  isGroup: false;
  path: string[];
  pathStr: string;
  value: unknown;
  isEditing: boolean;
};

type Group = {
  isGroup: true;
  path: string[];
  pathStr: string;
  opened: boolean;
  isEditing: boolean;
  items: (Item | Group)[];
};

const isEqual = (lhs: unknown, rhs: unknown) => {
  if (lhs === null || typeof lhs !== "object") return lhs === rhs;
  if (Array.isArray(lhs)) {
    if (!Array.isArray(rhs) || lhs.length !== rhs.length) return false;
    for (let i = 0; i < lhs.length; ++i) {
      if (!isEqual(lhs[i], rhs[i])) return false;
    }
  } else {
    if (rhs === null || typeof rhs !== "object") return false;
    const lhsEntries = Object.entries(lhs);
    const rhsEntries = Object.entries(rhs);
    if (lhsEntries.length !== rhsEntries.length) return false;
    lhsEntries.sort((a, b) => a[0].localeCompare(b[0]));
    rhsEntries.sort((a, b) => a[0].localeCompare(b[0]));
    for (let i = 0; i < lhsEntries.length; ++i) {
      if (lhsEntries[i][0] !== rhsEntries[i][0]) return false;
      if (!isEqual(lhsEntries[i][1], rhsEntries[i][1])) return false;
    }
  }
  return true;
};

const clone = (value: unknown) => {
  if (value === null || typeof value !== "object") return value;
  if (Array.isArray(value)) return [...value];
  return { ...value };
};

const groupToDbGroup = (group: Group): Db.DbGroup => ({
  name0: group.path[0],
  name1: group.path[1] ?? "",
  name2: group.path[2] ?? "",
  name3: group.path[3] ?? "",
  isGroup: 1,
});

const itemToDbItem = (item: Item): Db.DbItem => ({
  name0: item.path[0],
  name1: item.path[1] ?? "",
  name2: item.path[2] ?? "",
  name3: item.path[3] ?? "",
  isGroup: 0,
  value: clone(item.value),
});

const toDbItemOrGroup = (itemOrGroup: Item | Group): Db.DbItem | Db.DbGroup =>
  itemOrGroup.isGroup ? groupToDbGroup(itemOrGroup) : itemToDbItem(itemOrGroup);

type AddItemMessage = {
  type: "addItem";
  path: string[];
  value: unknown;
};

type OverwriteItemMessage = {
  type: "overwriteItem";
  path: string[];
  value: unknown;
};

type RenameItemMessage = {
  type: "renameItem";
  path: string[];
  newName: string;
};

type AddGroupMessage = {
  type: "addGroup";
  path: string[];
};

type DeleteItemMessage = {
  type: "deleteItem";
  path: string[];
};

type ImportJsonMessage = {
  type: "importJson";
};

type Message =
  | AddItemMessage
  | OverwriteItemMessage
  | RenameItemMessage
  | AddGroupMessage
  | DeleteItemMessage
  | ImportJsonMessage;

interface Props {
  storeName: string;
  index?: number;
  value: unknown;
  allowSave: boolean;
  hideImportExport?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  index: 0,
  hideImportExport: false,
});
const emit = defineEmits<{ (event: "load-item", value: unknown): void }>();

const data = ref<(Item | Group)[]>([]);
const selectedValue = ref(false as false | string);

const selectedItem = computed(() => {
  if (selectedValue.value === false) return null;
  const path = JSON.parse(selectedValue.value);
  let item: Item | Group | undefined;
  let items = data.value;
  for (let i = 0; i < path.length; ++i) {
    item = items.find((item) => item.path[i] === path[i]);
    if (item === undefined) return null;
    if (i < path.length - 1) {
      if (!item.isGroup) return null;
      items = item.items;
    }
  }
  return item ? { item, parentItems: items } : null;
});

const errorOccured = ref(false);

// load data from DB
const loadData = async () => {
  const masterData = await Db.getArray(props.storeName);

  masterData.sort(
    (a, b) =>
      Number(b.name1 === "") - Number(a.name1 === "") ||
      Number(b.name2 === "") - Number(a.name2 === "") ||
      Number(b.name3 === "") - Number(a.name3 === "") ||
      b.isGroup - a.isGroup ||
      a.name0.localeCompare(b.name0, undefined, { numeric: true }) ||
      a.name1.localeCompare(b.name1, undefined, { numeric: true }) ||
      a.name2.localeCompare(b.name2, undefined, { numeric: true }) ||
      a.name3.localeCompare(b.name3, undefined, { numeric: true })
  );

  data.value = [];
  selectedValue.value = false;

  const dataMap = new Map<string, Item | Group>();
  const dataItems = new Map<string, (Item | Group)[]>();
  dataItems.set(JSON.stringify([]), data.value);

  for (const masterItem of masterData) {
    if (masterItem.isGroup) {
      const path = [masterItem.name0, masterItem.name1, masterItem.name2, ""];
      const depth = path.findIndex((name) => name === "");
      path.length = depth;
      const pathStr = JSON.stringify(path);
      const parent = path.slice(0, -1);
      const parentStr = JSON.stringify(parent);
      const parentItems = dataItems.get(parentStr);
      if (parentItems) {
        const items: (Item | Group)[] = [];
        const group: Group = {
          isGroup: true,
          path,
          pathStr,
          opened: depth === 1,
          isEditing: false,
          items,
        };
        parentItems.push(group);
        dataMap.set(pathStr, group);
        dataItems.set(pathStr, items);
      } else {
        errorOccured.value = true;
        return;
      }
    } else {
      const path = [
        masterItem.name0,
        masterItem.name1,
        masterItem.name2,
        masterItem.name3,
        "",
      ];
      const depth = path.findIndex((name) => name === "");
      path.splice(depth);
      const pathStr = JSON.stringify(path);
      const parent = path.slice(0, -1);
      const parentStr = JSON.stringify(parent);
      const parentItems = dataItems.get(parentStr);
      if (parentItems) {
        const item: Item = {
          isGroup: false,
          path,
          pathStr,
          value: masterItem.value,
          isEditing: false,
        };
        parentItems.push(item);
        dataMap.set(pathStr, item);
      } else {
        errorOccured.value = true;
        return;
      }
    }
  }
};

loadData();

// Broadcast channel
const channel = new BroadcastChannel(`item-picker-${props.storeName}`);

channel.onmessage = async (event: MessageEvent<Message>) => {
  if (errorOccured.value) return;

  const message = event.data;
  const pathStr =
    message.type === "importJson" ? "" : JSON.stringify(message.path);

  if (message.type === "addItem") {
    // add item
    if (message.path.length > 1) {
      selectedValue.value = JSON.stringify(message.path.slice(0, -1));
      if (selectedItem.value === null) {
        errorOccured.value = true;
        selectedValue.value = false;
        return;
      }
    } else {
      selectedValue.value = false;
    }
    await addOrOverwriteItem(
      message.value,
      message.path[message.path.length - 1]
    );
  } else if (message.type === "overwriteItem") {
    // overwrite item
    selectedValue.value = pathStr;
    if (selectedItem.value === null) {
      errorOccured.value = true;
      selectedValue.value = false;
      return;
    }
    await addOrOverwriteItem(message.value);
  } else if (message.type === "renameItem") {
    // rename item
    selectedValue.value = pathStr;
    if (selectedItem.value === null) {
      errorOccured.value = true;
      selectedValue.value = false;
      return;
    }
    await tryRename(selectedItem.value.item, message.newName);
  } else if (message.type === "addGroup") {
    // add group
    if (message.path.length > 1) {
      selectedValue.value = JSON.stringify(message.path.slice(0, -1));
      if (selectedItem.value === null) {
        errorOccured.value = true;
        selectedValue.value = false;
        return;
      }
    } else {
      selectedValue.value = false;
    }
    await addGroup(message.path[message.path.length - 1]);
  } else if (message.type === "deleteItem") {
    // delete item
    selectedValue.value = pathStr;
    if (selectedItem.value === null) {
      errorOccured.value = true;
      selectedValue.value = false;
      return;
    }
    await deleteItem(false);
  } else if (message.type === "importJson") {
    // import JSON
    await loadData();
  }
};

const isEditing = ref(false);
const editingName = ref("");
const duplicateName = ref([] as string[]);
const nameInput = ref(null as HTMLInputElement[] | null);

const unselect = (ev: MouseEvent) => {
  const target = ev.target as HTMLElement;
  if (target.tagName === "DIV") {
    selectedValue.value = false;
  }
};

const closeGroup = (group: Group) => {
  group.opened = false;
  for (const child of group.items) {
    if (child.isGroup) {
      closeGroup(child);
    }
  }
};

const toggleGroup = (group: Group) => {
  selectedValue.value = false;
  if (group.opened) {
    closeGroup(group);
  } else {
    group.opened = true;
  }
};

const numDisplayedRows = (group: Group) => {
  if (!group.opened) return 0;
  let ret = group.items.length || 1;
  for (const child of group.items) {
    if (child.isGroup) {
      ret += numDisplayedRows(child);
    }
  }
  return ret;
};

const guideDecrease = (group: Group) => {
  if (group.items.length > 0) {
    const lastChild = group.items[group.items.length - 1];
    if (lastChild.isGroup) {
      return numDisplayedRows(lastChild);
    }
  }
  return 0;
};

const loadItem = () => {
  if (!selectedItem.value) return;
  if (selectedItem.value.item.isGroup === false) {
    emit("load-item", selectedItem.value.item.value);
  }
  selectedValue.value = false;
};

const addOrOverwriteItem = async (value?: unknown, name?: string) => {
  if (errorOccured.value) return;

  if (selectedItem.value?.item?.isGroup === false) {
    // overwrite
    selectedItem.value.item.value = value ?? props.value;

    if (value === undefined) {
      // save and broadcast
      errorOccured.value ||= !(await Db.overwriteItem(
        props.storeName,
        itemToDbItem(selectedItem.value.item)
      ));
      channel.postMessage({
        type: "overwriteItem",
        path: [...selectedItem.value.item.path],
        value: clone(props.value),
      } as OverwriteItemMessage);
    }

    selectedValue.value = false;
  } else {
    // add
    const path = selectedItem.value?.item
      ? [...selectedItem.value.item.path, name ?? ""]
      : [name ?? ""];
    const pathStr = JSON.stringify(path);
    const parentItems = selectedItem.value?.item?.items ?? data.value;
    parentItems.push({
      isGroup: false,
      path,
      pathStr,
      value: value ?? props.value,
      isEditing: false,
    });

    if (name === undefined) {
      // user must enter a name
      if (selectedItem.value) {
        selectedItem.value.item.opened = true;
      }
      selectedValue.value = pathStr;
      await renameItem();
    } else {
      // name is provided
      selectedValue.value = false;
      const depth = path.length - 1;
      parentItems.sort(
        (a, b) =>
          Number(b.isGroup) - Number(a.isGroup) ||
          a.path[depth].localeCompare(b.path[depth], undefined, {
            numeric: true,
          })
      );
    }
  }
};

const isNameValid = computed(() => {
  const name = editingName.value.trim();
  return name !== "" && !duplicateName.value.includes(name);
});

const renameItem = async () => {
  if (!selectedItem.value || errorOccured.value) return;
  const item = selectedItem.value.item;
  item.isEditing = true;
  isEditing.value = true;
  editingName.value = item.path[item.path.length - 1];
  duplicateName.value = selectedItem.value.parentItems
    .map((item) => item.path[item.path.length - 1])
    .filter((name) => name !== editingName.value);
  const defaultName = item.isGroup
    ? "New group"
    : `New ${props.storeName.slice(0, -1)}`; // remove "s" hack
  if (editingName.value === "") {
    let i = 2;
    let newName = defaultName;
    while (duplicateName.value.includes(newName)) {
      newName = `${defaultName} (${i++})`;
    }
    editingName.value = newName;
  }
  selectedValue.value = false;
  await nextTick();
  if (nameInput.value) {
    nameInput.value[0].select();
    nameInput.value[0].scrollIntoView({ block: "nearest" });
  }
};

const tryRename = async (item: Item | Group, newName?: string) => {
  // existence check (`tryRename()` might be called when an item is deleted)
  selectedValue.value = item.pathStr;
  if (selectedItem.value === null) {
    selectedValue.value = false;
    return;
  }

  const isUserAction = newName === undefined;

  if (newName === undefined) {
    // is user action
    item.isEditing = false;
    isEditing.value = false;

    if (!isNameValid.value) {
      editingName.value = "";
      if (item.path[item.path.length - 1] === "") {
        await deleteItem(false);
      }
      return;
    }

    newName = editingName.value.trim();
    editingName.value = "";
  }

  if (errorOccured.value) return;

  const depth = item.path.length - 1;
  const isNewItem = item.path[depth] === "";
  const prevPath = [...item.path];

  if (item.path[depth] === newName) {
    if (isUserAction) {
      // give focus
      await nextTick();
      const name = `item-picker-${props.storeName}-${props.index}`;
      const checked = document.querySelector(`input[name="${name}"]:checked`);
      if (checked) {
        (checked as HTMLInputElement).focus();
      }
    }

    return;
  }

  item.path[depth] = newName;
  item.pathStr = JSON.stringify(item.path);
  if (item.isGroup) {
    renameRecursive(item, newName, depth);
  }

  selectedValue.value = item.pathStr;
  const parentItems = selectedItem.value?.parentItems;
  if (parentItems) {
    parentItems.sort(
      (a, b) =>
        Number(b.isGroup) - Number(a.isGroup) ||
        a.path[depth].localeCompare(b.path[depth], undefined, {
          numeric: true,
        })
    );
  }

  if (isUserAction) {
    // save and broadcast
    if (isNewItem) {
      // add
      if (!item.isGroup) {
        // item
        errorOccured.value ||= !(await Db.addItem(
          props.storeName,
          itemToDbItem(item)
        ));
        channel.postMessage({
          type: "addItem",
          path: [...item.path],
          value: clone(item.value),
        } as AddItemMessage);
      } else {
        // group
        errorOccured.value ||= !(await Db.addGroup(
          props.storeName,
          groupToDbGroup(item)
        ));
        channel.postMessage({
          type: "addGroup",
          path: [...item.path],
        } as AddGroupMessage);
      }
    } else {
      // rename
      errorOccured.value ||= !(await Db.renameItem(
        props.storeName,
        toDbItemOrGroup({ ...item, path: prevPath }),
        newName
      ));
      channel.postMessage({
        type: "renameItem",
        path: prevPath,
        newName: newName,
      } as RenameItemMessage);
    }

    // give focus
    await nextTick();
    const name = `item-picker-${props.storeName}-${props.index}`;
    const checked = document.querySelector(`input[name="${name}"]:checked`);
    if (checked) {
      (checked as HTMLInputElement).focus();
    }
  } else {
    selectedValue.value = false;
  }
};

const renameRecursive = (group: Group, newName: string, depth: number) => {
  for (const item of group.items) {
    item.path[depth] = newName;
    item.pathStr = JSON.stringify(item.path);
    if (item.isGroup) {
      renameRecursive(item, newName, depth);
    }
  }
};

const cancelRename = async (item: Item | Group) => {
  item.isEditing = false;
  isEditing.value = false;
  editingName.value = "";
  if (item.path[item.path.length - 1] === "") {
    selectedValue.value = item.pathStr;
    await deleteItem(false);
  }
};

const addGroup = async (name?: string) => {
  if (errorOccured.value) return;

  const path = selectedItem.value
    ? selectedItem.value.item.isGroup
      ? [...selectedItem.value.item.path, name ?? ""]
      : [...selectedItem.value.item.path.slice(0, -1), name ?? ""]
    : [name ?? ""];
  if (path.length > 3) return;
  const pathStr = JSON.stringify(path);
  const parentItems = selectedItem.value
    ? selectedItem.value.item.isGroup
      ? selectedItem.value.item.items
      : selectedItem.value.parentItems
    : data.value;
  parentItems.push({
    isGroup: true,
    path,
    pathStr,
    opened: false,
    isEditing: false,
    items: [],
  });

  if (name === undefined) {
    // user must enter a name
    if (selectedItem.value?.item?.isGroup) {
      selectedItem.value.item.opened = true;
    }
    selectedValue.value = pathStr;
    renameItem();
  } else {
    // name is provided
    selectedValue.value = false;
    const depth = path.length - 1;
    parentItems.sort(
      (a, b) =>
        Number(b.isGroup) - Number(a.isGroup) ||
        a.path[depth].localeCompare(b.path[depth], undefined, {
          numeric: true,
        })
    );
  }
};

const deleteItem = async (isUserAction: boolean) => {
  if (!selectedItem.value || errorOccured.value) return;

  const path = [...selectedItem.value.item.path];
  const dbItem = toDbItemOrGroup(selectedItem.value.item);

  const parentItems = selectedItem.value.parentItems;
  parentItems.splice(parentItems.indexOf(selectedItem.value.item), 1);
  selectedValue.value = false;

  if (isUserAction) {
    // save and broadcast
    errorOccured.value ||= !(await Db.deleteItem(props.storeName, dbItem));
    channel.postMessage({
      type: "deleteItem",
      path,
    } as DeleteItemMessage);
  }
};

const importError = ref("");

type JsonItem = {
  path: string[];
  isGroup: boolean;
  value?: unknown;
};

const checkJson = (array: JsonItem[]) => {
  if (!Array.isArray(array)) return false;

  let valueType = "";
  if (props.storeName === "ranges") valueType = "string";
  if (props.storeName === "configurations") valueType = "object";
  if (valueType === "") return false;

  const map = new Map<string, boolean>();
  map.set(JSON.stringify([]), true);

  for (const item of array) {
    if (
      typeof item.isGroup !== "boolean" ||
      !Array.isArray(item.path) ||
      item.path.some((x) => typeof x !== "string") ||
      item.path.length > (item.isGroup ? 3 : 4) ||
      (!item.isGroup && typeof item.value !== valueType)
    ) {
      return false;
    }

    item.path = item.path.map((x) => x.trim());

    if (
      item.path.some((x) => x === "") ||
      !map.get(JSON.stringify(item.path.slice(0, -1))) ||
      map.has(JSON.stringify(item.path))
    ) {
      return false;
    }

    map.set(JSON.stringify(item.path), item.isGroup);
  }

  return true;
};

const getItemsToAdd = async (array: JsonItem[]) => {
  const masterData = await Db.getArray(props.storeName);
  const masterMap = new Map<string, Db.DbItem | Db.DbGroup>();
  for (const item of masterData) {
    const path = [item.name0, item.name1, item.name2, item.name3, ""];
    path.length = path.findIndex((x) => x === "");
    masterMap.set(JSON.stringify(path), item);
  }

  const ret: (Db.DbItem | Db.DbGroup)[] = [];

  for (const item of array) {
    const depth = item.path.length;
    const name = item.path[depth - 1];
    let pathStr = JSON.stringify(item.path);

    if (item.isGroup) {
      const master = masterMap.get(pathStr);
      if (master && master.isGroup === 0) return name;
      if (!master) {
        ret.push({
          name0: item.path[0],
          name1: item.path[1] ?? "",
          name2: item.path[2] ?? "",
          name3: item.path[3] ?? "",
          isGroup: 1,
        });
      }
    } else {
      let i = 2;
      let master = masterMap.get(pathStr);
      while (
        master &&
        (master.isGroup === 1 || !isEqual(master.value, item.value))
      ) {
        item.path[depth - 1] = `${name} (${i++})`;
        pathStr = JSON.stringify(item.path);
        master = masterMap.get(pathStr);
      }
      if (!master) {
        ret.push({
          name0: item.path[0],
          name1: item.path[1] ?? "",
          name2: item.path[2] ?? "",
          name3: item.path[3] ?? "",
          isGroup: 0,
          value: item.value,
        });
      }
    }
  }

  return ret;
};

const importJson = async () => {
  if (errorOccured.value || isEditing.value) return;

  let filePath = await open({
    defaultPath: `${props.storeName}.json`,
    filters: [{ name: "JSON Files", extensions: ["json"] }],
  });

  if (!filePath) return;
  if (Array.isArray(filePath)) filePath = filePath[0];

  importError.value = "";

  const text = await readTextFile(filePath);
  let obj: { version: number; name: string; data: JsonItem[] };
  try {
    obj = JSON.parse(text);
  } catch (e) {
    importError.value = "Parse error (invalid JSON)";
    return;
  }

  if (obj.version !== 2) {
    importError.value = "Version mismatch";
    return;
  }

  if (obj.name !== props.storeName) {
    importError.value = "Data type mismatch";
    return;
  }

  if (!checkJson(obj.data)) {
    importError.value = "Invalid data";
    return;
  }

  const itemsToAdd = await getItemsToAdd(obj.data);
  if (typeof itemsToAdd === "string") {
    importError.value = `Cannot create group "${itemsToAdd}" because the item already exists`;
    return;
  }

  errorOccured.value ||= !(await Db.bulkAdd(props.storeName, itemsToAdd));
  channel.postMessage({ type: "importJson" });
  await loadData();
};

const appendRecursive = (item: Item | Group, array: JsonItem[]) => {
  array.push({
    path: item.path,
    isGroup: item.isGroup,
    value: item.isGroup ? undefined : item.value,
  });
  if (item.isGroup) {
    for (const child of item.items) {
      appendRecursive(child, array);
    }
  }
};

const exportJson = async () => {
  if (errorOccured.value || isEditing.value) return;

  const array: JsonItem[] = [];
  for (const item of data.value) {
    appendRecursive(item, array);
  }

  const obj = { version: 2, name: props.storeName, data: array };
  const jsonStr = JSON.stringify(obj, undefined, 2);

  const filePath = await save({
    defaultPath: `${props.storeName}.json`,
    filters: [{ name: "JSON Files", extensions: ["json"] }],
  });

  if (filePath) {
    await writeTextFile(filePath, jsonStr);
  }
};
</script>

<style scoped>
.db-picker-container {
  @apply font-sans flex flex-col gap-4;
  --primary-color: #6366f1;
  --primary-dark: #4f46e5;
  --primary-light: #a5b4fc;
  --secondary-color: #38bdf8;
  --danger-color: #f43f5e;
  --success-color: #10b981;
  --warning-color: #f59e0b;
  --gray-light: #f3f4f6;
  --gray-medium: #d1d5db;
  --gray-dark: #4b5563;
  --card-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  --hover-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

.picker-content-wrapper {
  @apply rounded-lg overflow-hidden;
  padding: 2px;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  box-shadow: var(--card-shadow);
}

.picker-content-area {
  @apply w-full h-72 rounded-md bg-white overflow-y-auto;
}

.empty-state {
  @apply flex flex-col items-center justify-center h-full text-center p-6;
  color: var(--gray-dark);
}

.empty-icon {
  @apply text-4xl mb-2;
}

.items-wrapper {
  @apply p-3 space-y-2;
}

.item-card {
  @apply rounded-lg p-2 transition-all duration-200;
  background: linear-gradient(to right, var(--gray-light), white);
}

.item-card-leaf {
  @apply border-l-4 hover:translate-x-1;
  border-left-color: var(--secondary-color);
}

.item-card-group {
  @apply border-l-4;
  border-left-color: var(--primary-color);
}

.group-header {
  @apply flex items-center gap-2;
}

.toggle-btn {
  @apply flex items-center justify-center w-5 h-5 rounded-full bg-white;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.toggle-icon {
  @apply text-xs transition-transform duration-200;
  color: var(--primary-color);
}

.toggle-icon.open {
  transform: rotate(90deg);
}

.group-children {
  @apply ml-6 mt-2 space-y-1.5;
  position: relative;
}

.group-children::before {
  content: '';
  position: absolute;
  left: -12px;
  top: 0;
  bottom: 10px;
  width: 2px;
  background: linear-gradient(to bottom, var(--primary-light), transparent);
}

.child-item {
  @apply rounded p-1 pl-2 transition-all duration-200;
  position: relative;
}

.child-item::before {
  content: '';
  position: absolute;
  left: -12px;
  top: 50%;
  width: 12px;
  height: 2px;
  background-color: var(--primary-light);
}

.child-group {
  @apply bg-white shadow-sm;
}

.child-leaf {
  @apply hover:bg-gray-50;
}

.last-level {
  @apply pl-4;
}

.empty-group {
  @apply py-2 px-3 text-sm italic text-center rounded;
  color: var(--gray-dark);
  background-color: var(--gray-light);
}

.group-name-label, .item-name-label {
  @apply flex-grow cursor-pointer;
}

.group-name, .item-name {
  @apply block py-1 px-3 rounded-md text-sm transition-all duration-200;
}

.group-name {
  @apply font-medium;
  color: var(--primary-dark);
}

.item-name {
  color: var(--gray-dark);
}

.group-name.selected, .item-name.selected {
  @apply text-white;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.edit-input {
  @apply py-1 px-3 rounded-md text-sm border focus:outline-none focus:ring-2 transition-all duration-200;
  border-color: var(--gray-medium);
  width: calc(100% - 30px);
}

.edit-input:focus {
  @apply ring-2;
  ring-color: var(--primary-color);
}

.edit-input-error {
  @apply ring-2;
  border-color: var(--danger-color);
  background-color: rgba(244, 63, 94, 0.1);
  ring-color: var(--danger-color);
}

.error-message {
  @apply flex items-center gap-2 p-3 rounded-lg;
  background-color: rgba(244, 63, 94, 0.1);
  color: var(--danger-color);
}

.error-icon {
  @apply text-xl;
}

.control-panel {
  @apply flex flex-col gap-3 mt-4;
}

.grid-row-3 {
  @apply grid grid-cols-3 gap-3 w-full;
}

.grid-row-2 {
  @apply grid grid-cols-2 gap-3 w-full;
}

.action-btn {
  @apply flex items-center justify-center gap-1.5 rounded-md py-2 px-1 font-medium text-sm transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed;
  box-shadow: var(--card-shadow);
}

.action-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: var(--hover-shadow);
}

.action-btn:active:not(:disabled) {
  transform: translateY(0px);
}

.btn-icon {
  @apply text-base;
}

.import-error {
  @apply mt-3 rounded-lg overflow-hidden;
  box-shadow: var(--card-shadow);
}

.error-header {
  @apply flex items-center gap-2 p-3 text-white font-medium;
  background: linear-gradient(135deg, var(--danger-color), #e11d48);
}

.close-error-btn {
  @apply ml-auto flex items-center justify-center w-6 h-6 rounded-full bg-white bg-opacity-20 hover:bg-opacity-30 transition-all duration-200;
}

.close-icon {
  @apply w-4 h-4 text-white;
}

.error-details {
  @apply p-3 text-sm;
  background-color: white;
  color: var(--danger-color);
}
</style>

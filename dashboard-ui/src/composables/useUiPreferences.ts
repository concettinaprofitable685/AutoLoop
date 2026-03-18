import { computed, ref } from "vue";
import {
  fetchOperatorSettings,
  saveOperatorSettings,
  type OperatorSettings
} from "../services/operatorSettingsApi";

const STORAGE_KEY = "autoloop-ui-preferences";

const vendorPresets = {
  alibaba: {
    label: "Alibaba / Qwen",
    apiBaseUrl: "https://dashscope.aliyuncs.com/compatible-mode/v1",
    defaultModel: "qwen-plus-latest"
  },
  openai: {
    label: "OpenAI",
    apiBaseUrl: "https://api.openai.com/v1",
    defaultModel: "gpt-4.1-mini"
  },
  deepseek: {
    label: "DeepSeek",
    apiBaseUrl: "https://api.deepseek.com/v1",
    defaultModel: "deepseek-chat"
  }
} as const;

const fallbackSettings: OperatorSettings = {
  language: "zh-CN",
  providerVendor: "alibaba",
  apiBaseUrl: vendorPresets.alibaba.apiBaseUrl,
  defaultModel: vendorPresets.alibaba.defaultModel,
  apiKey: ""
};

const language = ref<"zh-CN" | "en-US">(fallbackSettings.language);
const providerVendor = ref(fallbackSettings.providerVendor);
const apiBaseUrl = ref(fallbackSettings.apiBaseUrl);
const defaultModel = ref(fallbackSettings.defaultModel);
const apiKey = ref(fallbackSettings.apiKey);
const settingsMessage = ref("");

const dictionary = {
  "zh-CN": {
    headerEyebrow: "AutoLoop 控制平面",
    headerTitle: "自治认知运行控制台",
    headerMetaRuntime: "GraphRAG 运行时",
    headerMetaGovernance: "能力治理",
    headerMetaReplay: "会话回放",
    headerMetaVendor: "模型厂商",
    summaryVerifier: "验证器",
    summaryCapabilities: "能力面",
    summaryOpenCircuits: "熔断器",
    summaryGraphNodes: "图谱节点",
    summaryAgents: "智能体",
    summaryTreatment: "实验流量",
    agentsTitle: "智能体",
    agentsSubtitle: "运行状态与信誉分",
    capabilitiesTitle: "能力目录",
    capabilitiesSubtitle: "快速巡检",
    sessionsTitle: "会话",
    sessionsSubtitle: "当前控制上下文",
    filtersTitle: "图谱筛选",
    filtersSubtitle: "实体与关系切片",
    globalTitle: "全局操作",
    globalSubtitle: "连接、刷新、运行配置",
    dashboardUrl: "Dashboard 地址",
    replayUrl: "Replay 地址",
    loadDashboard: "加载 dashboard",
    loadReplay: "加载 replay",
    refreshData: "刷新数据",
    clearFocus: "清空图谱焦点",
    language: "语言",
    vendor: "模型厂商",
    apiKey: "API Key",
    apiBaseUrl: "兼容接口地址",
    model: "默认模型",
    saveSettings: "保存设置",
    resetSettings: "恢复预设",
    settingsSaved: "设置已保存到运行时目录",
    settingsHint: "仅用于本机运行和一键启动脚本读取，不会写入仓库配置文件。",
    canvasTitle: "GraphRAG 主画布",
    canvasSubtitle: "图谱 + 运行时叠层",
    tabDetail: "详情",
    tabEvolution: "进化流",
    tabGovernance: "治理"
  },
  "en-US": {
    headerEyebrow: "AutoLoop Control Plane",
    headerTitle: "Autonomous Cognition Console",
    headerMetaRuntime: "GraphRAG Runtime",
    headerMetaGovernance: "Capability Governance",
    headerMetaReplay: "Session Replay",
    headerMetaVendor: "Model Vendor",
    summaryVerifier: "Verifier",
    summaryCapabilities: "Capabilities",
    summaryOpenCircuits: "Open Circuits",
    summaryGraphNodes: "Graph Nodes",
    summaryAgents: "Agents",
    summaryTreatment: "Treatment",
    agentsTitle: "Agents",
    agentsSubtitle: "Runtime states and reputation",
    capabilitiesTitle: "Capabilities",
    capabilitiesSubtitle: "Catalog quick scan",
    sessionsTitle: "Sessions",
    sessionsSubtitle: "Current control context",
    filtersTitle: "Graph Filters",
    filtersSubtitle: "Entity and relation slices",
    globalTitle: "Global Actions",
    globalSubtitle: "Connect, refresh, runtime settings",
    dashboardUrl: "Dashboard URL",
    replayUrl: "Replay URL",
    loadDashboard: "Load dashboard",
    loadReplay: "Load replay",
    refreshData: "Refresh data",
    clearFocus: "Clear graph focus",
    language: "Language",
    vendor: "Model vendor",
    apiKey: "API Key",
    apiBaseUrl: "Compatible base URL",
    model: "Default model",
    saveSettings: "Save settings",
    resetSettings: "Reset preset",
    settingsSaved: "Settings saved to runtime directory",
    settingsHint: "Used for local runtime and startup scripts only, not committed into repository config.",
    canvasTitle: "GraphRAG Canvas",
    canvasSubtitle: "Graph + runtime overlay",
    tabDetail: "Detail",
    tabEvolution: "Evolution",
    tabGovernance: "Governance"
  }
} as const;

function persistLocal() {
  localStorage.setItem(
    STORAGE_KEY,
    JSON.stringify({
      language: language.value,
      providerVendor: providerVendor.value,
      apiBaseUrl: apiBaseUrl.value,
      defaultModel: defaultModel.value,
      apiKey: apiKey.value
    })
  );
}

function applyVendorPreset(vendor: string) {
  const preset = vendorPresets[vendor as keyof typeof vendorPresets];
  if (!preset) return;
  providerVendor.value = vendor;
  apiBaseUrl.value = preset.apiBaseUrl;
  defaultModel.value = preset.defaultModel;
}

function resetToPreset() {
  applyVendorPreset(providerVendor.value);
  settingsMessage.value = "";
  persistLocal();
}

function loadLocal() {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (!raw) return;
  try {
    const parsed = JSON.parse(raw) as Partial<OperatorSettings>;
    language.value = parsed.language === "en-US" ? "en-US" : "zh-CN";
    providerVendor.value = parsed.providerVendor ?? fallbackSettings.providerVendor;
    apiBaseUrl.value = parsed.apiBaseUrl ?? fallbackSettings.apiBaseUrl;
    defaultModel.value = parsed.defaultModel ?? fallbackSettings.defaultModel;
    apiKey.value = parsed.apiKey ?? "";
  } catch {
    // ignore local parse failures
  }
}

async function loadRemote(baseUrl: string) {
  try {
    const remote = await fetchOperatorSettings(baseUrl);
    language.value = remote.language;
    providerVendor.value = remote.providerVendor;
    apiBaseUrl.value = remote.apiBaseUrl;
    defaultModel.value = remote.defaultModel;
    apiKey.value = remote.apiKey;
    persistLocal();
  } catch {
    loadLocal();
  }
}

async function saveRemote(baseUrl: string) {
  const payload: OperatorSettings = {
    language: language.value,
    providerVendor: providerVendor.value,
    apiBaseUrl: apiBaseUrl.value,
    defaultModel: defaultModel.value,
    apiKey: apiKey.value
  };
  await saveOperatorSettings(baseUrl, payload);
  persistLocal();
  settingsMessage.value = dictionary[language.value].settingsSaved;
}

function t(key: keyof typeof dictionary["zh-CN"]) {
  return dictionary[language.value][key];
}

loadLocal();

export function useUiPreferences() {
  return {
    language,
    providerVendor,
    apiBaseUrl,
    defaultModel,
    apiKey,
    settingsMessage,
    vendorOptions: computed(() =>
      Object.entries(vendorPresets).map(([value, preset]) => ({
        value,
        label: preset.label
      }))
    ),
    applyVendorPreset,
    resetToPreset,
    loadRemote,
    saveRemote,
    t
  };
}

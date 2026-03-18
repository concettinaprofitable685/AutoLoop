export interface OperatorSettings {
  language: "zh-CN" | "en-US";
  providerVendor: string;
  apiBaseUrl: string;
  defaultModel: string;
  apiKey: string;
}

function normalize(raw: Record<string, unknown>): OperatorSettings {
  return {
    language: raw.language === "en-US" ? "en-US" : "zh-CN",
    providerVendor: typeof raw.provider_vendor === "string"
      ? raw.provider_vendor
      : typeof raw.providerVendor === "string"
        ? raw.providerVendor
        : "alibaba",
    apiBaseUrl: typeof raw.api_base_url === "string"
      ? raw.api_base_url
      : typeof raw.apiBaseUrl === "string"
        ? raw.apiBaseUrl
        : "https://dashscope.aliyuncs.com/compatible-mode/v1",
    defaultModel: typeof raw.default_model === "string"
      ? raw.default_model
      : typeof raw.defaultModel === "string"
        ? raw.defaultModel
        : "qwen-plus-latest",
    apiKey: typeof raw.api_key === "string"
      ? raw.api_key
      : typeof raw.apiKey === "string"
        ? raw.apiKey
        : ""
  };
}

export async function fetchOperatorSettings(baseUrl: string): Promise<OperatorSettings> {
  const response = await fetch(`${baseUrl.replace(/\/$/, "")}/api/operator/settings`);
  const raw = (await response.json()) as Record<string, unknown>;
  return normalize(raw);
}

export async function saveOperatorSettings(
  baseUrl: string,
  settings: OperatorSettings
): Promise<void> {
  await fetch(`${baseUrl.replace(/\/$/, "")}/api/operator/settings`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      language: settings.language,
      provider_vendor: settings.providerVendor,
      api_base_url: settings.apiBaseUrl,
      default_model: settings.defaultModel,
      api_key: settings.apiKey
    })
  });
}

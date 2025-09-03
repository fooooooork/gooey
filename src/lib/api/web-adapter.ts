// Web API 适配器，将现有 Tauri API 调用转换为 HTTP 请求
export const webApi = {
  // 项目相关 API
  async listProjects() {
    const response = await fetch('/api/projects');
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async createProject(path: string) {
    const response = await fetch('/api/projects', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ path }),
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async getProject(id: string) {
    const response = await fetch(`/api/projects/${id}`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async getProjectSessions(projectId: string) {
    const response = await fetch(`/api/projects/${projectId}/sessions`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  // 代理相关 API
  async listAgents() {
    const response = await fetch('/api/agents');
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async createAgent(agent: any) {
    const response = await fetch('/api/agents', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(agent),
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async updateAgent(id: number, agent: any) {
    const response = await fetch(`/api/agents/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(agent),
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async deleteAgent(id: number) {
    const response = await fetch(`/api/agents/${id}`, {
      method: 'DELETE',
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
  },

  async getAgent(id: number) {
    const response = await fetch(`/api/agents/${id}`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async executeAgent(id: number, projectPath: string, task: string, model?: string) {
    const response = await fetch(`/api/agents/${id}/execute`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ projectPath, task, model }),
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const result = await response.json();
    return result.run_id;
  },

  // 使用统计 API
  async getUsageStats() {
    const response = await fetch('/api/usage/stats');
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async getUsageByDateRange(startDate: string, endDate: string) {
    const response = await fetch(`/api/usage/by-date?start=${startDate}&end=${endDate}`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  // MCP 服务器 API
  async mcpList() {
    const response = await fetch('/api/mcp/servers');
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async mcpAdd(server: any) {
    const response = await fetch('/api/mcp/servers', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(server),
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async mcpRemove(name: string) {
    const response = await fetch(`/api/mcp/servers/${name}`, {
      method: 'DELETE',
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.text();
  },

  async mcpTestConnection(name: string) {
    const response = await fetch(`/api/mcp/servers/${name}/test`, {
      method: 'POST',
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const result = await response.json();
    return result.result;
  },

  // 检查点 API
  async listCheckpoints() {
    const response = await fetch('/api/checkpoints');
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async createCheckpoint(checkpoint: any) {
    const response = await fetch('/api/checkpoints', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(checkpoint),
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  async restoreCheckpoint(id: string, data: any) {
    const response = await fetch(`/api/checkpoints/${id}/restore`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  },

  // 其他 API 方法
  async getHomeDirectory() {
    return "/"; // Web 环境下返回根路径
  },

  async getClaudeSettings() {
    return {}; // Web 环境下返回空设置
  },

  async saveClaudeSettings(settings: any) {
    return "Settings saved"; // Web 环境下的占位实现
  },

  async checkClaudeVersion() {
    return {
      is_installed: true,
      version: "web-version",
      output: "Web version running"
    };
  },

  // 占位实现，Web 环境下可能不支持的功能
  async openNewSession(path?: string) {
    console.warn("openNewSession not fully supported in web mode");
    return "web-session-id";
  },

  async executeClaudeCode(projectPath: string, prompt: string, model: string) {
    console.warn("executeClaudeCode not fully supported in web mode");
  },

  async continueClaudeCode(projectPath: string, prompt: string, model: string) {
    console.warn("continueClaudeCode not fully supported in web mode");
  },

  async resumeClaudeCode(projectPath: string, sessionId: string, prompt: string, model: string) {
    console.warn("resumeClaudeCode not fully supported in web mode");
  },

  async cancelClaudeExecution(sessionId?: string) {
    console.warn("cancelClaudeExecution not fully supported in web mode");
  },

  async listRunningClaudeSessions() {
    return []; // Web 环境下返回空数组
  },

  async getClaudeSessionOutput(sessionId: string) {
    return ""; // Web 环境下返回空字符串
  },

  async listDirectoryContents(directoryPath: string) {
    return []; // Web 环境下返回空数组
  },

  async searchFiles(basePath: string, query: string) {
    return []; // Web 环境下返回空数组
  },

  async getRecentlyModifiedFiles(projectPath: string) {
    return []; // Web 环境下返回空数组
  },

  async findClaudeMdFiles(projectPath: string) {
    return []; // Web 环境下返回空数组
  },

  async readClaudeMdFile(filePath: string) {
    return ""; // Web 环境下返回空字符串
  },

  async saveClaudeMdFile(filePath: string, content: string) {
    return "File saved"; // Web 环境下的占位实现
  },

  async getSystemPrompt() {
    return ""; // Web 环境下返回空字符串
  },

  async saveSystemPrompt(content: string) {
    return "System prompt saved"; // Web 环境下的占位实现
  },

  async loadSessionHistory(sessionId: string, projectId: string) {
    return []; // Web 环境下返回空数组
  },

  async loadAgentSessionHistory(sessionId: string) {
    return []; // Web 环境下返回空数组
  },

  async listAgentRuns(agentId?: number) {
    return []; // Web 环境下返回空数组
  },

  async listAgentRunsWithMetrics(agentId?: number) {
    return []; // Web 环境下返回空数组
  },

  async getAgentRun(id: number) {
    throw new Error("getAgentRun not supported in web mode");
  },

  async getAgentRunWithRealTimeMetrics(id: number) {
    throw new Error("getAgentRunWithRealTimeMetrics not supported in web mode");
  },

  async listRunningAgentSessions() {
    return []; // Web 环境下返回空数组
  },

  async killAgentSession(runId: number) {
    return false; // Web 环境下返回 false
  },

  async getSessionStatus(runId: number) {
    return null; // Web 环境下返回 null
  },

  async cleanupFinishedProcesses() {
    return []; // Web 环境下返回空数组
  },

  async getSessionOutput(runId: number) {
    return ""; // Web 环境下返回空字符串
  },

  async getLiveSessionOutput(runId: number) {
    return ""; // Web 环境下返回空字符串
  },

  async streamSessionOutput(runId: number) {
    // Web 环境下无操作
  },

  async exportAgent(id: number) {
    return "{}"; // Web 环境下返回空 JSON
  },

  async importAgent(jsonData: string) {
    throw new Error("importAgent not supported in web mode");
  },

  async importAgentFromFile(filePath: string) {
    throw new Error("importAgentFromFile not supported in web mode");
  },

  async fetchGitHubAgents() {
    return []; // Web 环境下返回空数组
  },

  async fetchGitHubAgentContent(downloadUrl: string) {
    throw new Error("fetchGitHubAgentContent not supported in web mode");
  },

  async importAgentFromGitHub(downloadUrl: string) {
    throw new Error("importAgentFromGitHub not supported in web mode");
  },

  async getClaudeBinaryPath() {
    return null; // Web 环境下返回 null
  },

  async setClaudeBinaryPath(path: string) {
    // Web 环境下无操作
  },

  async listClaudeInstallations() {
    return []; // Web 环境下返回空数组
  },

  async getUsageDetails(limit?: number) {
    return []; // Web 环境下返回空数组
  },

  async getSessionStats(since?: string, until?: string, order?: "asc" | "desc") {
    return []; // Web 环境下返回空数组
  },

  async createCheckpoint(sessionId: string, projectId: string, projectPath: string, messageIndex?: number, description?: string) {
    throw new Error("createCheckpoint not supported in web mode");
  },

  async restoreCheckpoint(checkpointId: string, sessionId: string, projectId: string, projectPath: string) {
    throw new Error("restoreCheckpoint not supported in web mode");
  },

  async listCheckpoints(sessionId: string, projectId: string, projectPath: string) {
    return []; // Web 环境下返回空数组
  },

  async forkFromCheckpoint(checkpointId: string, sessionId: string, projectId: string, projectPath: string, newSessionId: string, description?: string) {
    throw new Error("forkFromCheckpoint not supported in web mode");
  },

  async getSessionTimeline(sessionId: string, projectId: string, projectPath: string) {
    throw new Error("getSessionTimeline not supported in web mode");
  },

  async updateCheckpointSettings(sessionId: string, projectId: string, projectPath: string, autoCheckpointEnabled: boolean, checkpointStrategy: any) {
    // Web 环境下无操作
  },

  async getCheckpointDiff(fromCheckpointId: string, toCheckpointId: string, sessionId: string, projectId: string) {
    throw new Error("getCheckpointDiff not supported in web mode");
  },

  async trackCheckpointMessage(sessionId: string, projectId: string, projectPath: string, message: string) {
    // Web 环境下无操作
  },

  async checkAutoCheckpoint(sessionId: string, projectId: string, projectPath: string, message: string) {
    return false; // Web 环境下返回 false
  },

  async cleanupOldCheckpoints(sessionId: string, projectId: string, projectPath: string, keepCount: number) {
    return 0; // Web 环境下返回 0
  },

  async getCheckpointSettings(sessionId: string, projectId: string, projectPath: string) {
    return {
      auto_checkpoint_enabled: false,
      checkpoint_strategy: "manual",
      total_checkpoints: 0,
      current_checkpoint_id: null
    };
  },

  async clearCheckpointManager(sessionId: string) {
    // Web 环境下无操作
  },

  trackSessionMessages: async (sessionId: string, projectId: string, projectPath: string, messages: string[]) => {
    // Web 环境下无操作
  },

  async mcpAdd(name: string, transport: string, command?: string, args: string[] = [], env: Record<string, string> = {}, url?: string, scope: string = "local") {
    return this.mcpAdd({ name, transport, command, args, env, url, scope });
  },

  async mcpAddJson(name: string, jsonConfig: string, scope: string = "local") {
    const config = JSON.parse(jsonConfig);
    return this.mcpAdd({ name, ...config, scope });
  },

  async mcpAddFromClaudeDesktop(scope: string = "local") {
    return { imported_count: 0, failed_count: 0, servers: [] };
  },

  async mcpServe() {
    return "MCP server started";
  },

  async mcpResetProjectChoices() {
    return "Project choices reset";
  },

  async mcpGetServerStatus() {
    return {}; // Web 环境下返回空对象
  },

  async mcpReadProjectConfig(projectPath: string) {
    return { mcpServers: {} };
  },

  async mcpSaveProjectConfig(projectPath: string, config: any) {
    return "Project config saved";
  },

  async mcpGet(name: string) {
    throw new Error("mcpGet not supported in web mode");
  },

  async getHooksConfig(scope: 'user' | 'project' | 'local', projectPath?: string) {
    return { hooks: [] };
  },

  async updateHooksConfig(scope: 'user' | 'project' | 'local', hooks: any, projectPath?: string) {
    return "Hooks config updated";
  },

  async validateHookCommand(command: string) {
    return { valid: true, message: "Command is valid" };
  },

  async getMergedHooksConfig(projectPath: string) {
    return { hooks: [] };
  },

  async slashCommandsList(projectPath?: string) {
    return []; // Web 环境下返回空数组
  },

  async slashCommandGet(commandId: string) {
    throw new Error("slashCommandGet not supported in web mode");
  },

  async slashCommandSave(scope: string, name: string, namespace: string | undefined, content: string, description: string | undefined, allowedTools: string[], projectPath?: string) {
    throw new Error("slashCommandSave not supported in web mode");
  },

  async slashCommandDelete(commandId: string, projectPath?: string) {
    return "Command deleted";
  },

  async getSetting(key: string) {
    return localStorage.getItem(`app_setting:${key}`);
  },

  async saveSetting(key: string, value: string) {
    localStorage.setItem(`app_setting:${key}`, value);
  },

  async storageListTables() {
    return []; // Web 环境下返回空数组
  },

  async storageReadTable(tableName: string, page: number, pageSize: number, searchQuery?: string) {
    return { data: [], total: 0, page, pageSize, hasMore: false };
  },

  async storageUpdateRow(tableName: string, primaryKeyValues: Record<string, any>, updates: Record<string, any>) {
    // Web 环境下无操作
  },

  async storageDeleteRow(tableName: string, primaryKeyValues: Record<string, any>) {
    // Web 环境下无操作
  },

  async storageInsertRow(tableName: string, values: Record<string, any>) {
    return 1; // Web 环境下返回 1
  },

  async storageExecuteSql(query: string) {
    return []; // Web 环境下返回空数组
  },

  async storageResetDatabase() {
    // Web 环境下无操作
  }
};

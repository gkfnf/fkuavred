# UAVRed PRD（最终封卷版）

- **版本**: v5.0-final
- **状态**: Ready for Roadmap Decomposition
- **文档日期**: 2026-02-24
- **适用范围**: Desktop Product + Agent Runtime + Data/Execution Plane
- **对应仓库**: `apps/desktop`, `crates/{red,logic,infra,database,memory,ui}`, `tests/e2e`

---

## 0. 文档目标与使用方式

本 PRD 的目标不是“描述愿景”，而是**直接定义可开发、可测试、可验收**的产品规格。阅读完本文件后，团队应能够：

1. 明确产品最终形态（用户看到什么、如何操作、系统如何响应）。
2. 直接拆解 Roadmap（Epic -> Feature -> Story -> Test Case）。
3. 直接落到代码模块与测试模块（crate 级边界清晰）。
4. 用统一验收口径判断“是否完成”。

---

## 1. 产品定义（最终形态）

UAVRed 是一款面向无人机生态（RF/链路/地面站/云/供应链/运营）的**自主红队桌面平台**。
用户在 GUI 中完成资产建模、威胁建模、任务编排、执行审批与结果处置；
系统在后端通过 Agent Runtime（参考 pi-mono 架构思想）完成自主执行、复盘和证据沉淀。

### 1.1 核心价值（必须兑现）

| 价值 | 说明 | 交付判断标准 |
|---|---|---|
| 自主执行 | 用最少人工驱动完成渗透任务执行闭环 | Mission 在最少人工干预下完成并可复现 |
| 可控安全 | 高风险动作必须可审批、可中止、可追溯 | 审批链和审计日志完整可查 |
| 结果可信 | 发现必须有证据、有上下文、有状态流转 | 每个 Findings 都可追溯到执行证据 |
| 可持续迭代 | 需求、代码、测试可持续演进 | Roadmap/测试体系与 PRD 一一对应 |

### 1.2 明确不做（当前版本）

| 不做项 | 原因 |
|---|---|
| SaaS 多租户云端产品化 | 当前聚焦 Desktop 本地可控执行 |
| 自动修复生产系统漏洞 | 当前仅红队验证与建议，不直接运维修复 |
| 与 SIEM/SOAR 深度编排联动 | 当前先提供标准导出与 API 能力 |

---

## 2. 与代码仓映射（开发落地）

| crate | 必须承担能力 |
|---|---|
| `crates/red` | Agent Runtime、状态机、Worker/Tutor、调度执行 |
| `crates/logic` | Mission 编排、审批策略、流程门控、领域规则 |
| `crates/infra` | Sandbox/工具/协议/设备适配 |
| `crates/database` | 实体持久化、审计存储、查询索引 |
| `crates/memory` | 记忆检索、经验回灌 |
| `crates/ui` | 页面、组件、实时状态展示 |
| `apps/desktop` | 启动入口、模块组装、运行配置 |

---

## 3. 角色与权限模型

### 3.1 角色定义

| 角色 | 能力范围 | 关键限制 |
|---|---|---|
| Admin | 全局配置、策略管理、审批规则维护、用户权限 | 不可绕过审计 |
| Operator | 创建任务、执行授权内动作、维护信任图谱 | 高风险动作需审批 |
| Reviewer | 复核 Findings、状态流转、报告签发 | 不可直接改写执行日志 |
| Auditor | 只读审计、导出审计轨迹 | 不可触发执行 |

### 3.2 权限基础原则

1. 默认最小权限。
2. 所有敏感动作必须落审计日志。
3. 一次性授权仅作用于**单 Mission**。
4. **Revoke（撤销授权）必须即时生效**，并且撤销本身不触发 Tutor。

---

## 4. 产品全局信息架构（GUI）

## 4.1 一级导航（必须存在）

| 一级导航 | 功能定位 | 对应 crate 主责 |
|---|---|---|
| Dashboard | 全局态势、执行健康、风险概览 | `ui` + `logic` |
| Missions | 任务创建、编排、审批、执行控制 | `logic` + `red` |
| Trust Graph | 资产/节点/边界建模与路径关系 | `logic` + `database` |
| Checklist/UTT&CK | TTP 模板、策略与映射 | `logic` + `database` |
| Findings (Vulns) | 发现管理、复测、关闭 | `logic` + `database` |
| Evidence | 日志、命令、产物、时间线 | `database` + `ui` |
| Runtime | Agent 与沙箱实时状态 | `red` + `infra` + `ui` |
| Reports | 报告生成、模板、导出 | `logic` + `ui` |
| Settings | 模型/策略/审批/集成配置 | `logic` + `infra` |

### 4.2 全局交互规则

| 规则 | 规格 |
|---|---|
| 页面刷新 | 默认增量刷新（事件驱动）+
手动全量刷新 |
| 长任务反馈 | 必须显示阶段状态、进度、最新事件 |
| 危险操作 | 必须二次确认 + 审计记录 |
| 错误处理 | 必须展示错误码、原因、建议动作 |
| 可追溯性 | 所有状态变更可回放到事件源 |

---

## 5. 模块总表（功能颗粒度总览）

| 模块ID | 模块名称 | 目标 | 子功能数 |
|---|---|---|---|
| M01 | Dashboard | 全局可观测与优先级决策 | 12 |
| M02 | Mission Center | 任务全生命周期管理 | 18 |
| M03 | Trust Graph | 节点/边界/路径建模 | 16 |
| M04 | UTT&CK + Checklist | 方法论驱动执行计划 | 14 |
| M05 | Agent Runtime | Worker/Tutor 自主执行内核 | 24 |
| M06 | Sandbox Plane | 安全隔离执行环境 | 17 |
| M07 | Findings/Vulns | 发现处置闭环 | 15 |
| M08 | Evidence Center | 全量证据管理 | 13 |
| M09 | Report Center | 报告编排与导出 | 12 |
| M10 | Settings & Governance | 策略、审批、配置治理 | 20 |
| M11 | Memory & Knowledge | 记忆层与经验复用 | 10 |
| M12 | Integrations | 工具/设备/协议适配 | 14 |
| M13 | Audit & Compliance | 合规与审计保障 | 11 |
| M14 | Performance & Reliability | 质量属性保障 | 9 |

---

## 6. M01 Dashboard 详细规格

### 6.1 子功能表

| 功能ID | 功能名 | 输入 | 输出 | 交互 | 验收标准 |
|---|---|---|---|---|---|
| M01-F01 | 任务概览卡 | Mission 状态流 | 按状态计数 | 点击跳 Missions 过滤页 | 数据延迟 < 3s |
| M01-F02 | 风险分布 | Findings + UCVSS | 风险分桶图 | 点击桶跳 Findings 列表 | 与详情页计数一致 |
| M01-F03 | 执行健康 | Runtime heartbeat | 在线/异常节点 | 点击节点看日志 | 心跳超时自动告警 |
| M01-F04 | 最近关键事件 | 审计事件流 | 时间线列表 | 点击跳 Evidence 详情 | 顺序与时间戳一致 |
| M01-F05 | 待审批队列 | Approval queue | 列表 + SLA | 点击执行审批 | 状态实时变更 |
| M01-F06 | Zone 风险热力 | Trust + Findings | Zone 风险矩阵 | 点击 Zone Drill-down | 风险聚合准确 |
| M01-F07 | 设备状态概览 | Device manager | 在线/占用状态 | 点击跳 Runtime | 状态和执行一致 |
| M01-F08 | 策略违规提醒 | Policy engine | 违规卡片 | 点击查看策略命中详情 | 规则命中可解释 |
| M01-F09 | 报告待发布 | Report queue | 待审/待签发 | 点击进入报告中心 | 与报告状态一致 |
| M01-F10 | 导航快捷入口 | 系统状态 | 快速操作按钮 | 一键跳目标页 | 入口可配置 |
| M01-F11 | 自定义小组件 | 用户偏好 | 模块布局 | 拖拽排序 | 布局持久化 |
| M01-F12 | 全局搜索框 | 全局索引 | 跳转结果 | 输入后下拉建议 | 查询 < 500ms（P95） |

### 6.2 Dashboard 页面组件

| 区域 | 组件 | 说明 |
|---|---|---|
| 顶栏 | 时间范围选择、刷新、全局搜索 | 统一上下文 |
| 左列 | 任务与审批卡片 | 当日行动重点 |
| 中列 | 风险热力 + 趋势图 | 风险态势判断 |
| 右列 | 执行健康 + 关键事件 | 稳定性与可追踪性 |
| 底部 | 快捷入口与自定义组件 | 降低操作成本 |

---

## 7. M02 Mission Center（核心）

### 7.1 Mission 生命周期（固定）

| 状态 | 含义 | 进入条件 | 退出条件 |
|---|---|---|---|
| Todo | 任务已创建待执行 | 新建 Mission | 启动执行 |
| NeedReview | 需要人工复核 | 命中审批/策略门 | 审批通过或取消 |
| Done | 任务完成 | 执行完成并归档 | N/A |
| Cancelled | 任务取消 | 用户取消/策略终止 | N/A |

### 7.2 子功能表

| 功能ID | 功能名 | 说明 | 核心约束 |
|---|---|---|---|
| M02-F01 | Mission 新建向导 | 目标、范围、策略、资源 | 必填项校验 |
| M02-F02 | 执行模式选择 | Manual / Assisted / Autonomous | 权限依赖角色 |
| M02-F03 | Scope 定义 | Zone、节点、接口、排除项 | 禁止越权范围 |
| M02-F04 | TTP 选择 | 从 Checklist 绑定执行清单 | 支持版本锁定 |
| M02-F05 | 预算策略 | token/time/attempt 限制 | 超限触发策略 |
| M02-F06 | 审批策略绑定 | Mission 级审批流 | 一次性授权仅当前 Mission |
| M02-F07 | 运行前检查 | 资源、设备、依赖、权限 | 未通过不可启动 |
| M02-F08 | 启动执行 | 创建 run_id | 记录启动上下文 |
| M02-F09 | 暂停/恢复 | 可控执行 | 状态一致性保障 |
| M02-F10 | Revoke | 即时回收授权 | 不触发 Tutor |
| M02-F11 | Stop/Cancel | 强制终止任务 | 审计必留痕 |
| M02-F12 | 动态步骤视图 | LuaN1ao 风格执行轨迹 | 秒级刷新 |
| M02-F13 | 子任务拆分 | DAG 执行单元展示 | 节点依赖可见 |
| M02-F14 | 人工注入指令 | 对运行中任务追加约束 | 指令全审计 |
| M02-F15 | 失败重试 | 可控重试策略 | 次数受限 |
| M02-F16 | 完成归档 | 写入报告候选 | 证据完整性校验 |
| M02-F17 | 模板化复用 | Mission as template | 支持参数化 |
| M02-F18 | Mission 比对 | 历史结果对比 | 支持版本对比 |

---

## 8. M03 Trust Graph（资产/节点/信任边界）

### 8.1 数据建模核心

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| node_id | string | 是 | 节点唯一标识 |
| node_type | enum | 是 | Asset / Service / Human / Device / Agent / External |
| trust_zone | enum | 是 | Zone0~Zone5 |
| responsor | string | 是 | 责任人（非 owner） |
| interfaces | array | 是 | 接口列表，且与 node_type 耦合校验 |
| tags | array | 否 | 分类标签 |
| criticality | enum | 是 | C1~C4 |
| exposure | enum | 是 | internal/partner/public |
| status | enum | 是 | active/inactive/deprecated |

### 8.2 子功能表

| 功能ID | 功能名 | 说明 | 验收标准 |
|---|---|---|---|
| M03-F01 | 图谱画布 | 可视化节点与边 | 拖拽流畅 |
| M03-F02 | 节点 CRUD | 新增/编辑/归档 | 字段校验严格 |
| M03-F03 | 边关系管理 | trust/call/data/control | 不允许孤立边 |
| M03-F04 | Zone 边界校验 | 跨 Zone 路径检查 | 违规提示明确 |
| M03-F05 | 接口规范校验 | 类型与协议对应 | 保存前阻断错误 |
| M03-F06 | 批量导入 | CSV/JSON 导入 | 提供错误报告 |
| M03-F07 | 版本快照 | 图谱版本化 | 支持回滚 |
| M03-F08 | 变更审计 | 谁改了什么 | 可追溯 |
| M03-F09 | 路径查询 | 任意两点路径分析 | 支持最短/可达 |
| M03-F10 | 攻击路径预估 | 结合 UTT&CK | 给出候选路径 |
| M03-F11 | 与 Mission 绑定 | 作为范围输入 | 同步校验 |
| M03-F12 | 与 Findings 反查 | 从发现回溯节点 | 双向跳转 |
| M03-F13 | 图谱健康评分 | 完整性/一致性检查 | 可解释评分 |
| M03-F14 | 视图筛选 | 按 Zone/标签/风险过滤 | 响应 < 500ms |
| M03-F15 | 模板库 | 标准拓扑模板 | 一键应用 |
| M03-F16 | 外部依赖节点 | 第三方系统建模 | 边界标识清晰 |

---

## 9. M04 UTT&CK + Checklist

### 9.1 子功能表

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M04-F01 | UTT&CK 矩阵浏览 | tactic/technique/procedure 分层 |
| M04-F02 | Technique 详情页 | 前置条件、风险、证据模板 |
| M04-F03 | Checklist 模板管理 | 创建/版本化/发布 |
| M04-F04 | Checklist 参数化 | 变量替换、环境注入 |
| M04-F05 | Mission 绑定 | 在任务中引用版本 |
| M04-F06 | 步骤启停 | 运行中可按策略跳过/暂停 |
| M04-F07 | 策略门配置 | 某步骤必须审批 |
| M04-F08 | 安全护栏注入 | 禁止动作列表 |
| M04-F09 | 结果回写 | 每个步骤产出结构化结果 |
| M04-F10 | 失败分支 | 重试/回退/替代路径 |
| M04-F11 | 经验回灌 | 完成后优化模板 |
| M04-F12 | 兼容矩阵 | Technique 与 Zone/接口兼容性 |
| M04-F13 | 质量评分 | 模板可执行性评分 |
| M04-F14 | 模板审批 | 新模板发布前审查 |

---

## 10 M05 Agent Runtime（参考 pi-mono 的核心能力实现）

> 本模块是产品内核，必须细化到“代码要支持什么行为”。

### 10.1 Runtime 架构角色

| 角色 | 职责 | 关键能力 |
|---|---|---|
| Planner | 将 Mission 拆成执行 DAG | 依赖分析、资源估算 |
| Scheduler | 分配任务到 Worker/Tutor | 负载均衡、优先级、重试 |
| Worker Agent | 执行具体动作 | Observe-Think-Act 循环 |
| Tutor Agent | 高风险/失败时辅助 | 推理建议、路线修正 |
| Policy Engine | 执行前后策略评估 | allow/deny/review |
| Approval Gate | 人工审批接入点 | 阻塞式门控 |
| Execution Recorder | 记录完整执行证据 | 事件、命令、产物 |

### 10.2 Worker/Tutor 运行规则（强约束）

| 规则ID | 规则内容 |
|---|---|
| R-RT-01 | Worker 与 Tutor 使用同一代码基，角色由 runtime role 切换 |
| R-RT-02 | Worker 可持续尝试 PoC，直到达到 `max_attempts`（默认 10） |
| R-RT-03 | 触发 Tutor 的条件必须可配置（失败阈值/风险阈值/审批拒绝） |
| R-RT-04 | Revoke 后 Worker 必须在下一个可中断点停止高权限动作 |
| R-RT-05 | Revoke 事件不触发 Tutor，仅执行安全收敛流程 |
| R-RT-06 | 所有 Agent 决策均需写入推理摘要（可审计） |
| R-RT-07 | 工具调用必须带上下文与权限令牌，不允许裸调用 |
| R-RT-08 | 每次循环必须记录输入、动作、结果、耗时、错误码 |

### 10.3 Runtime 状态机

| 状态 | 描述 | 可转移状态 |
|---|---|---|
| INIT | 准备上下文 | READY / FAILED |
| READY | 等待调度 | RUNNING / CANCELLED |
| RUNNING | 执行中 | WAIT_APPROVAL / RETRYING / COMPLETED / FAILED / REVOKED |
| WAIT_APPROVAL | 等待人工审批 | RUNNING / CANCELLED |
| RETRYING | 失败重试 | RUNNING / FAILED |
| REVOKED | 授权被撤销 | SAFE_STOPPED |
| SAFE_STOPPED | 安全停机完成 | COMPLETED / CANCELLED |
| COMPLETED | 成功完成 | N/A |
| FAILED | 执行失败 | RETRYING / CANCELLED |
| CANCELLED | 人工或策略取消 | N/A |

### 10.4 Runtime 子功能表

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M05-F01 | DAG 构建 | 由 Mission + Checklist 生成执行图 |
| M05-F02 | 任务切片 | 大任务拆为可调度执行单元 |
| M05-F03 | 资源感知调度 | 按设备/沙箱/模型预算调度 |
| M05-F04 | Agent 上下文注入 | 目标、策略、历史记忆注入 |
| M05-F05 | Observe-Think-Act Loop | 核心循环 |
| M05-F06 | Tool 调用网关 | 工具统一调用入口 |
| M05-F07 | 失败分类器 | 可恢复/不可恢复分类 |
| M05-F08 | 重试策略引擎 | 指数退避、策略重试 |
| M05-F09 | Tutor 升级路径 | 失败时切 Tutor |
| M05-F10 | 审批阻塞门 | 高风险动作前阻塞 |
| M05-F11 | 一次性授权令牌 | mission scoped token |
| M05-F12 | Revoke 执行器 | 即时撤销权限 |
| M05-F13 | 安全收敛流程 | 停止、回滚、清理 |
| M05-F14 | 动态步骤事件推送 | GUI 秒级更新 |
| M05-F15 | 运行指标采集 | latency, success, cost |
| M05-F16 | 推理摘要记录 | 便于审计与复盘 |
| M05-F17 | 证据打包器 | 产物归档 |
| M05-F18 | 运行快照 | 中断续跑 |
| M05-F19 | 并发保护 | 并行执行一致性 |
| M05-F20 | 幂等保护 | 重试不重复破坏 |
| M05-F21 | 异常熔断 | 连续错误自动熔断 |
| M05-F22 | 预算守卫 | token/time/cost 限额 |
| M05-F23 | 运行模拟模式 | dry-run 校验 |
| M05-F24 | 版本兼容控制 | runtime/schema/version 对齐 |

---

## 11. M06 Sandbox Plane

### 11.1 子功能表

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M06-F01 | Sandbox 生命周期 | create/start/stop/destroy |
| M06-F02 | 镜像管理 | 基础镜像与任务镜像 |
| M06-F03 | 网络隔离策略 | 出口控制、目标白名单 |
| M06-F04 | 文件系统隔离 | 只读/可写挂载控制 |
| M06-F05 | Secret 注入 | 临时凭据注入与销毁 |
| M06-F06 | 工具安装策略 | 白名单工具集 |
| M06-F07 | 设备桥接 | RF 设备独占管理 |
| M06-F08 | 资源限流 | CPU/MEM/IO 限额 |
| M06-F09 | 会话回放支持 | 保留执行上下文 |
| M06-F10 | 异常沙箱回收 | 防泄漏回收 |
| M06-F11 | 输出产物抽取 | 安全导出证据 |
| M06-F12 | 快照与回滚 | 执行前后快照 |
| M06-F13 | 审计水位线 | 沙箱操作全留痕 |
| M06-F14 | 兼容 BoxLite/HVF | mac 环境运行支持 |
| M06-F15 | 多沙箱并发编排 | 任务级隔离执行 |
| M06-F16 | 风险动作阻断 | 策略命中立刻阻断 |
| M06-F17 | 清理验证 | 执行后残留检查 |

---

## 12. M07 Findings / Vulns

### 12.1 Findings 生命周期

| 状态 | 说明 |
|---|---|
| New | 新发现待确认 |
| Confirmed | 已确认为有效问题 |
| NeedRetest | 已提交修复待复测 |
| FalsePositive | 误报 |
| Closed | 生命周期结束 |

> 说明：Confirmed 后不引入额外 Mitigated 状态，统一走 NeedRetest/Closed 分支。

### 12.2 子功能表

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M07-F01 | 自动发现入库 | 从执行结果自动提取 |
| M07-F02 | 去重与聚合 | 相同根因聚合 |
| M07-F03 | 人工确认流程 | Reviewer 确认 |
| M07-F04 | UCVSS 自动打分 | 按因子计算 |
| M07-F05 | 证据绑定 | 命令/日志/截图绑定 |
| M07-F06 | 根因描述模板 | 结构化根因 |
| M07-F07 | 修复建议模板 | 可执行建议 |
| M07-F08 | 状态流转 | New->Confirmed->NeedRetest->Closed |
| M07-F09 | 误报处理 | FalsePositive 归档 |
| M07-F10 | 复测任务生成 | 一键生成 Retest Mission |
| M07-F11 | 风险排序 | 按 UCVSS + 暴露面 |
| M07-F12 | 节点反查 | 回溯 Trust Graph |
| M07-F13 | 报告联动 | 自动进入报告草稿 |
| M07-F14 | SLA 跟踪 | 修复时效统计 |
| M07-F15 | 变更审计 | 状态变更全留痕 |

### 12.3 UCVSS 计算规范（V1）

`UCVSS = Base(Impact, Exploitability) + Context(Exposure, MissionCriticality, Detectability, Recoverability)`

| 因子 | 级别 | 分值范围 |
|---|---|---|
| Impact | Low/Medium/High/Critical | 1/2/3/4 |
| Exploitability | Hard/Moderate/Easy/Trivial | 1/2/3/4 |
| Exposure | Internal/Partner/Public | 1/2/3 |
| MissionCriticality | C1/C2/C3/C4 | 1/2/3/4 |
| Detectability | Easy/Normal/Hard | 1/0/-1 |
| Recoverability | Fast/Normal/Slow | 0/1/2 |

---

## 13. M08 Evidence Center

### 13.1 子功能表

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M08-F01 | 事件时间线 | 按 mission/run 回放 |
| M08-F02 | 命令记录 | 命令、参数、返回码 |
| M08-F03 | 文件证据库 | 截图、payload、dump |
| M08-F04 | 网络证据 | pcap/http trace |
| M08-F05 | 推理摘要 | Agent 决策摘要 |
| M08-F06 | 审批证据 | 审批人、理由、时间 |
| M08-F07 | 策略命中记录 | allow/deny/review 轨迹 |
| M08-F08 | 过滤检索 | 多维索引检索 |
| M08-F09 | 证据导出 | 按 findings/report 导出 |
| M08-F10 | 完整性校验 | hash 与签名 |
| M08-F11 | 留存策略 | 生命周期管理 |
| M08-F12 | 隐私策略 | 分级访问控制 |
| M08-F13 | 链路追踪视图 | 关联 mission->run->finding |

---

## 14. M09 Report Center

### 14.1 子功能表

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M09-F01 | 报告模板 | Exec/Tech/Compliance 模板 |
| M09-F02 | 资产索引报告 | 按节点汇总 |
| M09-F03 | 任务索引报告 | 按 mission 汇总 |
| M09-F04 | Findings 自动填充 | 结构化拉取 |
| M09-F05 | 风险趋势图 | UCVSS 趋势 |
| M09-F06 | 修复建议章节 | 自动拼装 |
| M09-F07 | 手工编辑 | 富文本编辑 |
| M09-F08 | 审核流 | draft->review->approved |
| M09-F09 | 导出 HTML/PDF | 两种格式 |
| M09-F10 | 报告签名 | 审核人签名留痕 |
| M09-F11 | 版本管理 | 历史版本可追踪 |
| M09-F12 | 发布归档 | 报告归档与检索 |

---

## 15. M10 Settings & Governance

### 15.1 子功能表

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M10-F01 | 用户与角色管理 | RBAC |
| M10-F02 | 默认权限策略 | 最小权限默认 |
| M10-F03 | 审批流配置 | 多级审批 |
| M10-F04 | 一次性授权策略 | 作用域 Mission 级 |
| M10-F05 | Revoke 策略 | 即时回收 |
| M10-F06 | Policy 规则库 | allow/deny/review |
| M10-F07 | 模型配置 | 多 provider 路由 |
| M10-F08 | 成本预算 | token/cost 上限 |
| M10-F09 | Retry 默认策略 | 失败重试参数 |
| M10-F10 | Tutor 触发阈值 | 可配置规则 |
| M10-F11 | 日志保留策略 | 审计留存天数 |
| M10-F12 | 数据备份策略 | 本地备份计划 |
| M10-F13 | 恢复演练开关 | 备份恢复验证 |
| M10-F14 | 集成配置中心 | 工具/API/设备 |
| M10-F15 | 证书/密钥管理 | 加密材料管理 |
| M10-F16 | 主题与界面偏好 | UI 偏好 |
| M10-F17 | 告警策略 | 阈值与通知 |
| M10-F18 | 合规模板 | 法规映射配置 |
| M10-F19 | 字典与标签 | 分类体系 |
| M10-F20 | 实验特性开关 | Feature flag |

---

## 16. M11 Memory & Knowledge

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M11-F01 | 情景记忆 | mission/run 上下文 |
| M11-F02 | 语义记忆 | 技术知识向量索引 |
| M11-F03 | 程序记忆 | 可复用操作序列 |
| M11-F04 | 失败案例记忆 | 避免重复踩坑 |
| M11-F05 | 模板推荐 | 任务创建时推荐 checklist |
| M11-F06 | 风险偏好学习 | 结合人工反馈调整 |
| M11-F07 | 记忆质量评估 | 命中率与有效性 |
| M11-F08 | 记忆过期策略 | 自动清理 |
| M11-F09 | 隐私分级存储 | 敏感数据隔离 |
| M11-F10 | 可解释引用 | 回答/建议的记忆来源 |

---

## 17. M12 Integrations

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M12-F01 | HTTP 工具适配 | REST/GraphQL |
| M12-F02 | TrafficQL 查询层 | 流量语义查询 |
| M12-F03 | 浏览器自动化 | Web 攻击面验证 |
| M12-F04 | Caido/Burp 适配 | 代理生态联动 |
| M12-F05 | RF 设备管理 | USRP/HackRF/RFSoC |
| M12-F06 | MAVLink 工具链 | 协议测试 |
| M12-F07 | SSH/CLI 工具 | 终端操作 |
| M12-F08 | 文件存储适配 | 证据导入导出 |
| M12-F09 | 身份源对接 | LDAP/OIDC（预留） |
| M12-F10 | 消息通知 | Slack/Email（预留） |
| M12-F11 | PDF/HTML 渲染 | 报告生成依赖 |
| M12-F12 | 插件机制 | 第三方扩展 |
| M12-F13 | 适配器健康检查 | 可用性探测 |
| M12-F14 | 适配器沙箱执行 | 防止越权访问 |

---

## 18. M13 审计与合规

| 功能ID | 功能名 | 说明 |
|---|---|---|
| M13-F01 | 全量审计日志 | 用户/系统/Agent 动作 |
| M13-F02 | 不可抵赖签名 | 关键日志签名 |
| M13-F03 | 审计查询中心 | 多维查询与导出 |
| M13-F04 | 合规规则检查 | 对标内部规范 |
| M13-F05 | 高危动作告警 | 实时告警 |
| M13-F06 | 审计报表 | 周/月审计汇总 |
| M13-F07 | 异常行为检测 | 行为基线偏移 |
| M13-F08 | 授权链路回放 | 谁批准了什么 |
| M13-F09 | 证据链完整性 | hash 校验 |
| M13-F10 | 访问控制审计 | 数据访问记录 |
| M13-F11 | 审计留存策略 | 保留与销毁管理 |

---

## 19. M14 非功能与质量约束

| 类别 | 指标 | 目标 |
|---|---|---|
| 性能 | 页面交互响应 | P95 < 500ms |
| 性能 | Mission 事件可视延迟 | < 2s |
| 稳定性 | Runtime 可用性 | >= 99.5% |
| 稳定性 | 崩溃恢复 | 关键数据不丢失 |
| 安全 | 高危动作防误触 | 100% 二次确认 |
| 安全 | 审计覆盖率 | 100% 关键动作 |
| 可维护性 | crate 边界依赖清晰 | 禁止跨层反向依赖 |
| 可测试性 | 核心流程自动化覆盖 | >= 80%（核心路径） |
| 可扩展性 | 新工具接入 | 适配器模式可插拔 |

---

## 20. 数据模型总表（核心实体）

| 实体 | 主键 | 关键字段 |
|---|---|---|
| mission | mission_id | name, status, scope, policy_id, created_by |
| mission_run | run_id | mission_id, runtime_status, started_at, ended_at |
| runtime_step | step_id | run_id, node, action, result, duration_ms |
| approval_ticket | ticket_id | mission_id, action, approver, decision, expired_at |
| trust_node | node_id | node_type, trust_zone, responsor, interfaces |
| trust_edge | edge_id | from_node, to_node, edge_type |
| finding | finding_id | mission_id, severity, ucvss, status |
| evidence | evidence_id | run_id, finding_id, type, uri, hash |
| report | report_id | scope, version, status, export_format |
| policy_rule | rule_id | condition, effect, priority |
| audit_event | event_id | actor, action, target, timestamp |
| memory_item | memory_id | kind, embedding_ref, quality_score |

---

## 21. 事件与接口契约

### 21.1 关键事件 Topic（示例）

| Topic | Payload 核心字段 | 消费方 |
|---|---|---|
| mission.created | mission_id, creator, scope | ui, logic |
| mission.started | mission_id, run_id | ui, audit |
| runtime.step.updated | run_id, step_id, status | ui timeline |
| runtime.approval.required | ticket_id, reason, action | ui approval panel |
| runtime.revoked | mission_id, run_id, reason | ui, audit |
| finding.created | finding_id, mission_id, ucvss | findings, report |
| report.generated | report_id, format, uri | report center |
| policy.violated | rule_id, target, action | dashboard, audit |

### 21.2 API 设计原则

1. 所有写接口必须带审计上下文。
2. 所有执行接口必须带 mission_id + run_id。
3. 所有高危接口必须过 Approval Gate。
4. 事件与数据库模型版本需同步管理。

---

## 22. GUI 页面级规格（必须实现）

### 22.1 Runtime 页面（重点）

| 区块 | 必备能力 |
|---|---|
| Run Header | mission/run 状态、耗时、预算、控制按钮（pause/resume/revoke/stop） |
| Step Timeline | 实时步骤流，显示动作、结果、耗时、错误 |
| Agent Panel | Worker/Tutor 状态、当前思考摘要 |
| Approval Panel | 待审批动作列表，支持批准/拒绝 |
| Artifact Panel | 当前步骤产物预览 |
| Log Panel | 原始日志流 |

### 22.2 Missions 页面

| 区块 | 必备能力 |
|---|---|
| 列表区 | 多条件过滤、状态批量操作 |
| 详情区 | 配置、执行历史、相关 findings |
| 创建向导 | 目标/范围/TTP/策略/预算 分步创建 |

### 22.3 Findings 页面

| 区块 | 必备能力 |
|---|---|
| 列表区 | 按 UCVSS、状态、Zone 过滤排序 |
| 详情区 | 证据、根因、修复建议、状态流转 |
| 复测入口 | 一键发起 Retest Mission |

---

## 23. 测试与验收（开发必须对齐）

### 23.1 测试分层

| 层级 | 存放位置 | 目标 |
|---|---|---|
| 模块测试 | `crates/*/tests` | 验证 crate 内逻辑 |
| 端到端测试 | `tests/e2e` | 跨模块关键流程 |
| 回归测试 | `tests/e2e/regression_*` | 防止核心流程退化 |

### 23.2 P0 必测场景

| 用例ID | 场景 | 通过标准 |
|---|---|---|
| E2E-P0-01 | 创建 Mission 并完成一次执行 | 状态流转正确，证据完整 |
| E2E-P0-02 | 高风险动作触发审批 | 未审批不可执行 |
| E2E-P0-03 | 任务中途 Revoke | 高权限动作即时停止 |
| E2E-P0-04 | Worker 重试至上限 | 默认 10 次后按策略收敛 |
| E2E-P0-05 | 发现入库并打分 | UCVSS 可复算一致 |
| E2E-P0-06 | 生成 HTML/PDF 报告 | 报告内容完整且可导出 |
| E2E-P0-07 | 审计回放 | 可追溯到每个关键动作 |

### 23.3 Definition of Done（功能级）

1. PRD 功能项有对应实现与测试。
2. GUI 可完成用户闭环操作。
3. Runtime 行为符合状态机与策略门控。
4. Evidence/Audit 可追溯。
5. 关键路径 E2E 全绿。

---

## 24. 分阶段交付范围（Roadmap 输入）

| 阶段 | 范围 | 目标 |
|---|---|---|
| P0 | M01/M02/M03/M05/M06/M07/M10 基础能力 | 可执行、可控、可追溯 |
| P1 | M04/M08/M09/M11 增强能力 | 体系化与可复用 |
| P2 | M12/M13/M14 企业化能力 | 扩展与合规强化 |

---


## 25. 最终验收声明

当且仅当以下条件全部满足，产品视为达到本 PRD 目标：

1. 用户可在 GUI 中完成“建模 -> 任务 -> 审批 -> 执行 -> 发现 -> 报告”闭环。
2. Agent Runtime 可在策略约束下稳定执行，且支持 Revoke 即时生效。
3. Findings 具备完整证据链、可复测闭环与统一 UCVSS 风险评分。
4. 全部关键动作可审计、可回放、可导出。
5. 代码结构与测试结构满足仓库约束（crate tests + e2e）。

---

## 26. 术语（简）

| 术语 | 说明 |
|---|---|
| Mission | 一次完整红队任务单元 |
| Run | Mission 的一次具体执行实例 |
| Worker | 默认执行 Agent |
| Tutor | 辅助/升级 Agent |
| Revoke | 运行中撤销授权动作 |
| Trust Graph | 信任边界拓扑图谱 |
| UTT&CK | 无人机安全战术技术框架 |
| UCVSS | 本产品统一风险评分体系 |

# ChromTool 安全审核报告

审核日期：2026-10-01（America/New_York）。审核基线：`89d3e0a`；修复保留在工作区，没有创建 Git commit。

## 1. 项目概况与范围

- 应用：本地 Chromium 数据管理工具，版本 0.1.4；支持 Windows、macOS。
- 语言与框架：Vue 3、TypeScript、Vite 6；Rust 2021 edition、Tauri 2、rusqlite/SQLite。
- 包管理：pnpm 11.21.0、Cargo；清单为 `package.json`、`pnpm-workspace.yaml`、`src-tauri/Cargo.toml`；锁文件为 `pnpm-lock.yaml`、`src-tauri/Cargo.lock`。
- 直接依赖：原有 npm 9 项（运行 4、开发 5），Rust 8 项（运行 7、构建 1）；修复新增 Rust `tempfile`，目前 Rust 9 项。
- 最终锁文件：npm 116 条包/版本记录，Cargo 494 条外部包/版本记录；包含跨平台、可选和构建依赖，不能当成 Windows EXE 实际链接数量。
- 范围：全部第一方 Rust、Vue、TypeScript、CSS、HTML；Tauri 配置与 capabilities；两个 CI workflow；依赖清单和锁文件；README、资产说明与版本控制配置。凭据扫描另覆盖 `dist` 和应用 EXE，以及本地 `git log --all -p` 历史。
- 本机验证：Windows。没有启动前端开发服务器，没有操作真实浏览器资料或实际删除用户数据。macOS 编译、签名、安装包、实机 WebView 行为和 CI 在线执行均为「待确认」。
- 未对依赖缓存逐文件人工审计，也未扫描每个第三方 PDB 或还未生成的安装包；动态渗透测试和操作系统 WebView 自身漏洞不在本次可验证范围。

## 2. 问题汇总

严重级别默认采用公告；没有 CVSS 的内存健全性公告和代码问题采用本次工程评估。条件性问题不等于已证明可以从网络利用。

| 编号 | 类别 | 严重级别 | 位置 | 状态 |
|---|---|---|---|---|
| D01 | 依赖 | High / Medium | `pnpm-workspace.yaml:7`、`pnpm-lock.yaml` | 已修复：brace-expansion 2.1.4 → 2.1.7 |
| D02 | 依赖 | High | `src-tauri/Cargo.lock:3443`、`src-tauri/Cargo.toml:22` | 已修复：tauri 2.11.1 → 2.11.6 |
| D03 | 依赖 | Medium（可利用性待确认） | `src-tauri/Cargo.lock:1992`、`src-tauri/src/scanner.rs:793` | 已修复：SQLite 3.46.0 → 3.53.2；升级经授权 |
| D04 | 依赖 | High | `src-tauri/Cargo.lock:2689`、`src-tauri/Cargo.lock:2543` | 已修复：plist 1.9.0 → 1.10.1，quick-xml 0.39.4 → 0.42.0 |
| D05 | 依赖 | Medium（工程评估） | `src-tauri/Cargo.lock:45` | 已修复：anyhow 1.0.102 → 1.0.103 |
| D06 | 依赖 | Medium（工程评估） | `src-tauri/Cargo.lock:934` | 已修复：event-listener 5.4.1 → 5.4.2 |
| C01 | 代码 | High | `src-tauri/src/commands.rs:75`、`src-tauri/src/commands.rs:103`、`src-tauri/src/commands.rs:151`、`src-tauri/src/commands.rs:189` | 已修复：IPC 路径穿越 |
| C02 | 代码 | High | `src-tauri/src/utils.rs:164`、`src-tauri/src/commands.rs:222` | 已修复：静态符号链接/junction 越界；并发替换见剩余风险 |
| C03 | 代码 | Medium | `src-tauri/src/scanner.rs:319`、`src-tauri/src/scanner.rs:423`、`src-tauri/src/scanner.rs:479`、`src-tauri/src/scanner.rs:518`、`src-tauri/src/utils.rs:46` | 已修复：本地元数据指定路径越界读取 |
| C04 | 代码 | Medium | `src-tauri/src/utils.rs:93` | 已修复：临时数据库可预测目录与失败时残留 |
| C05 | 代码 | Medium | `src-tauri/tauri.conf.json:24` | 已修复：开启 CSP |
| C06 | 代码 | Low | `src-tauri/capabilities/default.json:6` | 已修复：移除未用 opener 权限，限制 dialog 为 open |
| C07 | 代码 | Medium | `.github/workflows/release.yml:71` | 已修复：标签名进入 PowerShell 源码 |
| C08 | 代码 | Medium | `.github/workflows/release.yml:36`、`.github/workflows/codeql.yml:31` | 已修复：14 处 Action 引用固定完整 SHA |
| C09 | 代码 | Low | `src-tauri/src/commands.rs:618`、`src-tauri/src/config_store.rs:172`、`src-tauri/src/utils.rs:52` | 已修复：JSON 原位截断写入 |
| R01 | 依赖 | Medium | `src-tauri/Cargo.lock:1322` | 建议关注：glib；开发者选择保留 |
| R02 | 依赖 | Low | `src-tauri/Cargo.lock:2656` | 建议关注：proc-macro-error 停止维护；开发者选择保留 |
| R03 | 依赖 | Low | `src-tauri/Cargo.lock:4105` 等 | 建议关注：5 个 unic-* 停止维护；开发者选择保留 |
| R04 | 代码 | High（需已攻陷 renderer） | `src-tauri/src/config_store.rs:50`、`src-tauri/src/commands.rs:255`、`src-tauri/src/commands.rs:103` | 建议关注：敏感 IPC 缺少独立原生确认；开发者选择后续加固 |
| R05 | 代码 | Low | `src-tauri/src/utils.rs:25`、`src-tauri/src/utils.rs:46` | 建议关注：大文件资源消耗；限额需产品取舍 |
| R06 | 代码 | Low | `src-tauri/src/commands.rs:618`、`src-tauri/src/config_store.rs:172` | 建议关注：并发修改和浏览器同时写入的竞态 |

没有确认的 Critical 问题；没有发现真实密钥泄露。表中位置是修复后文件行号。

## 3. 直接依赖完整清单

“未命中”仅表示截至本次查询，没有在数据库中匹配到该实际版本的有效公告，不是无漏洞保证。全部直接依赖名称均纳入 WebSearch；精确版本同时用 OSV API 逐项核对，npm 另用 pnpm audit 交叉核对。

### npm

| 依赖 | 类型 | 实际锁定版本 | 本次核对 |
|---|---|---|---|
| @tauri-apps/api | 运行 | 2.11.0 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |
| @tauri-apps/plugin-dialog | 运行 | 2.7.0 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |
| @tauri-apps/plugin-opener | 运行 | 2.5.3 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |
| vue | 运行 | 3.5.32 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |
| @tauri-apps/cli | 开发 | 2.11.1 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |
| @vitejs/plugin-vue | 开发 | 5.2.4 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |
| typescript | 开发 | 5.6.3 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |
| vite | 开发 | 6.4.3 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |
| vue-tsc | 开发 | 2.2.12 | 精确版本 OSV / pnpm audit 未命中；WebSearch 核对 |

### Cargo

| 依赖 | 类型 | 实际锁定版本 | 本次核对 |
|---|---|---|---|
| tauri | 运行 | 2.11.6 | 已修复安全版本，见 D02 |
| tauri-plugin-opener | 运行 | 2.5.4 | 精确版本 OSV 未命中；WebSearch 核对 |
| serde | 运行 | 1.0.228 | 精确版本 OSV 未命中；WebSearch 核对 |
| serde_json | 运行 | 1.0.149 | 精确版本 OSV 未命中；WebSearch 核对 |
| base64 | 运行 | 0.22.1 | 精确版本 OSV 未命中；WebSearch 核对 |
| tempfile | 运行 | 3.27.0 | 精确版本 OSV 未命中；WebSearch 核对 |
| tauri-plugin-dialog | 运行 | 2.7.0 | 精确版本 OSV 未命中；WebSearch 核对 |
| rusqlite | 运行 | 0.40.2 | 经授权升级，见 D03 |
| tauri-build | 构建 | 2.6.3 | 精确版本 OSV 未命中；WebSearch 核对 |

`tempfile` 是本次新增的直接依赖，但该版本原本已经出现在 Cargo.lock 的间接依赖中。引入它用于安全创建和清理临时目录、原子文件替换。

### 关键间接依赖

| 依赖 | 原实际版本 | 最终实际版本 | 用途与结论 |
|---|---|---|---|
| brace-expansion | 2.1.4 | 2.1.7 | vue-tsc → @vue/language-core → minimatch；修复 3 项公告 |
| minimatch | 9.0.9 | 9.0.9 | 构建用 glob；最终未命中 |
| @vue/language-core | 2.2.12 | 2.2.12 | 类型检查；最终未命中 |
| @vue/compiler-sfc | 3.5.32 | 3.5.32 | 编译可信 Vue 模板；最终未命中 |
| rollup | 4.63.1 | 4.63.1 | Vite 构建；最终未命中 |
| esbuild | 0.25.12 | 0.25.12 | Vite 转译；最终未命中 |
| postcss | 8.5.28 | 8.5.28 | 已有 override；最终未命中 |
| nanoid | 3.3.18 | 3.3.18 | 已有 override；最终未命中 |
| libsqlite3-sys | 0.30.1 | 0.38.2 | 实际内置 SQLite 3.46.0 → 3.53.2 |
| tauri-build / codegen / macros | 2.6.1 | 2.6.3 | 与安全修复版 Tauri 匹配 |
| tauri-utils | 2.9.1 | 2.9.3 | 与 Tauri 2.11.6 匹配 |
| tauri-runtime | 2.11.1 | 2.11.3 | 与 Tauri 2.11.6 匹配 |
| tauri-runtime-wry | 2.11.1 | 2.11.4 | 与 Tauri 2.11.6 匹配 |
| wry | 0.55.1 | 0.55.1 | WebView；最终未命中 |
| plist / quick-xml | 1.9.0 / 0.39.4 | 1.10.1 / 0.42.0 | 兼容升级移除 XML DoS 公告 |
| anyhow | 1.0.102 | 1.0.103 | 内存健全性补丁 |
| event-listener | 5.4.1 | 5.4.2 | 内存/线程健全性补丁 |
| glib | 0.18.5 | 0.18.5 | Linux 间接依赖；保留风险 |
| proc-macro-error | 1.0.4 | 1.0.4 | Linux 宏依赖；停止维护 |
| urlpattern | 0.3.0 | 0.3.0 | 引入下面的 unic-* 停止维护依赖 |
| unic-char-property / unic-char-range / unic-common / unic-ucd-ident / unic-ucd-version | 均 0.9.0 | 均 0.9.0 | 无兼容修复版；保留并记录 |

完整 610 条依赖记录（含实际版本、直接/间接标记和 OSV 公告 ID）在 [SECURITY_DEPENDENCIES.csv](SECURITY_DEPENDENCIES.csv)。这份清单包括全部锁文件依赖，不仅上表关键项。

## 4. 已修复依赖：依据、利用条件与实际影响

### D01：brace-expansion

公告：[GHSA-qhr7-859c-m2p7](https://github.com/advisories/GHSA-qhr7-859c-m2p7)（High，修复 2.1.6）、[GHSA-6j4f-fj2g-mc7p](https://github.com/advisories/GHSA-6j4f-fj2g-mc7p)（High，修复 2.1.5）、[GHSA-q2hr-2g5m-vwhr](https://github.com/advisories/GHSA-q2hr-2g5m-vwhr)（Medium，修复 2.1.7）。恶意 glob 的嵌套或重写可导致栈溢出/CPU 消耗。锁文件和 pnpm audit 均命中旧版。项目用于开发期类型检查，不处理桌面用户提供的 glob，未发现生产运行时入口。将既有 override 的范围和最低版本推进到 2.1.7，并同步锁文件，不升级 Vite 或 vue-tsc 大版本。

### D02：Tauri IPC channel

官方 [GHSA-w28w-mhc8-qvjv](https://github.com/tauri-apps/tauri/security/advisories/GHSA-w28w-mhc8-qvjv) 指出 tauri 2.0.0–2.11.5 的内部 channel fetch 缺少所属 WebView 校验，2.11.6 修复。攻击者需控制可调用 Tauri IPC 的 WebView，猜测缓存响应 ID，读取/消耗其他 WebView 响应。本项目只有一个配置的主窗口，没有主动创建不可信 WebView，未复现跨窗口窃取，但旧版在受影响范围内，且扫描会返回大量本地信息。更新 tauri 至 2.11.6，并把 manifest 最低要求提高至安全版本。

子组件最终锁定到 Tauri 2.11.6 清单声明的匹配版本。首次解析得到的更新子组件造成上游 API 不匹配，已经修正并成功构建；未来更新锁文件后仍应运行构建检查，不应随意升级单个 Tauri 子组件。

### D03：内置 SQLite

本机源码核实旧 `libsqlite3-sys 0.30.1` 内置 3.46.0，最终 `0.38.2` 内置 3.53.2。官方 [SQLite 安全说明](https://www.sqlite.org/cves.html) 列出后续版本修复，例如 CVE-2025-6965（3.50.2 修复，需要任意 SQL 注入能力）、CVE-2025-7709（3.50.3 修复，需要控制数据库及破坏 FTS5 索引）。本项目的 SQL 是固定 SELECT，没有直接拼接用户 SQL，也没有使用 FTS MATCH，因此这些具体漏洞是否能由当前读取路径触发为「待确认」，不把旧版本直接等同于可利用的 High。

开发者已授权跨不兼容版本线升级：rusqlite 0.32.1 → 0.40.2（manifest 要求 ^0.40.1）。[官方发布记录](https://github.com/rusqlite/rusqlite/releases)确认该版本线使用新版 SQLite。数据库读取与临时副本回归通过。没有修改既有存储结构或增加兼容代码。

### D04：quick-xml

[RUSTSEC-2026-0194](https://rustsec.org/advisories/RUSTSEC-2026-0194.html)、[RUSTSEC-2026-0195](https://rustsec.org/advisories/RUSTSEC-2026-0195.html) 均为 High，修复版 ≥0.41.0；分别涉及恶意 XML 属性导致二次复杂度和 NsReader 的命名空间内存消耗。项目的引入链为 Tauri → plist → quick-xml，没有直接接受用户 XML。上游 [plist 修复 PR](https://github.com/ebarnard/rust-plist/pull/191)说明 plist 未使用受影响属性/NsReader 路径，当前可利用性受限。兼容更新 plist 到 1.10.1，实际 quick-xml 到 0.42.0，移除公告命中。

### D05 / D06：内存健全性

[RUSTSEC-2026-0190](https://rustsec.org/advisories/RUSTSEC-2026-0190.html) 是 anyhow::Error::downcast_mut 的借用规则问题，修复 ≥1.0.103；[RUSTSEC-2026-0221](https://rustsec.org/advisories/RUSTSEC-2026-0221.html) 是 event-listener 的 StackSlot 允许非 Send tag 跨线程，修复 ≥5.4.2。官方类型是 INFO Unsound，没有统一 CVSS；本报告工程评估为 Medium。第一方代码没有直接调用受影响 API，间接库的全部可达性未证明。执行兼容补丁更新以移除已知缺陷。

## 5. 已修复代码：具体改动与边界

### C01 / C02：IPC 文件操作路径越界（High）

旧版直接 `user_data_dir.join(profile_id)`，扩展删除又拼接 `extension_id`；绝对路径、`..` 或 Windows 分隔符可改变目标位置。攻击者需调用主 WebView IPC（例如先取得 renderer 执行权）；可让历史文件删除、书签改写、扩展目录递归删除操作作用于配置根外、但当前 OS 用户可访问的位置。

改动：后端拒绝空组件、`.`、`..`、路径分隔符、盘符/ADS 冒号、控制字符和 Windows 尾随点/空格；profile canonicalize 后必须是数据根的直接子目录。执行文件操作前检查其实际目标与既有子树，拒绝符号链接/junction 跳出 profile。扫描也使用相同 profile 校验。校验局限在当前操作涉及的文件，不检查无关资料文件。

回归：遍历输入被拒绝，正常 `Default` / `Profile 1` / Unicode 名称通过；Windows junction 指向外部目录被拒绝，外部 History 哨兵内容未变。该修复处理静态路径与链接，不声称防御拥有同用户文件写权限的攻击者在检查后并发替换路径或硬链接；见第 8 节。

### C03：元数据路径读取越界（Medium）

攻击者需要提供/篡改浏览器 Local State、扩展 manifest 或 Preferences。头像、图标、locale、store extension 路径原先未经约束，可能让本应用读取并返回目录外图像/JSON。本次增加相对组件与 canonical 根约束；JSON 固定文件读取也校验实际目标；登录数据库和 sidecar 复制前检查链接。

Store 路径按扩展 ID 的完整首组件匹配，避免 `id-other` 被当成 `id`。非 store 扩展仅接受绝对安装路径，保留用户安装的 unpacked 扩展；相对外部路径不再按应用 cwd 解释。图片和 locale 必须留在该扩展安装目录内。新增扩展路径回归验证合法 store、合法绝对 unpacked 路径和恶意相对路径。

### C04：敏感 SQLite 副本（Medium）

原先用 PID/时间戳建立目录且 `create_dir_all` 可复用预先存在路径；数据库复制失败时，清理对象还没创建，目录可能留下副本。Login Data 可能包含用户名、加密密码及登录站点，即使查询只选站点列，也不能当成非敏感临时文件。

改用 tempfile 随机独占目录和 RAII 所有权：复制及 sidecar 复制任何失败都会释放目录；正常查询结束自动清理。Unix 明确目录权限 0700，Windows 使用当前用户临时目录继承权限。测试验证副本可只读查询、释放后目录移除、原数据库保留。强制结束进程/断电仍可能留下副本，没有自动删除用户原有临时文件；后续清理策略需另行设计。

### C05 / C06：WebView 防御与最小权限

原来 CSP 为 null，缺少阻止注入脚本和外传请求的防线。启用同源脚本、同源/数据图片、仅同源与 Tauri IPC 连接，禁止 object/frame、base 和 form；style 保留 unsafe-inline 以适配已有 Vue 样式绑定，script 不允许 unsafe-inline 或 unsafe-eval。Tauri 构建会注入所需 IPC 支持和哈希。

源码没有使用 opener API；移除其 capability 授权，同时把 dialog:default 缩到实际使用的 dialog:allow-open。保留包和 Rust 插件，未删除功能。没有发现 v-html、innerHTML、eval 或远程脚本入口；浏览器标题、URL、名称用 Vue 文本插值显示，书签 URL 没有绑定为可执行导航链接。

### C07 / C08：CI（Medium）

发布脚本原来把 `${{ github.ref_name }}` 放到 PowerShell 双引号源码中；具有创建触发标签能力的攻击者可用特殊标签字符注入脚本，在构建 runner 上执行。改成 step 环境变量 RELEASE_TAG，运行时展开其值，不让值变成脚本源码。matrix 插值来自固定 workflow 常量。

两个 workflow 共 14 处 Action 引用（10 个唯一 action/ref）经 GitHub 官方 commits API 解析后固定完整 40 位 SHA，保留原标签注释便于维护。没有 pull_request_target、没有将 PR title/body/分支名写入 run 源码；发布写权限仅在 release job，build 的 contents 为 read。在线 Action 运行与 artifact attestation 仍需 CI 验证。

### C09：JSON 写入中断（Low）

浏览器 Preferences/书签和 browser-configs.json 原来 fs::write 原位截断，写入中断可能损坏整个文件。改为同目录临时文件写完整并 sync_all 后原子替换；现有权限保留，失败时旧文件保留。回归验证替换结果且不遗留临时文件。未更改 JSON 数据结构。此措施不等于事务，不解决浏览器或其他 IPC 同时写造成的最后写入覆盖。

## 6. 密钥与凭据

- 扫描了 325 个当前文件/产物，模式涵盖私钥、AWS AccessKey、GitHub token、常见服务 key、带凭据连接串、Webhook 和字面量密码/secret；仅输出位置，不输出潜在秘密值。
- 同样扫描本地全部 Git refs 的补丁历史，约 1,046,310 字节；没有凭据模式命中。未遍历远端尚未获取的历史，也不能发现不符合模式的所有自定义 token。
- `.gitignore` 已忽略 `.env`、`.env.*`、`*.local`，没有发现已跟踪的敏感 env/key/pem 文件。`src/vite-env.d.ts` 是类型定义，不是环境密钥。
- `passwordSites` 相关名称为业务字段，不是硬编码密码；Base64 常量解码后是浏览器文件名与固定 SQL 关键词，不是加密或密钥。
- 没有实际秘密需要迁移，因此未添加无用途的环境变量、`.env.example` 或轮换配置。

## 7. 已征询的开发者决定

1. **rusqlite 跨不兼容版本线升级**：方案为升级并验证，或保留旧 SQLite 并限定输入。推荐升级；开发者明确授权，已完成。
2. **glib / proc-macro-error**：glib 0.18.5 的 [RUSTSEC-2024-0429](https://rustsec.org/advisories/RUSTSEC-2024-0429.html) 与 GHSA-wrw7-89jp-8q8g 是同一问题，不能算两处独立漏洞。涉及 VariantStrIter 的迭代 API，修复 ≥0.20.0，当前 GTK/Tauri Linux 依赖约束不兼容。Windows/macOS 构建不启用这条 Linux 运行链，Linux 全链可达性「待确认」。proc-macro-error 1.0.4 的 [RUSTSEC-2024-0370](https://rustsec.org/advisories/RUSTSEC-2024-0370.html) 是停止维护信息。方案是当前保留，或维护 GTK 上游补丁/更换框架。开发者选择保留并后续单独处理。
3. **unic-* 停止维护**：urlpattern 0.3.0 引入 5 个 0.9.0 包；[unic-char-property](https://rustsec.org/advisories/RUSTSEC-2025-0081.html)、[unic-char-range](https://rustsec.org/advisories/RUSTSEC-2025-0075.html)、[unic-common](https://rustsec.org/advisories/RUSTSEC-2025-0080.html)、[unic-ucd-ident](https://rustsec.org/advisories/RUSTSEC-2025-0100.html)、[unic-ucd-version](https://rustsec.org/advisories/RUSTSEC-2025-0098.html) 均无公告所列修复版本。这是维护风险，不是 5 个已证明可利用漏洞。兼容的 Tauri 2.11 子组件仍需这些包；方案是当前保留，或维护 urlpattern/Tauri 补丁。开发者选择保留并记录。
4. **敏感 IPC 原生确认**：自定义配置允许任意程序/数据根，是现有功能；前端确认不能限制已经取得 renderer 执行权的攻击者。可通过原生确认绑定具体程序/路径/删除对象、后端授权票据或只允许可信 executable 来加固，但会改变交互/功能。开发者选择本次记录，后续单独加固；未擅自删功能。

核实中还发现 GTK3 的旧“停止维护”公告 [RUSTSEC-2024-0415](https://rustsec.org/advisories/RUSTSEC-2024-0415.html) 已撤回（项目恢复维护），因此没有把整个 GTK3 依赖链错误地标成仍有效的停止维护漏洞。Vite 6.4 仍获得官方安全补丁回移，[官方支持说明](https://vite.dev/releases)支持保留当前 6.4.3；不需要为本次审核擅自升级 Vite 大版本。未发现名字仿冒/恶意包的证据，但未逐包验证发布者身份和供应链历史。

## 8. 剩余风险与建议

- **R04：renderer 信任边界**。没有发现当前可利用 XSS，但主 WebView 拥有敏感自定义 IPC。建议后续优先实现独立原生确认，避免将任意执行程序和数据删除权限完全交给 renderer。
- **路径竞态 / 硬链接**。canonical 校验与文件操作之间有时间窗口；同用户本地进程可替换文件或目录，硬链接也可能映射到其他位置。要承诺抵御这种本地对手，需要 handle-based、no-follow、文件 ID/权限检查等平台实现，属于后续设计范围。当前应用不应以管理员身份运行处理不可信资料。
- **R05：资源耗尽**。图像/JSON/SQLite 副本仍按完整文件读取或复制。能控制本地资料文件的攻击者可提供巨型文件，导致内存/磁盘消耗。建议先明确用户支持的最大书签、图像和数据库大小，再实现限额/流式读取；没有擅自增加可能截断正常数据的大小限制。
- **R06：并发写入**。原子替换防止截断，但多个删除 IPC 或浏览器同时写入仍可丢失更新。建议后续加入每个 profile 的串行队列/锁和浏览器关闭检测；前端 busy 标记不能当后端锁。批量操作也不是跨资料事务，后面的项失败时前面已完成的项不会回滚。
- **临时文件**。强制结束和断电可能阻止 RAII 清理，当前 Windows 默认 ACL 的实际部署效果「待确认」。建议如需崩溃后回收，设计 app 专属私有缓存及受约束的启动清理。
- **发布**。SHA 固定不等于审计 Action 的全部源码；Node 24、Rust stable、runner 镜像和 Homebrew 工具仍会变化。建议保持受控依赖更新、locked/frozen 构建与平台 CI；macOS 应验证签名/公证与安装体验。
- **已有功能范围**。没有 HTTP 后端、上传/解压、浏览器扩展 manifest、数据库写 SQL、关闭 TLS 证书校验、自造加密、Node integration 或不安全反序列化入口；SQL 是固定只读 SELECT，Shell 进程启动使用 Command 参数列表。用户名/路径错误只显示在本地界面，没有遥测或日志上传。外部图片不通过网络下载。
- **覆盖限制**。OSV 未命中不能替代官方公告查阅。例如 Tauri 最近公告通过 WebSearch 发现，实际升级即使数据库暂未同步也执行；esbuild 的 GHSA-g7r4-m6w7-qqqr 只声明 0.27.3，当前 0.25.12 不在公告列出的范围，且项目未启动 esbuild 自带 serve，不错误套用该 CVE。

## 9. 验证与交付

- `pnpm audit --json`：最终 0 Critical / High / Medium / Low。
- OSV querybatch：610 个包/版本逐项成功查询，最终 7 个包命中；glib 的两个 ID 合并为一个问题，其余 6 个是开发者接受保留的停止维护信息。原始证据保存在本机审核产物目录；CSV 保留全部包/版本与 ID。
- 本机未安装 cargo-audit，没有声称运行成功，也未安装新审计工具；Rust 用 OSV API + RustSec/官方公告交叉核对。
- `pnpm build`：通过 Vue 类型检查及生产构建。
- `cargo test --manifest-path src-tauri/Cargo.toml --locked`：8 项通过（2 项已有测试、6 项安全回归），最终结果已核实。
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`：通过。
- `pnpm tauri build --no-bundle`：Windows 正式应用构建通过；最终源文件更改后又执行 `cargo build --manifest-path src-tauri/Cargo.toml --release --locked`，通过；生成 src-tauri/target/release/ChromTool.exe。
- 两个 workflow YAML 解析通过，14 处 uses 都是完整 SHA，没有把 github.ref_name 写入 run 源码。
- `git diff --check`：通过。没有创建 Git commit，没有启动前端服务器，未在软件 UI 加入审核说明。

建议 commit message：

```text
fix(security): harden IPC paths and update vulnerable dependencies
```

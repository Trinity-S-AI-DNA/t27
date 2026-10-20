# AI code review and PR-agent GitHub apps: competitive analysis for the Queen

**As of:** 2026-10-08. **Scope:** CodeRabbit in depth, about 30 competitors, a feature matrix, growth patterns, and what our app (the Queen swarm as a GitHub App) should build, plus the risks.

**Method and conventions**
- Web research only. Nothing was posted and no accounts were created. Primary sources (pricing pages, docs, changelogs, GitHub Marketplace listings, company blogs, funding posts) are preferred. Press and third-party sources are used where no primary exists, and are labelled.
- Dates in parentheses are the publish or "last updated" date of the source. "fetched 2026-10-08" means a live page read today with no visible effective date.
- **unverified** = found only in a secondary source, a search snippet, or not confirmed at all.
- "vendor" = a figure the company reports about itself. Benchmarks run by a vendor that also competes are flagged.
- † = from background knowledge, not re-verified in this pass.
- The shared web-search budget ran out near the end of the research. Late items were checked by fetching primary pages directly, and the gaps are listed in section 7.

---

## 0. Executive summary

**The market in one paragraph.** AI code review became a standard part of GitHub in 2025-2026, and the leaders are now moving from *reviewing* code to *doing the work*. CodeRabbit (valued at $1.5B) now sells "Agentic Change Management": triage, auto-merging low-risk PRs, auto-fixing CI, and opening PRs from Slack. Copilot code review handles more than 1 in 5 reviews on GitHub. Cursor, Cognition (Devin), Linear, GitLab and Atlassian all bundle a reviewer with an agent that writes code. Two gaps stand out:
1. **No one combines reviewer + implementer + CI runner + release runner + merge gates as one free-for-OSS app.**
2. **No one verifies findings by execution as the default contract.** Their filters are LLM verification passes; nothing runs a compiler or tests before a comment is posted.

These are the two places we can win.

**Key numbers**

| Metric | Value | Source |
|---|---|---|
| CodeRabbit valuation / last round | $1.5B post-money, $143M Series C (2026-08-12); about $231M raised in total | [CodeRabbit newsroom](https://www.coderabbit.ai/newsroom/coderabbit-series-c-agentic-change-management), [TNW](https://thenextweb.com/news/coderabbit-raised-143m-to-read-the-code-ai-wrote) |
| CodeRabbit scale | >17,000 customers, >2M reviews/week, revenue >5x YoY (base undisclosed) | [TNW 2026-08-13](https://thenextweb.com/news/coderabbit-raised-143m-to-read-the-code-ai-wrote) |
| CodeRabbit ARR | >$15M at Sep 2025 (founder), growing 20%/month; third-party estimates of about $50M by Jul 2026 (**estimate**) | [TechCrunch](https://techcrunch.com/2025/09/16/coderabbit-raises-60m-valuing-the-2-year-old-ai-code-review-startup-at-550m/), [Sacra](https://sacra.com/c/coderabbit/) |
| CodeRabbit GitHub Marketplace installs | **339,485** (fetched 2026-10-08), up from ">80,000" in Aug 2025 | [Marketplace](https://github.com/marketplace/coderabbitai), [Kudelski](https://kudelskisecurity.com/research/how-we-exploited-coderabbit-from-a-simple-pr-to-rce-and-write-access-on-1m-repositories) |
| CodeRabbit OSS spend | $10M over 12 months of free capacity for OSS, "covering 150,000 projects" | [TNW](https://thenextweb.com/news/coderabbit-raised-143m-to-read-the-code-ai-wrote) |
| Copilot code review | 60M+ reviews; ">1 in 5 code reviews on GitHub"; 12,000+ orgs review every PR automatically (2026-03-05) | [GitHub blog](https://github.blog/ai-and-ml/github-copilot/60-million-copilot-code-reviews-and-counting/) |
| Bot review volume, 30 days (Feb 9 - Mar 10, 2026) | CodeRabbit 179,965; Copilot 91,596; Codex Connector 47,694; Gemini 43,520; Cursor 27,250; Devin 19,953; Claude 12,002. AI assistants are >60% of all bot PR reviews | [Star History, 2026-03-12](https://www.star-history.com/blog/state-of-coding-ai-on-github) |
| Bot-authored PRs, same 30 days | Dependabot 403K, Renovate 120K, Copilot 88,943 | same |
| Merged PRs on GitHub | 518.7M in 2025 (43.2M/month, +23% YoY) | [Octoverse 2025](https://github.blog/news-insights/octoverse/octoverse-a-new-developer-joins-github-every-second-as-ai-leads-typescript-to-1/) |

**The 2026 shake-out**
- **Retreats:**
  - Google shut down the free consumer Gemini Code Assist reviewer on 2026-07-17 ([Google](https://developers.google.com/gemini-code-assist/docs/deprecations/consumer-code-review)).
  - AWS is sunsetting Amazon Q Developer in favour of Kiro ([AWS](https://aws.amazon.com/blogs/devops/amazon-q-developer-end-of-support-announcement/)).
  - Korbit stopped operating, and its assets went to Boost Security ([SiliconANGLE](https://siliconangle.com/2026/05/06/boost-security-acquires-secureiqx-korbit-adds-4m-new-funding/)).
  - Sweep left GitHub for JetBrains.
  - Ellipsis pivoted to a generic "Agent Cloud".
- **Consolidation:**
  - Cursor agreed to acquire Graphite (2025-12-19) ([Cursor](https://cursor.com/blog/graphite)).
  - SpaceX then bought Cursor for $60B (closed 2026-08-14) ([Techzine](https://www.techzine.eu/news/devops/143619/spacex-completes-acquisition-of-cursor/)).
  - Cognition raised more than $2B at $48B with an almost $900M run-rate ([Cognition](https://cognition.com/blog/series-e)).
  - ClickUp bought Codegen; Tricentis bought Tabnine.
- **Pricing has moved from seats to usage:**
  - Copilot moved to token-metered "AI Credits" on 2026-06-01, and code review on private repos now also consumes Actions minutes ([GitHub changelog](https://github.blog/changelog/2026-04-27-github-copilot-code-review-will-start-consuming-github-actions-minutes-on-june-1-2026/)).
  - Cursor Bugbot went from $40/seat to about $1.00-1.50 per run ([Cursor](https://cursor.com/blog/may-2026-bugbot-changes)).
  - Claude Code Review costs about $15-25 per review ([Anthropic](https://claude.com/blog/code-review)).
  - GitLab charges a flat $0.25 per review ([GitLab](https://about.gitlab.com/blog/agentic-code-reviews-with-flat-rate-pricing)).
- **Compute is the constraint:** GitHub paused new Copilot Pro/Pro+/Student sign-ups on 2026-04-20 because "agents are doing more work", and reopened them in phases from 2026-06-17 ([InfoWorld](https://www.infoworld.com/article/4161278/github-s-new-copilot-sign-ups-as-agentic-ai-strains-infrastructure.html), [GitHub changelog](https://github.blog/changelog/2026-06-17-copilot-individual-plan-sign-ups-are-reopening/)).

---

## 1. CodeRabbit in depth

### 1.1 Company, funding, traction

| Fact | Value | Source (date) |
|---|---|---|
| Founded | Early 2023, San Francisco, by Harjot Gill. He previously sold Netsil to Nutanix (2018) and co-founded FluxNinja. | [TechCrunch 2025-09-16](https://techcrunch.com/2025/09/16/coderabbit-raises-60m-valuing-the-2-year-old-ai-code-review-startup-at-550m/) |
| Series A | $16M led by CRV (Aug 2024); Datadog CEO Olivier Pomel as an angel; sales and marketing team of 10 at the time | [SiliconANGLE 2024-08-14](https://siliconangle.com/2024/08/14/ai-code-review-startup-coderabbit-raises-16m-help-developers-debug-code-faster/) |
| Series B | $60M led by Scale Venture Partners (+NVentures, CRV) at $550M valuation; $88M total | [TechCrunch 2025-09-16](https://techcrunch.com/2025/09/16/coderabbit-raises-60m-valuing-the-2-year-old-ai-code-review-startup-at-550m/) |
| ARR at Series B | ">$15M ARR", "growing 20% a month" (founder); 8,000+ paying businesses | same |
| Series C | $143M at $1.5B post-money, co-led by Atomico and Smash Capital (+BMW i Ventures, Datadog, ...) | [CodeRabbit 2026-08-12](https://www.coderabbit.ai/newsroom/coderabbit-series-c-agentic-change-management), [Axios Pro](https://www.axios.com/pro/enterprise-software-deals/2026/08/12/code-review-coderabbit-datadog-raise) |
| Scale at Series C | >2M reviews/week, >17,000 customers, revenue >5x YoY, about 50 FTE in UK/EU | [TNW 2026-08-13](https://thenextweb.com/news/coderabbit-raised-143m-to-read-the-code-ai-wrote) |
| ARR estimates (third party) | Sacra: about $25M at end-2025 and about $50M in Jul 2026. arr.club: $11M to $40M in six months (dated 2026-06-25). **Estimates.** | [Sacra](https://sacra.com/c/coderabbit/), [arr.club](https://www.arr.club/coderabbit/coderabbit-arr-hit-40m-from-11m-in-six-months) |
| Homepage claims | "Trusted by 17K customers", "6M Repositories", "Most installed AI App on GitHub and GitLab" | [coderabbit.ai](https://www.coderabbit.ai/) (fetched 2026-10-08) |
| Marketplace | **339,485 installs**, verified publisher. Plans: Open Source $0 ("Open Source Advanced plan is free for Open Source projects"), Pro $30/seat, Pro Plus $60/seat, 14-day trial | [Marketplace](https://github.com/marketplace/coderabbitai) (fetched 2026-10-08) |
| Installs, Aug 2025 | ">80,000"; at the time CodeRabbit claimed it reviewed "1M repositories" | [Kudelski 2025-08-19](https://kudelskisecurity.com/research/how-we-exploited-coderabbit-from-a-simple-pr-to-rce-and-write-access-on-1m-repositories) |
| OSS footprint | 100,000+ OSS projects (Sep 2025, per Sacra citing the company). $1M OSS sponsorship pledge and >$600K paid to maintainers in 2025. $10M of free OSS capacity over 12 months, "150,000 projects" | [Sacra](https://sacra.com/c/coderabbit/), [CodeRabbit blog 2025-09-18](https://coderabbit.ai/blog/coderabbit-commits-1-million-to-open-source), [TNW](https://thenextweb.com/news/coderabbit-raised-143m-to-read-the-code-ai-wrote) |
| Named customers | NVIDIA, BMW, Adyen, Indeed, JFrog, Trivago, Swiggy, Clerk, Writer, Groupon, Bun, Life360 | [coderabbit.ai](https://www.coderabbit.ai/) |
| Org reach (public GitHub, 2025) | CodeRabbit appeared in 7,478 orgs vs Copilot's 29,316. CodeRabbit touched 632,256 PRs vs Copilot's 561,382. Copilot passed CodeRabbit in monthly PRs in Nov 2025 (109,272 vs 69,757). **Vendor-run analysis.** | [PullFlow on DEV](https://dev.to/pullflow/coderabbit-vs-github-copilot-vs-gemini-which-ai-code-review-agent-should-your-team-use-3m67), [PullFlow report](https://pullflow.com/state-of-ai-code-review-2025) |

The two usage measures disagree on Copilot vs CodeRabbit. PullFlow counts distinct PRs touched and shows Copilot ahead monthly since Nov 2025. Star History counts review events and shows CodeRabbit at about 2x Copilot in Feb-Mar 2026. "Who leads" therefore depends on the metric.

### 1.2 Pricing (fetched 2026-10-08)

Sources: [pricing page](https://www.coderabbit.ai/pricing) and [docs: plans](https://docs.coderabbit.ai/management/plans.md).

| Plan | Price | What you get | PR reviews/hour/dev |
|---|---|---|---|
| Free | $0 | Unlimited public and private repos, but **PR summaries only**. Real reviews only in the IDE extension and CLI (3/hour) | 1 (summary) |
| Open Source | $0 | Unlimited public repos, Team-tier features. Repos with **<10 stars must trigger reviews manually** | **1-10, scaled by star count** |
| Essentials (formerly "Pro") | $24/dev/mo annual, $30 monthly | PR reviews, 1-click fixes, learnings, pre-merge checks, chat, linters/SAST, Jira/Linear | 5 |
| Team (formerly "Pro+") | $48 annual, $60 monthly | + Triage, CodeRabbit Plan, unit-test generation, merge-conflict resolution, finishing touches | 8 |
| Advanced | $72 annual, $90 monthly | + CodeRabbit Security (continuous PR security review) | 10 |
| Enterprise | Custom | Self-hosting, SSO, RBAC, audit logs, API, multi-org, SLA, EU SaaS, AWS/GCP/Claude marketplace billing | 12 |

- **Trial:** 14 days, no credit card. New orgs get an Advanced trial and then revert to Free.
- **Billing unit:** only developers who open PRs are billed, and seats can be reassigned.
- **Usage-based add-ons (2026):**
  - Over-limit reviews: **$0.25 per reviewed file**, with a monthly cap (launched 2026-04-08).
  - **CodeRabbit Agent: $0.40 per agent-minute.** Two third-party listings say $0.50; which is current is **unverified**.
  - Security Deep Scan: usage-priced.
  - Sources: [pricing](https://www.coderabbit.ai/pricing), [changelog](https://docs.coderabbit.ai/changelog.md).
- **Adaptive throttling:** each developer's hourly bucket shrinks as their weekly volume grows, down to "1 review/hour, one at a time" (for example, Essentials at 60+ reviews in 7 days). A rate-limited push gets a comment and a *passing* "Review rate limited" check ([docs: rate limits](https://docs.coderabbit.ai/management/rate-limits.md)).
- **Price history:**
  - Pro was $15 in 2024 ([It's FOSS](https://itsfoss.com/news/coderabbit-free/)).
  - Pro+ added 2026-04-16.
  - Lite ($12) and Pro Legacy retired 2026-06-15 ([changelog](https://docs.coderabbit.ai/changelog.md)).
  - Direction: more tiers, higher prices, more metering.

### 1.3 Product surface (Oct 2026)

**PR reviews** run on GitHub, GitLab, Azure DevOps and Bitbucket. Each review produces:
- A walkthrough comment.
- A **"Summary by CodeRabbit"** block written into the PR description (on by default).
- Sequence diagrams and an effort estimate.
- Line comments with committable suggestions.
- `@coderabbitai` chat.
- "Learnings" from chat, stored as embeddings ([docs: summaries](https://docs.coderabbit.ai/pr-reviews/summaries.md)).

**Configuration:** `.coderabbit.yaml`. Since 2026-09-16 there is also a TypeScript config with PR-aware conditions and remote includes ([changelog](https://docs.coderabbit.ai/changelog.md)).

**IDE extension** (VS Code, Cursor, Windsurf), launched 2025-05-14:
- Free with rate limits; reviews uncommitted changes.
- "Fix with AI" hands the fix to a coding agent.
- Sources: [CodeRabbit](https://coderabbit.ai/blog/ai-code-reviews-vscode-cursor-windsurf), [The New Stack](https://thenewstack.io/coderabbits-ai-code-reviews-now-live-free-in-vs-code-cursor/).

**CLI**, launched 2025-09-16:
- Free at 3 reviews/hour.
- `--agent` JSON mode is built to be called by Claude Code, Codex, Cursor and Gemini CLI. There is a Claude Code plugin, `/coderabbit:review`.
- v0.8 (Sep 2026) added `cr review --deep` and `cr handoff` to cloud coding-agent tasks.
- Sources: [CLI blog](https://coderabbit.ai/blog/coderabbit-cli-free-ai-code-reviews-in-your-cli), [changelog](https://docs.coderabbit.ai/changelog.md).

**CodeRabbit Agent** (Slack 2026-04-17, Discord 2026-06-29):
- Investigates an issue and writes a "Coding Plan".
- Hands the plan to Claude Code, Codex, Cursor or Devin, or opens the PR itself.
- The Discord agent is free with caps for OSS communities.
- Sources: [docs: agent](https://docs.coderabbit.ai/agent.md), [IT Brief](https://itbrief.com.au/story/coderabbit-launches-slack-agent-for-engineering-teams).

**"Agentic Change Management" (Aug-Oct 2026):**
- **Triage:** a cross-repo PR queue ranked by value, urgency and risk.
- **Triage rules** (2026-09-28): **auto-merge low-risk PRs, close stale PRs, auto-fix CI and review feedback, send Slack reminders.**
- **Change Stack:** a layered review UI for very large agent-written diffs.
- **Security merge blocking** (2026-10-05).
- Sources: [newsroom](https://www.coderabbit.ai/newsroom/coderabbit-series-c-agentic-change-management), [changelog](https://docs.coderabbit.ai/changelog.md).

**"Finishing touches":** docstrings, unit-test generation, merge-conflict resolution (beta 2026-03-17), and autofix on GitLab and Azure DevOps ([changelog](https://docs.coderabbit.ai/changelog.md)).

**What this means for us:** CodeRabbit is moving into our lane (auto-fix CI, auto-merge, PRs from chat). Its own docs show three things it still does **not** do:
1. It does not run the customer's build or tests as a CI provider. It reacts to CI results.
2. It does not run releases.
3. It does not own an implementer. Code-writing is handed to third-party agents.

### 1.4 Install flow and permissions

Source: [docs: GitHub.com](https://docs.coderabbit.ai/platforms/github-com.md).

**Steps:**
1. Log in with GitHub OAuth. This grants read-only access to orgs, teams and email.
2. Choose the org or account, then "All repositories" or "Only select repositories". Click "Install & Authorize".
3. Optionally trigger a review on an existing PR right away. Value shows up within minutes.

**Repository permissions requested:**
- **Read:** Actions, discussions, members, metadata, merge queues.
- **Read & write:** checks, **code (contents)**, commit statuses, issues, pull requests, **workflows**.

**Install rules:** installing into an org needs an org owner; other members see a "Request" button. New features can require an admin to "Accept new permissions" ([docs](https://docs.coderabbit.ai/management/upgrading-permissions)).

**Takeaway:** for a personal account the install is about two clicks. The cost is a broad write scope (contents and workflows), which security reviewers flag.

### 1.5 Data handling

- **LLM calls:** "ephemeral", with zero retention at the LLM providers and no training on customer code. The review sandbox is disposed of after each review ([Trust Center](https://coderabbit.ai/coderabbit-trust-center), [docs](https://docs.coderabbit.ai)).
- **What is kept:**
  - Embeddings of chat "learnings" and of linked issues (Jira, Linear, GitHub).
  - An optional code and metadata cache for up to 7 days after the last review. This is documented in an *old* config reference, so it needs re-checking.
  - Opting out deletes the cache and the knowledge base ([old config ref](https://docs.coderabbit.ai/reference-old/configuration)).
- **Compliance:** SOC 2 Type II and GDPR ([Trust Center: SOC](https://www.coderabbit.ai/trust-center/soc)). The docs also say "HIPAA", but no attestation was found (**unverified**).
- **Enterprise:** self-hosted and EU SaaS options.

### 1.6 Growth mechanics that drove adoption (evidence)

1. **Two-click install, free on public repos, no card.** Sacra describes bottom-up adoption through a "two-click install" from the GitHub and GitLab app stores, and calls the free OSS tier a major acquisition channel ([Sacra](https://sacra.com/c/coderabbit/)). Installs grew from >80K (Aug 2025) to 339K (Oct 2026), about 4x in 14 months.
2. **The PR is the viral surface.**
   - Every review posts a branded walkthrough and writes "Summary by CodeRabbit" into the PR description by default. Every contributor and reviewer of a public PR sees `coderabbitai[bot]`.
   - Other tools now parse that header. The AbsaOSS release-notes action can use it ([PR](https://github.com/AbsaOSS/generate-release-notes/pull/193/files)), so the artifact became a de-facto standard.
   - Bot footers linked to Discord and X, and in 2024 the bot asked users to share it on social media ([moonbitlang/core#886](https://github.com/moonbitlang/core/pull/886)).
3. **Marketplace ranking:** a verified publisher whose "Most installed AI app" claim appears in all its marketing ([Marketplace](https://github.com/marketplace/coderabbitai)).
4. **Free individual surfaces feed team plans.** The IDE extension (May 2025) and CLI (Sep 2025) are free. The CLI deliberately rides other agents' adoption by acting as "the reviewer in the agent loop" ([The New Stack](https://thenewstack.io/coderabbits-ai-code-reviews-now-live-free-in-vs-code-cursor/)).
5. **Rode the vibe-coding wave.** CodeRabbit positioned itself as the quality gate for AI-written code ("AI-driven code now outpaces human capacity", [homepage](https://www.coderabbit.ai/)).
6. **Low friction for buyers.** Seats are only for PR authors and can be reassigned ([pricing](https://www.coderabbit.ai/pricing)).
7. **Spending on OSS goodwill:** $600K in sponsorships in 2025, a $1M pledge, and $10M of free capacity.
8. **Enterprise upsell:** SSO, RBAC, audit logs, self-hosting, EU SaaS, and cloud-marketplace billing are reserved for Enterprise.

### 1.7 Known weaknesses and complaints

- **Noise, nitpicks and verbosity** are the most common complaint.
  - The default output is a walkthrough, summary and diagram, followed by many inline comments that mix real findings with style nits.
  - On large PRs it marks trivial issues as major and comments out of scope. Re-reviews raise new issues the first pass missed.
  - Sources: [CuratorBits](https://curatorbits.com/reviews/coderabbit/), [G2](https://www.g2.com/products/coderabbit/reviews), [aicodereview.io](https://aicodereview.io/blog/coderabbit-alternatives/).
  - A Lychee-project audit of 28 PRs was reported second-hand as 35% genuine, 21% nitpicks and 15% useless (**unverified** primary).
- **Rate limits** hurt heavy users and agent loops: adaptive throttling down to 1 per hour, and OSS repos under 10 stars need manual triggers.
- **Cost creep:** from $15 per dev in 2024 to $24-90 per dev in 2026, plus metered files, agent minutes and scans.
- **Security incident**, the single most relevant lesson for an app that runs customers' code:
  - Kudelski Security showed a PR containing `.rubocop.yml` could execute code on CodeRabbit's production servers, because RuboCop ran outside the sandbox.
  - The exposed environment variables included the Anthropic and OpenAI keys, Postgres credentials, and **the GitHub App private key**. That key gave potential write access to every installation.
  - Reported 2025-01-24. Per CodeRabbit, RuboCop was disabled within 1 hour, credentials rotated within 3 hours, and the sandbox fix shipped within 12 hours. Published 2025-08-19.
  - Sources: [Kudelski](https://kudelskisecurity.com/research/how-we-exploited-coderabbit-from-a-simple-pr-to-rce-and-write-access-on-1m-repositories), [CodeRabbit response](https://coderabbit.ai/blog/our-response-to-the-january-2025-kudelski-security-vulnerability-disclosure-action-and-continuous-improvement), [Endor Labs](https://www.endorlabs.com/learn/when-coderabbit-became-pwnedrabbit-a-cautionary-tale-for-every-github-app-vendor-and-their-customers).
- **Privacy:** broad write scopes, learnings stored as embeddings, and a HIPAA claim without a visible attestation.
- **Competitive squeeze:** Copilot is bundled into existing seats and reached more orgs (see 1.1). Benchmark leadership changes between snapshots (see 2.5).

---

## 2. Competitors

### 2.1 Platform owners

#### GitHub Copilot code review (CCR) and Copilot cloud agent (formerly "coding agent")

**Pricing** ([docs: plans](https://docs.github.com/en/copilot/get-started/plans), [features page](https://github.com/features/copilot/plans), fetched 2026-10-08):

| Plan | Price | Monthly AI Credits |
|---|---|---|
| Free | $0 | No CCR (only "review selection" in VS Code) |
| Pro | $10 | 1,500 |
| Pro+ | $39 | 7,000 |
| Max | $100 | 20,000 |
| Business | $19/seat | 1,900/user |
| Enterprise | $39/seat | 3,900/user |

- 1 credit = $0.01, consumed per token at each model's API rate.
- **Usage-based since 2026-06-01** ([GitHub blog](https://github.blog/news-insights/company-news/GitHub-copilot-is-moving-to-usage-based-billing/)).
- **Cost per review:** the docs estimate about $0.05-$1 at "Lite" effort and $0.25-$5 at "Balanced". **On private repos, CCR also consumes Actions minutes**; public repos stay free ([changelog 2026-04-27](https://github.blog/changelog/2026-04-27-github-copilot-code-review-will-start-consuming-github-actions-minutes-on-june-1-2026/), [CCR docs](https://docs.github.com/en/copilot/concepts/agents/code-review)).
- **OSS:** free Copilot Pro for maintainers of "popular" repos. The criteria are not published and are re-checked monthly ([docs](https://docs.github.com/en/copilot/how-tos/manage-your-account/get-free-access-to-copilot-pro)).

**Distribution:** built into GitHub, with no app to install. CCR is available on github.com, CLI, Mobile, VS Code, Visual Studio, Xcode, JetBrains, and Azure DevOps (preview). Copilot CLI went GA on 2026-02-25.

**Features:**
- **Releases:** CCR GA 2025-04-04 ([changelog](https://github.blog/changelog/2025-04-04-copilot-code-review-now-generally-available/)). Agentic architecture with tool calls and repo context, running on Actions (2026-03-05) ([changelog](https://github.blog/changelog/2026-03-05-copilot-code-review-now-runs-on-an-agentic-architecture/)). Lite/Balanced effort levels (2026-09-23).
- **Automation:** automatic reviews set by ruleset ([changelog](https://github.blog/changelog/2025-09-10-copilot-code-review-independent-repository-rule-for-automatic-reviews/)).
- **Review output:** severity tags; "Copilot approvals" (preview, off by default).
- **Rules:** `copilot-instructions.md`, path instructions, `AGENTS.md`, skills, and MCP.
- **Fix handoff:** `@copilot` hands a fix to the cloud agent as a stacked PR.
- **Static analysis:** CodeQL and linters moved into the paid **Code Quality** add-on ($10 per active committer, GA 2026-07-20) ([changelog](https://github.blog/changelog/2026-07-20-github-code-quality-is-now-generally-available)).
- **Cloud agent:** GA 2025-09-25 ([changelog](https://github.blog/changelog/2025-09-25-copilot-coding-agent-is-now-generally-available/)). Assign an issue and it opens a draft PR, running on Actions. **Agent HQ** lets Pro+ and Enterprise users delegate to Claude or Codex inside GitHub ([GitHub blog](https://github.blog/news-insights/company-news/welcome-home-agents/), [The New Stack](https://thenewstack.io/github-embraces-the-coding-agent-competition-with-agent-hq/)).
- **Platform features that compete with apps:** a native merge queue (GA 2023) and native stacked PRs (public preview 2026-07-30; [changelog](https://github.blog/changelog/2026-07-30-stacked-pull-requests-are-now-in-public-preview/)).

**Numbers** ([GitHub blog 2026-03-05](https://github.blog/ai-and-ml/github-copilot/60-million-copilot-code-reviews-and-counting/)): 60M+ reviews, 10x growth in about a year, ">1 in 5 code reviews on GitHub", 71% of reviews give actionable feedback, 5.1 comments per review on average.

**Weaknesses:**
- **Noise:**
  - A maintainer found about 1 in 5 comments useful ([dev.to](https://dev.to/techgirl1908/how-i-taught-github-copilot-code-review-to-think-like-a-maintainer-3l2c)).
  - An Open edX pilot rated it "not very useful" ([discuss.openedx.org](https://discuss.openedx.org/t/github-copilot-reviews/16751)).
  - Every push triggers a re-review, creating a fix-push-review loop ([community #189767](https://github.com/orgs/community/discussions/189767)).
- **Billing shock** after the June 2026 switch, including an HN thread with 312 points on CCR consuming Actions minutes (**second-hand**).
- **Product-placement backlash:** Copilot inserted Raycast "tips" into about 11,000 PRs, and GitHub disabled the tips ([heise](https://www.heise.de/en/news/Advertisement-or-tip-GitHub-Copilot-added-product-tips-to-pull-requests-11241254.html)).

#### OpenAI Codex (code review and cloud tasks)

- **Pricing** ([pricing docs](https://learn.chatgpt.com/docs/pricing), fetched 2026-10-08): included in ChatGPT Free, Go $8, Plus $20, Pro $100+, Business $20-25/user, and Enterprise. Limits are per 5 hours, with credits for overage. No numeric cap for reviews was found (**unverified**).
- **Status:** per DevDay 2026-09-29 coverage, Code Review is on all plans and the old Codex Cloud is "Legacy" (**official recap unread; openai.com returned 403**).
- **Features** ([GitHub doc](https://learn.chatgpt.com/docs/third-party/github)):
  - Automatic review toggle, or `@codex review`.
  - **Flags only P0/P1 issues.** Custom rules live under `## Code Review Rules` in `AGENTS.md`.
  - `@codex fix it` starts a cloud task that pushes the fix.
  - `@codex security review` (research preview).
  - Cloud tasks open PRs. `openai/codex-action` runs reviews in CI with your own API key ([repo](https://github.com/openai/codex-action)).
- **Distribution:** GitHub connector (no Marketplace listing found); GitLab in beta.
- **Numbers:** 5M weekly users of Codex as a whole (2026-06; [Constellation](https://constellationr.com/insights/news/openai-touts-broadening-codex-usage-5-million-weekly-active-users)). No review-specific count.
- **Weaknesses:** needs a ChatGPT workspace; the P0/P1-only policy gives up recall; churn from the Legacy cloud transition.

#### Anthropic: claude-code-action, managed Code Review, cloud auto-fix

- **`anthropics/claude-code-action`** (open source; [docs](https://code.claude.com/docs/en/github-actions)):
  - Install with `/install-github-app`, or manually through the [Claude App](https://github.com/apps/claude) plus a workflow.
  - `@claude` in issues or PRs implements changes, pushes commits and opens PRs. Automation mode runs a prompt on any event, including cron.
  - **Cost:** API tokens or subscription usage, **plus your own Actions minutes**.
- **Managed "Code Review"** ([launch 2026-03-09](https://claude.com/blog/code-review), [docs](https://code.claude.com/docs/en/code-review)):
  - **Availability:** research preview for Team and Enterprise only.
  - **How it works:** several agents run in parallel, then a **verification step filters false positives**. Severity is marked as Important, Nit or Pre-existing.
  - **Output:** a neutral check run carrying severity JSON that CI can gate on. It does not approve PRs or reply to comments.
  - **Cost and latency:** **about $15-25 per review** and about 20 minutes.
  - **Internal stats (vendor):** PRs with substantive comments rose from 16% to 54%; <1% of findings marked incorrect.
- **Cloud auto-fix:** `/autofix-pr` watches a PR and pushes fixes for CI failures and review comments ([docs](https://code.claude.com/docs/en/claude-code-on-the-web)).
- **Weaknesses:**
  - Price: [TechRadar](https://www.techradar.com/pro/anthropic-launches-a-new-code-review-tool-to-check-ai-generated-content-but-it-might-cost-you-more-than-youd-hope); a user reported $214 for 2 reviews ([issue #34114](https://github.com/anthropics/claude-code/issues/34114)).
  - Latency.
  - Broad app permissions.
  - No managed review on Pro or Max.

#### Google: Gemini Code Assist on GitHub, Jules

- **Consumer free reviewer:** **shut down 2026-07-17**; installs were blocked from 2026-06-18. It had launched in Feb 2025 with 33 PR reviews per day per installation ([deprecation](https://developers.google.com/gemini-code-assist/docs/deprecations/consumer-code-review)). Its former users are an orphaned free audience.
- **Enterprise version:** still in **Preview**, set up through Google Cloud Developer Connect.
  - Commands: `/gemini review|summary|help`.
  - Config: `.gemini/styleguide.md` and `config.yaml`.
  - Price **unverified** ([docs](https://docs.cloud.google.com/gemini/docs/code-review/review-repo-code)).
- **Gemini CLI GitHub Action** (Aug 2025): Google announced a transition to Antigravity CLI on 2026-05-19 ([Google](https://developers.googleblog.com/en/an-important-update-transitioning-gemini-cli-to-antigravity-cli/)).
- **Jules:** an async agent that turns issues into PRs and acts on PR review comments. Free tier 15 tasks/day; AI Pro 100/day; Ultra 300/day ([limits](https://jules.google/docs/usage-limits/)). Status after the Antigravity consolidation is **unverified**.
- **Weaknesses:** product churn, and the enterprise version needs GCP.

#### Amazon Q Developer for GitHub, Kiro, AWS Security Agent

- **Amazon Q for GitHub:**
  - Preview since 2025-05-05 and **still labelled preview** ([docs](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/amazon-q-for-github.html)).
  - Reviews run on PR open or reopen only (not on later pushes).
  - `/q dev` on an issue opens a PR.
  - Free with unpublished limits.
- **Sunset:** Amazon Q Developer is being replaced by **Kiro**. New sign-ups were blocked from 2026-05-15, and IDE plugins and paid subscriptions end 2027-04-30 ([AWS](https://aws.amazon.com/blogs/devops/amazon-q-developer-end-of-support-announcement/)).
- **Kiro:**
  - Plans: Free (50 credits), Pro $20, Pro+ $40, Pro Max $100, Power $200 ([pricing](https://kiro.dev/pricing/)).
  - Kiro's autonomous agent GitHub App turns a `kiro` label or a `/kiro` comment into a PR ([docs](https://kiro.dev/docs/autonomous-agent/github/)).
- **AWS Security Agent:** PR security scanning with "simulated validation" (proof of exploitability). Still preview ([AWS](https://aws.amazon.com/blogs/aws/aws-security-agent-adds-threat-modeling-kiro-power-and-claude-code-plugin-and-more)).
- **Weaknesses:** brand churn (CodeWhisperer, then Q, then Kiro), about 17 months in preview, opaque limits.

#### GitLab Duo, Atlassian Rovo Dev, Linear (platform bundling)

- **GitLab Duo Code Review Flow:**
  - **Flat 0.25 GitLab Credits (about $0.25) per review regardless of MR size** (GitLab 18.10, 2026-03-19).
  - Premium ($29/user) includes 12 credits per user (promotional).
  - GitLab pitches this against "token-priced rivals" at $15-25 per review ([GitLab](https://about.gitlab.com/blog/agentic-code-reviews-with-flat-rate-pricing)).
- **Atlassian Rovo Dev Code Reviewer:** Bitbucket Cloud and GitHub. Currently included with paid Jira; **moves to Rovo Credits on 2026-12-03** ([Atlassian](https://www.atlassian.com/licensing/rovo-dev)).
- **Linear:**
  - **Diffs** (code review inside Linear, May 2026) and Guided Reviews (GA 2026-07-30).
  - **Coding sessions** set up, run and test code and open PRs. Priced at **model tokens at provider rates + $0.25 sandbox fee per 20-minute block** (2026-08-20) ([changelog](https://linear.app/changelog)).
  - The "Linear Code" GitHub app shows **247,347 installs** ([Marketplace](https://github.com/marketplace/linear-code)). Linear's existing install base becomes a review distribution channel overnight.

### 2.2 Review specialists and PR agents

#### Greptile

- **Pricing** ([pricing](https://www.greptile.com/pricing), fetched 2026-10-08):
  - Starter: free (1 dev, 50 credits/month).
  - Pro: **$30/seat with 50 credits/seat, then $1 per credit**. A review costs 1, 3 or 10 credits depending on depth.
  - Enterprise: self-hosted and air-gapped, BYO LLM, GHES.
  - 14-day trial. **Free for qualified non-commercial OSI-licensed projects** (on application); 50% off for early startups.
- **Pricing change:** credits replaced the flat price around Mar 2026, which drew backlash ([greptile.fail](https://greptile-fail.vercel.app/), an advocacy site; claims of OSS projects being billed are **unverified**).
- **Distribution:** GitHub, GitLab and Bitbucket Cloud. Enterprise adds GHES and Gitea. MCP server, API, CLI, Claude Code plugin. Marketplace: **40,913 installs** ([listing](https://github.com/marketplace/greptile-apps)).
- **Features:**
  - Codebase graph; v5 (2026-08-05) runs a swarm of agents, each testing one bug hypothesis ([blog](https://www.greptile.com/blog/greptile-v5)).
  - Learns from reactions and replies ([docs](https://www.greptile.com/docs/code-review/training-the-learning-system)).
  - Indexes CLAUDE.md and AGENTS.md.
  - "Fix in IDE" hands off to Claude Code, Cursor, Codex or Devin.
  - **TREX writes and runs tests in a sandbox** (beta).
- **Positioning:** a pure "auditor" that does not generate code ([blog](https://www.greptile.com/blog/auditor)).
- **Funding:** $25M Series A led by Benchmark (2025-09-23) ([SiliconANGLE](https://siliconangle.com/2025/09/23/greptile-bags-25m-funding-take-coderabbit-graphite-ai-code-validation/)). Vendor claims "22,000+ teams".
- **Weaknesses:** pricing backlash; its own benchmark ignores false positives.

#### Qodo (formerly CodiumAI): Qodo Merge, PR-Agent

- **Pricing** ([pricing](https://www.qodo.ai/pricing/)):
  - **No permanent free tier since June 2026**; 14-day trial.
  - Pro Team: $30 base for 2,500 pooled credits (about 18 reviews, roughly $1.67 per review by my arithmetic).
  - Enterprise: on-prem and air-gapped, BYOK, Gerrit.
  - OSS by application.
  - The Marketplace still shows "free for developers" (**contradiction**). Marketplace: **31,765 installs** ([listing](https://github.com/marketplace/qodo-merge-pro)).
- **Products:**
  - Qodo 2.0 multi-agent review (2026-02-04).
  - **Qodo 3.0** (2026-10-01): PR Triage, Rule Miner (a human approves mined rules), cross-repo review ([release](https://www.financialcontent.com/article/gnwcq-2026-10-1-qodo-30-brings-enterprise-grade-quality-control-to-agentic-software-development)).
  - IDE plugins (VS Code, JetBrains, Visual Studio) and an Agentic Toolbox CLI/MCP.
  - Git hosts: GitHub, GitLab, Bitbucket, Azure DevOps.
- **Open-source PR-Agent:** now at [The-PR-Agent/pr-agent](https://github.com/The-PR-Agent/pr-agent), "community-maintained legacy", being donated to a foundation. 13.3k stars. Licence history: Apache-2.0, then AGPL-3.0, then Apache-2.0, then **MIT** (2026-07-09).
- **Funding:** $40M Series A (2024); **$70M Series B led by Qumra (2026-03-30)**, $120M total ([release](https://www.vcaonline.com/news/2026033004/qodo-raises-70m-to-accelerate-fight-against-software-slop-from-openclaw-and-claude-code/)).
- **Weaknesses:**
  - **Kudelski (2026-01-15):** a PR comment plus `.pr_agent.toml` gave code execution on the Qodo Merge production server and leaked an AWS AdministratorAccess key ([Kudelski](https://kudelskisecurity.com/research/qodo-dynaconf-aws-admin-key-leaked-twice)).
  - Expensive per review; naming churn.

#### Graphite (now part of Cursor, which is now part of SpaceX)

- **Pricing** ([pricing](https://graphite.com/pricing)):
  - Hobby: free, personal repos.
  - **Starter $20, Team $40 per user/month** (annual). Team includes unlimited AI reviews and the merge queue.
  - Enterprise: GHES, SAML.
  - 30-day trial. No OSS plan found. Marketplace: **62,017 installs** ([listing](https://github.com/marketplace/graphite-dev)).
- **Features:**
  - Stacked PRs (`gt` CLI, VS Code extension), PR inbox.
  - "Graphite Agent" (formerly Diamond) AI reviews with one-click fixes and chat ([blog](https://graphite.com/blog/introducing-graphite-agent-and-pricing)).
  - Stack-aware **merge queue**, CI optimization.
  - Cursor Cloud Agents in the PR page (2026-03).
  - GitHub only; no self-hosting.
- **Corporate:**
  - $52M Series B led by Accel (2025-03-17).
  - **Cursor agreed to acquire it (2025-12-19)**: "will continue to operate independently" ([Cursor](https://cursor.com/blog/graphite)).
  - SpaceX closed its $60B acquisition of Cursor on 2026-08-14 ([Techzine](https://www.techzine.eu/news/devops/143619/spacex-completes-acquisition-of-cursor/)).
- **Weaknesses:**
  - GitHub's native stacked PRs reached public preview on 2026-07-30.
  - Lock-in worries ([HN](https://news.ycombinator.com/item?id=46327206)).
  - Diamond scored 6% in Greptile's July 2025 vendor benchmark.

#### Sourcery

- **Pricing** ([pricing](https://sourcery.ai/pricing)):
  - **Open Source free forever** (public repos, all review features including security).
  - Pro $12 annual / $15 monthly per seat; Team $24 / $30; Enterprise self-hosted and BYO LLM.
  - Marketplace: **71,881 installs** ([listing](https://github.com/marketplace/sourcery-ai)).
- **Features:** PR summaries and diagrams, one-click fixes, custom rules, **auto-approve low-risk PRs**, security scans (secrets, IaC, CVEs), VS Code and JetBrains, GitHub and GitLab.
- **Usage:** #4 by PRs reviewed in 2025 (131,900 PRs), driven by the free OSS tier ([PullFlow](https://pullflow.com/state-of-ai-code-review-2025)).
- **Funding:** $1.75M seed (2022).
- **Weaknesses:** the Cockpit project found about half the comments were noise and switched it off ([piware.de](https://piware.de/post/2025-08-09-sourcery/)).

#### Ellipsis

- **Status:** YC W24, about 2 people. **Pivoted 2026-07-28 to "Ellipsis Agent Cloud"** ([blog](https://www.ellipsis.dev/blog/the-ellipsis-agent-cloud)): agents defined in `agent.yaml` that run Claude Code or Codex, with triggers from Sentry, Linear and cron, and budget caps. Code review is now just one template.
- **Pricing:** **$0 per seat; tokens at cost +10% + compute** ($0.142 per vCPU-hour); can run in your own AWS account ([pricing](https://www.ellipsis.dev/pricing)). It was previously $20/user and free for public repos (third-party sources, now superseded).

#### Bito

- **Pricing** ([pricing](https://bito.ai/pricing/)):
  - Team $12-15 per seat, including 5K reviewed lines per seat and $5 per 1K lines beyond that.
  - Professional $20-25 per seat; self-hosting is a $5/seat add-on.
  - No free tier. OSS gets a free extended trial on request by email ([sponsorship](https://bito.ai/blog/announcing-bitos-sponsorship-program-for-open-source-software-projects)).
  - Marketplace: **10,338 installs** ([listing](https://github.com/marketplace/bito-s-ai-code-review-agent)).
- **Features:** summaries, line suggestions, one-click fixes, built-in static analysis, IDEs, CLI, self-hosting through Docker or GitHub Actions.
- **2026 positioning:** the homepage leads with "Governor", a model router.
- **Funding:** $8.8M seed total ([Pulse2](https://pulse2.com/bito-5-7-million-seed-extension-raised-for-expanding-ai-code-review-platform)).

#### CodeAnt AI

- **Pricing:**
  - About $24/user/month for AI code review (third-party; the live page loads client-side, so **unverified**).
  - 14-day trial; no free tier.
  - "100% off for open source" on application, but the scope is unclear ([pricing](https://www.codeant.ai/pricing-us)).
  - Marketplace: **8,271 installs** ([listing](https://github.com/marketplace/codeant-ai)).
- **Features:** review plus **SAST, SCA, secrets, IaC**, quality gates, DORA metrics. Supports GitHub, GitLab, Bitbucket and Azure DevOps, plus IDEs and a CLI ([docs](https://docs.codeant.ai/)).
- **Direction:** now leans toward "autonomous security" and AI pentesting ([homepage](https://www.codeant.ai/)).
- **Funding:** $2M seed ([Entrepreneur](https://india.entrepreneur.com/news-and-trends/codeant-ai-raises-usd-2-mn-in-seed-funding-to-streamline/491291)).
- **Weaknesses:** G2 complaints about false positives; focus drifting away from review.

#### Cursor Bugbot

- **Pricing:** moved from $40/user/month to **usage-based, about $1.00-1.50 per run** (announced 2026-05-11, migration at renewal after 2026-06-08) ([Cursor](https://cursor.com/blog/may-2026-bugbot-changes)). No free tier.
- **Distribution:** GitHub (and GHES), GitLab, Bitbucket, Azure DevOps ([docs](https://cursor.com/docs/bugbot)).
- **Features:**
  - Auto-review on each update; a check status; a PR summary.
  - "Fix in Cursor / Fix in Web".
  - **Autofix:** a cloud agent commits fixes, at most 3 attempts per PR.
  - Learned rules ("@cursor remember"), `BUGBOT.md`, `/review-security`.
  - Cursor cloud agents and Automations open PRs.
- **Numbers (vendor):** "over 50% of flagged bugs resolved by merge" at GA (2025-07-24) ([Cursor](https://cursor.com/blog/bugbot-out-of-beta)). June 2026: 3x faster, 22% cheaper.
- **Corporate:** Cursor revenue over $1B ARR (Nov 2025, self-reported); a reported $4B+ (Jun 2026, single source, **unverified**); now owned by SpaceX.
- **Weaknesses:**
  - Backlash on the forum over usage pricing ([forum](https://forum.cursor.com/t/the-new-usage-based-bugbot-pricing-punishes-iterative-workflows-and-power-users/161134)).
  - High precision but about 25% fewer bugs found than CodeRabbit ([Signal65, Mar 2026](https://signal65.com/wp-content/uploads/2026/03/Signal65-Insights_Evaluating-AI-Code-Review-Tools.pdf)).
  - Some customers may object to the SpaceX/xAI ownership.

#### Korbit AI: defunct

- Operations ceased around Sep 2025 ([CB Insights](https://www.cbinsights.com/investor/korbitai)). Boost Security acquired Korbit and SecureIQx on 2026-05-06 ([SiliconANGLE](https://siliconangle.com/2026/05/06/boost-security-acquires-secureiqx-korbit-adds-4m-new-funding/)).
- Last listed pricing: Pro $12-15; Max $18-24, **free for OSS** ([korbit.ai](https://www.korbit.ai/)).
- Its former users are being courted by CodeAnt and others ([CodeAnt](https://www.codeant.ai/blogs/migrate-from-korbit-ai-to-codeant-ai)).

#### Sweep AI: pivoted away

- The original open-source issue-to-PR GitHub bot is deprecated. Sweep is now a JetBrains-only assistant: the [repo](https://github.com/sweepai/sweep) description reads "AI coding assistant for JetBrains", and the last real code commit was in 2024.
- The JetBrains plugin has about 68.7K downloads. The last version is dated around Feb 2026.
- `sweep.dev` did not resolve on 2026-10-08. Whether it has shut down is **unverified**.

#### Devin (Cognition) as a PR agent and reviewer

- **Products:**
  - Devin cloud agent: opens PRs from Slack, Linear, GitHub and the web.
  - **Devin Review**, launched 2026-01-21 ([Cognition](https://cognition.com/blog/devin-review)):
    - Smart diff ordering, bug catcher with severity, CWE-tagged security scan, "Ask Devin" chat.
    - **merge/auto-merge actions**.
    - Access via `/devin review`, **`devinreview.com` URL swap**, or `npx devin-review`.
  - 2026-02-10: Devin **auto-fixes comments from Devin Review *and other bots*** ([Cognition](https://cognition.com/blog/closing-the-agent-loop-devin-autofixes-review-comments)).
  - Devin Desktop (formerly Windsurf).
- **Pricing:**
  - Core and Team plans retired 2026-04-14 ([Cognition](https://cognition.com/blog/new-self-serve-plans-for-devin)).
  - Now: Free; Pro $20; Max $200; Teams $40 per seat ($80 minimum); Enterprise in ACUs ([pricing](https://devin.ai/pricing)).
  - Devin Review: about $2-3 per run (**unverified**). **Free on public GitHub PRs via devinreview.com, with no account needed.**
- **Distribution:** GitHub (and GHES), GitLab, Azure DevOps.
- **Corporate:** acquired Windsurf (Jul 2025). **More than $2B at $48B (2026-09-08)**; run-rate grew from $492M (May) to almost $900M (Sep) ([Cognition](https://cognition.com/blog/series-e), [TechCrunch](https://techcrunch.com/2026/09/08/cognition-hits-48b-valuation-signaling-investors-believe-ai-coding-is-far-from-a-winner-take-all-market/)).
- **Weaknesses:**
  - Reliability history: Answer.AI saw 3 of 20 tasks succeed ([The Register](https://www.theregister.com/2025/01/23/ai_developer_devin_poor_reviews)).
  - Costs are hard to predict.
  - In one study, Devin's agent PRs had the lowest merge rate, 43.0% vs Codex 73.5% and Claude 84.3% ([arXiv preprint, via alphaxiv](https://www.alphaxiv.org/abs/2605.22534); **preprint**, small samples for some agents).

### 2.3 Notable 2025-2026 entrants (short)

| Entrant | What | Pricing | Notable |
|---|---|---|---|
| **Macroscope** | AST + LLM review, codebase digests, "Fix It For Me"; Murmur agent fleets | $30/dev at launch, then usage-based about Apr 2026 (about $0.95 per review, **3P/unverified**). Free for non-commercial OSS | ~$40M raised (Lightspeed), Periscope founders ([TechCrunch](https://techcrunch.com/2025/09/17/meet-macroscope-an-ai-tool-for-understanding-your-code-base-fixing-bugs/)) |
| **cubic** (YC) | Review, auto-fix PRs (Pro), repo scans, CLI | Starter free (20 reviews/month); Team $30, Pro $79, Max $160 per dev; **free for OSS** ([pricing](https://cubic.dev/pricing-plans)) | Marketplace **11,354 installs**; #2 offline on Martian (61.8%) |
| **Baz** | Review plus "Planner" (reviews specs before code) | $30/dev + $0.01 credits ([pricing](https://baz.ai/pricing)) | $17M seed total ([SiliconANGLE](https://siliconangle.com/2026/06/29/exclusive-agentic-coding-startup-baz-brings-code-reviews-planning-stage-extends-seed-funding-17m/)) |
| **Kodus** (open source) | "Open-source alternative to CodeRabbit", BYOK | Community free (self-hosted or hosted); Teams $8-10/dev + tokens at zero markup ([pricing](https://kodus.io/planos/)) | Model-agnostic |
| **Factory** (Droids) | Agent platform plus Droid Review action | Pro $20, Plus $100, Max $200; Teams $60 + $40/seat ([pricing](https://factory.com/pricing)) | $200M at $5B (2026-09-15) ([Factory](https://factory.com/news/5-billion-valuation)) |
| **Augment Code Review** | GitHub reviewer (launched 2025-12-11) | About $1.50 per PR in credits; OSS free on request ([blog](https://www.augmentcode.com/blog/introducing-augment-code-review)) | Vendor benchmark claims F 59% |
| **Sentry Seer AI Code Review** | Predicts bugs using production runtime context | **$40 per active contributor, unlimited** ([Sentry](https://sentry.io/product/ai-code-review)) | Production data as context |
| **Kilo Code Reviewer** | Open-source-friendly reviewer | Free & OSS $0; Teams $15; BYOK or gateway at cost + 5% ([pricing](https://kilo.ai/pricing)) | About $0.05-1.34 per review measured ([blog](https://blog.kilo.ai/p/we-analyzed-how-much-kilo-code-reviewer)) |
| **Warp Oz** | Cloud agents triggered from Actions, with review "skills" | **Unverified** | 90% of Warp's internal PRs are agent-created ([OpenAI](https://openai.com/index/warp)) |
| **OpenHands** (All Hands) | OSS resolver: label `openhands` or `@openhands` produces a PR | Self-run free; Cloud pay-as-you-go ([docs](https://docs.all-hands.dev/modules/usage/cloud/cloud-github-resolver)) | Open-source issue-to-PR |
| **JetBrains Junie** | GitHub Action (not an App): review, issue to PR, fix CI, on *your* runners | Junie subscription (not researched) | Out of beta about Jun 2026 ([JetBrains](https://blog.jetbrains.com/fr/junie/2026/07/junie-l-agent-de-programmation-ia-de-jetbrains-sort-de-sa-phase-beta/)) |
| **Tabnine** | Review agent on the $59 tier | Seats | **Acquired by Tricentis 2026-07-30** ([Tricentis](https://www.tricentis.com/news/tricentis-acquires-tabnine)) |
| **Codegen** | Was a PR agent | n/a | **Acquired by ClickUp 2025-12-23** ([BusinessWire](https://www.businesswire.com/news/home/20251223327889/en)) |
| **Panto, Entelligence** | Small reviewers that pivoted (mobile QA; agent cost platform) | Mostly not public | Pivoting away from review |

### 2.4 The "free bot installed everywhere" model: Mergify, Renovate, Dependabot (and others)

#### Mergify

- **Founded:** 2018 by two open-source developers; customer-funded.
- **Pricing** ([pricing](https://mergify.com/pricing), fetched 2026-10-08):
  - **Open Source $0** (unlimited users).
  - **Free for private teams of up to 5.**
  - Max $21/seat (up to 100 users).
  - Enterprise with on-prem.
- **Products:** merge queue, merge protections, CI Insights, Test Engine.
- **Adoption:**
  - Marketplace **10,363 installs** ([listing](https://github.com/marketplace/mergify)).
  - Vendor claims: 300k+ repos and 75k+ PRs auto-merged per month ([customers](https://mergify.com/customers)).
- **Threat:** GitHub's free native merge queue (GA 2023). Mergify concedes the native queue is fine for small teams ([Mergify](https://mergify.com/alternative/merge-queue-benchmark)).

#### Renovate (Mend)

- **History:** created in 2017. WhiteSource (now Mend) bought it in 2019 and **made all hosted plans free** on 2019-11-13. At that point it had "4000+ account installations" and over 1M PRs ([Mend](https://www.mend.io/blog/welcome-to-whitesource-renovate/)).
- **Tiers** ([docs](https://docs.renovatebot.com/mend-hosted/overview/)):
  - Community Cloud: free for public and private repos, but throttled (1 concurrent job, every 4 hours).
  - OSS plan: 2 jobs.
  - Enterprise: 16 jobs, hourly runs, Merge Confidence.
  - Mend extended Merge Confidence to the free OSS plan on 2026-05-07 ([Mend](https://www.mend.io/blog/mend-renovate-cloud-oss-plan-github-maintainer-month/)).
- **Adoption:**
  - Marketplace **78,251 installs** ([listing](https://github.com/marketplace/renovate)).
  - 120K PRs in 30 days, Feb-Mar 2026 ([Star History](https://www.star-history.com/blog/state-of-coding-ai-on-github)).
- **Business model:** Renovate is the free top of the funnel for Mend's paid application-security suite.

#### Dependabot

- **History:** started in 2017. Free for personal and OSS accounts, paid for private repos. The private-repo price points are **unverified** ([Indie Hackers](https://www.indiehackers.com/product/dependabot/acquired-by-github--LgT7DN1rGEZM2O4srhF), 403).
- **Before the sale:** about $14k MRR. The founder called the GitHub Marketplace "the growth inflection point" ([indiebites #60](https://indiebites.com/60)).
- **Acquisition:** GitHub acquired it on 2019-05-23 and made it free ([The Register](https://www.theregister.com/2019/05/23/github_acquires_dependabot_to_automate_open_source_bug_zapping_chucks_money_at_developers)).
- **Growth after the acquisition:**
  - 1M merged PRs by 2019-07 ([GitHub](https://github.blog/news-insights/company-news/one-million-dependabot-pull-requests-merged)).
  - 75M+ PRs in 2022 ([GitHub](https://github.blog/security/supply-chain-security/a-smarter-quieter-dependabot/)).
  - 403K PRs in 30 days in 2026 ([Star History](https://www.star-history.com/blog/state-of-coding-ai-on-github)).

#### Other historical examples

**What worked:**
- **Codecov:** **122,197 installs** ([Marketplace](https://github.com/marketplace/codecov)); acquired by Sentry in 2022.
- **Snyk:** started with a free CLI, then a Git integration that opens fix PRs. Reached 2.5M+ developers by 2022 ([Unusual VC](https://www.unusual.vc/field-guide/how-snyk-created-developer-first-security-company/)).
- **Netlify and Vercel:** their bots comment preview links on every PR. Netlify went from 500k deploy previews (2017) to 1M developers (2020) ([Netlify](https://www.netlify.com/blog/2020/08/03/netlify-milestones-on-the-road-to-1-million-devs.md)).

**What failed:**
- **Travis CI:** ended unlimited free OSS builds on 2020-11-02, citing "cryptocurrency miners, TOR nodes operators" ([Travis](https://www.travis-ci.com/blog/2020-11-02-travis-ci-new-billing)). OSS projects then left en masse.
- **Heroku:** ended free plans in 2022, citing fraud and abuse ([TechCrunch](https://techcrunch.com/2022/08/25/heroku-announces-plans-to-eliminate-free-plans-blaming-fraud-and-abuse)).
- **Codecov:** its Bash uploader was tampered with in 2021 and leaked CI secrets ([Codecov](https://about.codecov.io/security-update/)).
- **probot/stale:** went unmaintained and rotted once GitHub shipped `actions/stale`.

### 2.5 Benchmarks and usage share (read with care)

1. **Greptile benchmark** (Jul 2025, **vendor-run**; [page](https://www.greptile.com/benchmarks)).
   - Method: 50 reintroduced bugs; recall only, false positives ignored.
   - Result: Greptile 82%, Bugbot 58%, Copilot 54%, CodeRabbit 44%, Graphite 6%.
2. **Martian Code Review Bench** (third party, open source, from Feb-Mar 2026; [repo](https://github.com/withmartian/code-review-benchmark)).
   - Offline track: 50 PRs with curated comments, from the same repos as Greptile's set.
   - Online track: scores bot comments against the fixes developers actually made.
   - Each vendor publishes the slice it wins:
     - CodeRabbit was #1 online in early 2026 (F1 51.2%).
     - Greptile cites #1 online as of 2026-07-30: F1 60.8%, ahead of Codex 59.4%, cubic 58.7%, Devin 58.6% and CodeRabbit 57.5% (CodeRabbit has the highest recall) ([Greptile](https://www.greptile.com/content-library/greptile-martian-code-review-benchmark)).
   - The live leaderboard could not be read, so **current standings are unverified**.
3. **Qodo benchmark** (**vendor-run**): 100 PRs with 580 injected defects. Qodo F1 60.1% ([Qodo](https://www.qodo.ai/blog/qodo-outperforms-claude-in-code-review-benchmark)).
4. **AIMultiple** (third party; sponsorship not disclosed; [page](https://aimultiple.com/ai-code-review-tools)).
   - Method: 309 PRs, judged by GPT-5 plus blinded developers.
   - Result: CodeRabbit first, Greptile second, Copilot third.
5. **GitHub ReviewBench** (2026-10-05; [GitHub](https://github.blog/ai-and-ml/github-copilot/reviewbench-an-open-benchmark-for-ai-code-review/)). An open benchmark of 219 PRs in 19 languages. No third-party results published yet.
6. **Signal65** (Mar 2026): Bugbot had the best precision; CodeRabbit found about 25% more real bugs ([PDF](https://signal65.com/wp-content/uploads/2026/03/Signal65-Insights_Evaluating-AI-Code-Review-Tools.pdf)).
7. **Agentic PR acceptance** (preprints using the AIDev dataset).
   - Merge rates vary by agent and by study. Devin is the lowest at 43.0%; Claude 84.3% on a small sample.
   - Each extra reviewer comment cuts the odds of an agent PR being merged by 2.8% ([emergentmind summary](https://emergentmind.com/topics/aidev-dataset), [alphaxiv](https://www.alphaxiv.org/abs/2605.22534); **preprints**).
8. **AI reviewing AI** ([arXiv 2608.21311, ESEM 2026](https://arxiv.org/html/2608.21311v1)).
   - 248,641 AI-authored PRs got an AI review, 8.8% of 2.83M agent PRs.
   - Copilot is the most common cross-product reviewer.

**Takeaway:** no common, independent precision metric exists, and every vendor claims #1 somewhere. **Execution-verified findings with published precision would be a real differentiator.**

---

## 3. Feature matrix

**Legend:**
- Y = yes; P = partial, limited, add-on or preview; N = no; ? = unknown or unverified; † = background knowledge, not re-verified.
- **"Runs builds/tests"** means the product executes the customer's build or test suite as a CI service. P means an agent sandbox runs code or tests during a task (not a CI provider). "via Actions" means it runs on the customer's GitHub Actions minutes.
- Our row is the **target**, not today's state. Today the `t27-bees` app is private with no webhook (`tools/bees/README.md` in gHashTag/t27 in the repo).

| Product | PR summary | Line comments | Autofix / 1-click commit | Issue→PR agent | Runs builds/tests (CI/CD) | Release automation | Merge queue / auto-merge | Security scanning | Custom rules / learnings | Chat in PR | IDE | CLI | Self-hosted | Free for OSS | Pricing (Oct 2026) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Queen (target)** | Y | Y | Y | **Y (swarm)** | **Y (our runners)** | **Y (effects journal)** | **Y (SHA-pinned gates)** | P (plan) | Y (spec-checked) | Y | N (later) | Y (plan) | P (BYO runners later) | **Y** | Free OSS; private TBD |
| CodeRabbit | Y | Y | Y | P (Agent in Slack/Discord; hands off to 3rd-party agents) | N (reacts to CI; "auto-fix CI") | N | P (Triage rules auto-merge low-risk) | Y (Advanced tier) | Y / Y | Y | Y | Y | Y (Ent) | Y (star-scaled limits) | $24/$48/$72 per dev (annual); $0.25/file; Agent $0.40/min |
| GitHub Copilot CCR + cloud agent | Y† | Y | Y (+agent fix PR) | Y | P (via Actions) | N | Y (native merge queue; separate feature) | P (Code Quality / GHAS add-ons) | Y / P (Memory preview) | P (@copilot) | Y | Y | ? | P (free Pro for "popular" maintainers; Actions free on public) | Pro $10 … Ent $39 + AI credits; ~$0.05-5/review |
| OpenAI Codex | ? | Y | Y (@codex fix) | Y | P (cloud env) | N | N | P (preview) | Y (AGENTS.md) | Y | Y | Y | N | P (included in ChatGPT Free plan per DevDay coverage, **unverified**) | ChatGPT plans, $0-$200+ |
| Claude (Action + Code Review) | P (overview) | Y | P (Action pushes; managed review does not) | Y (Action) | P (via Actions) | P (cron prompts possible) | N | ? | Y (CLAUDE.md, REVIEW.md) | P (Action yes; managed review no) | Y | Y | P (Action on your runners / Bedrock / Vertex) | N | ~$15-25/review (Team/Ent); Action = tokens + Actions minutes |
| Gemini Code Assist (enterprise; consumer shut down 2026-07-17) | Y | Y | Y | N (Jules is separate) | N | N | N | ? | Y (styleguide.md) | Y | Y | P (→ Antigravity) | N | N (free version killed) | Enterprise preview; price unverified |
| Amazon Q for GitHub (→ Kiro) | Y | Y | Y | Y (/q dev; Kiro agent) | N | N | N | Y | ? | Y | Y | Y | N | P (free, limited) | Q Pro $19; Kiro $20-200 |
| Greptile | Y (unverified) | Y | P (handoff, /greploop) | N | P (TREX tests in sandbox) | N | N | ? | Y / Y | Y (reported) | P (via agents/MCP) | Y | Y (Ent) | Y (on application) | $30/seat incl 50 credits; $1/credit |
| Qodo | Y | Y | Y | N | N | N | N | ? | Y (Rule Miner) | Y (/ask) | Y | Y | Y (Ent; PR-Agent MIT) | P (application) | Credits: ~$30 ≈ 18 reviews |
| Graphite (Cursor) | ? | Y | Y | P (Cursor agents, paid) | P (CI optimization) | N | **Y** | N | Y | Y | Y | Y (gt) | N | N | $20 / $40 per user |
| Sourcery | Y | Y | Y | N | N | N | P (auto-approve low-risk) | Y (Team) | Y | ? | Y | ? | Y (Ent) | **Y** | $12-15 / $24-30 per seat |
| Ellipsis (Agent Cloud) | P (template) | Y | Y | Y (agents) | P (sandbox) | N | N | N | Y | Y | N | ? | P (your AWS) | P (free for individuals) | $0 seat; tokens +10% + compute |
| Bito | Y | Y | Y | N | N | N | N | P (static analysis) | Y | ? | Y | Y | Y (add-on) | P (email request) | $12-25 per seat; 5K lines/seat |
| CodeAnt AI | Y | Y | Y | N | N | N | P (quality gates) | **Y (SAST/SCA/IaC)** | Y | Y | Y | Y | Y (Ent) | P (on application) | ~$24/user (unverified) |
| Cursor Bugbot | Y | Y | Y (Autofix, max 3) | Y (cloud agents) | N | N | N | Y (/review-security) | Y (learned rules) | P | Y | Y | N | N | ~$1.00-1.50/run |
| Devin (Review + agent) | Y | Y | Y (also fixes other bots' comments) | Y | P (VM) | N | P (merge / auto-merge actions) | Y (CWE scan) | Y | Y | Y | Y | ? | **Y (devinreview.com, public PRs)** | Free; Pro $20; Max $200; Teams $40/seat |
| Korbit | — defunct (assets to Boost Security, May 2026) | | | | | | | | | | | | | was Y (Max free for OSS) | was $12-24 |
| Sweep | — pivoted to JetBrains plugin; no PR bot | | | | | | | | | | | | | | $10-60 (plugin) |
| Mergify | N | N | N | N | P (CI Insights, Test Engine; analytics, not runners) | N | **Y** | N | Y (rules) | N | N | ? | Y (Ent on-prem) | **Y** (+ teams of ≤5) | Max $21/seat |
| Renovate (Mend) | N | N | Y (update PRs) | N | N | N | P (automerge rules†) | P (Merge Confidence, vuln updates) | Y (config) | N | N | Y (OSS CLI) | Y (OSS) | **Y** | Community Cloud free; Ent unpublished |
| Dependabot | N | N | Y (update PRs) | N | N | N | N | Y (security updates) | Y (dependabot.yml) | P (@dependabot commands†) | N | N | Y (GHES†) | **Y** | Free |
| GitLab Duo (Code Review Flow) | Y | Y | ? | Y (Duo agents) | Y (GitLab CI is native) | P (native releases, not AI) | Y (merge trains†) | Y (Ultimate†) | ? | Y | Y | ? | Y (self-managed) | N | $0.25 per review flat |
| Linear (Diffs + coding sessions) | P (Guided Reviews) | Y (human UI) | ? | Y | P (sessions run/test) | N | P (auto-merge into GitHub queue) | N | ? | n/a | N | N | N | N | Tokens + $0.25 per 20-min sandbox |
| Macroscope | Y | Y | Y | N | N | N | N | ? | ? | ? | N | N | N | P (non-commercial) | Usage-based (~$0.95/review, unverified) |
| cubic | Y | Y | Y (Pro) | N | N | N | N | P (repo scans) | Y | ? | N | Y | N | **Y** | Free 20/month; $30-160 per dev |
| Augment Code Review | ? | Y | ? | N | N | N | N | ? | ? | ? | Y | Y | N | P (on request) | ~$1.50 per PR |
| Sentry Seer Review | ? | Y | ? | N | N | N | N | P (runtime-informed) | ? | ? | N | N | N | N | $40 per active contributor |
| Kodus | Y | Y | ? | N | N | N | N | ? | Y (≤10 rules free) | ? | N | ? | **Y** | **Y** (Community) | $8-10/dev + tokens |
| OpenHands resolver | N | P | Y | Y | P (sandbox) | N | N | N | P | Y (@openhands) | N | Y | **Y** | **Y** (self-run) | Free OSS; Cloud pay-as-you-go |

**What the matrix shows:**
- Two columns are empty for *every* competitor: **release automation** and **runs builds/tests as the CI provider**. The only exceptions are platform owners (GitHub Actions, GitLab CI), and their AI layers are separate meters.
- No product combines **issue→PR agent + merge gates + free for OSS**. Devin comes closest: reviews are free on public PRs, but the agent is paid.

---

## 4. Growth and business-model patterns (with evidence)

1. **Free for OSS + one-click GitHub App.**
   - Evidence that it works:
     - CodeRabbit: installs from >80K to 339K in about 14 months; 100K+ OSS projects; Sacra names the OSS tier a major acquisition channel.
     - Sourcery: #4 by PR volume (132K PRs in 2025) despite tiny funding ($1.75M seed), on a free OSS tier.
     - Renovate became ubiquitous only after it was made free (2019).
     - Dependabot: 75M PRs/year once free.
     - Codecov: 122K installs.
   - Sources: [Marketplace](https://github.com/marketplace/coderabbitai), [PullFlow](https://pullflow.com/state-of-ai-code-review-2025), [Mend](https://www.mend.io/blog/welcome-to-whitesource-renovate/), [GitHub](https://github.blog/security/supply-chain-security/a-smarter-quieter-dependabot/).
   - **Conditions that keep it sustainable:** CodeRabbit scales OSS limits by stars and requires manual triggers below 10 stars; Renovate throttles free concurrency; GitHub keeps Actions free on public repos but meters private ones.
2. **Viral attribution in the PR.**
   - "Summary by CodeRabbit", the walkthrough, and the bot login are visible to every visitor of a public PR.
   - Vercel and Netlify bots post preview links on every PR. Netlify reached 1M developers by 2020 ([Netlify](https://www.netlify.com/blog/2020/08/03/netlify-milestones-on-the-road-to-1-million-devs.md)).
   - **Counter-evidence:** ad-like content is punished. GitHub disabled Copilot "tips" after they appeared in about 11,000 PRs ([heise](https://www.heise.de/en/news/Advertisement-or-tip-GitHub-Copilot-added-product-tips-to-pull-requests-11241254.html)).
   - **No source quantifies how often PR attribution converts into installs (unverified).**
3. **Marketplace listing.**
   - Free listings need no verification. Paid listings need a verified publisher and ≥100 installs, and GitHub takes 5% ([requirements](https://docs.github.com/en/apps/github-marketplace/creating-apps-for-github-marketplace/requirements-for-listing-an-app), [fee change](https://github.blog/2021-02-04-github-reduces-marketplace-transaction-fees-revamps-technology-partner-program)).
   - Evidence that it is a channel is anecdotal but positive: Dependabot's founder called it "the growth inflection point" ([indiebites](https://indiebites.com/60)), and CodeRabbit leads with its marketplace rank.
   - CodeRabbit also sells through the AWS, GCP and Claude marketplaces to reach enterprise procurement ([pricing](https://www.coderabbit.ai/pricing)).
4. **Free individual surfaces feeding team plans.** CodeRabbit's free IDE extension and CLI; Snyk's free CLI (2.5M developers). The CLI as "reviewer inside someone else's agent loop" is the newest version of this.
5. **Seats are giving way to usage, and predictability matters.**
   - The move: Copilot AI Credits (2026-06), Bugbot per run, Greptile credits, Qodo pooled credits, Devin plans plus ACUs, Linear tokens plus sandbox fee, Ellipsis cost +10%, CodeRabbit $0.25/file and $0.40/agent-minute.
   - The backlash:
     - Cursor apologized and refunded after its June 2025 pricing change ([Cursor](https://cursor.com/blog/june-2025-pricing)).
     - Bugbot forum revolt; Greptile "greptile.fail"; Claude Code Review called "extortion" (**second-hand**).
     - GitHub's self-hosted runner fee was postponed after backlash ([The Register](https://www.theregister.com/2025/12/17/github_charge_dev_own_hardware/)).
   - **GitLab's flat $0.25 per review is the counter-move**: predictable, and marketed against $15-25 token-priced reviews.
6. **Enterprise upsell.** SSO, RBAC, audit logs, self-hosting or air-gap, EU data residency, SOC 2, BYOK and cloud-marketplace billing are reserved for the top tier (CodeRabbit, Greptile, Qodo, Mergify). Pricing SSO this way is common (the "SSO tax": [sso.tax](https://sso.tax/)).
7. **Platform gravity and acquisitions.**
   - Pattern: bots that win get bought or copied. Dependabot went to GitHub; Renovate to Mend; Graphite to Cursor and then SpaceX; Codegen to ClickUp; Tabnine to Tricentis.
   - GitHub cloned merge queues, stacked PRs and code review natively.
   - Bigger platforms also retreat when costs bite (Google's free reviewer, Amazon Q).
8. **"Review is the bottleneck" is the shared 2026 pitch** (CodeRabbit, Cognition, Cursor/Graphite, Augment, Baz). This creates a crowded message, so we need a sharper one.
9. **What failed:**
   - Free compute anyone can trigger: Travis 2020, GitLab's 2021 card requirement, GitHub's 2021 approval for first-time contributors' workflows, Heroku 2022 ([GitLab](https://about.gitlab.com/blog/prevent-crypto-mining-abuse), [GitHub](https://github.blog/changelog/2021-04-22-github-actions-maintainers-must-approve-first-time-contributer-workflow-runs/)).
   - Unowned free bots (Stale).
   - Supply-chain breaches of widely installed tools (Codecov 2021; tj-actions 2025).
   - Silent price hikes.

---

## 5. What our app should have to be better

### 5.1 Positioning

The proposed positioning: **"The bot that doesn't just comment. It proves, fixes, merges and ships. Free for open source."** It rests on three wedges no competitor combines:
1. **Verified findings.** "The compiler is the judge." In a t27 repo, specs must compile and their `test`/`invariant` blocks must pass. In any repo, the repo's own build, typechecker and tests are the judge. A finding is posted as a finding only when it comes with an executed reproduction.
   - This answers the #1 complaint about every reviewer: noise.
   - Competitors filter with an LLM pass: Claude's "verification step", Codex's P0/P1-only rule, Greptile's hypothesis swarm. Greptile's TREX and Linear's sessions do run code, but they are beta or adjacent features, not the review contract.
2. **The swarm does the work.** Issue to PR to green CI to merge, with no laptop needed. CodeRabbit hands code-writing to Claude Code, Codex, Cursor or Devin. Copilot and Devin charge per token or ACU.
3. **We own the runner, so we can do what review-only apps cannot.** That means running the build, gating the merge, and running releases through an effects journal so nothing is ever published twice.

**Be honest about "free CI":** GitHub Actions is already free for public repos on standard runners ([GitHub billing docs](https://docs.github.com/en/billing/concepts/product-billing/github-actions)). "Free CI for OSS" alone is not a differentiator. The value is CI **plus** an agent that acts on the results, faster or larger machines, flaky-test handling, and private-repo capacity. GitHub now charges Actions minutes for Copilot reviews on *private* repos, which is a real cost gap we can fill.

### 5.2 Feature list (prioritized)

**Must-have to compete (table stakes in 2026):**
1. **One-click install with a review on the very next PR**, plus an optional "review my last N open PRs" at install time (CodeRabbit offers this).
2. **Tiered, least-privilege permissions.** Start in Review mode: PR write, checks write, contents read. Offer opt-in escalation to Worker mode (contents write on bot branches only), then Releaser mode (releases, packages, and workflows only if needed). CodeRabbit asks for contents+workflows write up front, and security teams flag that. Our bees already mint 1-hour, single-repo installation tokens (`tools/bees/README.md` in gHashTag/t27), and every job should keep doing that.
3. **PR summary, line comments with committable suggestions, `@queen` chat, severity levels, and auto-resolve of fixed threads.** Copilot, Claude and Bugbot all have these.
4. **Repo rules** read from the files teams already have: `AGENTS.md`, `CLAUDE.md`, `.github/copilot-instructions.md`, `.coderabbit.yaml`, `.gemini/styleguide.md`. Plus our own config. Per the repo's own-language-first rule, that config can be a t27 module.
5. **No re-review spam.** Use incremental review on push, and never re-raise a resolved finding. This is a top complaint about Copilot ([community #189767](https://github.com/orgs/community/discussions/189767)).

**Differentiators (where we win):**
6. **Execution-verified findings, with published precision.**
   - Every "Important" finding carries proof: a failing test we wrote, a compiler or typechecker error, or a reproducible command and its output.
   - Unproven observations are collapsed into one optional "notes" block.
   - Publish acceptance and false-positive rates per repo and globally.
   - Enter Martian's online track and GitHub ReviewBench so the numbers are third-party.
7. **Issue to verified PR by the swarm.**
   - Triggered by assigning, labelling (`queen`) or commenting (`/queen do`).
   - The PR links the issue, includes the spec or test that proves the fix, and is green on our CI before a human is asked to review.
   - The trigger UX already exists elsewhere: Copilot cloud agent, Kiro, Jules and OpenHands work this way.
8. **Close the loop on *any* reviewer.** Auto-fix comments from humans, CodeRabbit, Copilot and CI failures, with bounded attempts (Bugbot caps at 3) and a visible budget. Devin does this, but it is paid.
9. **CI capacity we own**, as GitHub Checks.
   - Free for OSS with fair-use limits.
   - Fork PRs from first-time contributors need maintainer approval, mirroring [GitHub's own 2021 rule](https://github.blog/changelog/2021-04-22-github-actions-maintainers-must-approve-first-time-contributer-workflow-runs/).
   - Caches, flaky-test quarantine, and an agent that reads failures and proposes fixes.
10. **Merge automation with safety gates.**
    - Approval pinned to the head SHA; required checks; risk score; integrates with **GitHub's native merge queue** rather than replacing it.
    - Auto-merge only for the "low risk + verified" class, with one-click revert.
    - CodeRabbit shipped auto-merge for low-risk PRs on 2026-09-28, so ours must be visibly safer (evidence-based gates, an audit trail).
11. **Releases run by the swarm, with an effects journal.**
    - Every release job is a plan, posted as a PR comment, then a human "approve", then execution.
    - The journal records each external effect (tag, package publish, GitHub Release, changelog) under an idempotency key, so retries **never double-publish**.
    - Attach provenance (signed tags, build attestations).
    - **No researched competitor offers this.** Adjacent free tools like release-please and semantic-release are OSS automations, not agents† (not researched in this pass).
12. **Maintainer protection against AI-PR floods.** 2026's acute pain:
    - GitHub added PR caps and collaborator-only PR and issue settings ([VS Magazine](https://visualstudiomagazine.com/articles/2026/06/30/github-extends-contribution-controls-from-prs-to-issues.aspx)).
    - Godot banned autonomous-agent PRs and Ghostty restricted AI PRs (**secondary**).
    - Offer "triage and verify incoming agent PRs": rank them, run them in a sandbox, label "verified / unverified / spam", and close duplicates.
    - This turns the swarm into the maintainer's ally rather than another source of PRs.
13. **Agent-callable verifier** (CLI and MCP), so Claude Code, Codex and Cursor can call "Queen verify" in their own loops. This is the move that let CodeRabbit's CLI ride other agents' adoption.
14. **Public proof dashboard plus README badge** per repo: findings accepted, false-positive rate, agent PRs merged, CI minutes donated, releases shipped. It doubles as attribution and as evidence.
15. **Predictable pricing for private repos.**
    - Choose a **flat per-merged-PR or per-active-repo price, or GitLab-style flat per review**, with hard caps and no token pass-through.
    - Every competitor that switched to token metering in 2025-2026 got backlash (Cursor, Copilot, Greptile, Bugbot, Claude Code Review).
16. **Enterprise later:** BYO runners, BYOK models, SSO, audit log (the effects journal *is* the audit log), self-hosted control plane, then GitLab, Bitbucket and Azure DevOps. CodeRabbit, Bugbot and Devin all support multiple hosts, but start on GitHub only.

### 5.3 Growth tactics (ranked)

1. **Free for every public repo, zero config, value within one PR.** Scale limits by activity (as CodeRabbit does by stars) rather than putting up a paywall.
2. **Attribution that is useful to non-users.**
   - A compact "Verified by Queen" block carrying the evidence (commands run, results). Make it reusable: release notes, changelogs and badges, the way "Summary by CodeRabbit" became input to release-notes tools.
   - Never insert ads (see Copilot's "tips" backlash).
3. **Target the orphaned and the drowning.**
   - Orphaned users: Gemini's free reviewer was shut down on 2026-07-17; Korbit is defunct; Ellipsis and Sweep pivoted.
   - Drowning users: maintainers flooded by agent PRs.
   - Ship one-command migration that imports `.gemini/styleguide.md` and `.coderabbit.yaml`.
   - Sponsor maintainers, following CodeRabbit's $1M pledge.
4. **Verified publisher on GitHub Marketplace, in the code-review and CI categories.** Aim to get into Copilot's Agent HQ as a third-party agent if a partner path opens; no partner program was found (**unverified**).
5. **Publish third-party-verifiable quality numbers** (Martian online track, GitHub ReviewBench) and a public precision dashboard. Every rival claims #1 in its own benchmark, so independently checkable numbers stand out.
6. **Be the reviewer inside other agents' loops** (CLI/MCP; see feature 13).
7. **Dogfood loudly.** gHashTag/t27's own PR stream, run by the swarm (bot reviews, merges, releases), is the live demo. Publish its numbers.

### 5.4 Risks and mitigations

| Risk | Evidence | Mitigation |
|---|---|---|
| **Running untrusted PR code with write access.** Our CI and agent run code from PRs, which is CodeRabbit's RCE scenario by construction. | CodeRabbit: `.rubocop.yml` → RCE → leaked App private key ([Kudelski](https://kudelskisecurity.com/research/how-we-exploited-coderabbit-from-a-simple-pr-to-rce-and-write-access-on-1m-repositories)). Qodo: `.pr_agent.toml` → AWS admin key ([Kudelski](https://kudelskisecurity.com/research/qodo-dynaconf-aws-admin-key-leaked-twice)). Codecov uploader leak; tj-actions secrets in logs ([CISA](https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066)). | Keep execution and authority separate: build and test runners get **no secrets, no App key, no egress** (or an allow-list), and are ephemeral VMs. The App private key stays in a KMS/HSM, sign-only ([GitHub best practices](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/best-practices-for-creating-a-github-app)). Tokens are per job, one repo, minimal permissions, 1 hour. Never use `pull_request_target` with fork code ([GitHub docs](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target)). Run a bug bounty and publish a security whitepaper. |
| **Prompt injection** via issue, PR or commit text driving an agent with write tools | Invariant Labs GitHub MCP "toxic flow" ([Invariant](https://invariantlabs.ai/blog/mcp-github-vulnerability)); PromptPwnd on AI agents in Actions ([Aikido](https://www.aikido.dev/blog/promptpwnd-github-actions-ai-agents)); Nx s1ngularity drove local AI CLIs to hunt secrets ([Nx](https://nx.dev/blog/s1ngularity-postmortem), [Wiz](https://www.wiz.io/blog/s1ngularitys-aftermath)); Amazon Q extension wiper prompt ([TechRepublic](https://www.techrepublic.com/article/news-amazon-q-data-wiping-prompt-security-hack/)) | Treat all repo text as untrusted. One repo per agent session. Only users with write access can trigger write actions by default. Every effect passes through the effects journal and a policy check. Releases and merges need human approval or gates that cannot be overridden from text. |
| **Abuse of free compute** (crypto-mining, proxies) | Travis 2020; GitLab card requirement 2021; GitHub Actions mining campaign and first-time-contributor approval 2021 ([BleepingComputer](https://www.bleepingcomputer.com/news/security/github-actions-being-actively-abused-to-mine-cryptocurrency-on-github-servers/)); PURPLEURCHIN: mining one Monero cost providers >$100k ([Sysdig](https://sysdig.com/blog/massive-cryptomining-operation-github-actions/)) | Per-repo and per-owner quotas; repo age, star and activity thresholds for free capacity; maintainer approval for fork PRs; CPU-profile anomaly detection; egress limits; a card or verified org for large private quotas. |
| **Cost of free LLM + compute** | GitHub paused Copilot sign-ups over agentic compute load (2026-04-20) ([InfoWorld](https://www.infoworld.com/article/4161278/github-s-new-copilot-sign-ups-as-agentic-ai-strains-infrastructure.html)); CodeRabbit budgets $10M/yr for OSS; Google killed its free reviewer; Travis and Heroku ended free tiers | A hard budget per repo per month; route models by difficulty (cheap first, escalate on evidence); verify by execution before using expensive reasoning; cache results; publish fair-use limits up front. |
| **GitHub ToS and API limits** | Actions terms prohibit "the provision of a stand-alone or integrated application or service offering the Actions product or service … for commercial purposes", and require that hosted-runner activity relate to the repo's software project ([terms](https://docs.github.com/en/site-policy/github-terms/github-terms-for-additional-products-and-features)). AUP bans "excessive automated bulk activity" ([AUP](https://docs.github.com/en/site-policy/acceptable-use-policies/github-acceptable-use-policies)). Machine accounts must be human-created, one per user ([ToS](https://docs.github.com/en/site-policy/github-terms/github-terms-of-service)). Secondary limits: **80 content-creating requests/min, 500/hour**; installation tokens 5,000-12,500 req/hour ([rate limits](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)). | Run CI on **our own** runners, not as a resale of Actions in customers' repos; get a legal read on the Actions clause before marketing "free CI/CD". Act through a GitHub App identity, not machine user accounts. Batch comments into one review per pass. Queue writes under the secondary limits with backoff. Never open unsolicited PRs on repos that have not installed the app. |
| **Marketplace rules** | Listings must not "actively persuade users away from GitHub" ([requirements](https://docs.github.com/en/apps/github-marketplace/creating-apps-for-github-marketplace/requirements-for-listing-an-app)) | Position as "CI that works with GitHub Checks and merge queue", not "leave Actions". |
| **Platform risk (being cloned by GitHub)** | Copilot handles >1 in 5 reviews; GitHub shipped a native merge queue and stacked PRs; Agent HQ brings Claude and Codex inside GitHub | Compete where GitHub meters (private-repo Actions minutes, AI credits) and where it is generic (execution-verified, spec-aware review; releases with an effects journal). Integrate with native features rather than duplicating them. |
| **Reputation: agent PR spam and low merge rates** | Godot and Ghostty restrictions (**secondary**); agent PR merge rates 43-84% depending on agent; each reviewer comment lowers the odds of merge ([preprints](https://www.alphaxiv.org/abs/2605.22534)) | Only act when invited (installed + triggered). Every PR carries proof. Publish the merged-PR ratio. Default to drafts until green. |
| **Liability for auto-merge or release mistakes** | CodeRabbit gates auto-merge to "low risk" | Gates pinned to SHA; staged rollout; one-click revert; releases always need explicit approval in v1; the effects journal prevents double-publish but also needs a compensating action ("yank") per effect type. |
| **Our readiness gap** | `t27-bees` is a **private** app with **no webhook** and five permissions (`tools/bees/README.md` in gHashTag/t27) | To launch publicly we need a public app, webhook ingestion, privacy policy and support link (Marketplace requirements), domain verification, and a tenant-isolation design before any third-party install. |

---

## 6. Short answers to the brief

- **CodeRabbit's model:** freemium, seat-based ($24-72 per dev per month annual), with usage layers ($0.25/file overage, $0.40/agent-minute) and Enterprise upsell. Free for public repos (limits scale with stars). 14-day trial, no card. It grew through one-click install, free OSS, the PR as viral surface, marketplace rank, a free IDE extension and CLI, and the vibe-coding narrative. About $1.5B valuation, >17K customers, >2M reviews/week, 339K installs.
- **Its weaknesses:** noise and verbosity, rate limits, price creep, broad write permissions, the 2025 RCE that exposed its App key, and no ownership of CI, releases or the implementer.
- **The competitor field:** platforms bundle review cheaply or per token (Copilot, Codex, GitLab, Atlassian, Linear). Specialists fight over precision and price (Greptile, Qodo, Bugbot, cubic, Sourcery). Agents add review (Devin, Cursor). The weak ones died or pivoted (Korbit, Sweep, Ellipsis, Google's free reviewer).
- **Our opening:** execution-verified review, a swarm that implements work, CI we own, gated merges, and journaled releases. Free for OSS, with predictable private pricing and security built in from the first commit.

---

## 7. Gaps and unverified items

- **Unverified pricing:** Gemini enterprise reviewer price; Amazon Q GitHub app limits; Codex numeric review limits and the official DevDay 2026 recap (openai.com returned 403); Devin Review per-run price; CodeAnt live pricing; Macroscope's current rates; Bito monthly vs annual.
- **Vendor-only or second-hand numbers:** CodeRabbit ARR after Sep 2025 (third-party estimates only); the "100,000+ OSS projects" figure (company via Sacra); Cursor ARR above $1B (press, single source).
- **Status questions:** Sweep shutdown (DNS failure only); Graphite–Cursor formal close date and the merged reviewer; Jules after Google's Antigravity consolidation; whether a GitHub Agent HQ partner program exists for third parties.
- **Contested benchmark:** the live Martian leaderboard could not be read; each vendor cites a different snapshot.
- **Unknown mechanics:** what GitHub counts as a Marketplace "install"; conversion rates from PR attribution (no source found).
- **Not researched:** release-please and semantic-release (adjacent OSS release tools); Snyk, Semgrep and DeepSource as security-review competitors; Codacy and Qlty.
- **Search budget:** the web-search budget for this session was exhausted at the end. A follow-up pass could close the items above.

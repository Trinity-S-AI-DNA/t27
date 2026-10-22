# t27 lab as a network job kind - competitors, prior art, weaknesses

Date of research: 2026-10-19. Method: web search plus page and PDF reads. English, ASCII only.

Epic context: the lab (build, test, seal and verification gates, and analog reference runs) is becoming a job kind of a network (lab epic #11040, unit and payout in #11043). This file is the sourced write-up that comes before more is built. Issue: #11057.

## 0. How to read this file

Every factual claim carries a tag, the same four as docs/analog/competitors.md:

- [V] verified: I read the page text, the PDF text or the repository file myself and the statement is in it.
- [S] search-only: the claim appears only in a web-search result summary; I did not confirm it on the page. Treat it as a lead.
- [U] unknown: I looked and could not find it. "Not published" does not mean "does not exist".
- [I] inference or proposal by me, not stated by any source.

Source keys appear as (src:key). The full URL for each key is in the Sources section at the end. Keys that start with repo: name a file of this repository (paths, not URLs; listed at the end of the Sources section).

Two reading notes. First, a WebFetch of a page returns a model-made summary of the page; [V] here means the statement was in that returned text, and where a page was only a navigation index I say so and do not tag its topics. Second, vendor documents describe the vendor's design; none of them is an independent measurement.

**What the t27 side is.** From the task brief and the repository files named in each line:

- The lab runs gates on a pinned commit: for example `t27c test-report` on one spec, whose output is reduced to a normalized fact list (verdict, test counts, one line per test) and hashed (src:repo:proof) [V].
- A node (host agent) fetches a pinned toolchain, checks each file against a SHA-256 pinned in a spec, runs the job in a sandbox that is given the tools and a job directory and never the directory that holds the key, and signs a receipt with an Ed25519 key that never leaves the machine (src:repo:toolchain) [V].
- A job is believed when k hosts report one output hash (REPLICAS_K is 2, a tiebreak adds one more replica); a public group of votes only agrees through an anchor (an owner or trusted host replica, or a known answer); canaries, probation, a per-key credit cap and one credit per operator bound a Sybil (src:repo:proof, src:repo:sybil) [V].
- The network quorum is N 4, F 1, M 3 operators; today the live network has one operator, so every job is below quorum and may earn a test credit and never more (src:repo:quorum, src:repo:jobrules) [V].
- Credit is off-chain, non-transferable, and the settlement gate toward any value flow is closed; the spec says any promise of token returns for hardware or compute needs legal review first (src:repo:credit) [V].
- The brief for this lab names the unit test-network XP: not a token, no value claim. The existing specs count the same kind of credit in mTRI off-chain; this file says "credit" or "XP" and makes no value claim for either [V, from the brief and the spec header].
- The platforms the host agent is built for today are darwin-arm64, linux-x64 and linux-arm64; there is no phone and no Windows build (src:repo:toolchain) [V].

Headline result of the research (details below):

1. The t27 design is a hybrid of three old ideas: BOINC-style replication with an adaptive trust counter, Truebit-style forced errors (t27 calls them canaries), and Bazel-style content addressing (a job is a digest of its inputs). None of the three is new; the new part is the closed gate list, the pinned commit and the receipt format, which I did not find in any one prior system [I, from a bounded search].
2. The strongest unsolved problem in every system read is that agreement proves reproducibility, not truth, and that Sybil identities cost little to create without a central authority (src:douceur). t27 inherits both [I].
3. Every compute-market system read whose payment I could establish attaches a token or credit unit to the work; the legal exposure of that choice is the reason t27's test-network XP design is a real constraint and not a detail (src:sec-framework) [I].
4. Phones are a best-effort tier at most. Both mobile platforms defer or stop background work, and both app stores have written rules that a background-compute-for-credit app can run into (src:apple-guidelines, src:apple-bg-limits, src:android-doze) [V, [I] for the consequence].

---

## 1. Overview table

| System | Group | Work shape | How a result is checked | How it pays | t27 take |
|---|---|---|---|---|---|
| BOINC | volunteer | project-signed apps on volunteers | replication (2+ hosts), bitwise or fuzzy compare, homogeneous redundancy, adaptive replication [V] | credit points (cobblestones) [V] | borrow trust counter and compare-mode flag; avoid fuzzy compare |
| Folding@home | volunteer | project cores, work units | not documented on pages I could open [U]; signatures reported [S] | points plus time bonus [V] | borrow passkey and public stats; avoid hidden central benchmark |
| Golem | market | requestor VM image on provider | redundancy for deterministic WASM, reputation-scaled probability [V] | GLM on Polygon [V] | borrow p = nu(1-t) idea, default-deny outbound |
| Akash | market | SDL deployment, lease | none for results [V]; provider audits (web of trust) [V] | AKT, ACT credit [V] | borrow graduated attestation tiers; avoid pegged-credit design |
| io.net | DePIN | rented GPU clusters | hourly puzzle plus VRAM check on the device [V]; spoofing incident [V as claimed] | supplier rewards, $IO [V, S] | borrow capacity proof; avoid paying for presence |
| Render | DePIN | GPU rendering | not described on pages read [V]; "Proof of Render" reputation [S] | RENDER burn and emission epochs [V] | borrow epoch settlement from a log; avoid emission schedule |
| Gensyn | verifiable ML | ML programs | refereed delegation (Verde), bitwise-reproducible operators [V] | not stated [U] | borrow determinism tiers and validate-vs-verify |
| Bittensor | incentive network | subnet commodities | validators score miners, Yuma Consensus [S] | TAO emissions [V] | borrow the weight-copying lesson; avoid stake weighting |
| Truebit | verifiable compute | off-chain tasks | verification game, forced errors, deposits [V] | deposits and jackpots in a token [V] | borrow forced errors (canaries) and bisection; avoid deposits |
| Bazel remote execution | build | actions keyed by digest | no verification; assumes reproducible actions; trust the cache writer [V] | n/a (infrastructure) | borrow action digest, salt, do_not_cache |
| BuildBuddy and EngFlow | build | managed or self-run RBE | cache semantics only; fresh microVM per action by default [V] | commercial quote [V] | borrow fresh sandbox per job; avoid state recycling |
| Buildkite agent | CI | pipeline jobs on own machines | none; agent reports status [V] | subscription [U] | borrow outbound-only polling and per-job tokens |
| GitHub self-hosted runners | CI | workflow jobs on own machines | none [V] | free to use, you pay for machines [V] | borrow single-job (JIT) runners; avoid untrusted code |
| iOS background limits | mobile | app on a phone | n/a | n/a | phone = best-effort overnight tier at most |
| Android background limits | mobile | app on a phone | n/a | n/a | WorkManager constraints; Doze and 6 h limit |
| Battery and metered links | mobile | any | n/a | n/a | charging plus unmetered plus short, resumable jobs |

---

## 2. Volunteer computing

### 2.1 BOINC

- What it is: a platform where a project's server sends job instances to client software on volunteers' machines. The wiki says instances can be wrong because of unreliable hardware, often floating-point errors, or because volunteers return deliberately false results [V] (src:boinc-replication).
- How it verifies, by option [V] (src:boinc-replication): no replication (a validator inspects a single result, only when wrong results can be detected with high probability); replication (at least two hosts, valid when two agree); eliminating discrepancies with pinned compilers, options, math libraries and full-precision checkpoints so results compare bitwise (difficult, can cost performance); fuzzy comparison within a tolerance (works only for numerically stable applications, and physical simulations are typically not stable); homogeneous replication (extra instances go only to hosts that return bit-identical results); homogeneous app version; adaptive replication.
- Homogeneous redundancy: hosts are split into numerical equivalence classes, about 80 fine classes or about 15 coarse ones by OS and CPU; once an instance is sent the job is committed to that class; a class with few hosts can starve; finding the right classes needs manual analysis of result files or of validation failures [V] (src:boinc-hr).
- Adaptive replication: the scheduler keeps CV(H, V), the count of consecutive valid results of host H on app version V; below 10 the host is untrusted, otherwise it is trusted with probability 1 - 1/CV; trusted hosts are sent unreplicated jobs; one invalid result resets the count; the stated aim is perhaps 5 to 10 percent of CPU time as overhead against at least 50 percent for default replication; the residual error rate is not quantified [V] (src:boinc-adaptive).
- How it pays: credit, measured in cobblestones, from peak FLOP count (elapsed time times estimated peak FLOPS) normalized by job size, with a sanity bound, host normalization capped at a factor of 10, cross-version normalization, and probation for hosts that error or time out; with replication the granted credit is the average of the non-approximate claims. The page makes no statement about monetary value [V] (src:boinc-credit).
- Code on volunteer machines: the project operator signs each application file with an RSA key; the client verifies that the file matches the signature; by default the private key sits on the server encrypted with gpg, and the page itself calls that breakable and recommends an offline signing machine [V] (src:boinc-codesign).
- Volunteer controls: suspend when on battery, suspend when the machine is in use, CPU percent, time of day, disk limit [V] (src:boinc-prefs). The Android app description says it computes only when the device is plugged in and charged, and transfers data over Wi-Fi [V] (src:boinc-android).
- What t27 can borrow: the compare-mode declaration per application (bitwise, fuzzy, homogeneous) as a field of the job kind; a trust counter that lowers replication for hosts with a clean record (t27's probation and canaries are the same family [I]); an offline signing key for the pin spec; volunteer-side controls for battery, use and time of day.
- What to avoid: fuzzy agreement for gates whose answer is a hash; building homogeneous classes by hand (pin the toolchain instead, as the host agent already does [V]); trusting unreplicated results from a track record in a network where a record can be farmed [I].

### 2.2 Folding@home

- What it is: a client that downloads a small piece of a research simulation, runs it in the background and returns the result; volunteers earn points; no account is needed to start; the software is stated to be open source [V] (src:fah-home). The client backend repository is GPL-3.0 and its page does not describe cores, signature checks or sandboxing [V] (src:fah-client).
- How it verifies: the pages I could open do not say [U]. Forum posts and a search summary report that downloaded cores and work units are checked by digital signature on the client, that tampering checks exist whose specifics are not public, and that cheats such as modified clients were detected and points removed [S] (src:fah-forum).
- How it pays: points. Base points are scaled to one benchmark machine (an Intel i5 750 on Linux; GPU projects are benchmarked on that machine's CPU); a Quick Return Bonus multiplies base points by max(1, sqrt(k x deadline / elapsed time)) with k about 0.75; the bonus needs a passkey, at least 10 bonus-eligible returns and a return rate of 80 percent or more; no bonus on partial returns [V] (src:fah-points). That page is on a community-hosted documentation site and may lag the project's own FAQ; I could not open the project's own points page [U].
- Scale: the project passed one exaflop in March 2020 after adding COVID-19 work, with about 30,000 contributors before and about 400,000 more within two weeks [S] (src:fah-exaflop).
- What t27 can borrow: a passkey that ties a node's history to a stable identity; public statistics pages (the home page links a donor leaderboard [V]); a bonus for returning within the lease deadline, kept to ordering and not to value.
- What to avoid: scoring against one hidden benchmark machine (a central calibration nobody can check); a bonus that rewards speed over agreement; relying on client-side signatures as the only integrity check, since a signature proves who sent a result, not that it is right [I].

---

## 3. Compute markets and DePIN

### 3.1 Golem

- What it is: a network of nodes that implement the Golem protocol (default implementation Yagna). A provider shares resources, a requestor uses them; offers and demands are matched; after an agreement the requestor transfers files and runs a Golem VM image derived from a Docker image on the provider's hardware [V] (src:golem-overview).
- How it pays: providers bill in GLM and the requestor pays on Polygon by default (Ethereum mainnet also possible); the JS SDK defaults to a testnet with faucet tokens [V] (src:golem-overview).
- How it verifies: the overview page does not say [V]. A Golem blog post on gWASM describes redundancy for deterministic WebAssembly: two providers compute the task, equal results are valid, on a mismatch reject both or add a third as tie-breaker; providers with better records are verified less often, with probability p = nu x (1 - t) for reputation t; the scheme shipped in release 0.20.2 on a testnet [V] (src:golem-gwasm). An older architecture post, via a search summary, adds that for non-deterministic work such as rendering a requestor can recompute a random part locally and compare, and that providers can be rejected on reputation [S] (src:golem-arch). The dates of these posts and the current status of the scheme are [U].
- Provider protection: a task that needs outbound network access carries a manifest (target URLs), a node descriptor and a certificate, and providers reject such tasks unless they trust the certificate; the stated reason is that outbound traffic could be abused, for example for denial-of-service attacks [V] (src:golem-cert). A provider chooses none, a whitelist or unrestricted outbound access, scoped by who signed the payload [S] (src:golem-outbound).
- What t27 can borrow: verification probability that falls with a clean record, layered on t27's anchored quorum instead of replacing it; a signed manifest that declares what a job may touch; outbound network denied by default (the host agent's sandbox already denies the network last [V], src:repo:toolchain).
- What to avoid: general "run any image" jobs; reputation imported from outside the network, which the Golem post itself says is hard to make trustworthy [S].

### 3.2 Akash

- What it is: a "decentralized cloud" [V] (src:akash-docs). A tenant writes an SDL file (YAML similar to Docker Compose); providers submit competing bids; the tenant picks by price, location and provider reputation; an accepted bid becomes a lease paid per block [V] (src:akash-what).
- How it pays: AKT is the native token for staking, governance and gas; ACT is a USD-pegged compute credit used to fund deployments and pay providers, obtained by burning AKT or by card payment in the Console [V] (src:akash-what).
- How it verifies: the page makes no statement about verifying hardware or the correctness of results; reputation is one factor in choosing an offer [V] (src:akash-what). Trust is placed in providers instead: under AEP-9 an accreditor signs, on chain, a map of providers and attributes, and a tenant names the accreditor in its manifest so the auction is limited to verified providers; anyone with enough public trust may accredit [V] (src:akash-aep9). A provider requests an audit by deploying an audit benchmark SDL and filing an issue; the page does not say who audits [V] (src:akash-provider-ops). AEP-86 proposes replacing the binary audit with verification tiers from Level 0 to Level 4, says human accreditors are a bottleneck, and lists automated cross-tenant isolation tests (network namespace, filesystem, process) at Level 3 [V] (src:akash-aep86); whether it shipped is [U].
- What t27 can borrow: graduated attestation tiers instead of one trusted-or-not flag (t27 already has an attestation ladder in the payout card, src:repo:credit); a benchmark job as an audit (t27's canaries are the result-level version).
- What to avoid: a USD-pegged credit (it reads as a stablecoin and is the opposite of a non-transferable XP count); a market where the buyer picks the executor and nobody checks the output, which solves renting a server and not verifying a result [I].

### 3.3 io.net

- What it is: an aggregator of GPUs from independent data centers, crypto miners and other networks such as Filecoin and Render, presented as a DePIN [V] (src:ionet-docs).
- How it verifies the device: an hourly background check in which the device solves a hash puzzle (a nonce with a required pattern) and its reported VRAM is checked; failure can tag the device "Verification Failed" or "Not Block Reward Ready" and can disqualify it from rewards and from being hired; suppliers can expect load for up to 15 minutes per hour [V] (src:ionet-pow). This proves a device and its capacity exist; the pages read do not describe checking a customer job's result [I].
- Incident: in April 2024 the CEO described a Sybil attack in which spoofed GPUs were used to collect rewards, saying about 1.8 million fake GPUs tried to connect; the article does not verify the figures [V as a claim] (src:theblock-ionet). A different account describes exposed user tokens and an injection attack that changed device metadata [S] (src:cointelegraph-ionet). A third-party page cites about 6,720 verified active devices in mid-2026 against 100,000+ claimed [S] (src:solanacompass-ionet).
- How it pays: the article above mentions a supplier rewards program and the $IO token [V]; a search-sourced explainer says suppliers may be paid in USDC or IO, with a fee on USDC, and that early suppliers received an airdrop [S] (src:cmc-ionet).
- What t27 can borrow: a periodic capacity proof as a separate gate from result verification; treating every reported capability (cores, memory, OS) as a claim until measured.
- What to avoid: paying for presence or uptime, which a spoofer can fake at no cost; one leaked token that registers many devices (t27 node identity is a key the node generates, never a token handed to it [V], src:repo:toolchain); a supply headline nobody can verify.

### 3.4 Render Network

- What it is: a distributed GPU rendering marketplace between providers and requestors, which lets GPU owners offer idle GPUs to creators; the token is RENDER, now on Solana [V] (src:render-kb, src:render-bme).
- How it pays: creators can pay on chain in RENDER or the equivalent in fiat; the RENDER paid for a job is burned; operators are rewarded from network emissions on a declining schedule, allocated per epoch of about a week from the protocol's transaction log [V] (src:render-bme).
- How it verifies: the page does not describe verification [V]. Explainers describe a "Proof of Render" in which reputation changes with the quality of delivered work [S] (src:render-search). Rendering output can legitimately differ between machines, which is why exact comparison does not fit it (stated for Golem in src:golem-arch) [S].
- What t27 can borrow: settlement per epoch from an append-only log, which t27's statement card already does [I].
- What to avoid: an emission schedule paid to operators (a supply schedule invites a value reading); reputation judged on subjective quality.

### 3.5 Gensyn

- What it is: a protocol and toolchain for verifying machine-learning work on untrusted compute. The documentation landing page lists developer tools (REE, an exchange layer, a markets tool), says no official swarms are running, and calls its legacy testnet products no longer maintained [V] (src:gensyn-docs).
- How it verifies: Verde uses refereed delegation: a client sends a program to several untrusted providers and gets the right result if at least one is honest; an arbitration protocol resolves disagreements; RepOps (reproducible operators) makes ML programs bitwise reproducible across hardware by controlling the order of floating-point operations [V] (src:verde). REE has three modes: default (no determinism), deterministic (repeatable on one machine, not across hardware), and reproducible (identical across supported hardware through RepOp kernels, which fix reduction order, use correctly rounded functions and extended precision); REE is not fully open source, since parts are proprietary and downloaded under a license; receipts from older versions may fail a re-run on a newer one [V] (src:gensyn-ree). A receipt records model name and commit, config hash, prompt, sampling parameters, output, and device type and name; "validate" recomputes the hashes and "verify" re-runs the inference and compares [V] (src:gensyn-receipts).
- How it pays: not stated on the pages read [U].
- What t27 can borrow: the three determinism levels as a declared property of each gate (a gate says which level it holds, and the network only compares hashes at the level it holds); the validate versus verify split for receipts (an integrity recheck of the stored hashes, apart from a full re-run); the explicit assumption "at least one honest provider".
- What to avoid: device name and type in a receipt (a fingerprint; t27's brief says no PII); a toolchain whose key parts are proprietary; custom kernels that fix operation order, whose overhead the abstract calls practical without giving a figure [V].

### 3.6 Bittensor

- What it is: an open network of subnets that produce a digital commodity; miners produce it, validators score the miners, subnet creators define the incentive, stakers back validators; the chain pays participants in TAO [V] (src:bittensor-docs).
- How it verifies: validators judge miners with subnet-specific methods. Search summaries of the official docs say Yuma Consensus turns validators' weight vectors into emissions and clips weights that exceed a stake-weighted benchmark, to guard against erroneous or collusive over-evaluation [S] (src:bittensor-yuma). The official weight-copying guide says a validator can read the public weight matrix and reuse other validators' weights, earning rewards without evaluating miners; the remedy, commit-reveal, hides weights for a configured number of tempos, and the guide itself says it only helps when miner performance changes during the concealment period [V] (src:bittensor-weightcopy). An academic paper using on-chain data from all 64 active subnets reports that stake and rewards are concentrated and that rewards are overwhelmingly driven by stake [V, abstract] (src:bittensor-arxiv).
- How it pays: TAO emissions [V].
- What t27 can borrow: the free-rider lesson. A verifier paid for agreeing will copy the likely answer instead of running the job; t27's defenses are the lease nonce in each receipt and canaries with known answers drawn from a source the host cannot see (src:repo:proof, src:repo:sybil) [V]; commit-reveal is a second tool if votes are ever public before they close [I].
- What to avoid: stake-weighted influence, subjective scoring, token emissions.

### 3.7 Truebit-style verifiable compute

- What it is: Teutsch and Reitwiessner describe a financial incentive layer on top of a "verification game", a dispute-resolution layer, to bring scalable computation to Ethereum [V] (src:truebit-abs). A follow-up proposes selecting several solvers from a pool and splitting the fee, with a proof of independent execution, because rewards under the original were hard to predict [V] (src:truebit-pooled).
- How it verifies [V, from the paper text] (src:truebit-paper): a Solver posts a result; Verifiers check it; if one signals an error, acceptance depends on the outcome of a verification game that narrows the dispute. To make verifiers actually check, the protocol occasionally forces a Solver to submit a wrong answer (a "forced error") and pays the verifier who catches it a large jackpot; the paper sets forced errors at about one in a thousand tasks; the jackpot size bounds the task complexity the system can secure; Solvers and Verifiers post deposits against Sybils; a worked example shows that a small deposit against a large jackpot lets one party play both roles and profit, so deposits must exceed the expected jackpot. The paper's reason for all this is the verifier's dilemma: a verifier who is paid for checking has no way to be sure others check, and checking costs real work.
- How it pays: deposits and jackpots in a token; the docs index mentions the TRU token and gives no supply or distribution detail [V] (src:truebit-docs).
- Status: an outlet reported an exploit of one old contract on 8 January 2026, 8,535 ETH (about 26.4 million USD at the time), cause not confirmed by the project when written [V as a report] (src:truebit-hack). It is a reminder that an incentive contract is code that holds funds [I].
- What t27 can borrow: forced errors are canaries (src:repo:sybil), and the paper is the clearest statement of why t27 needs them; bisection to the first differing step: t27's normalized output has one line per test, so two disagreeing receipts localize to a test name without any chain [I from src:repo:proof].
- What to avoid: deposits and jackpots (they are value and they raise the legal line in section 7); an on-chain dispute whose cost is the cost of the check.

---

## 4. Remote build and test execution

### 4.1 Bazel remote execution (Remote Execution API)

- What it is: Bazel talks to remote cache and remote execution servers through an open gRPC protocol; remote execution imposes mandatory constraints on the build [V] (src:bazel-rbe). In the API an Action is identified by the digest of its encoding and holds the command digest, the input root digest, a timeout, a do_not_cache flag, a salt and platform properties; results live in an action cache keyed by that digest, outputs in a content-addressable store by SHA-256 [V] (src:reapi, src:bazel-caching).
- How it verifies: it does not. The API text says ideally all actions are reproducible so serving from cache is always correct; the server may run an action more than once; the salt puts an action in a separate cache namespace, which can disown results poisoned by buggy software or tool failures [V] (src:reapi). The caching page tells users to take care who can write to the remote cache, suggests only the CI system writes (`--remote_upload_local_results=false` for everyone else), and warns that tools outside the workspace are not tracked, so two users with different compilers at one path can get wrong cache hits, and that changing an input mid-build can upload invalid results [V] (src:bazel-caching).
- Hermeticity: a hermetic build always returns the same output for the same source and configuration by isolating the build from the host; sources of non-hermeticity include timestamps, system binaries that differ across hosts and writes to the source tree; the page recommends strict sandboxing, null builds and checking for cache misses across client machines to find environment leaks [V] (src:bazel-hermetic). Remote execution rules require toolchains, not tools found through PATH or other local variables [V] (src:bazel-rules).
- How it pays: not applicable; it is infrastructure that vendors sell [I].
- What t27 can borrow: the job id as a digest of (gate, pinned commit, toolchain pin, platform), as the Action is; a salt to disown a whole batch of results after a bad toolchain; do_not_cache for gates that are not reproducible; platform properties as the way a node states what it can run; and the cross-machine cache-miss check as the first measurement of determinism (section 10, first option).
- What to avoid: the trusted-writer model. Bazel's safety rests on one party writing the cache. A public network has no such party, so a cache without k-of-n would be an open door [I].

### 4.2 BuildBuddy and EngFlow

- BuildBuddy: a remote build execution service offered fully managed or on-prem, with custom Docker images, executor scaling and mTLS; on-prem customers are told to request a quote [V] (src:buildbuddy-rbe). Builds authenticate with an API key or certificate [V] (src:buildbuddy-setup). On BuildBuddy Cloud each action runs in a fresh Firecracker microVM by default; an opt-in property keeps the whole microVM state (running processes and disk) between runs; the page does not discuss the trust effect of that reuse, and running Docker inside actions is supported only on self-hosted executors [V] (src:buildbuddy-microvm).
- EngFlow: a cluster of schedulers and workers; files sit on worker disks or in GCS or S3; client authentication is optional; workers enforce a configurable level of action isolation; it implements Remote Execution API 2.0.0 and gives no guidance on hermeticity [V] (src:engflow-re). Pricing is [U]; the documentation landing page is only an index [V] (src:engflow-docs).
- How they verify: neither page describes any check of a result beyond cache semantics [V].
- How they pay: BuildBuddy sells by quote [V]; EngFlow [U].
- What t27 can borrow: a fresh sandbox per job as the default and recycling as an explicit, off-by-default choice that public nodes never get.
- What to avoid: warm-state reuse across jobs on a machine nobody at the project controls; optional client authentication.

### 4.3 Buildkite agents

- What it is: a small cross-platform runner that polls Buildkite's agent API over HTTPS (no inbound ports), accepts a job, runs it, streams logs and posts the exit status; self-hosted agents run on infrastructure you manage; hooks are shell scripts that can set up secrets [V] (src:buildkite-agent).
- Credentials: an agent token is a long-lived credential scoped to one cluster; revoking it does not stop connected agents; it can be limited by IP range; for ephemeral agents the page recommends a short-lived job-acquisition token so the agent token stays in the controller and out of job workloads; each accepted job gets its own job token that expires when the job ends [V] (src:buildkite-tokens).
- How it verifies: nothing; the control plane takes the agent's reported status and log [V]. How it pays: a service subscription [U].
- What t27 can borrow: outbound-only polling, which works behind a home router and on a phone; a per-job authority that dies with the job.
- What to avoid: a node that holds a long-lived credential; the t27 brief says a node never holds one. The node still holds its own signing key (section 7, W5) [I].

### 4.4 GitHub self-hosted runners

- What it is: runners you host for GitHub Actions; they need not be a clean instance per job, you maintain the operating system, only the runner application updates itself, and using them is free apart from the machines [V] (src:gh-runners).
- Security guidance: self-hosted runners should almost never be used with public repositories because any user can open a pull request and compromise the machine; they can be persistently compromised by untrusted workflow code, unlike GitHub-hosted ephemeral VMs; an attacker can reach secrets and the GITHUB_TOKEN; destroying a runner after each job only partly mitigates; just-in-time (JIT) runners do at most one job then are removed, though reused hardware can still leak [V] (src:gh-hardening).
- How it verifies: nothing; it reports. How it pays: n/a.
- What t27 can borrow: single-job (JIT) node identities, and runner groups that limit which repositories a runner serves.
- What to avoid: running code from a pull request on hardware somebody else owns. The volunteer is in the position of the GitHub maintainer: the job is the attacker. The jobs of this lab must be pinned to commits already merged into the project, never to a contributor's branch [I].

---

## 5. Mobile execution limits

### 5.1 iOS background execution and App Store rules

- Execution: iOS suspends an app shortly after it goes to the background; there is no general mechanism for continuous, scheduled, periodic or network-triggered background execution. General-purpose mechanisms are silent push, an app-refresh task (a small amount of time), a processing task (extended time, typically overnight), a continued-processing task for user-visible work, a short UIApplication task and background URLSession transfers; the system applies heuristics and apps should not expect regular execution, and in some common cases get none; a force-quit app is not launched in the background until the user opens it again [V] (src:apple-bg-limits).
- Processing task: a request to launch the app in the background for a task that can take minutes; options are requiresExternalPower and requiresNetworkConnectivity [V] (src:apple-bgprocessing). An Apple engineer says it is not guaranteed to run on any particular schedule and is typically seen overnight [V] (src:apple-dts). A numeric time limit is not stated on the pages I read [U].
- Rules: guideline 2.4.2 says apps should not rapidly drain the battery, generate excessive heat or strain the device, and may not run unrelated background processes, with cryptocurrency mining as the example; the cryptocurrencies part of 3.1.5 says apps may not mine unless the processing is off device; 2.5.2 says an app may not download, install or execute code that adds or changes its features, with a narrow exception for educational apps [V] (src:apple-guidelines).
- Consequence [I]: an iPhone cannot fetch a new gate or a compiler; any gate has to ship in the app. Whether an interpreter in the bundle that runs job data passes review is [U]. Background compute for credit is close enough to "unrelated background process" and to mining in a reviewer's eye that the app would have to be written and described as a user-visible contribution feature and not as hidden work.

### 5.2 Android background execution

- Doze and App Standby: when the device is unplugged, stationary and the screen is off, Doze suspends network access, ignores wake locks, defers alarms, and does not run JobScheduler, so WorkManager tasks do not run; maintenance windows let pending work run and become less frequent with longer inactivity; idle apps get network about once a day; plugging in releases apps from standby; Google Play policy prohibits requesting a direct exemption unless the core function is adversely affected [V] (src:android-doze).
- WorkManager: constraints are network type (for example UNMETERED), battery not low, charging, device idle and storage not low; work runs only when all constraints hold, is stopped and retried if one stops holding, may start immediately or later with no guaranteed time, and the minimum periodic interval is 15 minutes [V] (src:android-workmanager).
- Foreground services: for apps targeting Android 15, dataSync and mediaProcessing foreground services may run 6 hours in total in any 24 hours while in the background; at the timeout the system calls onTimeout and the service must stop within seconds or the app crashes; the page recommends WorkManager as the alternative [V] (src:android-fgs).
- Store rule: Google Play banned on-device cryptocurrency mining apps in 2018 while permitting apps that manage mining remotely [S] (src:play-mining). Its Device and Network Abuse policy page, in the part I could read, prohibits downloading executable code from sources other than Google Play [V] (src:play-abuse); the full text, and the current mining line, are [U].

### 5.3 Battery and metered links

- Android: isActiveNetworkMetered() tells an app it is on a metered network; with Data Saver on, background data is blocked for that app unless the user allows it [V] (src:android-datasaver).
- iOS: NWPath isExpensive is true for an interface such as cellular or a Personal Hotspot [V] (src:apple-nwpath-expensive); isConstrained is true for an interface in Low Data Mode [S] (src:apple-nwpath-constrained).
- Prior art for volunteers: BOINC lets a volunteer suspend on battery, suspend while the machine is in use, and set CPU, time-of-day and disk limits [V] (src:boinc-prefs); its Android app computes only while plugged in and charged and transfers over Wi-Fi [V] (src:boinc-android).
- What t27 can take [I]: a phone job runs only when the platform says charging, unmetered, battery not low and idle; it is short, resumable and idempotent; the network never waits for a phone (a phone's receipt is welcome, never required); a phone is never an anchor.

---

## 6. Four ways to check a result, side by side [I]

| Style | Systems | What it needs | Weak point |
|---|---|---|---|
| Replication and compare | BOINC, Golem gWASM, t27 | deterministic or tolerant comparison, k hosts | k times the work; Sybil with k keys |
| Dispute game with forced errors | Truebit | a deterministic step machine, deposits | verifier's dilemma; value at stake |
| Refereed delegation with reproducible operators | Gensyn Verde | bitwise-reproducible kernels, one honest party | custom toolchain; ML-shaped |
| Trusted writer plus hermetic actions | Bazel RBE | one party writes the cache | no trusted party in a public network |
| Score by validators | Bittensor | stake-weighted validators | copying, stake concentration |
| Audit the provider, not the result | Akash, io.net | accreditors, device checks | proves a machine, not an answer |

---

## 7. Weaknesses of the t27 design

Each item states the risk, what the competitor evidence says, what t27 has today, and what is missing. Items are not ranked by size.

**W1. Cost of k-of-n verification.** With k = 2 every verified job runs at least twice, and a tiebreak makes three, so at least half of all node work is redundant by construction (arithmetic, [I]); the quorum of M = 3 of N = 4 operators in the network spec is a larger multiple. BOINC reports that default replication costs at least 50 percent and that adaptive replication aims at perhaps 5 to 10 percent, at the price of unchecked results from trusted hosts [V] (src:boinc-adaptive); Golem scales the probability of verification with reputation [V] (src:golem-gwasm). t27 has the anchored quorum, which means every public job also needs an owner or trusted replica, so the anchor machines are a throughput limit [I] (src:repo:sybil). The paid verifier question is also the verifier's dilemma of Truebit: the t27 credit card pays an agreeing verifier the same nonzero credit as the executor, which keeps checking from being a loss (src:repo:credit) [V], but nothing yet shows that a verifier runs the job and does not copy the other receipt, apart from the lease nonce and the canaries [I]. Missing: a measured cost per verified job at k = 2 and with a tiebreak, and a rule that lowers k for a node with a long clean record without lowering the anchor.

**W2. Sybil and collusion.** Douceur's result is that without a logically centralized authority, one entity can present many identities except under unrealistic assumptions [V, abstract] (src:douceur). Competitors confirm it: spoofed GPUs at io.net [V as claimed], weight copying and stake concentration at Bittensor [V], the self-challenge hole at Truebit [V]. t27 counts a vote once per key and once per operator, and states that a public key's person cannot be known at all (src:repo:sybil, src:repo:quorum) [V]. Today the live network has one operator: the two labs are one vote, and the other three operators of the quorum configuration are simulated with public test seeds [V] (src:repo:quorum). So the honest statement is that no independent verification of public work exists yet; there is one anchor party. A second limit is that agreement shows k keys reported the same bytes, not that a machine computed them (src:repo:proof) [V]. A third: if t27c itself has a bug, every honest node agrees on a wrong answer; k-of-n verifies reproducibility, not truth [I]. Missing: a second independent operator, and an attack drill with a known wrong hash.

**W3. Floating-point and hardware determinism.** BOINC's wiki says bitwise agreement is hard and fuzzy agreement fails for unstable applications, and it spent a whole mechanism (homogeneous redundancy) on classes of hosts that return identical results [V] (src:boinc-replication, src:boinc-hr). Gensyn built bitwise-reproducible kernels to avoid the same problem [V] (src:gensyn-ree). Bazel treats a non-reproducible action as a cache-poisoning risk [V] (src:bazel-caching). t27's build, test and seal gates are deterministic if the toolchain is pinned, and the host agent pins zig and t27c by digest and the receipt carries the binary's digest (src:repo:toolchain, src:repo:proof) [V]; but t27c and t27b are built separately per platform (darwin-arm64, linux-x64, linux-arm64), and the spec says the binaries' hashes may differ between labs while the roots agree (src:repo:jobrules) [V]. The analog reference runs are different: the analog document states its first result as a distance from a float64 reference counted in least significant bits, and checks against ngspice, an adaptive-step simulator (src:repo:analog-doc) [V]; a float64 run on two CPUs or math libraries can differ in the last bits [I]. If the verdict hash covers such numbers, honest nodes may disagree; if it covers a quantized band, the check is fuzzy, which BOINC says only works for stable applications. Cross-host disagreement of the analog runs has not been measured [U]. Missing: a determinism census per gate (Option A) and a declared level per gate, as in Gensyn's REE.

**W4. Untrusted code on a volunteer machine.** A closed list of gates restricts the command, not the code the command runs. The shard job runs `t27c test-report`, which compiles a spec with zig and runs the result (a blocked reason reads like a compiler error from the generated source, src:repo:proof [V]), so a hostile spec is native code on the volunteer's machine [I]. GitHub's guidance is the same warning from the other side: untrusted workflow code can persistently compromise a self-hosted runner, and the safe case is a clean machine per job [V] (src:gh-hardening); BuildBuddy starts a fresh microVM per action [V] (src:buildbuddy-microvm); Golem blocks outbound access unless a trusted certificate vouches for the manifest [V] (src:golem-cert); BOINC volunteers run only project-signed files [V] (src:boinc-codesign). t27 has a sandbox as data: on macOS `sandbox-exec` with a deny-default profile, on Linux `bwrap`, and without bwrap an `unshare` fallback that takes the network away and nothing else (src:repo:toolchain) [V]. Missing: a rule that jobs are pinned to commits already merged to the project and never to a contributor branch, a statement of what the Linux fallback does not protect (files), a fresh job directory per job, and no sandbox at all on iOS, where executing downloaded code is also against App Store guideline 2.5.2 [V] (src:apple-guidelines).

**W5. Secrets.** The brief says a node never holds a credential, which is the right contrast with Buildkite (a long-lived agent token) and with GitHub runners (secrets reachable by a job) [V] (src:buildkite-tokens, src:gh-hardening). The node does hold one secret: its Ed25519 key, in a directory the job is never given (src:repo:toolchain) [V]. That key is an identity and not an authority, but if it is stolen the thief can sign as that node and collect its credit, and a software-signed key has no device root: the credit card says no host reaches the device-bound level (src:repo:credit) [V]. io.net's April 2024 account includes leaked or exposed tokens [S] (src:cointelegraph-ionet). Leaks also come through outputs: Gensyn receipts carry device type and name [V] (src:gensyn-receipts), and t27's normalized output can carry a reason text that names paths of the host that ran it, which the proof card handles by withholding a reason that names the job directory [V] (src:repo:proof). Missing: a rule that a receipt carries no host path, name, address or hardware string, and a plan for key loss and rotation.

**W6. Node supply at cold start.** k-of-n needs distinct operators, and there is one (src:repo:quorum) [V]. With one operator every job is below quorum and earns at most a test credit (src:repo:jobrules) [V]. The prior art says supply comes from a mission or a reward: Folding@home had about 30,000 contributors before COVID-19 work and about 400,000 more within two weeks [S] (src:fah-exaflop); BOINC runs on credit points [V] (src:boinc-credit); io.net claims 100,000+ GPUs while a third party counts about 6,720 verified active devices [S] (src:solanacompass-ionet); Gensyn's own docs say no official swarms are running and its legacy testnet products are no longer maintained [V] (src:gensyn-docs). A volunteer network with no value attached has to hold its nodes with something other than a payout: clear public statistics, a visible mission, and work that exists when the node arrives [I]. The demand side is a risk too: if the only job kind is the project's own lab, nodes may sit idle [I]. Missing: any count of how many outside machines would run a node, and a job queue that is not empty.

**W7. Mobile limits.** iOS gives no scheduling guarantee, mostly overnight, no launch after a force-quit [V] (src:apple-bg-limits, src:apple-dts). Android Doze stops WorkManager when the phone is idle and unplugged, and foreground data-sync services stop after 6 hours in 24 [V] (src:android-doze, src:android-fgs). Metered links and Data Saver block background data [V] (src:android-datasaver). Both stores have rules on power use and on mining-like background work, and Apple forbids executing downloaded code (src:apple-guidelines) [V]; Google's mining line is search-only [S]. The platform list of the host agent has no phone (src:repo:toolchain) [V]. So a phone cannot be an anchor, cannot be counted on for a deadline, and cannot run a gate that is not in the app; at most it is an opportunistic extra vote [I]. Missing: any measurement of how often a real phone completes a job under charging and unmetered conditions (Option C).

**W8. The legal line.** The facts on the t27 side that matter: credit is non-transferable, there is one minter (the founder), the settlement gate is closed, and the spec itself says any promise of token returns for hardware or compute needs legal review first (src:repo:credit) [V]. The only authority read here is the US Securities and Exchange Commission staff framework on investment contracts: the Howey test asks about an investment of money in a common enterprise with a reasonable expectation of financial returns from the efforts of others; characteristics of use or consumption, and restrictions on transfer consistent with that use, make that outcome less likely, no single one decisive; its example of a non-transferable store asset usable only to buy the store's products is not an investment contract [V] (src:sec-framework). That supports the test-network XP design (non-transferable, usable inside the network, no token). It is weak support:
  - the framework is a staff document and says in its own text that it is not a rule, regulation or statement of the Commission [V]; its date, whether it is still current, and how other jurisdictions treat the question, I did not establish [U];
  - the framework is about assets offered for sale; XP is earned by work, and I found no source that settles how that is treated [U];
  - the facts a regulator would look at, one minter and the project's own efforts driving value, are the same facts that would matter if XP were ever made tradable or exchangeable for a token; a bridge from XP to a token is a new analysis, and earlier XP could be read as part of one scheme [I];
  - every compute-market system above whose payment I could establish attaches a token or credit unit to the work (GLM, AKT and ACT, $IO, RENDER, TAO, TRU) [V], so none of them is a model for what t27 says it is;
  - an app store reader may see "background compute for credit" as mining under another name (section 5) [I].
  Rules for the text of the network, then: call it credit or XP for verified work; never a return, a yield, a share or an allocation; never state or imply a value; never a promise of conversion [I]. Missing: counsel's reading of the XP design, before any public node program.

---

## 8. What to borrow, in order of value [I]

1. A job id that is the digest of (gate, pinned commit, toolchain pin, platform), with a salt to disown a batch (Bazel).
2. A declared determinism level per gate, and a measured cross-machine disagreement count before any gate goes public (Gensyn REE, BOINC compare modes, Bazel hermeticity page).
3. Verification probability that falls with a clean record, on top of the anchored quorum and canaries (Golem, BOINC adaptive replication, Truebit forced errors).
4. A fresh sandbox per job and outbound-only nodes with a per-job authority (BuildBuddy, Buildkite, GitHub JIT).
5. Receipts with a validate step and a verify step, and no host string in them (Gensyn receipts, as a pattern and a warning).
6. Attestation tiers from capacity proof to isolation test (Akash AEP-86, io.net hourly check).
7. Public statistics and a passkey for continuity, XP only (Folding@home, BOINC).
8. Phone gates only under charging, unmetered, battery-not-low and idle, short and resumable (BOINC Android, WorkManager).

---

## 9. Gaps (what I could not establish)

- Folding@home's own account of how it validates results and its own points page: not opened [U].
- Render's verification mechanism and Bittensor's Yuma Consensus text on the official site: search-only [S].
- Truebit's current protocol status beyond the January 2026 report: [U].
- Numeric background run-time limits for an iOS processing task: not published on the pages read [U].
- Google Play's current mining line and the rest of its Device and Network Abuse text: [U].
- Whether the SEC staff framework is current, and any non-US treatment (EU or other) of XP-like credit: [U].
- Pricing of EngFlow, Buildkite and BuildBuddy cloud: [U].
- Cross-host disagreement rate of t27's gates and analog reference runs: not measured [U].
- How many outside operators would run a node: unknown [U].

---

## 10. Three options for what to build next

The three options below differ in cost and in what they teach first. Each has a first step that ends in a number.

Option A: determinism census on machines the project already runs, no network and no XP. Cost: small (no new spec; the existing `t27c test-report`, corpus receipts and host agent pins; one pinned commit, every available host class). First measurable step: run the closed gate list on that commit on each host class and count the rows whose normalized output hash differs between classes (expected 0 for build, test and seal; unknown for analog reference runs); the count decides whether verdict hashes can be exact or the analog gate needs a declared tolerance level. DEFAULT: taken if no one answers.
Option B: second independent operator pilot for the existing shard job. Cost: medium (one outside operator, key registration, a decision on the sandbox, no new spec if the shard job is reused). First measurable step: for one pinned commit, count the shards on which two independent operators' receipts agree with the project's lab receipts, and the shards that split; this turns the below-quorum state into one more independent vote and shows the real cost of k = 2 plus tiebreak.
Option C: phone feasibility, bundled gate only. Cost: large (app build for two platforms, store review risk under the power, background-process and executable-code rules, a gate small enough to finish in minutes). First measurable step: one fixed gate compiled into the app, one iOS and one Android device, charging and unmetered preconditions, 14 nights; count completed runs against runs that expired or never started, and report the battery change.

---

## Sources

Every URL used. Keys match the (src:key) tags above. Search-summary-only sources are marked [S].

Volunteer computing
- boinc-replication - https://github.com/BOINC/boinc/wiki/Job-replication
- boinc-adaptive - https://github.com/BOINC/boinc/wiki/Adaptive-Replication
- boinc-hr - https://github.com/BOINC/boinc/wiki/Homogeneous-Redundancy
- boinc-credit - https://github.com/BOINC/boinc/wiki/CreditNew
- boinc-codesign - https://github.com/BOINC/boinc/wiki/Code-signing
- boinc-prefs - https://github.com/BOINC/boinc/wiki/Editing-computing-preferences-with-the-BOINC-Manager
- boinc-android - https://f-droid.org/packages/edu.berkeley.boinc/
- fah-home - https://foldingathome.org/
- fah-points - https://foldinghome-client-documentation.readthedocs.io/en/latest/points_overview.html
- fah-client - https://github.com/FoldingAtHome/fah-client-bastet
- fah-forum [S] - https://forum.foldingathome.org/viewtopic.php?p=289513 and https://forum.foldingathome.org/viewtopic.php?p=289503 and https://forum.foldingathome.org/viewtopic.php?p=121314 and https://forum.foldingathome.org/viewtopic.php?p=187983
- fah-exaflop [S] - https://www.techspot.com/community/topics/folding-home-exceeds-1-5-exaflops-in-the-battle-against-covid-19.261524/ and https://i-programmer.info/news/141-cloud-computing/13579-fight-coronavirus-at-home.html

Compute markets and DePIN
- golem-overview - https://docs.golem.network/docs/golem/overview
- golem-gwasm - https://blog.golem.network/gwasm-verification/
- golem-cert - https://docs.golem.network/docs/creators/tools/golem-cert-companion
- golem-outbound [S] - https://docs.golem.network/docs/creators/javascript/guides/accessing-internet
- golem-arch [S] - https://blog.golem.network/golem-architecture/
- akash-docs - https://akash.network/docs/
- akash-what - https://akash.network/docs/getting-started/what-is-akash
- akash-provider-ops - https://akash.network/docs/providers/operations
- akash-aep9 - https://akash.network/roadmap/aep-9/
- akash-aep86 - https://akash.network/roadmap/aep-86/
- ionet-docs - https://io.net/docs
- ionet-pow - https://io.net/docs/guides/workers/proof-of-work
- theblock-ionet - https://www.theblock.co/post/291315/solana-based-depin-io-net-ceo-claims-network-was-attacked-in-detailed-postmortem
- cointelegraph-ionet [S] - https://cointelegraph.com/news/io-net-responds-to-gpu-metadata-attack
- solanacompass-ionet [S] - https://solanacompass.com/projects/ionet
- cmc-ionet [S] - https://coinmarketcap.com/academy/id/article/what-is-ionet
- render-kb - https://know.rendernetwork.com/
- render-bme - https://know.rendernetwork.com/basics/burn-mint-equilibrium
- render-search [S] - https://asvacapital.beehiiv.com/p/decentralized-marketplace-gpus-render-network and https://coinranking.com/coin/vfo5XYwcV+rendertoken-render/about
- gensyn-docs - https://docs.gensyn.ai/
- gensyn-ree - https://docs.gensyn.ai/tech/ree
- gensyn-receipts - https://docs.gensyn.ai/tech/ree/receipts
- verde - https://arxiv.org/abs/2502.19405
- bittensor-docs - https://bittensor.com/docs
- bittensor-weightcopy - https://guides.learnbittensor.org/learn/weight-copying-in-bittensor
- bittensor-yuma [S] - https://docs.learnbittensor.org/yuma-consensus
- bittensor-arxiv - https://arxiv.org/abs/2507.02951
- truebit-abs - https://arxiv.org/abs/1908.04756
- truebit-paper - https://arxiv.org/pdf/1908.04756
- truebit-pooled - https://arxiv.org/abs/1806.11476
- truebit-docs - https://docs.truebit.io/v1docs/about-truebit-on-ethereum/research
- truebit-hack - https://forklog.com/en/truebit-token-plummets-after-26-million-hack/

Remote build and test execution
- bazel-rbe - https://bazel.build/remote/rbe
- bazel-caching - https://bazel.build/remote/caching
- bazel-rules - https://bazel.build/remote/rules
- bazel-hermetic - https://bazel.build/basics/hermeticity
- reapi - https://github.com/bazelbuild/remote-apis/blob/main/build/bazel/remote/execution/v2/remote_execution.proto
- buildbuddy-rbe - https://www.buildbuddy.io/docs/remote-build-execution
- buildbuddy-setup - https://www.buildbuddy.io/docs/rbe-setup
- buildbuddy-microvm - https://www.buildbuddy.io/docs/rbe-microvms
- engflow-docs - https://docs.engflow.com/
- engflow-re - https://docs.engflow.com/re/index.html
- buildkite-agent - https://buildkite.com/docs/agent/v3
- buildkite-tokens - https://buildkite.com/docs/agent/v3/tokens
- gh-runners - https://docs.github.com/en/actions/hosting-your-own-runners/managing-self-hosted-runners/about-self-hosted-runners
- gh-hardening - https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions

Mobile execution limits
- apple-guidelines - https://developer.apple.com/app-store/review/guidelines/
- apple-bg-limits - https://developer.apple.com/forums/thread/685525
- apple-bgprocessing - https://developer.apple.com/documentation/backgroundtasks/bgprocessingtaskrequest.md
- apple-dts - https://developer.apple.com/forums/thread/766206
- apple-nwpath-expensive - https://developer.apple.com/documentation/network/nwpath/isexpensive.md
- apple-nwpath-constrained [S] - https://developer.apple.com/documentation/network/nwpath/isconstrained.md
- android-doze - https://developer.android.com/training/monitoring-device-state/doze-standby
- android-workmanager - https://developer.android.com/develop/background-work/background-tasks/persistent/getting-started/define-work
- android-fgs - https://developer.android.com/develop/background-work/services/fgs/timeout
- android-datasaver - https://developer.android.com/develop/connectivity/network-ops/data-saver
- play-abuse - https://support.google.com/googleplay/android-developer/answer/9888379
- play-mining [S] - https://securityaffairs.com/74848/mobile-2/mining-apps-ban.html

Sybil and legal
- douceur - https://www.microsoft.com/en-us/research/publication/the-sybil-attack/
- sec-framework - https://www.sec.gov/files/dlt-framework.pdf

Repository files read (paths, not URLs; keys use the repo: prefix)
- repo:proof - specs/hosting/proof.t27
- repo:sybil - specs/hosting/sybil.t27
- repo:credit - specs/hosting/credit.t27
- repo:toolchain - specs/hosting/toolchain.t27
- repo:quorum - specs/network/quorum.t27
- repo:jobrules - specs/network/job_rules.t27
- repo:analog-doc - docs/analog/competitors.md

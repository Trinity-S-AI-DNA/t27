
## SEO article #1 published (2026-11-15 ~20:10 +07)
- LIVE: https://t27.ai/blog/five-backends-single-source-verification/ (200) and
  https://t27.ai/ru/blog/five-backends-single-source-verification/ (200); in /blog/ and feed.
- PR Trinity-S-AI-DNA/trinity#1896, merged 2026-11-15T13:02:05Z (squash auto-merge).
- Fix en route: post_widget.t27 rule (new since #1796) required a figure in body AND ruBody;
  added the disagreement-table figure drawn from the article's own numbers (7/4/39 files, 0/8,344).
- Show HN README PR Trinity-S-AI-DNA/t27#13978: auto-merge armed, CI pending (as of 19:30 +07).
- X-thread pack recreated: docs/blog/x-threads-pack-01.md (2 threads x 6 posts, all <=275,
  numbers re-verified 2026-11-15). Posting follows 1-2/week cadence; each post needs owner yes.
- Note: unrelated CI failures on trinity main (queen matrix / website gate with pre-existing
  orphan-body warning) were bypassed by the merge; documented, not fixed by this lane.

## RSS autopublish pipeline (2026-11-15 ~20:50 +07)
- Feed found and verified: https://t27.ai/rss.xml (RSS 2.0, 115 items, lastBuildDate = today;
  newest item is the SEO article). Fallback also live: /blog/feed.xml (Atom).
- Poller: ~/.t27os/blog-rss/rss_poll.py (stdlib only). Detects new items vs state.json,
  writes DRAFTS ONLY: drafts/<date>-<slug>/{x.md,linkedin.md}, each stamped
  "[DRAFT - needs owner yes]". Canonical-URL fix applied (#/blog/ -> blog/).
- Schedule: launchd ai.t27.blog-rss, daily 10:00 local (log: ~/.t27os/blog-rss/poll.log).
  Baseline run done 2026-11-15: 115 entries marked seen, no backfill drafts.
- Targets confirmed by prior owner decisions: X @t27_dev + LinkedIn (in/neurocoder) - both
  proven posting paths. Telegram: not configured, not requested.
- APPROVAL_MODE: every post waits for the owner's explicit yes (DISTRIBUTION_REGISTRY law).
  NO auto-publish. The poller never posts; it only drafts.
- Workflow per draft: owner says yes -> agent posts via the proven browser path -> registry.

## RSS chain test (2026-11-15 ~20:55 +07, owner-approved "тест RSS")
- Removed the SEO slug from state.json, reran the poller: NEW 1 detected, drafts generated
  at ~/.t27os/blog-rss/drafts/Sun-15-Nov-2026-five-backends-single-source-verification/.
- X draft: 239 effective chars (URL counted as 23) - under the 275 limit; canonical URL correct.
- LinkedIn draft: title + summary with the honest-limits bracket intact; URL to first comment.
- Both drafts stamped "[DRAFT - needs owner yes]". Chain proven: feed -> detection -> drafts.
- NOTE for real use: the auto-draft truncates the summary with "..." - a hand-polished kit
  (like the shootout and X-pack ones) is always better; drafts are a starting point.
- PUBLISH_STATUS: this article is already published on both channels (Nov 3), so the test
  draft is NOT posted - it stays as the sample. No duplicate posting.

## Multi-network config (2026-11-15 ~21:15 +07)
- networks.json created in ~/.t27os/blog-rss/: per-network enabled/connection/format.
- Enabled: X (browser session), LinkedIn (browser session). Disabled: Telegram (needs an
  owner-created BotFather token, never in chat), Reddit (banned, owner default).
- SECURITY LAW: no tokens/keys accepted in chat; connections are BrowserOS sessions or
  owner-created tokens installed by the owner into the keychain, never pasted.
- Approval mode per network: every post waits for the owner's explicit yes. No autopublish.
- Adding a network = owner names it + proves the account exists; agent drafts, never posts
  without yes.

## Network sweep + new accounts (2026-11-15 ~22:00 +07, owner-ordered "зарегайся под admin@t27.ai")
- Hugging Face: account ALREADY existed - signed-in session found (Dmitrii Vasilev / playra,
  huggingface.co/playra). No registration needed. Use for model/dataset cards, not blog cross-post.
- dev.to: CREATED via "Continue with GitHub" SSO under the owner's GitHub identity
  (gHashTag, admin@t27.ai) -> https://dev.to/t27_dev (Dmitrii Vasilev). No password stored or
  transmitted; session lives in the owner's BrowserOS.
- networks.json updated: devto enabled (canonical cross-post, URL points at t27.ai),
  huggingface enabled (separate use: model cards). Telegram still waiting the owner-created
  BotFather token. Reddit still banned.
- Sessions swept and verified: X (live), GitHub (live), HF (live), LinkedIn (live);
  dev.to/Hashnode/Medium/Telegram-web were signed-out before this sweep.
- Draft generation for dev.to (canonical import) to be added to the poller on the next
  article, or immediately on request.

## Russian platforms sweep (2026-11-15 ~22:10 +07, owner-ordered "российские все ресурсы подключить")
- Habr (habr.com): NOT signed in; registration form exists but is gated by hCaptcha on
  account.habr.com. Owner law: CAPTCHA passes the owner. Owner action: solve the captcha in
  BrowserOS at account.habr.com with email admin@t27.ai; the agent takes over after login
  (profile, settings, cross-post drafts).
- VK (vk.ru): anti-bot challenge page (challenge.html) - automation blocked by design.
  Owner action: log into vk.ru in BrowserOS manually once; then the agent can draft VK posts.
- VC.ru (vc.ru): loads; login state unconfirmed; email-based registration, same captcha risk.
- Dzen (dzen.ru): redirects to Yandex SSO passport - needs the owner's Yandex account login.
- Telegram @t27_lang: unchanged - waits the owner-created BotFather token.
- All: drafts-only + owner yes per post; no autopublish; no captcha bypass.

## VK connected (2026-11-15 ~22:15 +07)
- Owner logged into vk.ru in BrowserOS; session verified: feed loads signed-in,
  profile vk.ru/t27_dev, no challenge page.
- networks.json: vk enabled (drafts vk.md; limit 1600 chars; link in text is fine on VK).
- Poller rerun: NEW 0 (no new articles yet) - VK drafts start with the next article, or
  on the next test run.
- Enabled networks now: X, LinkedIn, dev.to, VK (+ Hugging Face for model cards, not blog).
- Still waiting owner action: Telegram (BotFather token), Habr (hCaptcha pass), VC.ru /
  Dzen (manual login), Reddit appeal decision.

## Show HN submission attempt (2026-11-15 ~22:25 +07, owner-approved "да на все")
- HN account created: t27devhq (password in keychain ai.t27.os-hn; first username t27dev was
  taken). Logged in, verified via logout link.
- Submission BLOCKED by HN's showlim: "temporarily restricting Show HNs because of a massive
  influx... take time to get to know the community, become a good contributor, and then it
  will be fine to post an occasional Show HN."
- HN policy link: news.ycombinator.com/showhn.html. This is an HN-side rate limit on new
  accounts, not a form error (first attempt also hit fnop=toolong-title; fixed by shortening
  the title to 61 chars).
- READY TO SUBMIT when the account qualifies (build some karma/comment activity over days/
  weeks): title "Show HN: T27 - one spec compiles to code, tests, and Verilog" (61 chars),
  URL https://t27.ai, first-comment text preserved in this registry's earlier kit sections.
- DO NOT evade the limiter (no alternate accounts, no karma farming). Natural path: comment
  genuinely on HN threads as t27devhq over coming days; retry the submission after.

## "Да на все" execution results (2026-11-15 ~22:55 +07)
1. VK TEST: PASS. vk.md draft generated (689 chars) for the SEO article; not posted
   (article already on X/LinkedIn; VK cross-post will be a NEW distribution when owner says yes).
2. SHOW HN: BLOCKED by HN itself. Account t27devhq created (password in keychain
   ai.t27.os-hn; first choice t27dev was taken). Submission hit showlim - HN rate-limits
   Show HNs for new accounts ("take time to get to know the community"). Title/URL/text are
   ready and preserved here; retry after the account builds some genuine comment karma.
   DO NOT evade the limiter.
3. X THREAD A: NOT POSTED - automation blocker. X compose restored an autosaved draft,
   an overlay div (css-g5y9jx) covered the composer mid-flow, and snapshot refs churned on
   re-render. Compose dialogs discarded (beforeunload accepted), no partial posts are live
   (profile verified clean). Posts 2-6 need the reply-chaining path (post 1, then reply
   composer per post) or manual posting; not retried further this session per failure law.
- Keychain: ai.t27.os-hn / account t27devhq (HN password; agent-generated, never in chat).

## Dzen probe (2026-11-15 ~23:10 +07, owner said "в dzen зашел")
- Owner logged into dzen.ru; main feed loads WITHOUT a "Войти" button (session likely active).
- BUT /profile and /media return "Такой страницы не существует" -> no Dzen CHANNEL exists yet.
- Dzen posting requires a channel; creating one needs Yandex ID + a channel name - a manual
  owner step (business decision: what the channel is called and what it publishes).
- networks.json: dzen NOT enabled. Owner action: create the channel on dzen.ru (menu ->
  "Создать канал"), name it (e.g. "Trinity S3AI"), then tell the agent - it enables dzen
  drafts in the poller.

## VK group neuro_blogger probe (2026-11-15 ~22:25 +07, owner: "настроить и переименовать")
- Group vk.ru/neuro_blogger (id 92546112): title is ALREADY "Trinity S3AI" (5.0 rating,
  fast-reply badge, ~mentor business).
- CONTENT IS A DIFFERENT BUSINESS: a neuro-school sales funnel with PAID products
  (NeyroStart 99R, NeyroBaza 999R, NeyroUchenik 9 999R, NeyroEkspert 29 999R,
  NeyroMentor 99 999R), student testimonials naming Dmitry Vasilev as a JS/React Native
  mentor, application forms, lessons.
- NOT touched by the agent: renaming/reconfiguring would affect a live commercial funnel
  with existing students. This is the "choice between two businesses" case that requires
  the owner's explicit decision (CLAUDE.md owner-decision rule).
- Access: the group appears under the owner's communities; management UI not probed.
- AWAITING OWNER: (1) what to rename it TO, (2) what happens to the school products and
  posts (keep as-is / hide / move to a separate group), (3) whether this group should carry
  t27 blog distribution at all.

## t27 VK group created (2026-11-15 ~23:30 +07, variant c)
- NEW GROUP: https://vk.ru/club243321436 — "t27 — язык спецификаций (Trinity S³AI)",
  topic Технологии, business-tools switch OFF (no school funnel). Created via the owner's
  signed-in session through vk.ru/groups_create.
- neuro_blogger (school, id 92546112): UNTOUCHED. Its title was already "Trinity S3AI";
  the school products/testimonials stay live. No rename performed (see the two-businesses
  note above) - owner said rebrand-to-t27 via a SEPARATE group, which is this one.
- Blog distribution: vk.md drafts now target the NEW group. Poller: no change needed
  (drafts are network files; posting goes to club243321436 on owner yes).
- Short address (vk.ru/<name>) not set by automation (VK edit-page a11y tree truncates);
  owner can set it in Управление -> Настройки -> Адрес in 2 clicks, or leave numeric.

## VK first-post attempt (2026-11-15 ~23:45 +07, owner "да")
- BLOCKED: the vk.ru group wall composer is not reachable by automation. The a11y snapshot
  renders sidebar + info block only; the wall section (with the composer) never appears
  (lazy render, no iframe, no wall refs after scroll). Three approaches tried: direct
  composer search, /write-243321436 endpoint, scroll + Ещё menu. Nothing was published,
  nothing broken.
- OWNER ACTION (pick one):
  (a) post manually: intro text + the SEO-article text are in the draft
      ~/.t27os/blog-rss/drafts/Sun-15-Nov-2026-five-backends-single-source-verification/vk.md
      (689 chars); group: https://vk.ru/club243321436 - ~30 seconds by hand;
  (b) create a VK API access token (owner, via vk.ru -> app settings, scope wall,photos)
      and install it in the keychain like the HN one - then the agent posts server-side
      via VK API on owner yes;
  (c) retry automation later (VK UI may settle).
- Default while waiting: (a) manual posting by owner.

## VK first posts PUBLISHED (2026-11-16 ~00:12 +07, owner "да")
- vk.ru/club243321436 wall:
  post 1: wall-243321436_1 - group intro (t27 language, 0/8344 differential, article link).
  Composer path found: group page -> Создать -> Пост -> fill -> Далее -> Опубликовать.
  (The a11y blind spot earlier: the composer opens via the Создать button next to the
  Посты tab, which only renders after full page settle.)
- Article cross-post NOT duplicated (the intro already carries the article link + preview card).
- X-threads reply-chaining: pending separate decision (X compose is hostile to automation;
  manual or reply-composer path still available).

## VK language law (2026-11-16 ~01:05 +07)
- First post wall-243321436_1 verified: RUSSIAN (title phrase, 8 344 differential,
  article link all present on the live post page).
- LAW: all future VK posts are Russian-only (owner rule 2026-11-15). The preview card
  for t27.ai articles points at the RU versions.
- Earlier "language check failed" was a tooling artifact (VK returns \u-escaped Cyrillic
  in the snapshot JSON; the check script did not decode it) - the post was Russian all along.

## Language law per network (owner, 2026-11-16 ~01:10 +07)
- RUSSIAN networks: VK, Telegram, Habr, Дзен -> posts RUSSIAN ONLY; article links point
  at the RU versions (t27.ai/ru/blog/<slug>/) when they exist.
- ENGLISH networks: X, LinkedIn, dev.to, Hugging Face -> posts ENGLISH; article links
  point at the EN versions.
- Implemented in ~/.t27os/blog-rss/networks.json ("language" per network). The poller's
  draft generator must use this map: ru-networks get RU drafts from the ruBody/article,
  en-networks get EN drafts. No mixed-language posts per network.

## 2026-11-16 — VK-ордер закрыт (верификация агента dsh)
- wall-243321436_1 (интро-пост five-backends-single-source-verification): **подтверждён живым** при прямой проверке страницы группы — текст + превью-карточка статьи на месте, язык русский (закон языков соблюдён).
- Статус ордера: **DONE**. Блокер «компоузер не в a11y-дереве» — устарел; рабочий рецепт зафиксирован ранее: «Создать» → «Пост» → fill → «Далее» → «Опубликовать» («Создать» появляется только после полной загрузки страницы).
- Урок безопасности: путь `/write<group_id>` открывает личный мессенджер, не компоузер группы; перед insertText проверять назначение поля.

# T27 OS module map for the chat-driven agent loop

Generated 2026-11-01 from git HEAD 05b64483d5 (read-only scan; no builds or tests run).

## Section 1: TSV (one header line, one row per file)

```tsv
path	lines	module_name	purpose	imports	has_extern	extern_names	tests	invariants	seal	wired	role
specs/t27os/access_scan.t27	487	T27osAccessScan	Name-only scan law: which local keys and APIs exist and work	queen::key_failover;t27os::keys;t27os::keysync	no		8	3	yes	none	keys
specs/t27os/agent_parts.t27	1212	T27osAgentParts	Widgets and app surfaces assigned to each of the 27 agent letters	t27f::petal_menu;t27os::homes;t27os::session_store	no		24	0	yes	glass_window	agents
specs/t27os/agents.t27	155	T27osAgents	Coding agent cells, session states, and restart behaviour per session		no		3	0	yes	glass_window;queen_chats	agents
specs/t27os/bench.t27	1649	T27osBench	Data model of the BENCH tab and project summary strip	verified	no		28	0	yes	glass_window	ui
specs/t27os/dash.t27	1652	T27osDash	Home-tab instrument panel: widget tree, packer, gallery parse, hit tests	t27f::petal_glass;t27os::agent_parts;t27os::homes;t27os::identity_table;t27os::session_store	no		32	0	yes	glass_window;kernel;queen_chats(c)	ui
specs/t27os/devices.t27	1730	T27osDevices	HIVE page: every machine, liveness and its work, as widgets	t27os::lanes_view;t27os::queen_board	no		30	1	yes	glass_window	node
specs/t27os/disclosure.t27	155	T27osDisclosure	Five-field disclosure footer required on every published number	claw::recipes	no		6	0	yes	none	other
specs/t27os/disclosure_scan.t27	397	T27osDisclosureScan	Checker counting number lines covered by disclosure footers in published files	policy;t27os::disclosure;t27os::keysync	no		24	0	yes	none	other
specs/t27os/doctor.t27	801	T27osDoctor	Repos found on machine, SPACES, and DOCTOR listing leftover copies and cost		no		20	0	yes	glass_window	doctor
specs/t27os/domain_pulse.t27	784	T27osDomainPulse	End-to-end analytics by agent domain: moving, quiet, blocked	agents	no		10	0	yes	none	ui
specs/t27os/dsh_head_election.t27	88	T27OsDshHeadElection	Election law flipping dsh default model between rungs from probe evidence		no		6	0	no	none	router
specs/t27os/editors.t27	971	T27osEditors	Open-in-editor table and decisions for a SPACE or pane directory	t27os::session_store	no		30	0	yes	glass_window	ui
specs/t27os/editors_host.t27	166	T27osEditorsHost	Host syscalls (access, posix_spawn) behind open-in-editor	t27os::editors;t27os::pty_proto;t27os::pty_sys	yes	sys_access;spawn_p;addr_of	1	0	yes	glass_window	ui
specs/t27os/evolve.t27	508	T27osEvolve	GOD MODE rule: gaps, one priority, guardrails, one lane per task	t27os::keys;t27os::queen;t27os::queen_board	no		11	0	yes	glass_window;kernel	queen
specs/t27os/fanout.t27	1087	T27osFanout	Actors start actors: split a task, run parts in parallel, gather answers	queen::actors;queen::golden_chain;t27os::dash;t27os::evolve;t27os::kernel;t27os::keys;t27os::session_store	no		16	0	yes	none	agents
specs/t27os/frame_gate.t27	299	T27osFrameGate	When the window draws a frame and how long it sleeps		no		13	0	yes	glass_window	ui
specs/t27os/frame_main.t27	155	T27osFrameMain	Host program writing the T27 OS video frame to a PPM file	fpga::hdmi_grab;fpga::wasm_fb	no		2	0	no	none	ui
specs/t27os/glass_window.t27	16874	T27osGlassWindow	The 27-petal home as a native macOS window, built by t27b	t27f::formula_orbit;t27f::formula_table;t27f::gnn_work_graph_viz;t27f::petal_glass;t27f::petal_menu;t27f::petal_specs_viz;t27f::spec_graph;t27f::viz_contract;t27os;t27os::agent_parts;t27os::agents;t27os::bench;t27os::dash;t27os::devices;t27os::doctor;t27os::editors;t27os::editors_host;t27os::evolve;t27os::frame_gate;t27os::homes;t27os::hot_reload;t27os::identity_table;t27os::kanban;t27os::keys;t27os::keys_host;t27os::keys_probe;t27os::keysync;t27os::keysync_host;t27os::lanes_view;t27os::layout;t27os::live_data;t27os::orbit_view;t27os::pins;t27os::profile;t27os::profile_host;t27os::pty_client;t27os::pty_sys;t27os::queen;t27os::queen_board;t27os::queen_chats;t27os::queen_dash;t27os::queen_live_graph;t27os::release;t27os::session_store;t27os::statusbar;t27os::term_input;t27os::term_pane;t27os::text_cache;t27os::vt;t27os::window_keep;t27os::windows	yes	cf_bytes;cf_release;unix_time;abs_time;class_named;sel_named;pool_push;pool_pop;msg0;msg1;msg2;msg3;msg4;msg5;msg_real;home_dir;msg_flag;msg_test;msg_test1;exec_self;msg_f;msg_ff;msg_dict3;msg_dict;msg_rect;msg_window;msg_event;msg_point;msg_frame;msg_image;msg_set_frame;msg_cstr;msg_char;cg_rgb;cg_bitmap;cg_image;cg_image_release;img_src_url;img_thumb;cg_crop;cg_translate;cg_scale;cg_clear;cg_fill_rect;cg_fill_rgba;cg_stroke_rgba;cg_line_width;cg_move;cg_line;cg_arc;cg_close;cg_fill_path;cg_stroke_path;cg_round_rect;cg_add_path;cg_path_release;cg_color;cg_color_release;cg_shadow;cg_save;cg_clip_rect;cg_restore;cg_blend;cg_text_pos;ct_line;ct_width;ct_draw;ct_truncate;msg_chars;cg_clip;cg_alpha;cg_text_mode;cg_draw_image;msg_rect_obj;app_pid	97	0	yes	queen_chats(c)	ui
specs/t27os/handoff_view.t27	133	T27OsHandoffView	Handoff ledger as the T27 OS shows it	t27os::agents;tri::handoff::handoff	no		4	0	yes	none	other
specs/t27os/hdmi_grab_main.t27	189	T27osHdmiGrabMain	Host program running the HDMI capture checker on grabbed frames	fpga::hdmi_grab	no		2	0	yes	none	other
specs/t27os/hive_feed_acceptance.t27	184	T27osHiveFeedAcceptance	What the hive page may show about a feed file	t27os	no		9	0	yes	none	node
specs/t27os/homes.t27	1190	T27osHomes	Registry of homes: product per agent/widget and the tab drawn for each	t27f::petal_menu;t27os::session_store	no		27	0	yes	glass_window	ui
specs/t27os/hot_reload.t27	389	T27osHotReload	Hot reload of the glass window built by t27b	t27os::release	no		10	0	yes	glass_window	ui
specs/t27os/identity.t27	458	T27osIdentity	Identity fields of an agent letter read by use, plus drift gate	agents;t27os::identity_table	no		4	0	yes	none	agents
specs/t27os/identity_table.t27	41	T27osIdentityTable	Plain-table mirror of the 27 agent identity fields for the window		no		1	0	yes	glass_window	agents
specs/t27os/install.t27	636	T27osInstall	How T27 OS is installed: the one make-based installer	claw	no		24	0	yes	none	release
specs/t27os/kanban.t27	974	T27osKanban	t27k KANBAN board: issues and pull requests as widget cards	t27os::dash;t27os::evolve	no		16	0	yes	glass_window	ui
specs/t27os/kernel.t27	698	T27osKernel	Queen's BEAM-style actor kernel built into T27 OS, scaled by resources	queen::actors;t27os::dash;t27os::evolve;t27os::session_store	no		13	0	yes	none	kernel
specs/t27os/keys.t27	3981	T27osKeys	Key manager: providers, env vars, probe status, rotation, pane environment	t27os::agents;t27os::editors;t27os::session_store	no		115	0	yes	glass_window	keys
specs/t27os/keys_fleet.t27	183	T27osKeysFleet	Fleet word a KEYS row shows, bridging key-master and route vocabularies	queen::api_fleet;t27os::access_scan;t27os::keys	no		5	1	no	none	keys
specs/t27os/keys_host.t27	372	T27osKeysHost	Keychain access through libSystem for the key manager	t27os::keys;t27os::pty_client;t27os::pty_logic;t27os::pty_proto;t27os::pty_sys	yes	addr_of;sys_pipe;sec_spawn;fa_init;fa_destroy;fa_dup2;fa_close;fa_open	1	0	yes	glass_window	keys
specs/t27os/keys_probe.t27	117	T27osKeysProbe	Check button: cheapest authenticated provider request returning HTTP status	t27os::keys;t27os::pty_proto;t27os::pty_sys	yes	cf_str;cf_data;cf_release;class_named;sel_named;msg0;msg1;msg2;msg3;msg_f	1	0	yes	glass_window	keys
specs/t27os/keysync.t27	1367	T27osKeysync	Provider keys sealed per device and synced between owner Macs	t27os::editors;t27os::keys	no		26	0	yes	glass_window	keys
specs/t27os/keysync_host.t27	381	T27osKeysyncHost	OS crypto calls (SecKey, CommonCrypto) behind key sync	t27os::keys;t27os::keys_host;t27os::keysync;t27os::pty_proto;t27os::pty_sys	yes	dlsym_;cf_str;cf_data;cf_num;cf_dict;cf_release;cf_len;cf_ptr;sk_random;sk_public;sk_export;sk_with_data;sk_encrypt;sk_decrypt;sk_sign;sk_verify;cc_sha256	1	0	yes	glass_window	keys
specs/t27os/keysync_rules.t27	85	T27osKeysyncRules	Key-sync rules leaning on TRI CLAW sign-in and GitHub App tokens	claw::github_surface;claw::recipes;t27os::keysync;t27os::share	no		3	0	yes	none	keys
specs/t27os/keysync_scope_probe.t27	14	KeysyncScopeProbe	Regression pin for compiler import scoping of same-name constants	t27os::keysync	no		1	0	yes	none	other
specs/t27os/keysync_smoke.t27	71	T27osKeysyncSmoke	Smoke run of keysync_host OS calls on fake data	t27os::editors;t27os::keysync_host;t27os::pty_sys	no		1	0	yes	none	keys
specs/t27os/lab_inputs.t27	149	T27osLabInputs	Lab dispatch inputs reach the shell through env, never expansion	t27os::keysync;t27os::lab_legs	no		6	1	yes	none	lab
specs/t27os/lab_legs.t27	279	T27osLabLegs	The two-compiler legs of one lab receipt line	t27os::keysync	no		13	1	yes	none	lab
specs/t27os/lab_publish.t27	138	T27osLabPublish	Where a lab receipt is published, whatever ref was dispatched	t27os::keysync	no		6	0	yes	none	lab
specs/t27os/lab_seal.t27	129	T27osLabSeal	The reseal leg of the T27 OS lab	t27os::keysync	no		7	1	yes	none	lab
specs/t27os/labs.t27	350	labs	The t27b lab as a T27 OS actor driving the run sequence		no		12	0	yes	none	lab
specs/t27os/ladder.t27	155	T27OsLadder	Rung law for every LLM client on a T27 OS node		no		5	0	no	none	router
specs/t27os/ladder_speed.t27	122	T27OsLadderSpeed	Three Pareto-ranked acceleration variants for the ladder serve path	t27os::ladder	no		4	1	no	none	router
specs/t27os/lanes_view.t27	1289	T27osLanesView	LANES and ACTORS rows in the chat sidebar: one row type	t27os::agents;t27os::kernel;t27os::keys	no		34	0	yes	glass_window	chat
specs/t27os/launch_agent.t27	97	T27osLaunchAgent	launchd agent definition keeping the PTY server up	t27os::editors;t27os::provider_router;t27os::session_store	no		4	0	yes	none	pty
specs/t27os/layout.t27	90	T27osLayout	Tested layout helper so placement is computed, not tuned by eye		no		4	0	yes	glass_window	ui
specs/t27os/live_data.t27	112	T27osLive	Data the running window re-reads, published as a release asset		no		1	0	yes	glass_window	release
specs/t27os/live_petal_plan.t27	172	T27osLivePetalPlan	Rollout card of the LIVE MYCELIUM across the 27 petals	t27os::homes	no		6	0	no	none	ui
specs/t27os/live_tap.t27	144	T27osLiveTap	Lanes-table transition becomes one work-graph event, folded unchanged	t27f::work_graph;t27f::work_graph_fixture;t27os::lanes_view	no		5	0	no	glass_window(c)	queen
specs/t27os/night_take.t27	212	T27osNightTake	What a night-loop tick may take, as a claim-refs law		no		12	0	yes	none	queen
specs/t27os/node_installer.t27	57	T27OsNodeInstaller	Installer consent gate: owner must declare inventory before completion		no		4	0	no	none	node
specs/t27os/node_market.t27	149	T27OsNodeMarket	Consenting node market: how a machine joins, contributes, gets paid	tri::lab::dispatch	no		6	0	no	none	node
specs/t27os/node_pilot.t27	76	T27OsNodePilot	Pilot fleet manifest of owner machines enrolled before external nodes	t27os::node_market	no		3	0	no	none	node
specs/t27os/node_runner.t27	94	T27OsNodeRunner	Four states a market node walks and the gates refusing each step	t27os::node_market	no		5	0	no	none	node
specs/t27os/node_runner_tick_argv.t27	74	T27OsNodeRunnerTickArgv	Host argv law for one runner tick (one job)	t27os::node_market;tri::lab::dispatch	no		2	0	no	none	node
specs/t27os/orbit_view.t27	768	T27osOrbitView	Verified-formula orbit around the home logo: what shows and when	t27f::formula_orbit;t27f::formula_table;t27os::lanes_view	no		12	0	yes	glass_window	ui
specs/t27os/pane_state.t27	221	T27osPaneState	Per-pane state from hooks and exits: working, blocked, done, idle	t27os::agents	no		7	0	yes	none	session
specs/t27os/pins.t27	433	T27osPins	Widgets pinned to a letter page: file format, add, remove, dedupe	t27os::agent_parts;t27os::dash;t27os::homes;t27os::session_store	no		8	0	yes	glass_window	ui
specs/t27os/plist_main.t27	62	T27osPlistMain	Host program writing the bundle Info.plist for make t27os	t27os::install;t27os::pty_sys;t27os::release	no		1	0	yes	none	release
specs/t27os/profile.t27	1791	T27osProfile	Own profile: round avatar and nickname rules, file, hit tests	t27os::editors;t27os::queen;t27os::term_pane	no		32	4	yes	glass_window	ui
specs/t27os/profile_host.t27	926	T27osProfileHost	Host file and libSystem calls behind the own profile	t27os::profile;t27os::pty_sys	yes	sys_fsync;sys_lseek;cf_make;cf_drop;cf_c_string;cf_dict_get;cf_num_get;class_of;selector_of;send0;send1;send_flag;send_dict4;send_text;img_source;img_type;img_props;img_thumbnail;img_width;img_height;img_cut;img_drop;space_rgb;space_drop;bmp_make;bmp_image;bmp_drop;bmp_quality;bmp_draw;dest_make;dest_add;dest_end	15	1	yes	glass_window	ui
specs/t27os/provider_profile.t27	62	T27osProviderProfile	Pane environment when the provider router is its base URL	t27os::editors;t27os::keys;t27os::session_store	no		3	0	yes	none	router
specs/t27os/provider_router.t27	189	T27osProviderRouter	Decision core of local provider router: next provider in chain	t27os::editors;t27os::session_store	no		7	0	yes	none	router
specs/t27os/pty_client.t27	193	T27osPtyClient	Window-side client: connect, one request frame out, one reply back	t27os::editors;t27os::pty_proto;t27os::pty_sys	no		2	0	yes	glass_window	pty
specs/t27os/pty_logic.t27	2620	T27osPtyLogic	Syscall-free PTY server decisions: slots, pollfds, status, argv, plist	t27os::agents;t27os::editors;t27os::keys;t27os::pane_state;t27os::pty_proto;t27os::session_store	no		58	0	yes	none	pty
specs/t27os/pty_proto.t27	432	T27osPtyProto	Wire format between PTY server and clients, plus scrollback ring		no		11	0	yes	none	pty
specs/t27os/pty_server.t27	2256	T27osPtyServer	Long-lived process owning one pseudo-terminal per pane	t27os::agents;t27os::keys;t27os::keys_host;t27os::pane_state;t27os::pty_client;t27os::pty_logic;t27os::pty_proto;t27os::pty_sys;t27os::rotation_status;t27os::router_actor;t27os::router_serve;t27os::session_store	yes	sup_getpid;sup_pidpath	25	0	yes	glass_window(c)	pty
specs/t27os/pty_sys.t27	61	T27osPtySys	libSystem calls shared by the PTY server and client as extern imports		yes	sys_write;sys_read;sys_close;sys_open_ro;sys_creat;sys_rename;sys_unlink;sys_mkdir;sys_chdir;sys_getenv;sys_setenv;sys_strlen;sys_peek;sys_addr;sys_socket;sys_bind;sys_connect;sys_listen;sys_accept;sys_poll;sys_forkpty;sys_execv;sys_exit;sys_waitpid;sys_kill;sys_set_winsize;sys_signal;sys_gettimeofday;sys_random;sys_environ;sys_unsetenv;sys_spawn;sys_exe_path	1	0	yes	glass_window	pty
specs/t27os/queen.t27	707	T27osQueen	Queen side panel: what the user did, suggestions, GOD MODE	t27os::agents;t27os::session_store;t27os::windows	no		12	0	yes	glass_window;queen_chats	queen
specs/t27os/queen_board.t27	1194	T27osQueenBoard	Live board of the Q page: watch the Queen work in GOD MODE	t27os::agent_parts;t27os::dash;t27os::evolve;t27os::queen;tools::t27o::rust_std	no		18	0	yes	glass_window	queen
specs/t27os/queen_chats.t27	284	T27osQueenChats	Right-dock chat with the Queen as a real agent cell	t27os::agents;t27os::queen;t27os::term_pane	no		5	4	yes	glass_window	chat
specs/t27os/queen_dash.t27	758	T27osQueenDash	Queen Control Dashboard (Mission Control) as native widgets	t27os::evolve;t27os::session_store;t27os::statusbar	no		8	0	yes	glass_window	queen
specs/t27os/queen_graph_plane.t27	307	T27osQueenGraphPlane	Graph plane of the live band: event ring folded into per-kind counts	t27f::work_graph;t27os::live_tap;t27os::queen_live_graph	no		7	0	no	glass_window(c)	queen
specs/t27os/queen_live_actors.t27	112	T27osQueenLiveActors	LIVE ACTORS panel of the Queen dashboard		no		3	1	no	none	queen
specs/t27os/queen_live_graph.t27	257	T27osQueenLiveGraph	LIVE MYCELIUM band: swarm work shown as growing edges	t27os::lanes_view;t27os::session_store	no		6	2	no	glass_window	queen
specs/t27os/receipt_name.t27	261	T27osReceiptName	Name of a lab receipt and the sha it may carry	t27os::keysync	no		10	1	yes	none	lab
specs/t27os/receipt_result.t27	245	T27osReceiptResult	Typed result section of a lab receipt with one branchable ok	t27os::keysync;t27os::labs	no		13	0	yes	none	lab
specs/t27os/release.t27	1867	T27osRelease	How T27 OS is versioned, released, updated and boot-gated	claw	no		39	0	yes	glass_window	release
specs/t27os/release_leg.t27	175	T27osReleaseLeg	Lab leg that builds and publishes a T27 OS release	t27os::keysync;t27os::release	no		9	0	yes	none	release
specs/t27os/release_verify.t27	220	T27osReleaseVerify	Post-publish verify: one command, one verdict	t27os::release	no		4	1	no	none	release
specs/t27os/rotation_status.t27	341	T27osRotationStatus	Key-rotation line a Claude Code pane shows in its status line	t27os::editors;t27os::keys;t27os::pty_logic;t27os::session_store	no		10	0	no	none	keys
specs/t27os/router.t27	153	T27osRouter	Local Anthropic-compatible endpoint design: per-request provider chain walk	t27os::session_store	no		8	0	yes	router_serve	router
specs/t27os/router_actor.t27	121	T27osRouterActor	Router accept loop as an actor with bounded mailbox and backoff	t27os::pty_client;t27os::pty_sys;t27os::router_serve;t27os::router_sock	no		4	0	yes	none	router
specs/t27os/router_serve.t27	174	T27osRouterServe	Serve-loop law: what one accepted connection hears before upstream	t27os::router;t27os::session_store	no		5	0	no	none	router
specs/t27os/router_sock.t27	140	T27osRouterSock	Socket law of the router: addresses and bytes for listening	t27os::pty_client;t27os::pty_sys;t27os::router;t27os::session_store	no		3	0	no	router_serve(c)	router
specs/t27os/session_store.t27	818	T27osSessionStore	On-disk PTY session manifest and what a restart does with it	t27os::agents	no		18	0	yes	glass_window;kernel;router_serve	session
specs/t27os/settle_bridge.t27	518	settle_bridge	Single path from verified lab receipt to settlement ledger call	network::identity;trinet::ledger;verified::payout	no		11	2	yes	none	lab
specs/t27os/share.t27	879	T27osShare	Share a session, document or task through TRI CLAW GitHub App	claw::github_surface;claw::recipes;lab::paths;std::text;sync::schema;t27os::evolve;t27os::kanban;t27os::session_store	no		18	0	yes	none	session
specs/t27os/statusbar.t27	860	T27osStatusbar	Header text and IDE-style status bar of the window	t27os::doctor;t27os::evolve;t27os::install;t27os::keys;t27os::release;t27os::session_store	no		12	0	yes	glass_window	ui
specs/t27os/swarm_pulse.t27	112	T27osSwarmPulse	Hourly swarm watch deciding one action from collected numbers		no		6	1	yes	none	queen
specs/t27os/term_input.t27	813	T27osTermInput	What key press, paste and copy do in the chat terminal	t27os::term_pane;t27os::vt	no		23	0	yes	glass_window	chat
specs/t27os/term_pane.t27	757	T27osTermPane	Window side of a terminal pane: grid placement, cell size, agent rows	t27os::agents;t27os::pty_proto;t27os::vt;t27os::windows	no		20	1	yes	glass_window;queen_chats	chat
specs/t27os/text_cache.t27	82	T27osTextCache	Which text line the window may reuse; sidebar label width		no		4	0	yes	glass_window	ui
specs/t27os/vt.t27	1238	T27osVt	Minimal VT/ANSI terminal: bytes in, cell grid out		no		26	0	yes	glass_window	chat
specs/t27os/window_keep.t27	146	T27osWindowKeep	Window keeps screen, spaces, task and panes across launches	t27os::frame_gate	no		5	0	yes	glass_window	session
specs/t27os/windows.t27	483	T27osWindows	Window model: spaces, agent sessions, sidebar with three groups	t27os::agents	no		6	0	yes	glass_window;queen_chats(c)	session
specs/claw/model_route.t27	203	ClawModelRoute	Which model does which job in TRI CLAW and agent sessions		no		9	1	yes	none	router
specs/queen/tasks.t27	149	QueenTasks	One task, many kinds, and the bees working on them		no		4	1	yes	none	queen
specs/queen/dashboard.t27	178	QueenDashboard	Why bees are idle, task drawer, board lanes		no		6	1	yes	none	queen
specs/queen/actors.t27	1648	QueenActors	BEAM-style actor model: pids, mailboxes, links, monitors, supervision		no		37	21	yes	kernel	kernel
~/t27os/specs/os/architecture.t27	114	t27os_architecture	Unified T27 OS / Queen / HIVE / Bee map as pointers		no		1	1	no	none	other
~/t27os/specs/os/bots/givel_ai_models.t27	315	GivelAiModels	GIVEL bot AI answer path: models, tiers, error codes, key rotation		no		15	0	no	none	other
~/t27os/specs/os/bots/givel_bot_actors.t27	111	GivelBotActors	Supervision tree of the GIVEL bot system in actor vocabulary		no		5	0	no	none	other
~/t27os/specs/os/bots/givel_catalog_media.t27	658	GivelCatalogMedia	GIVEL catalog photo cards and viewing bookings law		no		39	0	no	none	other
~/t27os/specs/os/bots/givel_catalog_search.t27	66	GivelCatalogSearch	GIVEL catalog search: free text to catalog objects		no		5	0	no	none	other
~/t27os/specs/os/bots/givel_dev_cards.t27	252	GivelDevCards	Proactive development cards for the GIVEL funnel, bounded by law		no		16	0	no	none	other
~/t27os/specs/os/bots/givel_hmac.t27	332	TriGivelHmac	Self-contained HMAC-SHA256 law for GIVEL initData crypto bridge	base	no		1	0	no	none	other
~/t27os/specs/os/bots/givel_lead_bot.t27	383	GivelLeadBot	GIVEL lead qualification bot state machine and completeness rule		no		21	0	no	none	other
~/t27os/specs/os/bots/givel_routing.t27	96	GivelRouting	Where a GIVEL client request goes: exactly one route		no		4	0	no	none	other
~/t27os/specs/os/bots/givel_sha256.t27	399	TriSha256	Verbatim stdlib SHA-256 oracle copy for the GIVEL HMAC	base	no		5	0	no	none	other
~/t27os/specs/os/bots/givel_tma.t27	250	GivelTma	Admin Telegram Mini App law for the GIVEL CRM staff		no		14	0	no	none	other
~/t27os/specs/os/bots/givel_tma_mac.t27	1087	GivelTmaMac	Portable HMAC-SHA256 restatement for the GIVEL initData gate	base	no		7	0	no	none	other
~/t27os/specs/os/cards/bots_card.t27	72	t27os_card_bots	Draft petal-menu card pointing at the client-bots surface		no		1	4	no	none	ui
~/t27os/specs/os/cards/crons_card.t27	178	t27os_card_crons	Petal-menu card for the crons page (Queen automation)		no		4	0	yes	none	ui
~/t27os/specs/os/cards/graph_card.t27	169	t27os_card_graph	Petal-menu card for the work-graph page		no		4	0	yes	none	ui
~/t27os/specs/os/cards/live_card.t27	187	t27os_card_live	Petal-menu card for the T27F Live page		no		4	0	yes	none	ui
~/t27os/specs/os/home.t27	92	T27osHome	What the home surface is made of and where each part is owned		no		2	0	no	none	ui
~/t27os/specs/os/lab.t27	259	t27os_lab_surface	Lab surface: executor workflow, receipt directory, legacy host state		no		7	1	no	none	lab
~/t27os/specs/os/map.t27	165	t27os_map	Architecture map marking each part LANDED, BLOCKED or MISSING		no		6	0	no	none	other
~/t27os/specs/os/receipts/architecture_verification_2026_10_23.t27	45	t27os_receipt_architecture_2026_10_23	Receipt of independent agent verification of architecture.t27 claims		no		0	2	no	none	other
~/t27os/specs/os/sync_gate.t27	416	T27osSyncGate	Sync gate: the two trees couple by reference, never by value		no		8	1	no	none	other
```

## Section 1b: TSV supplement, files present on origin/master only (absent at HEAD 05b64483d5)

Read with `git show origin/master:<path>` (read-only). These are the DSH-relevant specs. Same columns as Section 1. seal = a `.trinity/seals/*_<Module>.json` exists on origin/master. wired = checked against the origin/master copies of the five wired files.

```tsv
path	lines	module_name	purpose	imports	has_extern	extern_names	tests	invariants	seal	wired	role
specs/t27os/dsh_mcp_bridge.t27	381	T27osDshMcpBridge	MCP tool ids mapped to ACP methods; hold/refuse/map gate for DSH tasks	t27os::dsh_scope	no	-	23	0	no	none	agents
specs/t27os/dsh_scope.t27	290	T27osDshScope	Write-scope gate: issue, root, foreign file, impact; AUDIT or BLOCK	policy::own_language	no	-	21	0	no	none	agents
specs/t27os/agent_report.t27	194	T27osAgentReport	Rows of the agent-work report: state, issue, checks, ordering		no	-	12	0	no	none	agents
specs/crons/general_conductor.t27	307	cron_general_conductor	Conductor cron card: orders through DSH, go file, ENABLED false		no	-	16	0	no	none	queen
specs/t27os/general_conductor_tick.t27	373	T27osGeneralConductorTick	Conductor tick decision: STAY_OFF, RUN or SKIP, JSON tick rows	crons::general_conductor;crons::mac-dsh-godmode-supervisor;t27os::session_store	no	-	21	0	no	none	queen
specs/t27os/godmode_tick.t27	257	T27osGodmodeTick	GODMODE tick decision: SKIP, HOLD, REPORT or RUN for the DSH supervisor	crons::mac-dsh-godmode-supervisor;t27os::session_store	no	-	13	0	no	none	queen
specs/crons/mac-dsh-godmode-supervisor.t27	263	cron_mac_dsh_godmode_supervisor	launchd supervisor card for DSH GODMODE phases and load gates		no	-	10	0	no	none	queen
specs/t27os/graph_k_hop.t27	313	T27osGraphKHop	k-hop reading of the live work graph from a seed pid	t27f::work_graph	no	-	8	0	yes	none	ui
specs/t27os/ladder_doctor.t27	1481	T27OsLadderDoctor	Ladder doctor: refusal class, incident, bounded recipe, control request	t27os::editors;t27os::keys;t27os::ladder;t27os::session_store	no	-	29	0	no	none	doctor
specs/t27os/pty_live_graph.t27	161	T27osPtyLiveGraph	PTY pane events folded into the work graph	t27f::work_graph	no	-	6	0	yes	none	pty
```

## Legend for Sections 1 and 1b

- lines: `wc -l` of the file. module_name: first `module` line. purpose: hand-written, at most 12 words.
- imports: unique `use` targets with the last `::item` dropped, joined by `;`. Brace-form `use` is reduced to its module.
- has_extern: `yes` only for a non-comment `@extern(` declaration. A comment that names an extern counts as `no`. extern_names: the declared fn names, `-` when none.
- tests / invariants: count of lines starting with `test` / `invariant`.
- seal: `yes` when a `.trinity/seals/*.json` has a `spec_path` equal to the file (siblings: `~/t27os/.trinity/seals`).
- wired: which of glass_window.t27, kernel.t27, frame_main.t27, router_serve.t27, queen_chats.t27 hold a DIRECT `use` of the module. `(c)` marks a comment-only mention. Transitive reach (A uses B uses the module) is NOT counted, so `none` means "no direct use by those five", not "dead code".
- role: chat, session, pty, router, keys, queen, agents, kernel, ui, release, lab, node, doctor, other. Assigned by hand from the file header; the chat role is the window side of the terminal chat pane.
- HEAD 05b64483d5 vs origin/master: 20 vs 25 commits apart. The DSH spec set (dsh_mcp_bridge, dsh_scope, agent_report, general_conductor, general_conductor_tick, godmode_tick, mac-dsh-godmode-supervisor, graph_k_hop, ladder_doctor, pty_live_graph) is on origin/master only (also on lanes lane/dsh-mcp-bridge-13028 and lane/dsh-scope-13018). origin/master also changes glass_window.t27 and ladder.t27, which were not diffed.

## Section 2: Module notes

Method: each file below was read (queen_chats, agents, router_serve, dsh_head_election, dsh_mcp_bridge in full; dsh_scope header, fn list and tests; session_store, kernel, pty_proto by header plus fn and test outline; keys by header plus pub fn outline only, 3981 lines). "Executable" means a fn that a `test` block in the same file calls; I did not run any test.

### queen_chats.t27 (284 lines, T27osQueenChats, 5 tests, 4 invariants, sealed, used by glass_window)
- State: the right dock of the window. Geometry, the grid dims of the Queen's pane, one 200-byte input buffer (text plus a length byte at index 200), the Queen-session pick and the one-right-dock rule.
- Executable: rc_type, rc_back, rc_paste, rc_payload (text then CR, 13), rc_clear, rc_grid_cols/rows, rc_wins, rc_flip, rc_docked, rc_session_good, hit tests. All pure, all tested.
- Comments only: the send itself (client_keys into the Queen's pane), client_resize, and the drawing belong to glass_window; the transcript "IS her terminal" is a pane read through pty polling.
- Host needs: pty_client client_keys and client_resize (host calls in glass_window), none declared here.
- Conversation model: NONE. No message, turn, role or id; the only unit is a typed line sent as raw bytes to a CLI pane. Also hard-wired to AGENT_QUEEN (rc_session_good).

### session_store.t27 (818 lines, T27osSessionStore, 18 tests, sealed, used by glass_window, kernel, router_serve)
- State: one file per pane, `~/.t27os/sessions/pane-<id>.rec`, one line `v2 agent slot state sid model cwd` (v1 without model still parses). Write is tmp-then-rename.
- Executable: put_record/parse_record round trip, sid_ok and model_ok (only plain ids reach a command line), restore_plan (R_RESUME/R_RELAUNCH/R_SKIP), build_command and fresh_command (the resume or fresh argv), json_field (hook payload "session_id"), project_dir_name/transcript_path/uuid_from_jsonl/last_model (find and read the Claude Code transcript), hooks_json (settings that name the binary, a status line), plus put_dec/put_bytes used by other specs.
- Comments only: the file reads and writes, the SessionStart hook transport, the directory scan on start (host loop "only reads and writes bytes").
- Host needs: file I/O, rename, directory listing, the hook process. No @extern in the spec.
- Conversation model: a SESSION record (agent, state, sid, model, cwd) only. It locates the CLI's own transcript jsonl but holds no message, so history is the CLI's, not T27 OS's.

### kernel.t27 (698 lines, T27osKernel, 13 tests, sealed, used by NONE of the five wired files)
- State: the supervision tree as data (ROOT, QUEEN, LANES, SERVICES with 5 service children, one transient node per lane), the resource formula (cap from mem, free, ncpu, load, battery), shedding order, mailbox caps per kind, and the view model (gauges, indented rows, nv_pack word).
- Executable: node_* tree shape, lane_decision/service_decision/lane_restart_wait, agent_cap/cap_reason/spawn_hold/may_spawn/kn_lane_gate, lanes_to_shed/shed_lane/resume_ok, mbox_cap/kernel_deliver/kernel_lane, nv_pack*/gauge_fill/kv_* layout, request_default_due. Restart law is imported from queen/actors.t27, not restated.
- Comments only: the sysctl, getloadavg, host_statistics64, IOKit and proc_pidinfo reads (stated "NOTHING here calls them"); no number is measured on this Mac.
- Host needs: those macOS reads on a KERNEL_TICK_SECONDS cadence, and a window page that draws the view. No @extern here; the kernel is not referenced by the window, so the tree is a model, not a running supervisor.
- Conversation model: NONE. Tags are K_STOP, K_ESCALATE, K_TASK (task offer); a "lane" is a process slot, not a conversation.

### agents.t27 (155 lines, T27osAgents, 3 tests, sealed, used by glass_window and queen_chats)
- State: 9 comb cells (Claude Code, Codex, Gemini, OpenCode, Queen at the centre cell 4, Crush, Cursor, Aider, Copilot), their launch bins, resume kinds, and 5 session states (STOPPED, WORKING, BLOCKED, IDLE, DONE).
- Executable: on_event (7 events: output, prompt, quiet, exit ok/fail, reboot, input), needs_you, restore_action, restored_state, cell_row/cell_col.
- Comments only: the "host side still missing" list (terminal owner, session store, state reader); most of it has since appeared in pty_server and session_store.
- Host needs: the terminal owner that raises the events. No @extern.
- Conversation model: NONE. It is a per-CLI state machine driven by terminal events; there is no DSH or Queen-chat agent entry (the Queen uses the AGENT_QUEEN cell with RESUME_UNKNOWN, so she is relaunched fresh).

### router_serve.t27 (174 lines, T27osRouterServe, 5 tests, NO seal, wired none)
- State: none persistent. It is the HTTP surface of the stage-2 router: parse the request line, serve only `POST /v1/messages`, answer 200 head, 404, or 502 (bodyless).
- Executable: request_method, request_path, line_is_served, ok_head, refuse_404, refuse_502, serve_answer (head plus body). Imports put_bytes/put_dec from session_store and `serves` from router.
- Comments only: the socket (router_sock.t27), the route walk and answer actions (router.t27), the request translation (provider/transform.t27). One connection at a time, no fork.
- Host needs: a socket accept loop; none declared. Not wired into any of the five files and unsealed.
- Conversation model: it carries a model call body as opaque bytes. No message type, no stream framing here.

### pty_proto.t27 (432 lines, T27osPtyProto, 11 tests, sealed, wired none directly)
- State: the wire frame `[op u8][pane u8][len u16 LE][payload]` (HEADER 4, MAX_PAYLOAD checked) and each pane's scrollback ring (RING_CAP bytes, a running u64 count; the byte with count n lives at n % RING_CAP).
- Executable: put_frame, frame_need, frame_complete, is_request/is_reply, ring_window (a stale cursor is clamped), ring_append, ring_copy, data_chunk, put_list_entry, spawn_ok, resize_ok, put_winsize, socket_path. Ops: LIST, SPAWN, ATTACH, DETACH, KEYS, RESIZE, READ, KILL, HOOK, SETKEY; replies OK, ERR, DATA, LISTING.
- Comments only: the unix socket `~/.t27os/pty.sock` and the endpoints (pty_server, pty_client both `use` this module).
- Host needs: socket I/O, in pty_server (declares 2 externs, sup_getpid and sup_pidpath) and pty_sys glue.
- Conversation model: byte stream only. OP_KEYS writes bytes, OP_READ polls from a cursor, OP_HOOK carries an event name plus JSON. The cursor survives a detach but the ring is bounded, so old output is lost.

### keys.t27 (3981 lines, T27osKeys, 115 tests, sealed, used by glass_window; header and pub fn outline read, not every fn)
- State: providers and their env vars, per-key status, the circuit breaker (circuit_fails, circuit_open), LRU plus cooldown rotation (rotate_pick, rotate_pick_except), profiles with tiered models (tier_model, model_for_profile, profile_base_url, pick_profile, next_profile), the child environment of a new pane (spawn_mask, env_stripped, claude_strips), and the KEYS tab layout and clicks (ky_*).
- Executable: status_after_probe, mask_key, trim_key, probe_url/probe_header_*, the rotation and profile picks, keychain_label. Tests use fake values only.
- Comments only: the macOS Keychain access (service ai.t27.os, via /usr/bin/security) and the probe call. The long MAP block describes the app.t27.ai account/keys and says a client PROXY to spend a stored key is MISSING.
- Host needs: Keychain through /usr/bin/security, the probe HTTP call, the child env (keys_host.t27, pty_server.t27). No @extern in this file.
- Conversation model: NONE. Routing here is per pane at spawn (stage 1); per-request routing is router.t27 and ladder.t27, both unwired.

### dsh_head_election.t27 (88 lines, T27OsDshHeadElection, 6 tests, no seal, wired none)
- State: none; two inputs (zai and ollama consecutive-probe streaks) and the current head (HEAD_ZAI 0 or HEAD_OLLAMA 1).
- Executable: elect (E_FLIP only when the challenger has >= 3 wins in a row and the head has 0; unknown head returns E_KEEP) and other. PROBE_INTERVAL_S 60, PROBE_TOKENS 100.
- Comments only: the probing itself (one turn-sized probe per rung per minute, thinking off on zai).
- Host needs: a probe sender and a place to store the streaks and the head; nothing declared.
- Conversation model: NONE. It is a rung hysteresis law for DSH's default model.

### dsh_mcp_bridge.t27 (origin/master only, 381 lines, T27osDshMcpBridge, 23 tests, no seal, wired none)
- State: none. A table of 6 tool ids (dsh_task_create, prompt, cancel, list, resume, close) mapped to ACP methods (session/new, prompt, cancel, list, resume, close; cancel is the one notification). session/load, fork and set_mode are deliberately absent.
- Executable: why/decide (HOLD, REFUSE or MAP in the order unknown tool, issue 0, out of scope for create, no go), needs_go (cancel and close never wait for the go), mapped_method, acp_requests, decide_path via dsh_scope.
- Comments only (its own list of 8): MCP initialize, spawning the dsh ACP profile, stdio JSON-RPC framing, reading the go file, deriving issue and path from a call, registering the server (owner's word), logging. The spec says "none of the integration exists today".
- Host needs: all of the above; no @extern. No DSH MCP server exists.
- Conversation model: a SESSION-level tool table only (create, prompt, cancel, list, resume, close). It carries no prompt text, no tool-call payload and no result.

### dsh_scope.t27 (origin/master only, 290 lines, T27osDshScope, 21 tests, no seal, wired none; read to line 60 plus fn and test outline)
- State: none. Four facts (has issue, path inside a T27 OS root, not a hand-written foreign file, a visual-core impact declared) decide a verdict.
- Executable: why, verdict_of_reason, decide, write_allowed, counts_toward_progress, path_in_root, path_is_foreign (delegates to policy::own_language classify), impact_declared, decide_path. AUDIT is the default and never blocks; BLOCK is the owner's flip; a missing impact is V_INTERNAL_ONLY in both modes.
- Comments only: the PreToolUse host that would call it and the task-id export DSH does not provide yet.
- Host needs: a hook host; no @extern. It gates WRITES; it is not a read-only tool allowlist.
- Conversation model: NONE.

## Section 3: Capability map (providing module, or GAP)

Status words: HAVE = a pure tested decision exists in scope; PARTIAL = part of it exists; GAP = nothing in scope models it. "Wired" follows the Section 1 definition (direct `use` by the five files). A grep of `specs/t27os`, `specs/queen/{tasks,dashboard,actors}.t27` for tool_use, tool_call, tool_result, allowlist, reconcile and idempot found no hit.

| # | Capability | Providing module | Status and note |
|---|---|---|---|
| 1 | Conversation and message model | GAP | Nearest: queen_chats (input line + CLI pane as transcript) and session_store (session record). No message, turn, role or message id exists. |
| 2 | Send with ack | PARTIAL: pty_proto, queen_chats | rc_payload frames text plus CR into OP_KEYS; the frame protocol has RE_OK and RE_ERR replies. That is a transport ack at best; I did not verify an ack for OP_KEYS. No turn-level or delivered-to-agent ack. |
| 3 | Streaming output | PARTIAL: pty_proto | OP_READ with a u64 cursor over a bounded ring (ring_window, ring_append, data_chunk). Raw bytes, polled, lossy past RING_CAP. Structured stream parsing is specs/provider/stream.t27 (outside this scope, named by router.t27). |
| 4 | Tool call and structured result | GAP | dsh_mcp_bridge (origin/master) is a table of 6 session tools, not a call or result payload. The only structured result is fanout res_pack (lane result: status, verdicts, files, plus, minus). |
| 5 | Cancel, error, retry, reconcile | PARTIAL: actors, kernel, fanout, agents, dsh_mcp_bridge | Cancel: dsh_mcp_bridge T_CANCEL (origin/master), actors stop_signal and hung_turn_signal, kernel K_STOP. Error: agents on_event E_EXIT_FAIL, actors on_child_exit, kernel lane_decision. Retry: actors backoff_seconds and with_backoff, fanout tries_after_retry and join_retry_which. Reconcile and idempotency: GAP. |
| 6 | History and session restore | PARTIAL: session_store, agents, window_keep | Restore of a session: HAVE (restore_plan, build_command, hooks_json, transcript_path; window_keep keeps spaces and panes). Message history: GAP (it stays in the CLI's own jsonl; the pty ring is volatile). |
| 7 | Task, attempt, executor | PARTIAL: queen/tasks, queen/dashboard, agents, session_store, pty_logic | Task kinds (issue, review, job) and 7 states: tasks.t27. Executor: agents cells plus build_command argv. Attempt counts: only fanout `tries`. No spec ties a task to a session record. |
| 8 | Recursive delegation | fanout (wired none), queen/actors call_admit and chain | spawn_admit takes parent_depth, decompose splits a Boundary into disjoint path sets, tree_order gives the tree. kernel (not wired) holds the lane tree. Neither is reachable from the window. |
| 9 | Result return and accept | PARTIAL: fanout | K_RESULT, result_delivery, child_end, join_decide, fanin_land, owner_default. The accept step toward the OWNER in chat is GAP (agent_report on origin/master is a report row model, not an accept action). |
| 10 | Shared event log for chat, tree and graph | PARTIAL: live_tap, queen_graph_plane, lanes_view (+ t27f/work_graph, pty_live_graph and graph_k_hop on origin/master) | Lane-table transitions become work-graph events and fold into per-kind counts. No chat or message event kind exists; chat is outside the log. |
| 11 | Read-only tool allowlist | GAP | dsh_scope (origin/master) gates WRITES by issue, root, foreign file and impact. No allowlist of read-only tools exists. |
| 12 | Provider and model routing | keys, router, router_serve, ladder, dsh_head_election, provider_router, router_actor, claw/model_route | keys: per-pane profile and model at spawn, rotation, circuit breaker (wired by glass_window). Per-request walk and HTTP surface: router and router_serve (router_serve unsealed, both unwired). Rung law: ladder. DSH head flip: dsh_head_election. Role to tier: claw/model_route. Only keys is reachable from the window. |

GAP list (short): conversation/message model; turn-level ack; tool-call payload and structured tool result; reconcile and idempotency; message history store; accept-in-chat; chat events in the shared log; read-only tool allowlist. Host gaps already named in the specs: DSH MCP server and ACP stdio framing (dsh_mcp_bridge), go-file read and DSH-alive probe (godmode_tick, general_conductor_tick), PreToolUse host (dsh_scope), the kernel's macOS resource reads.

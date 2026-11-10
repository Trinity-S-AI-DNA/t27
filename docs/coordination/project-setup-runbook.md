# Project setup runbook: T27 Delivery (ORDER T27_GITHUB_AUTOMATION_V1, Waves 2-5, 8)

Ready-to-run steps for the moment the owner grants the `project` scope.
Owner runs everything below as an authenticated `gh` session (gHashTag).
ASCII-only; every command is copy-pasteable. Do not invent API surface that
is not listed here.

## Wave 0 - Prerequisite

The token lacks Projects v2 scope. Owner runs this interactively (it prompts
in a browser):

```
gh auth refresh -h github.com -s project
```

## Wave 2 - Project and fields

### 2.1 Create the project

```
gh project create --owner Trinity-S-AI-DNA --title "T27 Delivery" --format json
```

Echo and save the project number from the JSON output:

```
PROJECT_NUMBER=<number from output>
echo "PROJECT_NUMBER=$PROJECT_NUMBER"
```

GraphQL equivalent (same result, use one or the other):

```
gh api graphql -f query='
mutation {
  createProjectV2(input: {ownerId: "<org node id>", title: "T27 Delivery"}) {
    projectV2 { number id }
  }
}'
```

(Obtain the org node id first with
`gh api graphql -f query='{ organization(login: "Trinity-S-AI-DNA") { id } }'`
if you go the GraphQL route.)

### 2.2 Create fields, exactly in this order

The built-in `Status` field already exists with default options; delete-free
path: recreate Status explicitly only if its options differ. To guarantee
exact options, create Status as below and ignore/repurpose the built-in per
owner preference -- no duplicate fields may remain.

```
gh project field-create $PROJECT_NUMBER --owner Trinity-S-AI-DNA \
  --name "Status" --data-type SINGLE_SELECT \
  --single-select-options "Triage,Todo,In progress,In review,Blocked,Done"

gh project field-create $PROJECT_NUMBER --owner Trinity-S-AI-DNA \
  --name "Priority" --data-type SINGLE_SELECT \
  --single-select-options "P0,P1,P2,P3"

gh project field-create $PROJECT_NUMBER --owner Trinity-S-AI-DNA \
  --name "Area" --data-type SINGLE_SELECT \
  --single-select-options "Compiler,T27OS,FPGA,Docs,Infra,Community"
```

Optional number fields -- create only if used:

```
gh project field-create $PROJECT_NUMBER --owner Trinity-S-AI-DNA \
  --name "Estimate" --data-type NUMBER

gh project field-create $PROJECT_NUMBER --owner Trinity-S-AI-DNA \
  --name "Target" --data-type NUMBER
```

GraphQL alternative for a single-select field (exact mutation):

```
gh api graphql -f query='
mutation {
  createProjectV2Field(input: {
    projectId: "<project node id>",
    dataType: SINGLE_SELECT,
    name: "Priority",
    singleSelectOptions: [
      {name: "P0", color: RED},
      {name: "P1", color: ORANGE},
      {name: "P2", color: YELLOW},
      {name: "P3", color: GRAY}
    ]
  }) { projectV2Field { ... on ProjectV2SingleSelectField { id name } } }
}'
```

Run `gh project field-list $PROJECT_NUMBER --owner Trinity-S-AI-DNA` after
this wave: each field name must appear exactly once.

## Wave 3 - Saved views (UI-only)

Projects v2 has no API for creating saved views. The owner creates all 8 in
the project UI (Trinity-S-AI-DNA -> T27 Delivery), one per tab, pasting the
filter text into the view filter box:

| # | View name       | Filter to paste                                        |
|---|-----------------|--------------------------------------------------------|
| 1 | Triage inbox    | `status:Triage`                                        |
| 2 | Active sprint   | `milestone:"M0_PROMOTION_KERNEL_SPRINT"`               |
| 3 | Blocked         | `status:Blocked`                                       |
| 4 | Review queue    | `status:"In review"`                                   |
| 5 | Compiler        | `area:Compiler`                                        |
| 6 | T27OS           | `area:T27OS`                                           |
| 7 | FPGA            | `area:FPGA`                                            |
| 8 | Docs + community| `area:Docs,area:Community`                             |

If a filter errors in the UI, verify the field names match Wave 2 exactly
(spelling and case).

## Wave 4 - Auto-add workflow (UI-only)

Project -> Workflows -> "Auto-add to project": filter
`repo:Trinity-S-AI-DNA/t27 is:open`, enable. UI-only (no API). Not
retroactive: issues/PRs that already exist must be added manually once.

## Wave 5 - Automation secrets (after the workflow file lands)

1. Create a fine-grained PAT: repository access = Trinity-S-AI-DNA/t27 only;
   permission = Projects (v2) write; nothing else.
2. Save it as the repository secret:

```
gh secret set GH_PROJECT_TOKEN --repo Trinity-S-AI-DNA/t27
```

3. If the project number from Wave 2 is not 1, change it in
   `.github/workflows/project-automation.yml` -- the number is inlined as
   `-F n=1` in the three GraphQL calls of the board step (no PROJECT_NUMBER
   env var), so edit those three occurrences.

## Wave 8 - Verify

```
gh project list --owner Trinity-S-AI-DNA
gh project view $PROJECT_NUMBER --owner Trinity-S-AI-DNA --format json
gh project field-list $PROJECT_NUMBER --owner Trinity-S-AI-DNA
```

Checks: project titled "T27 Delivery" is listed; the JSON shows the expected
number of fields (Status, Priority, Area, plus Estimate/Target if created);
no duplicated field names.

## Checklist

- [ ] OWNER: `gh auth refresh -h github.com -s project`
- [ ] SCRIPT: `gh project create --owner Trinity-S-AI-DNA --title "T27 Delivery" --format json`; capture number
- [ ] SCRIPT: create Status field (Triage/Todo/In progress/In review/Blocked/Done), exactly once
- [ ] SCRIPT: create Priority field (P0/P1/P2/P3)
- [ ] SCRIPT: create Area field (Compiler/T27OS/FPGA/Docs/Infra/Community)
- [ ] SCRIPT: create Estimate (NUMBER) -- only if used
- [ ] SCRIPT: create Target (NUMBER) -- only if used
- [ ] SCRIPT: `gh project field-list` -- no duplicates
- [ ] OWNER: create 8 saved views with the filter table above (UI-only)
- [ ] OWNER: enable Auto-add workflow, filter `repo:Trinity-S-AI-DNA/t27 is:open` (UI-only, not retroactive)
- [ ] OWNER: fine-grained PAT (Projects v2 write, this repo only)
- [ ] SCRIPT: `gh secret set GH_PROJECT_TOKEN --repo Trinity-S-AI-DNA/t27`
- [ ] SCRIPT: set the project number in .github/workflows/project-automation.yml if not 1 (three `-F n=1` occurrences)
- [ ] SCRIPT: Wave 8 verify commands pass

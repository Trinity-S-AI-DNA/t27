# Course Factory skill

**Owner, 2026-10-11**: This skill contains the recipe and traps for building t27.ai courses. The Queen uses this to create new courses that tell part of the t27 project story.

## Overview

The course factory turns t27 specs into engaging educational content. Each course has 27 lessons, one widget per lesson, and tells part of the story: t27, Trinity, the ternary machine, the FPGA work, and the hive.

## The recipe: One course, three pieces of work

### 1. Lesson logic in t27
**Purpose**: Create the computational foundation for course widgets

**Process**:
- Analyze the course story and identify required specs
- Create child issues with `## Boundary` for any missing specs
- Ensure all specs exist and run green (`t27c parse` && `t27c typecheck`)
- Verify widget logic with `widget-logic.mjs --check`

**Deliverable**: Complete set of specs that power the course widgets

**Traps to avoid**:
- ❌ Don't build courses on specs that don't run green
- ❌ Don't use numbers not backed by the specs or repository
- ❌ Don't skip the `widget-logic.mjs --check` verification

### 2. The course in gHashTag/trinity
**Purpose**: Build the complete course experience with lessons and widgets

**Process**:
- Create 27 lessons in EN and RU
- Each lesson opens with a curious question
- Each lesson has one live, colourful widget
- Widget numbers come from compiled t27 spec (WebAssembly)
- Write a blog post about the course
- Update the course catalog (`apps/website/specs/course/courses.t27`)

**Roles**:
- **Coder bee**: Builds the course content and widgets
- **Reviewer bee**: Reviews and merges the work

**Deliverable**: Complete course with all lessons, widgets, and blog post

**Traps to avoid**:
- ❌ Don't use superlatives ("the first", "the only")
- ❌ Don't create lessons without widgets
- ❌ Don't skip RU version (must match EN numbers)
- ❌ Don't forget the blog post

### 3. Proof it is live
**Purpose**: Verify the course is accessible and working

**Process**:
- Confirm files appear in apex repo (gHashTag.github.io)
- Verify publish run is green
- Test all widgets are functional
- Update round report with merged PR names

**Deliverable**: Confirmed live course with working widgets

**Traps to avoid**:
- ❌ Don't assume deployment is automatic
- ❌ Don't skip widget functionality testing
- ❌ Don't forget to update the round report

## Course queue management

### Current queue order
The Queen takes courses in this order. Reordering requires a written reason in the round report.

| # | Course | Status | Next step |
|---|--------|--------|-----------|
| 10 | Neural networks in t27 | Queue | Verify specs/ml and specs/nn |
| 11 | The Queen and the hive | Queue | Verify specs/queen, specs/automation, specs/agents |
| 12 | One spec, many targets | Queue | Verify specs/compiler |
| 13 | The t27a ternary processor | Queue | Verify specs/isa, specs/vm |

### Queue management rules
- **Never skip ahead**: Courses must be built in order
- **Spec-first**: All widget specs must run green before starting
- **Quality over speed**: 27 complete lessons > rushed content
- **Story continuity**: Each course builds on the previous ones

## Quality gates

### Spec verification
```bash
# All widget specs must pass
t27c parse <spec-file>
t27c typecheck <spec-file>

# Widget logic must check
widget-logic.mjs --check
```

### Content verification
- [ ] 27 lessons in EN
- [ ] 27 lessons in RU (same numbers)
- [ ] 27 working widgets
- [ ] Blog post written
- [ ] Course catalog updated

### Live verification
- [ ] Files in apex repo
- [ ] Publish run green
- [ ] Widgets functional
- [ ] Round report updated

## The traps: What to avoid

### Recipe traps
- **Trap 1**: Building courses on untested specs
  - **Avoid**: Always run `t27c parse` && `t27c typecheck` on all specs
- **Trap 2**: Inconsistent numbers between EN and RU
  - **Avoid**: Use the same computed numbers in both languages
- **Trap 3**: Missing widgets
  - **Avoid**: Every lesson must have exactly one working widget
- **Trap 4**: Superlatives and unsupported claims
  - **Avoid**: Only use numbers backed by specs and repository

### Process traps
- **Trap 1**: Skipping the queue order
  - **Avoid**: Build courses in sequence, document any reordering
- **Trap 2**: Rushing quality assurance
  - **Avoid**: Test all widgets and verify deployment
- **Trap 3**: Forgetting the blog post
  - **Avoid**: Blog post is required for course completion
- **Trap 4**: Incomplete round reporting
  - **Avoid**: Name all merged PRs in the round report

### Boundary traps
- **Trap 1**: Working outside authorized paths
  - **Avoid**: Only edit: `apps/website/specs/course/courses.t27`, `.claude/agents/queen-course-keeper.md`, `.claude/skills/course-factory/SKILL.md`
- **Trap 2**: Pushing to shared repositories
  - **Avoid**: Use the Queen's `queen-N` branches for work
- **Trap 3**: Skipping compiler checks
  - **Avoid**: Run t27c on all .t27 files before committing

## Success metrics

For each course delivered:
- ✅ All 27 lessons live in EN and RU
- ✅ Every widget imports from compiled spec logic
- ✅ Blog post published
- ✅ Course catalog updated
- ✅ Apex repo files deployed
- ✅ Publish run successful
- ✅ Round report with merged PRs

## Handoffs

- **To Queen**: Clear specifications and queue status
- **From Queen**: Completed course verification
- **To owner**: Progress reports with next steps

## Continuous improvement

After each course, review:
- What worked well in the recipe?
- What traps were caught?
- What can be improved for the next course?
- Update this skill with lessons learned
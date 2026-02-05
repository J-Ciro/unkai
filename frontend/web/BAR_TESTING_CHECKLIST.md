# Bar Management System - Testing Checklist

Use this checklist to verify all bar management features are working correctly.

---

## Pre-Testing Setup

### Environment Verification

- [ ] **Rust compiler installed:** `rustc --version` ≥ 1.56
- [ ] **Node.js installed:** `node --version` ≥ 16.0
- [ ] **npm installed:** `npm --version` ≥ 8.0
- [ ] **Workspace clean:** No pending changes in git

### Build Backend

```bash
cd unkai/
cargo build --release
```

- [ ] **Build succeeds** without errors
- [ ] **No warnings** in bar.rs or bar_discovery.rs
- [ ] **Binary created:** `target/release/yasb.exe` (Windows) or `target/release/yasb` (Unix)

### Verify Frontend Types

```bash
cd frontend/web/
npm run typecheck  # or tsc --noEmit
```

- [ ] **TypeScript compiles** without errors
- [ ] **No type errors** in types/bar.ts
- [ ] **No type errors** in providers/bar.ts
- [ ] **No type errors** in hooks/useBarManager.ts
- [ ] **No type errors** in components/BarManager.tsx

---

## Phase 1: Discovery System

### Test 1.1: Empty Bars Directory

**Goal:** Verify system handles no bars gracefully

```bash
rm -rf ~/.unkai/bars/*  # Clear all bars
```

**Steps:**
1. Launch YASB
2. Open Settings → Bar Management

**Expected Results:**
- [ ] No errors in console
- [ ] UI shows "No bars available"
- [ ] Create Bar form still accessible
- [ ] No crashes

---

### Test 1.2: Create First Bar

**Goal:** Verify bar scaffold generation

**Steps:**
1. Settings → Bar Management → Create Bar tab
2. Enter bar name: "Test Bar"
3. Click "Create"

**Expected Results:**
- [ ] Success message appears
- [ ] Directory created: `~/.unkai/bars/test-bar/`
- [ ] Files created:
  - [ ] `barpack.json`
  - [ ] `package.json`
  - [ ] `vite.config.ts`
  - [ ] `index.html`
  - [ ] `src/main.tsx`
  - [ ] `src/App.tsx`
  - [ ] `src/index.css`
- [ ] Bar appears in "All Bars" tab
- [ ] barpack.json has correct structure

**Verify barpack.json:**
```bash
cat ~/.unkai/bars/test-bar/barpack.json
```
- [ ] Contains: name, version, description, entry, window

---

### Test 1.3: Build Created Bar

**Goal:** Verify scaffold is buildable

**Steps:**
```bash
cd ~/.unkai/bars/test-bar/
npm install
npm run build
```

**Expected Results:**
- [ ] `npm install` completes successfully
- [ ] Dependencies installed: react, react-dom, vite, typescript
- [ ] `npm run build` compiles without errors
- [ ] `dist/` directory created
- [ ] `dist/index.html` exists
- [ ] `dist/assets/` contains bundled JS/CSS

**Verify build:**
```bash
ls -la ~/.unkai/bars/test-bar/dist/
```
- [ ] index.html present (entry point)
- [ ] assets/*.js present
- [ ] assets/*.css present

---

### Test 1.4: Discover Multiple Bars

**Goal:** Verify discovery finds all bars

**Steps:**
1. Create 2 more bars: "System Monitor", "Workspace Bar"
2. Build both bars (npm install && npm run build)
3. Refresh YASB settings

**Expected Results:**
- [ ] All 3 bars appear in "All Bars" list
- [ ] Each shows correct name, version, description
- [ ] Grid layout displays properly
- [ ] No duplicate entries

---

## Phase 2: Selection System

### Test 2.1: Select Bar via Dropdown

**Goal:** Verify bar selection persists

**Steps:**
1. Settings → Bar Management → Select Bar tab
2. Choose "Test Bar" from dropdown
3. Close settings
4. Reopen settings

**Expected Results:**
- [ ] Dropdown shows "Test Bar" selected
- [ ] Bar info displayed (name, version, description)
- [ ] Active badge shown on bar card in "All Bars" tab

**Verify config:**
```bash
cat ~/.unkai/config.json
```
- [ ] Contains: `"selected_bar": "test-bar"`

---

### Test 2.2: Select Bar via Cards

**Goal:** Verify click-to-select from grid

**Steps:**
1. Settings → Bar Management → All Bars tab
2. Click on "System Monitor" card

**Expected Results:**
- [ ] Card gets "active" styling
- [ ] Other cards lose "active" styling
- [ ] Selected bar updates in dropdown (Select Bar tab)
- [ ] Config updated

**Verify:**
```bash
cat ~/.unkai/config.json
```
- [ ] Contains: `"selected_bar": "system-monitor"`

---

### Test 2.3: Persistence Across Restarts

**Goal:** Verify selection survives app restart

**Steps:**
1. Select "Workspace Bar"
2. Close YASB completely
3. Restart YASB
4. Open Settings → Bar Management

**Expected Results:**
- [ ] "Workspace Bar" still selected in dropdown
- [ ] Active badge on correct card
- [ ] No reversion to previous selection

---

## Phase 3: IPC Commands

### Test 3.1: discover_available_bars

**Goal:** Verify IPC command returns bars

**Steps:**
Open browser DevTools console (F12) in YASB window:
```javascript
const { invoke } = window.__TAURI__.tauri
const result = await invoke('discover_available_bars')
console.log(result)
```

**Expected Output:**
```javascript
{
  data: [
    { id: "test-bar", name: "Test Bar", ... },
    { id: "system-monitor", ... },
    { id: "workspace-bar", ... }
  ],
  error: null,
  code: 0
}
```

- [ ] `data` is array of 3 bars
- [ ] Each bar has: id, name, description, version, path, active
- [ ] One bar has `active: true`
- [ ] `error` is null
- [ ] `code` is 0

---

### Test 3.2: select_bar

**Goal:** Verify selection IPC command works

**Steps:**
In DevTools console:
```javascript
const result = await invoke('select_bar', { barId: 'test-bar' })
console.log(result)
```

**Expected Output:**
```javascript
{
  data: "Bar 'test-bar' selected successfully",
  error: null,
  code: 0
}
```

**Verify:**
```bash
cat ~/.unkai/config.json | grep selected_bar
```
- [ ] Shows: `"selected_bar": "test-bar"`

---

### Test 3.3: get_selected_bar

**Goal:** Verify current selection retrieval

**Steps:**
In DevTools console:
```javascript
const result = await invoke('get_selected_bar')
console.log(result)
```

**Expected Output:**
```javascript
{
  data: {
    id: "test-bar",
    name: "Test Bar",
    description: "...",
    version: "1.0.0",
    path: "C:\\Users\\skybl\\.unkai\\bars\\test-bar",
    active: true
  },
  error: null,
  code: 0
}
```

- [ ] Returns currently selected bar
- [ ] Has all required fields
- [ ] `active` is true

---

### Test 3.4: create_bar_scaffold

**Goal:** Verify creation IPC command

**Steps:**
In DevTools console:
```javascript
const result = await invoke('create_bar_scaffold', { barName: 'IPC Test Bar' })
console.log(result)
```

**Expected Output:**
```javascript
{
  data: "Bar created at: ~/.unkai/bars/ipc-test-bar",
  error: null,
  code: 0
}
```

**Verify:**
```bash
ls ~/.unkai/bars/ipc-test-bar/
```
- [ ] Directory exists
- [ ] barpack.json created
- [ ] All scaffold files present

---

### Test 3.5: get_bar_manifest

**Goal:** Verify manifest retrieval

**Steps:**
In DevTools console:
```javascript
const result = await invoke('get_bar_manifest', { barId: 'test-bar' })
console.log(result)
```

**Expected Output:**
```javascript
{
  data: {
    name: "Test Bar",
    version: "1.0.0",
    description: "...",
    entry: "dist/index.html",
    window: {
      width: 1920,
      height: 40,
      ...
    }
  },
  error: null,
  code: 0
}
```

- [ ] Returns full manifest
- [ ] Has window config
- [ ] All fields match barpack.json

---

## Phase 4: Frontend Hooks

### Test 4.1: useBarManager

**Goal:** Verify main hook returns state and methods

**Steps:**
Create test component:
```tsx
import { useBarManager } from '@unkai/sdk'

function Test() {
  const bar = useBarManager()
  console.log('State:', bar)
  return <pre>{JSON.stringify(bar, null, 2)}</pre>
}
```

**Expected Console Output:**
```javascript
{
  availableBars: [...],  // array of bars
  selectedBar: {...},    // current bar
  isLoading: false,
  error: null,
  discoverBars: [Function],
  selectBar: [Function],
  getSelectedBar: [Function],
  createBar: [Function]
}
```

- [ ] All state fields present
- [ ] All methods callable
- [ ] No errors in console

---

### Test 4.2: useAvailableBars

**Goal:** Verify bars list hook

**Steps:**
```tsx
import { useAvailableBars } from '@unkai/sdk'

function Test() {
  const { bars, isLoading, error, refresh } = useAvailableBars()
  
  return (
    <div>
      <h1>Total: {bars.length}</h1>
      <button onClick={refresh}>Refresh</button>
      {bars.map(b => <div key={b.id}>{b.name}</div>)}
    </div>
  )
}
```

**Expected Results:**
- [ ] Bars list populates on mount
- [ ] Click "Refresh" updates list
- [ ] No duplicate bars
- [ ] isLoading toggles correctly

---

### Test 4.3: useSelectedBar

**Goal:** Verify selection hook

**Steps:**
```tsx
import { useSelectedBar } from '@unkai/sdk'

function Test() {
  const { bar, isLoading, selectBar } = useSelectedBar()
  
  return (
    <div>
      <h1>Active: {bar?.name}</h1>
      <button onClick={() => selectBar('other-bar')}>Switch</button>
    </div>
  )
}
```

**Expected Results:**
- [ ] Current bar loads on mount
- [ ] Click "Switch" changes bar
- [ ] Component re-renders with new bar
- [ ] Config persists

---

## Phase 5: React Components

### Test 5.1: BarSelector Component

**Goal:** Verify dropdown selector

**Steps:**
```tsx
import { BarSelector } from '@unkai/sdk'

<BarSelector />
```

**Expected Results:**
- [ ] Dropdown shows all bars
- [ ] Current selection highlighted
- [ ] Change selection updates bar
- [ ] Shows bar info below dropdown

---

### Test 5.2: BarsList Component

**Goal:** Verify grid view

**Steps:**
```tsx
import { BarsList } from '@unkai/sdk'

<BarsList />
```

**Expected Results:**
- [ ] Cards displayed in grid
- [ ] Each card shows: name, version, description, id
- [ ] Active bar has special styling
- [ ] Click card to select works
- [ ] Refresh button works

---

### Test 5.3: CreateBarForm Component

**Goal:** Verify creation form

**Steps:**
```tsx
import { CreateBarForm } from '@unkai/sdk'

<CreateBarForm />
```

**Expected Results:**
- [ ] Input field for bar name
- [ ] Submit button
- [ ] Enter name → click Create → success message
- [ ] New bar appears in list
- [ ] Error shown if name invalid

---

### Test 5.4: BarSettingsPanel Component

**Goal:** Verify full settings UI

**Steps:**
```tsx
import { BarSettingsPanel } from '@unkai/sdk'

<BarSettingsPanel />
```

**Expected Results:**
- [ ] 3 tabs shown: "Select Bar", "All Bars", "Create Bar"
- [ ] Click tabs switches content
- [ ] Each tab shows correct component
- [ ] Active tab has underline
- [ ] CSS styling applied (dark theme)

---

### Test 5.5: QuickBarSelector Component

**Goal:** Verify compact selector

**Steps:**
```tsx
import { QuickBarSelector } from '@unkai/sdk'

<QuickBarSelector />
```

**Expected Results:**
- [ ] Button list of bars
- [ ] Active bar has checkmark
- [ ] Click button selects bar
- [ ] Compact styling

---

## Phase 6: End-to-End User Flows

### Flow 1: First-Time User

**Scenario:** User installs YASB for first time

**Steps:**
1. Delete all bars: `rm -rf ~/.unkai/bars/*`
2. Delete config: `rm ~/.unkai/config.json`
3. Start YASB
4. Right-click tray → Settings

**Expected:**
- [ ] Settings open without errors
- [ ] Bar Management section visible
- [ ] Shows "No bars available"
- [ ] Create Bar form accessible
- [ ] No crashes or warnings

**Create First Bar:**
5. Create Bar → Enter "My Bar" → Submit
6. Wait for success
7. cd to bar directory
8. Run npm install && npm run build
9. Refresh YASB → Select "My Bar"

**Expected:**
- [ ] Bar created successfully
- [ ] Build completes
- [ ] Bar appears in selector
- [ ] Selection persists

---

### Flow 2: Advanced User - Multiple Bars

**Scenario:** User manages multiple bars

**Steps:**
1. Create 5 bars: "Top", "Bottom", "Left", "Right", "Center"
2. Build all bars
3. Switch between them via dropdown
4. Switch via grid cards
5. Delete one bar from filesystem
6. Refresh bars list

**Expected:**
- [ ] All 5 bars created
- [ ] All 5 appear in UI
- [ ] Switching via dropdown works
- [ ] Switching via cards works
- [ ] Deleted bar removed from list after refresh
- [ ] No errors from missing bar

---

### Flow 3: Developer Workflow

**Scenario:** Developer creates custom bar

**Steps:**
1. Create bar: "Custom Dashboard"
2. cd to bar directory
3. npm install
4. npm run dev (Vite dev server)
5. Edit src/App.tsx (add widgets)
6. Save changes → see hot reload
7. npm run build
8. Select in YASB

**Expected:**
- [ ] Vite dev server starts (port 5173)
- [ ] Can access http://localhost:5173
- [ ] Hot module replacement works
- [ ] Changes reflected immediately
- [ ] Build succeeds after edits
- [ ] Bar runs in YASB with changes

---

### Flow 4: Tray Menu Integration

**Scenario:** User switches bars via tray menu

**Steps:**
1. Right-click YASB tray icon
2. See bar list in menu
3. Click different bar
4. Verify change

**Expected:**
- [ ] Tray menu shows QuickBarSelector
- [ ] Current bar has checkmark
- [ ] Click bar switches immediately
- [ ] Config saved

---

## Phase 7: Error Handling

### Error 7.1: Invalid barpack.json

**Goal:** Verify graceful handling of malformed manifest

**Steps:**
1. Create bar: "Bad Bar"
2. Edit barpack.json: Remove `"name"` field
3. Refresh bars

**Expected:**
- [ ] Bar skipped during discovery
- [ ] Error logged to console
- [ ] Other bars still load
- [ ] No crashes

---

### Error 7.2: Missing Entry File

**Goal:** Verify handling of missing dist/index.html

**Steps:**
1. Create bar: "No Dist"
2. Don't run npm run build (dist/ doesn't exist)
3. Try to activate bar

**Expected:**
- [ ] Error shown: "Entry file not found"
- [ ] Bar not activated
- [ ] Previous bar remains active

---

### Error 7.3: Invalid Bar Name

**Goal:** Verify input validation

**Steps:**
1. Create Bar → Enter name with: `/`, `\`, or empty
2. Submit

**Expected:**
- [ ] Error shown: "Invalid bar name"
- [ ] Bar not created
- [ ] Form stays open

---

### Error 7.4: Duplicate Bar Name

**Goal:** Verify duplicate detection

**Steps:**
1. Create bar: "Duplicate Test"
2. Create another bar: "Duplicate Test"

**Expected:**
- [ ] Either:
  - Prevents creation (error shown)
  - Or: Creates "duplicate-test-2" (numbered variant)
- [ ] No files overwritten

---

## Phase 8: Performance

### Perf 8.1: Large Bar Count

**Goal:** Verify scales to many bars

**Steps:**
1. Create 20 bars
2. Refresh bars list
3. Measure load time

**Expected:**
- [ ] All 20 bars discovered
- [ ] Discovery completes < 1 second
- [ ] UI renders smoothly (no lag)
- [ ] Grid layout responsive

---

### Perf 8.2: Frequent Refreshes

**Goal:** Verify no memory leaks from polling

**Steps:**
1. Click "Refresh" 50 times rapidly
2. Monitor memory usage

**Expected:**
- [ ] No errors
- [ ] Memory stable (no unbounded growth)
- [ ] UI remains responsive

---

## Phase 9: Cross-Platform

### Platform 9.1: Windows

**Path:** `C:\Users\skybl\.unkai\bars\`

- [ ] Bars directory created correctly
- [ ] barpack.json uses correct paths
- [ ] Vite build works
- [ ] YASB discovers bars

---

### Platform 9.2: macOS

**Path:** `~/.unkai/bars/`

- [ ] Bars directory created
- [ ] Permissions correct
- [ ] All features work

---

### Platform 9.3: Linux

**Path:** `~/.unkai/bars/`

- [ ] Bars directory created
- [ ] Permissions correct
- [ ] All features work

---

## Test Summary

### Critical Issues (Must Fix)
- [ ] None found

### High Priority (Should Fix)
- [ ] List here

### Low Priority (Nice to Have)
- [ ] List here

### Pass Criteria

**All tests pass if:**
- [ ] ✅ Backend compiles without errors
- [ ] ✅ Frontend types check successfully
- [ ] ✅ All IPC commands return expected results
- [ ] ✅ All React hooks work correctly
- [ ] ✅ All React components render without errors
- [ ] ✅ Bar creation scaffold generates working project
- [ ] ✅ Bar selection persists across restarts
- [ ] ✅ Error handling prevents crashes
- [ ] ✅ Performance acceptable (< 1s discovery for 20 bars)

---

## Next Steps After Testing

1. **If all tests pass:**
   - Tag release: `git tag v1.0.0-bar-system`
   - Update documentation
   - Create user guide video
   - Announce feature

2. **If tests fail:**
   - Document failures
   - Create bug fixes
   - Re-test
   - Iterate

3. **Future enhancements:**
   - Bar marketplace
   - Community bar sharing
   - Bar templates
   - Visual bar editor


# Test: should_not_move_over_side_effects

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
export const $promoteToRoot$ = (ref: SeenRef) => {
	const path = $getObjectPath$(ref) as string;
	// should stay before the push
	const idx = roots.length;
	roots.push(new BackRef(path));
	ref.$parent$ = null;
	ref.$index$ = idx;
	return idx;
};
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
export const $promoteToRoot$ = (ref)=>{
    const path = $getObjectPath$(ref);
    // should stay before the push
    const idx = roots.length;
    roots.push(new BackRef(path));
    ref.$parent$ = null;
    ref.$index$ = idx;
    return idx;
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

## Conventions Applied

*No optimizer conventions detected in output.*

**Key behavior:** `const idx = roots.length` must stay before `roots.push(...)` because push modifies length.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `$getObjectPath$` | test.js | (global) | 1 |

## Diagnostics

```json
[]
```

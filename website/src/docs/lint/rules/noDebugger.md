---
title: Lint Rule noDebugger
layout: layouts/rule.liquid
---

# noDebugger (since v0.7.0)

> This rule is recommended by Rome.

Disallow the use of `debugger`

## Examples

### Invalid

```jsx
debugger;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDebugger.js:1:1 <a href="https://rome.tools/docs/lint/rules/noDebugger">lint/correctness/noDebugger</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This is an unexpected use of the </span><span style="color: Tomato;"><strong>debugger</strong></span><span style="color: Tomato;"> statement.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noDebugger.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">b</span><span style="color: Tomato;">u</span><span style="color: Tomato;">g</span><span style="color: Tomato;">g</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove debugger statement</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">debugger;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span>
  
</code></pre>{% endraw %}

### Valid

```jsx
const test = { debugger: 1 };
test.debugger;
```


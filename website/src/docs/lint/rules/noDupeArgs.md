---
title: Lint Rule noDupeArgs
layout: layouts/rule.liquid
---

# noDupeArgs (since v0.9.0)

> This rule is recommended by Rome.

Disallow duplicate function arguments name.

## Examples

### Invalid

```jsx
var f = function(a, b, b) {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDupeArgs.js:1:24 <a href="https://rome.tools/docs/lint/rules/noDupeArgs">lint/correctness/noDupeArgs</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate argument name</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noDupeArgs.js:1:24
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var f = function(a, b, <span style="color: Tomato;">b</span>) {}
    <span style="color: rgb(38, 148, 255);">│</span>                        <span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
function b(a, b, b) {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDupeArgs.js:1:18 <a href="https://rome.tools/docs/lint/rules/noDupeArgs">lint/correctness/noDupeArgs</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate argument name</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noDupeArgs.js:1:18
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function b(a, b, <span style="color: Tomato;">b</span>) {}
    <span style="color: rgb(38, 148, 255);">│</span>                  <span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
function i(i, b, c) {}
var j = function (j, b, c) {};
function k({ k, b }, { c, d }) {}
function l([, l]) {}
function foo([[a, b], [c, d]]) {}
```


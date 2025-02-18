---
title: Lint Rule noImplicitBoolean
layout: layouts/rule.liquid
---

# noImplicitBoolean (since v0.7.0)

> This rule is recommended by Rome.

Disallow implicit `true` values on JSX boolean attributes

## Examples

### Invalid

```jsx
<input disabled />
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noImplicitBoolean.js:1:8 <a href="https://rome.tools/docs/lint/rules/noImplicitBoolean">lint/correctness/noImplicitBoolean</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use explicit boolean values for boolean JSX props.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImplicitBoolean.js:1:8
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;input <span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">s</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">d</span> /&gt;
    <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Add explicit `true` literal for this attribute</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;input disabled /&gt;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&lt;input disabled={true} /&gt;</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
<input disabled={false} />
```

```jsx
<input disabled={''} />
```

```jsx
<input disabled={0} />
```

```jsx
<input disabled={undefined} />
```

```jsx
<input disabled='false' />
```


---
title: Lint Rule useCamelCase
layout: layouts/rule.liquid
---

# useCamelCase (since v0.8.0)

Enforce camel case naming convention.

## Examples

### Invalid

```jsx
let snake_case;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useCamelCase.js:1:5 <a href="https://rome.tools/docs/lint/rules/useCamelCase">lint/nursery/useCamelCase</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Prefer variables names in camel case.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useCamelCase.js:1:5
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let <span style="color: Tomato;">s</span><span style="color: Tomato;">n</span><span style="color: Tomato;">a</span><span style="color: Tomato;">k</span><span style="color: Tomato;">e</span><span style="color: Tomato;">_</span><span style="color: Tomato;">c</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span>;
    <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Rename this symbol to camel case</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let snake_case;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let snakeCase;</span>
  
</code></pre>{% endraw %}

```jsx
let PascalCase;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useCamelCase.js:1:5 <a href="https://rome.tools/docs/lint/rules/useCamelCase">lint/nursery/useCamelCase</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Prefer variables names in camel case.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useCamelCase.js:1:5
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let <span style="color: Tomato;">P</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">c</span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">C</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span>;
    <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Rename this symbol to camel case</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let PascalCase;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let pascalCase;</span>
  
</code></pre>{% endraw %}

## Valid

```jsx
let camelCase;
```


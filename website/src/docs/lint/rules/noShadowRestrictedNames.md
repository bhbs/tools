---
title: Lint Rule noShadowRestrictedNames
layout: layouts/rule.liquid
---

# noShadowRestrictedNames (since v0.9.0)

> This rule is recommended by Rome.

Disallow identifiers from shadowing restricted names.

## Examples

### Invalid

```jsx
function NaN() {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noShadowRestrictedNames.js:1:10 <a href="https://rome.tools/docs/lint/rules/noShadowRestrictedNames">lint/correctness/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;NaN&quot; property.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noShadowRestrictedNames.js:1:10
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function <span style="color: Tomato;">N</span><span style="color: Tomato;">a</span><span style="color: Tomato;">N</span>() {}
    <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>{% endraw %}

```jsx
let Set;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noShadowRestrictedNames.js:1:5 <a href="https://rome.tools/docs/lint/rules/noShadowRestrictedNames">lint/correctness/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Set&quot; property.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noShadowRestrictedNames.js:1:5
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let <span style="color: Tomato;">S</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span>;
    <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>{% endraw %}

```jsx
try {	} catch(Object) {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noShadowRestrictedNames.js:1:15 <a href="https://rome.tools/docs/lint/rules/noShadowRestrictedNames">lint/correctness/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Object&quot; property.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noShadowRestrictedNames.js:1:15
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> try {   } catch(<span style="color: Tomato;">O</span><span style="color: Tomato;">b</span><span style="color: Tomato;">j</span><span style="color: Tomato;">e</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span>) {}
    <span style="color: rgb(38, 148, 255);">│</span>                 <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>{% endraw %}

```jsx
function Array() {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noShadowRestrictedNames.js:1:10 <a href="https://rome.tools/docs/lint/rules/noShadowRestrictedNames">lint/correctness/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Array&quot; property.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noShadowRestrictedNames.js:1:10
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function <span style="color: Tomato;">A</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">y</span>() {}
    <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>{% endraw %}

```jsx
function test(JSON) {console.log(JSON)}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noShadowRestrictedNames.js:1:15 <a href="https://rome.tools/docs/lint/rules/noShadowRestrictedNames">lint/correctness/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;JSON&quot; property.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noShadowRestrictedNames.js:1:15
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function test(<span style="color: Tomato;">J</span><span style="color: Tomato;">S</span><span style="color: Tomato;">O</span><span style="color: Tomato;">N</span>) {console.log(JSON)}
    <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>{% endraw %}


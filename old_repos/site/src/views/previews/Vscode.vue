<template>
<p class="c5"><span class="c2">import</span><span class="c0">&nbsp;{ computed, ref } </span><span class="c2">from</span><span class="c0">&nbsp;</span><span class="c6">"vue"</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c2">import</span><span class="c0">&nbsp;</span><span class="c2">type</span><span class="c0">&nbsp;{ Ref, WritableComputedRef, InjectionKey } </span><span class="c2">from</span><span class="c0">&nbsp;</span><span class="c6">"vue"</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c2">import</span><span class="c0">&nbsp;{ I9, N9 } </span><span class="c2">from</span><span class="c0">&nbsp;</span><span class="c6">"./helpers"</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c2">import</span><span class="c0">&nbsp;Color </span><span class="c2">from</span><span class="c0">&nbsp;</span><span class="c6">"color"</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c2">import</span><span class="c0">&nbsp;_ </span><span class="c2">from</span><span class="c0">&nbsp;</span><span class="c6">"lodash"</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c2">import</span><span class="c0">&nbsp;{ getData } </span><span class="c2">from</span><span class="c0">&nbsp;</span><span class="c6">"base9-builder"</span><span class="c0 c3">;</span></p>
<p class="c5 c9"><span class="c0 c3"></span></p>
<p class="c5"><span class="c2">export</span><span class="c0">&nbsp;</span><span class="c2">const</span><span class="c0">&nbsp;DEFAULT_PALETTE </span><span class="c2">=</span><span class="c0">&nbsp;</span><span class="c6">'16161D-E0E5C8-CEAA7A-A6B2C1-CBA8B3-8AB9B2-DBA595-93BC95-B0B47A'</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c2">export</span><span class="c0">&nbsp;</span><span class="c2">const</span><span class="c0">&nbsp;PALETTE_REGEX </span><span class="c2">=</span><span class="c6">&nbsp;/</span><span class="c2">^</span><span class="c10">(</span><span class="c0">[</span><span class="c4">0-9a-fA-F</span><span class="c0">]</span><span class="c2">{6}</span><span class="c6">-</span><span class="c10">)</span><span class="c2">{8}</span><span class="c0">[</span><span class="c4">0-9a-fA-F</span><span class="c0">]</span><span class="c2">{6}$</span><span class="c6">/</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c2">export</span><span class="c0">&nbsp;</span><span class="c2">const</span><span class="c0">&nbsp;palette_key </span><span class="c2">=</span><span class="c0">&nbsp;</span><span class="c4">Symbol</span><span class="c0">() </span><span class="c2">as</span><span class="c0">&nbsp;</span><span class="c10">InjectionKey</span><span class="c0">&lt;</span><span class="c10">Palette</span><span class="c0 c3">&gt;</span></p>
<p class="c5 c9"><span class="c0 c3"></span></p>
<p class="c5"><span class="c2">export</span><span class="c0">&nbsp;</span><span class="c2">class</span><span class="c0">&nbsp;</span><span class="c10">Palette</span><span class="c0 c3">&nbsp;{</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp;</span><span class="c4">palette</span><span class="c2">:</span><span class="c0">&nbsp;</span><span class="c10">Ref</span><span class="c0">&lt;</span><span class="c10">string</span><span class="c0 c3">&gt;;</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp;</span><span class="c4">hexs</span><span class="c0">&nbsp;</span><span class="c2">=</span><span class="c0">&nbsp;_.</span><span class="c4">times</span><span class="c0">(N9, </span><span class="c7">i</span><span class="c0">&nbsp;</span><span class="c10">=&gt;</span><span class="c0">&nbsp;</span><span class="c4">computed</span><span class="c0 c3">({</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp;</span><span class="c4">get</span><span class="c0">: () </span><span class="c10">=&gt;</span><span class="c0">&nbsp;</span><span class="c2">this</span><span class="c0">.</span><span class="c4">palette</span><span class="c0">.</span><span class="c4">value</span><span class="c0">.</span><span class="c4">substring</span><span class="c0">(</span><span class="c7">i</span><span class="c2">*</span><span class="c8">7</span><span class="c0">, </span><span class="c7">i</span><span class="c2">*</span><span class="c8">7</span><span class="c2">+</span><span class="c8">6</span><span class="c0 c3">),</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp;</span><span class="c4">set</span><span class="c0">: (</span><span class="c7">c</span><span class="c2">:</span><span class="c0">&nbsp;</span><span class="c10">string</span><span class="c0">) </span><span class="c10">=&gt;</span><span class="c0 c3">&nbsp;{</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp; &nbsp;</span><span class="c2">const</span><span class="c0">&nbsp;start </span><span class="c2">=</span><span class="c0">&nbsp;</span><span class="c7">i</span><span class="c2">*</span><span class="c8">7</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp; &nbsp;</span><span class="c2">const</span><span class="c0">&nbsp;end </span><span class="c2">=</span><span class="c0">&nbsp;</span><span class="c7">i</span><span class="c2">*</span><span class="c8">7</span><span class="c2">+</span><span class="c8">6</span><span class="c0 c3">;</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp; &nbsp;</span><span class="c2">this</span><span class="c0">.</span><span class="c4">palette</span><span class="c0">.</span><span class="c4">value</span><span class="c0">&nbsp;</span><span class="c2 c3">=</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;</span><span class="c2">this</span><span class="c0">.</span><span class="c4">palette</span><span class="c0">.</span><span class="c4">value</span><span class="c0">.</span><span class="c4">substring</span><span class="c0">(</span><span class="c8">0</span><span class="c0">,start) </span><span class="c2 c3">+</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;</span><span class="c7">c</span><span class="c0">&nbsp;</span><span class="c2 c3">+</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;</span><span class="c2">this</span><span class="c0">.</span><span class="c4">palette</span><span class="c0">.</span><span class="c4">value</span><span class="c0">.</span><span class="c4">substring</span><span class="c0 c3">(end);</span></p>
<p class="c5"><span class="c0 c3">&nbsp; &nbsp; &nbsp;}</span></p>
<p class="c5"><span class="c0 c3">&nbsp; &nbsp;}));</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp;</span><span class="c4">colors</span><span class="c0">&nbsp;</span><span class="c2">=</span><span class="c0">&nbsp;_.</span><span class="c4">times</span><span class="c0">(N9, </span><span class="c7">i</span><span class="c0">&nbsp;</span><span class="c10">=&gt;</span><span class="c0">&nbsp;</span><span class="c4">computed</span><span class="c0 c3">({</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp;</span><span class="c4">get</span><span class="c0">: () </span><span class="c10">=&gt;</span><span class="c0">&nbsp;</span><span class="c10">Color</span><span class="c0">(</span><span class="c6">"#"</span><span class="c2">+this</span><span class="c0">.</span><span class="c4">hexs</span><span class="c0">[</span><span class="c7">i</span><span class="c0">].</span><span class="c4">value</span><span class="c0 c3">),</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp;</span><span class="c4">set</span><span class="c0">: (</span><span class="c7">c</span><span class="c2">:</span><span class="c0">&nbsp;</span><span class="c10">Color</span><span class="c0">) </span><span class="c10">=&gt;</span><span class="c0 c3">&nbsp;{</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp; &nbsp;</span><span class="c2">this</span><span class="c0">.</span><span class="c4">hexs</span><span class="c0">[</span><span class="c7">i</span><span class="c0">].</span><span class="c4">value</span><span class="c0">&nbsp;</span><span class="c2">=</span><span class="c0">&nbsp;</span><span class="c7">c</span><span class="c0">.</span><span class="c4">hex</span><span class="c0">().</span><span class="c4">substring</span><span class="c0">(</span><span class="c8">1</span><span class="c0 c3">);</span></p>
<p class="c5"><span class="c0 c3">&nbsp; &nbsp; &nbsp;}</span></p>
<p class="c5"><span class="c0 c3">&nbsp; &nbsp;}));</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp;</span><span class="c4">colorData</span><span class="c0">&nbsp;</span><span class="c2">=</span><span class="c0">&nbsp;</span><span class="c4">computed</span><span class="c0">(() </span><span class="c10">=&gt;</span><span class="c0">&nbsp;</span><span class="c4">getData</span><span class="c0">(</span><span class="c2">this</span><span class="c0">.</span><span class="c4">palette</span><span class="c0">.</span><span class="c4">value</span><span class="c0 c3">));</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp;</span><span class="c4">dark</span><span class="c0">&nbsp;</span><span class="c2">=</span><span class="c0">&nbsp;</span><span class="c4">computed</span><span class="c0">(() </span><span class="c10">=&gt;</span><span class="c0">&nbsp;</span><span class="c2">this</span><span class="c0">.</span><span class="c4">colors</span><span class="c0">[</span><span class="c8">0</span><span class="c0">].</span><span class="c4">value</span><span class="c0">.</span><span class="c4">l</span><span class="c0">() </span><span class="c2">&lt;</span><span class="c0">&nbsp;</span><span class="c2">this</span><span class="c0">.</span><span class="c4">colors</span><span class="c0">[</span><span class="c8">1</span><span class="c0">].</span><span class="c4">value</span><span class="c0">.</span><span class="c4">l</span><span class="c0 c3">());</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp;</span><span class="c2">constructor</span><span class="c0">(</span><span class="c7">palette</span><span class="c2">:</span><span class="c0">&nbsp;</span><span class="c10">string</span><span class="c0 c3">) {</span></p>
<p class="c5"><span class="c0">&nbsp; &nbsp; &nbsp;</span><span class="c2">this</span><span class="c0">.</span><span class="c4">palette</span><span class="c0">&nbsp;</span><span class="c2">=</span><span class="c0">&nbsp;</span><span class="c4">ref</span><span class="c0">(</span><span class="c7">palette</span><span class="c0 c3">);</span></p>
<p class="c5"><span class="c0 c3">&nbsp; &nbsp;}</span></p>
<p class="c5"><span class="c0 c3">}</span></p>
<p class="c5 c9"><span class="c0 c3"></span></p>
<p class="c9 c11"><span class="c1"></span></p>
</template>
<style scoped>
@import url('https://themes.googleusercontent.com/fonts/css?kit=sDU-RIIs3Wq_4pUcDwWu-05zdwzqyXAFhQ3EpAK6bTA');
ol{
    margin:0;
    padding:0
}
table td,table th{
    padding:0
}
.c1{
    color:#000000;
    font-weight:400;
    text-decoration:none;
    vertical-align:baseline;
    font-size:12pt;
    font-family:"Cambria";
    font-style:normal
}
.c5{
    background-color: var(--base9-background);
    padding-top:0pt;
    padding-bottom:0pt;
    line-height:1.3571428571428572;
    orphans:2;
    widows:2;
    text-align:left
}
.c11{
    padding-top:0pt;
    padding-bottom:0pt;
    line-height:1.0;
    orphans:2;
    widows:2;
    text-align:left
}
.c6{
    font-size:10.5pt;
    font-family:"Courier New";
    color: var(--base9-c3-p100);
    font-weight:400
}
.c2{
    font-size:10.5pt;
    font-family:"Courier New";
    color: var(--base9-c2-p100);
    font-weight:400
}
.c4{
    font-size:10.5pt;
    font-family:"Courier New";
    color: var(--base9-c4-p100);
    font-weight:400
}
.c0{
    font-size:10.5pt;
    font-family:"Courier New";
    color: var(--base9-c0-p100);
    font-weight:400
}
.c8{
    font-size:10.5pt;
    font-family:"Courier New";
    color: var(--base9-c5-p100);
    font-weight:400
}
.c10{
    font-size:10.5pt;
    font-family:"Courier New";
    color: var(--base9-c1-p100);
    font-weight:400
}
.c7{
    font-size:10.5pt;
    font-family:"Courier New";
    color: var(--base9-c6-p100);
    font-weight:400
}
.c12{
    background-color:#ffffff;
    max-width:468pt;
    padding:72pt 72pt 72pt 72pt
}
.c3{
    text-decoration:none;
    vertical-align:baseline;
    font-style:normal
}
.c9{
    height:12pt
}
.title{
    padding-top:24pt;
    color:#000000;
    font-weight:700;
    font-size:36pt;
    padding-bottom:6pt;
    font-family:"Cambria";
    line-height:1.0;
    page-break-after:avoid;
    orphans:2;
    widows:2;
    text-align:left
}
.subtitle{
    padding-top:18pt;
    color:#666666;
    font-size:24pt;
    padding-bottom:4pt;
    font-family:"Georgia";
    line-height:1.0;
    page-break-after:avoid;
    font-style:italic;
    orphans:2;
    widows:2;
    text-align:left
}
li{
    color:#000000;
    font-size:12pt;
    font-family:"Cambria"
}
p{
    margin:0;
    color:#000000;
    font-size:12pt;
    font-family:"Cambria"
}
h1{
    padding-top:24pt;
    color:#000000;
    font-weight:700;
    font-size:24pt;
    padding-bottom:6pt;
    font-family:"Cambria";
    line-height:1.0;
    page-break-after:avoid;
    orphans:2;
    widows:2;
    text-align:left
}
h2{
    padding-top:18pt;
    color:#000000;
    font-weight:700;
    font-size:18pt;
    padding-bottom:4pt;
    font-family:"Cambria";
    line-height:1.0;
    page-break-after:avoid;
    orphans:2;
    widows:2;
    text-align:left
}
h3{
    padding-top:14pt;
    color:#000000;
    font-weight:700;
    font-size:14pt;
    padding-bottom:4pt;
    font-family:"Cambria";
    line-height:1.0;
    page-break-after:avoid;
    orphans:2;
    widows:2;
    text-align:left
}
h4{
    padding-top:12pt;
    color:#000000;
    font-weight:700;
    font-size:12pt;
    padding-bottom:2pt;
    font-family:"Cambria";
    line-height:1.0;
    page-break-after:avoid;
    orphans:2;
    widows:2;
    text-align:left
}
h5{
    padding-top:11pt;
    color:#000000;
    font-weight:700;
    font-size:11pt;
    padding-bottom:2pt;
    font-family:"Cambria";
    line-height:1.0;
    page-break-after:avoid;
    orphans:2;
    widows:2;
    text-align:left
}
h6{
    padding-top:10pt;
    color:#000000;
    font-weight:700;
    font-size:10pt;
    padding-bottom:2pt;
    font-family:"Cambria";
    line-height:1.0;
    page-break-after:avoid;
    orphans:2;
    widows:2;
    text-align:left
}

</style>

<script setup lang="ts">
</script>

<template>
<Main>
<div class="container2">
    <PaletteEditor :label="true" class="palette"></PaletteEditor>
    <div class="main">
        <n-layout position="absolute" has-sider sider-placement="right">
            <n-layout-content content-style="padding: 24px;" :native-scrollbar="false">
                <div id="install">
                    <h1>Install</h1>
                    <Install></Install>
                </div>
                <div id="terminal">
                    <h1>Terminal</h1>
                    <Ansi></Ansi>
                </div>
                <div id="editor">
                    <h1>Syntax Highlighting</h1>
                    <h2>vscode</h2>
                    <Vscode></Vscode>
                </div>
                <div id="contrast">
                    <h1>Contrast</h1>
                    <p><a href="https://coolors.co/contrast-checker">WCAG 2.0 Contrast</a></p>
                    <Contrast></Contrast>
                </div>
            </n-layout-content>
            <n-layout-sider
                content-style="padding: 24px;"
                :native-scrollbar="false"
            >
                <n-anchor :show-rail="false" :show-background="false">
                    <n-anchor-link title="Install" href="#install" />
                    <n-anchor-link title="Terminal" href="#terminal" />
                    <n-anchor-link title="Syntax Highlighting" href="#editor" />
                    <n-anchor-link title="Contrast" href="#contrast" />
                </n-anchor>
            </n-layout-sider>
        </n-layout>
    </div>
</div>
</Main>
</template>
<style scoped>
.container2 {
    display: flex;
    flex-direction: column;
    height: 100%;
}
.main {
    /* box-sizing: border-box; */
    position: relative;
    flex-grow: 1000;
    
}
.palette {
    /* height: 18vh; */
    margin-bottom: 1em;
    width: 70vw;
    padding-left: 15vw;
}
</style>
<script setup lang="ts">
import Install from './previews/Install.vue';
import Contrast from './previews/Contrast.vue';
import Ansi from './previews/Ansi.vue';
import Vscode from './previews/Vscode.vue';
import { inject, reactive } from 'vue';
import { Palette } from '../helper2';

import type { Ref } from 'vue';
import type { ColorPalette, List9 } from '../helpers';
import { useRouter } from 'vue-router';
import { renderString } from 'base9-builder';

const router = useRouter();
const palette = inject<Palette>('palette')!;

</script>
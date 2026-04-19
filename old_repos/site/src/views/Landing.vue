<template>
<Main>
    <div className="container">
      <Logo className="logo" size="20vh" :palette="palette.palette.value"/>
      <div className="title">base9</div>
      <div className="sub-title">
        The easiest way to customize color theme across applications.
      </div>
      <div className="action">
        <n-button class="button" type="primary" @click="$router.push('/pick')">
          Pick curated themes
        </n-button>
        <n-button class="button" type="primary" @click="$router.push('/build')">
          Build my own theme<br>(trust me, it's not hard)
        </n-button>
      </div>
      <div className="footer">
        <a href="https://github.com/base9-theme/base9">github </a> |
        <!-- <a href="https://github.com/base9-theme/base9">discord </a> | -->
        <router-link :to="{query: { palette: null }}">reset theme</router-link>
      </div>
    </div>
</Main>
</template>
<style scoped>
.container {
  background-color: var(--base9-0);
  color: var(--base9-1);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  /* height: 100vh; */
}
.logo {

}
.title {
  text-align: center;
  font-size: 10vh;
}

.sub-title {
  text-align: center;
  margin-bottom: 1vh;
}
.action {
  display: flex;
  align-items: center;
  gap: 3vw;
}
.button {
  height: 5vh;
  width: 16em;
}
</style>
<script setup lang="ts">
import type { Ref } from "vue";
import type { ColorPalette } from "../helpers";
import { inject } from "vue";
import { Palette } from "../helper2";

const palette = inject('palette') as Palette;

</script>

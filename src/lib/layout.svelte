<script lang="ts">
  import Chrome from "./navigation/chrome.svelte";
  import Navbar from "./navigation/navbar.svelte";
  import Toolbar from "./navigation/toolbar.svelte";
  import VerticalPanels from "./windows/vertical-panels.svelte";
</script>

<div
  class="inline-grid w-screen h-screen overflow-hidden
  grid-cols-[min-content_1fr] grid-rows-[min-content_1fr]">

  <Chrome />
  <Navbar>
    <slot name="links"></slot>
  </Navbar>
  <VerticalPanels>
    <slot name="routes"></slot>
  </VerticalPanels>
  <Toolbar />
</div>

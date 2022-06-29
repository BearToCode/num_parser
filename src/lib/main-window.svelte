<script lang="ts">
  import { Pane, Splitpanes } from "svelte-splitpanes";

</script>

<Splitpanes>
  <Pane>
    <Splitpanes horizontal={true} theme="default-theme">
      <Pane>
        <slot></slot>
      </Pane>
      <Pane class="bg-red-400">
    
      </Pane>
    </Splitpanes>
  </Pane>
  <Pane>
    
  </Pane>
</Splitpanes>

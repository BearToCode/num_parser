<script>
  import { Link, useLocation } from "svelte-navigator";

  const currentLocation = useLocation();


  export let to;
</script>

<div class="w-full py-3 flex justify-center items-center text-3xl
  relative { $currentLocation.pathname == to ? "text-white" : "text-neutral-500" }">
  <Link to={to} class="w-full h-full flex justify-center items-center">
    <slot></slot>
  </Link>
  {#if $currentLocation.pathname == to}
  <div class="absolute left-0 top-0 bottom-0 w-0.5 bg-amber-500" />
  {/if}
</div>
<script>
	import { Link, useLocation } from 'svelte-navigator';

	const currentLocation = useLocation();

	export let to;
</script>

<div
	class="w-full py-3 flex justify-center items-center text-3xl
  relative {$currentLocation.pathname == to ? 'text-white bg-primary-600' : 'text-neutral-500'}"
>
	<Link {to} class="w-full h-full flex justify-center items-center">
		<slot />
	</Link>
	{#if $currentLocation.pathname == to}
		<div class="absolute left-0 top-0 bottom-0 w-0.5 bg-secondary-500" />
	{/if}
</div>

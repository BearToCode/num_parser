<script lang="ts">
	import Icon from 'svelte-icons-pack';

	export let src: string;
</script>

<Icon className="icon" color="currentcolor" {src} />

<style>
	:global(.icon *) {
		stroke: inherit;
		font: inherit;
	}
</style>

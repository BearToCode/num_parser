<script lang="ts">
</script>

<div class="w-full overflow-y-scroll text-primary-200" id="terminal">Hello</div>

<style lang="scss">
	#terminal {
		/* width */
		&::-webkit-scrollbar {
			@apply w-3;
		}

		&:hover::-webkit-scrollbar-thumb {
			@apply bg-primary-600;
		}

		/* Handle on hover */
		&::-webkit-scrollbar-thumb:hover {
			@apply bg-primary-500;
		}
	}
</style>

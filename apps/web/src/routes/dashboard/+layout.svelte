<script lang="ts">
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Toast from '$lib/components/ui/Toast.svelte';
	import CommandPalette from '$lib/components/ui/CommandPalette.svelte';

	let { children } = $props();
</script>

<div class="flex min-h-screen">
	<Sidebar />

	<main class="flex-1 ml-[var(--spacing-sidebar)] transition-all duration-[var(--transition-slow)]">
		{@render children()}
	</main>
</div>

<Toast />
<CommandPalette />

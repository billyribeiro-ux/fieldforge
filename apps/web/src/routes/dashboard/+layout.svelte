<script lang="ts">
	import { goto } from '$app/navigation';
	import { auth } from '$lib/stores/auth.svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Toast from '$lib/components/ui/Toast.svelte';
	import CommandPalette from '$lib/components/ui/CommandPalette.svelte';

	let { children } = $props();

	$effect(() => {
		if (!auth.isLoading && !auth.isAuthenticated) {
			goto('/auth');
		}
	});
</script>

{#if auth.isLoading}
	<div class="flex items-center justify-center min-h-screen">
		<div class="animate-spin w-8 h-8 border-4 border-forge-200 border-t-forge-600 rounded-full"></div>
	</div>
{:else if auth.isAuthenticated}
	<div class="flex min-h-screen">
		<Sidebar />

		<main class="flex-1 ml-0 lg:ml-[var(--spacing-sidebar)] transition-all duration-[var(--transition-slow)]">
			{@render children()}
		</main>
	</div>

	<Toast />
	<CommandPalette />
{/if}

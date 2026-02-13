<script lang="ts">
	import { goto } from '$app/navigation';
	import { navigating } from '$app/state';
	import { auth } from '$lib/stores/auth.svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Toast from '$lib/components/ui/Toast.svelte';
	import CommandPalette from '$lib/components/ui/CommandPalette.svelte';
	import { handleShortcuts, type Shortcut } from '$lib/utils/shortcuts';

	let { children } = $props();
	let commandPaletteOpen = $state(false);

	$effect(() => {
		if (!auth.isLoading && !auth.isAuthenticated) {
			goto('/auth');
		}
	});

	const shortcuts: Shortcut[] = [
		{ key: 'k', ctrl: true, action: () => (commandPaletteOpen = !commandPaletteOpen), description: 'Toggle command palette' },
		{ key: 'n', ctrl: true, shift: true, action: () => goto('/dashboard/jobs'), description: 'Go to jobs' },
	];

	function onKeydown(event: KeyboardEvent) {
		handleShortcuts(event, shortcuts);
	}
</script>

<svelte:window onkeydown={onKeydown} />

{#if auth.isLoading}
	<div class="flex items-center justify-center min-h-screen">
		<div class="animate-spin w-8 h-8 border-4 border-forge-200 border-t-forge-600 rounded-full"></div>
	</div>
{:else if auth.isAuthenticated}
	{#if navigating.to}
		<div class="fixed top-0 left-0 right-0 z-[100] h-0.5 bg-forge-100">
			<div class="h-full bg-forge-500 animate-pulse" style="width: 80%; transition: width 0.3s ease;"></div>
		</div>
	{/if}

	<div class="flex min-h-screen">
		<Sidebar />

		<main id="main-content" class="flex-1 ml-0 lg:ml-[var(--spacing-sidebar)] transition-all duration-[var(--transition-slow)]">
			{@render children()}
		</main>
	</div>

	<Toast />
	<CommandPalette />
{/if}

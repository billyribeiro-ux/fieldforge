<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import { AlertTriangle, RefreshCw } from 'lucide-svelte';

	interface Props {
		error?: Error | string | null;
		title?: string;
		children?: import('svelte').Snippet;
	}

	let { error = null, title = 'Something went wrong', children }: Props = $props();

	let errorMessage = $derived(
		error instanceof Error ? error.message : typeof error === 'string' ? error : 'An unexpected error occurred'
	);
</script>

{#if error}
	<div class="flex items-center justify-center min-h-[400px] p-8">
		<div class="text-center max-w-md">
			<div class="w-16 h-16 bg-danger-50 rounded-2xl flex items-center justify-center mx-auto mb-4">
				<AlertTriangle class="w-8 h-8 text-danger-500" />
			</div>
			<h2 class="text-lg font-semibold text-surface-900 mb-2">{title}</h2>
			<p class="text-sm text-surface-500 mb-6">{errorMessage}</p>
			<div class="flex items-center justify-center gap-3">
				<Button variant="primary" onclick={() => window.location.reload()}>
					<RefreshCw class="w-4 h-4" />
					Retry
				</Button>
				<Button variant="outline" onclick={() => history.back()}>
					Go Back
				</Button>
			</div>
		</div>
	</div>
{:else if children}
	{@render children()}
{/if}

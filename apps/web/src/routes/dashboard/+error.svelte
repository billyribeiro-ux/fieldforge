<script lang="ts">
	import { page } from '$app/state';
	import Button from '$lib/components/ui/Button.svelte';
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import { AlertTriangle, Home, ArrowLeft, RefreshCw } from 'lucide-svelte';
</script>

<TopBar title="Error" />

<div class="p-6">
	<div class="flex items-center justify-center min-h-[60vh]">
		<div class="text-center max-w-md">
			<div class="w-16 h-16 bg-danger-50 rounded-2xl flex items-center justify-center mx-auto mb-4">
				<AlertTriangle class="w-8 h-8 text-danger-500" />
			</div>

			<h1 class="text-4xl font-bold text-surface-900 mb-2">{page.status}</h1>
			<p class="text-surface-500 mb-8">
				{#if page.status === 404}
					This page doesn't exist or the item you're looking for has been removed.
				{:else if page.status === 403}
					You don't have permission to view this page.
				{:else}
					{page.error?.message ?? 'Something went wrong. Please try again.'}
				{/if}
			</p>

			<div class="flex items-center justify-center gap-3">
				<Button variant="primary" onclick={() => window.location.reload()}>
					<RefreshCw class="w-4 h-4" />
					Retry
				</Button>
				<Button variant="outline" onclick={() => history.back()}>
					<ArrowLeft class="w-4 h-4" />
					Go Back
				</Button>
				<a href="/dashboard">
					<Button variant="ghost">
						<Home class="w-4 h-4" />
						Dashboard
					</Button>
				</a>
			</div>
		</div>
	</div>
</div>

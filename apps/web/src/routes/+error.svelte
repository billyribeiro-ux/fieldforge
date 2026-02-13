<script lang="ts">
	import { page } from '$app/state';
	import Button from '$lib/components/ui/Button.svelte';
	import { Wrench, Home, ArrowLeft } from 'lucide-svelte';
</script>

<div class="min-h-screen bg-surface-50 flex items-center justify-center p-8">
	<div class="text-center max-w-md">
		<div class="w-20 h-20 bg-forge-100 rounded-2xl flex items-center justify-center mx-auto mb-6">
			<Wrench class="w-10 h-10 text-forge-500" />
		</div>

		<h1 class="text-6xl font-bold text-surface-900 mb-2">{page.status}</h1>
		<p class="text-lg text-surface-600 mb-8">
			{#if page.status === 404}
				The page you're looking for doesn't exist or has been moved.
			{:else if page.status === 500}
				Something went wrong on our end. We're working on it.
			{:else if page.status === 403}
				You don't have permission to access this page.
			{:else}
				{page.error?.message ?? 'An unexpected error occurred.'}
			{/if}
		</p>

		<div class="flex items-center justify-center gap-3">
			<Button variant="primary" onclick={() => window.location.href = '/dashboard'}>
				<Home class="w-4 h-4" />
				Go to Dashboard
			</Button>
			<Button variant="outline" onclick={() => history.back()}>
				<ArrowLeft class="w-4 h-4" />
				Go Back
			</Button>
		</div>
	</div>
</div>

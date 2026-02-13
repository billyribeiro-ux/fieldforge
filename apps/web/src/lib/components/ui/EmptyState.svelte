<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import { Plus } from 'lucide-svelte';

	interface Props {
		icon?: import('svelte').Component;
		title: string;
		description?: string;
		actionLabel?: string;
		actionHref?: string;
		onaction?: () => void;
	}

	let { icon, title, description, actionLabel, actionHref, onaction }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center py-16 px-8 text-center">
	{#if icon}
		<div class="w-16 h-16 bg-surface-100 rounded-2xl flex items-center justify-center mb-4">
			{@const Icon = icon}
			<Icon class="w-8 h-8 text-surface-400" />
		</div>
	{/if}

	<h3 class="text-base font-semibold text-surface-900 mb-1">{title}</h3>

	{#if description}
		<p class="text-sm text-surface-500 max-w-sm mb-6">{description}</p>
	{/if}

	{#if actionLabel}
		{#if actionHref}
			<a href={actionHref}>
				<Button variant="primary" size="md">
					<Plus class="w-4 h-4" />
					{actionLabel}
				</Button>
			</a>
		{:else if onaction}
			<Button variant="primary" size="md" onclick={onaction}>
				<Plus class="w-4 h-4" />
				{actionLabel}
			</Button>
		{/if}
	{/if}
</div>

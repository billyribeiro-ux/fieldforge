<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import Card from '$lib/components/ui/Card.svelte';
	import type { Component } from 'svelte';

	interface Props {
		title: string;
		value: string;
		change?: string;
		changeType?: 'positive' | 'negative' | 'neutral';
		icon?: Component;
		iconColor?: string;
	}

	let {
		title,
		value,
		change,
		changeType = 'neutral',
		icon: Icon,
		iconColor = 'bg-forge-50 text-forge-600'
	}: Props = $props();

	const changeColors = {
		positive: 'text-green-600',
		negative: 'text-red-600',
		neutral: 'text-surface-500'
	};
</script>

<Card hover>
	<div class="flex items-start justify-between">
		<div class="space-y-2">
			<p class="text-sm font-medium text-surface-500">{title}</p>
			<p class="text-2xl font-bold text-surface-900">{value}</p>
			{#if change}
				<p class={cn('text-sm font-medium', changeColors[changeType])}>
					{change}
				</p>
			{/if}
		</div>
		{#if Icon}
			<div class={cn('p-3 rounded-xl', iconColor)}>
				<Icon class="w-6 h-6" />
			</div>
		{/if}
	</div>
</Card>

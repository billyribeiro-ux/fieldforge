<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		src?: string | null;
		alt?: string;
		initials?: string;
		size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
		class?: string;
	}

	let { src, alt = '', initials = '', size = 'md', class: className = '' }: Props = $props();

	const sizes = {
		xs: 'w-6 h-6 text-[10px]',
		sm: 'w-8 h-8 text-xs',
		md: 'w-10 h-10 text-sm',
		lg: 'w-12 h-12 text-base',
		xl: 'w-16 h-16 text-lg'
	};

	const colors = [
		'bg-forge-100 text-forge-700',
		'bg-purple-100 text-purple-700',
		'bg-green-100 text-green-700',
		'bg-amber-100 text-amber-700',
		'bg-rose-100 text-rose-700',
		'bg-cyan-100 text-cyan-700',
		'bg-indigo-100 text-indigo-700',
		'bg-orange-100 text-orange-700'
	];

	let colorIndex = $derived(
		initials ? initials.charCodeAt(0) % colors.length : 0
	);
</script>

{#if src}
	<img
		{src}
		{alt}
		class={cn('rounded-full object-cover flex-shrink-0', sizes[size], className)}
	/>
{:else}
	<div
		class={cn(
			'rounded-full flex items-center justify-center font-bold flex-shrink-0',
			sizes[size],
			colors[colorIndex],
			className
		)}
	>
		{initials || '?'}
	</div>
{/if}

<script lang="ts">
	import { toast } from '$lib/stores/toast.svelte';
	import { CheckCircle2, XCircle, AlertTriangle, Info, X } from 'lucide-svelte';

	const icons = {
		success: CheckCircle2,
		error: XCircle,
		warning: AlertTriangle,
		info: Info
	};

	const colors = {
		success: 'bg-green-50 border-green-200 text-green-800',
		error: 'bg-red-50 border-red-200 text-red-800',
		warning: 'bg-yellow-50 border-yellow-200 text-yellow-800',
		info: 'bg-forge-50 border-forge-200 text-forge-800'
	};

	const iconColors = {
		success: 'text-green-500',
		error: 'text-red-500',
		warning: 'text-yellow-500',
		info: 'text-forge-500'
	};
</script>

{#if toast.toasts.length > 0}
	<div class="fixed bottom-4 right-4 z-[100] space-y-2 max-w-sm">
		{#each toast.toasts as t (t.id)}
			{@const Icon = icons[t.type]}
			<div class="flex items-start gap-3 px-4 py-3 rounded-xl border shadow-lg {colors[t.type]} animate-slide-up">
				<Icon class="w-5 h-5 flex-shrink-0 mt-0.5 {iconColors[t.type]}" />
				<div class="flex-1 min-w-0">
					<p class="text-sm font-semibold">{t.title}</p>
					{#if t.message}
						<p class="text-xs mt-0.5 opacity-80">{t.message}</p>
					{/if}
				</div>
				<button
					onclick={() => toast.dismiss(t.id)}
					class="p-0.5 opacity-60 hover:opacity-100 transition-opacity cursor-pointer"
				>
					<X class="w-4 h-4" />
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	@keyframes slide-up {
		from {
			opacity: 0;
			transform: translateY(1rem);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	:global(.animate-slide-up) {
		animation: slide-up 0.2s ease-out;
	}
</style>

<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { auth } from '$lib/stores/auth.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { Wrench, ArrowRight, Briefcase, Users, Calendar, BarChart3 } from 'lucide-svelte';

	onMount(() => {
		if (auth.isAuthenticated) {
			goto('/dashboard');
		}
	});
</script>

<div class="min-h-screen bg-surface-50">
	<!-- Hero -->
	<header class="border-b border-surface-200 bg-white">
		<div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
			<div class="flex items-center gap-3">
				<div class="w-9 h-9 bg-forge-600 rounded-xl flex items-center justify-center">
					<Wrench class="w-5 h-5 text-white" />
				</div>
				<span class="text-xl font-bold text-surface-900">FieldForge</span>
			</div>
			<div class="flex items-center gap-3">
				<a href="/auth" class="text-sm font-medium text-surface-600 hover:text-surface-900 px-4 py-2">Sign In</a>
				<a href="/auth">
					<Button variant="primary" size="md">Get Started Free</Button>
				</a>
			</div>
		</div>
	</header>

	<main class="max-w-7xl mx-auto px-6 py-24">
		<div class="text-center max-w-3xl mx-auto">
			<div class="inline-flex items-center gap-2 px-3 py-1 bg-forge-50 text-forge-700 rounded-full text-sm font-medium mb-6">
				<span class="w-2 h-2 bg-forge-500 rounded-full"></span>
				#1 Job Management for Tradespeople
			</div>

			<h1 class="text-5xl font-extrabold text-surface-900 tracking-tight leading-tight">
				Run your trade business<br />
				<span class="text-forge-600">like a Fortune 500</span>
			</h1>

			<p class="mt-6 text-lg text-surface-500 max-w-2xl mx-auto leading-relaxed">
				From lead to paid in 3 taps. FieldForge gives HVAC, plumbing, electrical, and roofing
				professionals the tools to manage jobs, customers, estimates, and invoices — with AI-powered
				photo estimation and smart scheduling.
			</p>

			<div class="mt-10 flex items-center justify-center gap-4">
				<a href="/auth">
					<Button variant="primary" size="lg">
						Start Free Trial
						<ArrowRight class="w-5 h-5" />
					</Button>
				</a>
				<Button variant="outline" size="lg">Watch Demo</Button>
			</div>
		</div>

		<!-- Features Grid -->
		<div class="mt-24 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
			{#each [
				{ icon: Briefcase, title: 'Job Management', desc: 'Track every job from lead to paid. Status updates, time tracking, photos, and checklists.' },
				{ icon: Users, title: 'Customer CRM', desc: 'Complete customer profiles with property history, equipment registry, and communication log.' },
				{ icon: Calendar, title: 'Smart Scheduling', desc: 'AI-optimized routes and schedules. Drag-and-drop calendar with real-time GPS tracking.' },
				{ icon: BarChart3, title: 'Business Intelligence', desc: 'Revenue forecasting, profitability analysis, and actionable insights powered by AI.' }
			] as feature}
				<div class="bg-white rounded-xl p-6 border border-surface-200 shadow-sm hover:shadow-md transition-shadow">
					<div class="w-12 h-12 bg-forge-50 rounded-xl flex items-center justify-center mb-4">
						<feature.icon class="w-6 h-6 text-forge-600" />
					</div>
					<h3 class="text-base font-semibold text-surface-900">{feature.title}</h3>
					<p class="mt-2 text-sm text-surface-500 leading-relaxed">{feature.desc}</p>
				</div>
			{/each}
		</div>

		<!-- Trades -->
		<div class="mt-24 text-center">
			<p class="text-sm font-medium text-surface-400 uppercase tracking-wider mb-6">Built for every trade</p>
			<div class="flex items-center justify-center gap-8 flex-wrap text-surface-500">
				{#each ['HVAC', 'Plumbing', 'Electrical', 'Roofing', 'General Contractor', 'Painting', 'Landscaping', 'Carpentry'] as trade}
					<span class="text-sm font-medium">{trade}</span>
				{/each}
			</div>
		</div>
	</main>

	<footer class="border-t border-surface-200 bg-white mt-24">
		<div class="max-w-7xl mx-auto px-6 py-8 flex items-center justify-between">
			<p class="text-sm text-surface-400">© 2024 FieldForge. All rights reserved.</p>
			<div class="flex items-center gap-6 text-sm text-surface-400">
				<a href="/terms" class="hover:text-surface-600">Terms</a>
				<a href="/privacy" class="hover:text-surface-600">Privacy</a>
				<a href="/contact" class="hover:text-surface-600">Contact</a>
			</div>
		</div>
	</footer>
</div>

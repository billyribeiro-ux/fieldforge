<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import { auth } from '$lib/stores/auth.svelte';
	import { Wrench } from 'lucide-svelte';

	let mode = $state<'login' | 'register'>('login');
	let email = $state('');
	let password = $state('');
	let firstName = $state('');
	let lastName = $state('');
	let trade = $state('');
	let error = $state('');
	let loading = $state(false);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		loading = true;

		try {
			if (mode === 'login') {
				await auth.login(email, password);
			} else {
				await auth.register({
					email,
					password,
					first_name: firstName,
					last_name: lastName,
					trade: trade || undefined
				});
			}
			goto('/dashboard');
		} catch (err: any) {
			error = err.message || 'An error occurred';
		} finally {
			loading = false;
		}
	}

	const trades = [
		'HVAC', 'Plumbing', 'Electrical', 'Roofing', 'General Contractor',
		'Painting', 'Landscaping', 'Carpentry', 'Flooring', 'Other'
	];
</script>

<svelte:head>
	<title>Sign In — FieldForge</title>
</svelte:head>

<div class="min-h-screen flex items-center justify-center bg-surface-50 px-4">
	<div class="w-full max-w-md">
		<!-- Logo -->
		<div class="flex flex-col items-center mb-8">
			<div class="w-14 h-14 bg-forge-600 rounded-2xl flex items-center justify-center mb-4 shadow-lg">
				<Wrench class="w-8 h-8 text-white" />
			</div>
			<h1 class="text-2xl font-bold text-surface-900">FieldForge</h1>
			<p class="text-sm text-surface-500 mt-1">Job management for tradespeople</p>
		</div>

		<Card>
			<!-- Tab switcher -->
			<div class="flex rounded-lg bg-surface-100 p-1 mb-6">
				<button
					onclick={() => (mode = 'login')}
					class="flex-1 py-2 text-sm font-medium rounded-md transition-all cursor-pointer {mode === 'login'
						? 'bg-white text-surface-900 shadow-sm'
						: 'text-surface-500 hover:text-surface-700'}"
				>
					Sign In
				</button>
				<button
					onclick={() => (mode = 'register')}
					class="flex-1 py-2 text-sm font-medium rounded-md transition-all cursor-pointer {mode === 'register'
						? 'bg-white text-surface-900 shadow-sm'
						: 'text-surface-500 hover:text-surface-700'}"
				>
					Create Account
				</button>
			</div>

			{#if error}
				<div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-700">
					{error}
				</div>
			{/if}

			<form onsubmit={handleSubmit} class="space-y-4">
				{#if mode === 'register'}
					<div class="grid grid-cols-2 gap-3">
						<Input label="First Name" bind:value={firstName} required placeholder="John" />
						<Input label="Last Name" bind:value={lastName} required placeholder="Smith" />
					</div>
				{/if}

				<Input label="Email" type="email" bind:value={email} required placeholder="you@company.com" />
				<Input label="Password" type="password" bind:value={password} required placeholder="••••••••" />

				{#if mode === 'register'}
					<div class="flex flex-col gap-1.5">
						<label for="trade" class="text-sm font-medium text-surface-700">Primary Trade</label>
						<select
							id="trade"
							bind:value={trade}
							class="w-full rounded-[var(--radius-input)] border border-surface-300 bg-white px-3 py-2 text-sm text-surface-900 focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500"
						>
							<option value="">Select your trade</option>
							{#each trades as t}
								<option value={t}>{t}</option>
							{/each}
						</select>
					</div>
				{/if}

				<Button variant="primary" size="lg" type="submit" class="w-full" {loading}>
					{mode === 'login' ? 'Sign In' : 'Create Account'}
				</Button>
			</form>

			{#if mode === 'login'}
				<p class="mt-4 text-center text-xs text-surface-400">
					<a href="/auth/forgot-password" class="text-forge-600 hover:text-forge-700 font-medium">Forgot password?</a>
				</p>
			{/if}
		</Card>

		<p class="mt-6 text-center text-xs text-surface-400">
			By continuing, you agree to FieldForge's
			<a href="/terms" class="underline hover:text-surface-600">Terms of Service</a>
			and
			<a href="/privacy" class="underline hover:text-surface-600">Privacy Policy</a>.
		</p>
	</div>
</div>

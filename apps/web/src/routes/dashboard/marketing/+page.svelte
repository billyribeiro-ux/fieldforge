<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Tabs from '$lib/components/ui/Tabs.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import { Star, MessageSquare, TrendingUp, Users, ThumbsUp, ThumbsDown, ExternalLink, Send, Globe, Award } from 'lucide-svelte';

	let activeTab = $state('reviews');

	const tabs = [
		{ id: 'reviews', label: 'Reviews' },
		{ id: 'referrals', label: 'Referrals' },
		{ id: 'reputation', label: 'Reputation' }
	];

	const reviews = [
		{ id: '1', platform: 'google', rating: 5, reviewer: 'Sarah J.', content: 'Mike was fantastic! Replaced our AC unit in under 4 hours. Very professional and clean work. Highly recommend Smith HVAC!', date: '2024-12-10', responded: true, response: 'Thank you Sarah! We appreciate your kind words. It was a pleasure working with you!' },
		{ id: '2', platform: 'google', rating: 5, reviewer: 'Tom W.', content: 'Great annual maintenance service. Thorough inspection and honest assessment. Fair pricing too.', date: '2024-12-08', responded: false, response: null },
		{ id: '3', platform: 'yelp', rating: 4, reviewer: 'Mike C.', content: 'Good work on the plumbing inspection. Found a couple issues I wasn\'t aware of. Only reason for 4 stars is scheduling took a bit longer than expected.', date: '2024-12-05', responded: true, response: 'Thanks for the feedback Mike! We\'re working on improving our scheduling process.' },
		{ id: '4', platform: 'google', rating: 5, reviewer: 'Lisa R.', content: 'Called for an emergency leak and they were here within the hour. Fixed it quickly and didn\'t overcharge. Will definitely use again!', date: '2024-12-02', responded: false, response: null },
		{ id: '5', platform: 'facebook', rating: 3, reviewer: 'Karen W.', content: 'Work quality was good but the project took longer than the original estimate. Communication could have been better about the delays.', date: '2024-11-28', responded: false, response: null },
		{ id: '6', platform: 'nextdoor', rating: 5, reviewer: 'Robert K.', content: 'Best contractor experience I\'ve ever had. The whole team was professional, on time, and the kitchen remodel looks amazing!', date: '2024-11-25', responded: true, response: 'Thank you Robert! Your kitchen turned out beautifully. Enjoy!' }
	];

	const referrals = [
		{ id: '1', referrer: 'Sarah Johnson', referred: 'Amy Foster', status: 'converted', date: '2024-12-01', revenue: 3500 },
		{ id: '2', referrer: 'Tom Williams', referred: 'David Park', status: 'lead', date: '2024-12-05', revenue: 0 },
		{ id: '3', referrer: 'Robert Kim', referred: 'James Lee', status: 'converted', date: '2024-11-15', revenue: 1200 },
		{ id: '4', referrer: 'Lisa Rodriguez', referred: 'Maria Santos', status: 'pending', date: '2024-12-10', revenue: 0 }
	];

	const platforms = ['google', 'yelp', 'facebook', 'nextdoor'] as const;

	let avgRating = $derived(+(reviews.reduce((s, r) => s + r.rating, 0) / reviews.length).toFixed(1));
	let fiveStarPct = $derived(Math.round((reviews.filter((r) => r.rating === 5).length / reviews.length) * 100));
	let responseRate = $derived(Math.round((reviews.filter((r) => r.responded).length / reviews.length) * 100));

	function platformColor(p: string): 'info' | 'danger' | 'success' | 'warning' | 'default' {
		switch (p) {
			case 'google': return 'info';
			case 'yelp': return 'danger';
			case 'facebook': return 'info';
			case 'nextdoor': return 'success';
			default: return 'default';
		}
	}

	function platformLabel(p: string): string {
		return p.charAt(0).toUpperCase() + p.slice(1);
	}

	function renderStars(rating: number): string {
		return '★'.repeat(rating) + '☆'.repeat(5 - rating);
	}

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(n);
	}

	let respondingTo = $state<string | null>(null);
	let responseText = $state('');

	function platformAvg(platform: string): number {
		const pr = reviews.filter((r) => r.platform === platform);
		return pr.length > 0 ? +(pr.reduce((s, r) => s + r.rating, 0) / pr.length).toFixed(1) : 0;
	}

	function platformCount(platform: string): number {
		return reviews.filter((r) => r.platform === platform).length;
	}

	function ratingCount(stars: number): number {
		return reviews.filter((r) => r.rating === stars).length;
	}

	function ratingPct(stars: number): number {
		return Math.round((ratingCount(stars) / reviews.length) * 100);
	}
</script>

<svelte:head>
	<title>Marketing — FieldForge</title>
</svelte:head>

<TopBar title="Marketing & Reviews">
	{#snippet actions()}
		<Button variant="primary" size="md">
			<Send class="w-4 h-4" />
			Request Reviews
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Summary Cards -->
	<div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-warning-50 rounded-lg flex items-center justify-center">
					<Star class="w-5 h-5 text-warning-500" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Avg Rating</p>
					<p class="text-xl font-bold text-surface-900">{avgRating} <span class="text-warning-500 text-sm">★</span></p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-success-50 rounded-lg flex items-center justify-center">
					<ThumbsUp class="w-5 h-5 text-success-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">5-Star Rate</p>
					<p class="text-xl font-bold text-success-600">{fiveStarPct}%</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-forge-50 rounded-lg flex items-center justify-center">
					<MessageSquare class="w-5 h-5 text-forge-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Response Rate</p>
					<p class="text-xl font-bold text-forge-600">{responseRate}%</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-accent-50 rounded-lg flex items-center justify-center">
					<Users class="w-5 h-5 text-accent-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Referrals</p>
					<p class="text-xl font-bold text-surface-900">{referrals.length}</p>
				</div>
			</div>
		</Card>
	</div>

	<!-- Tabs -->
	<Tabs {tabs} bind:activeTab />

	{#if activeTab === 'reviews'}
		<div class="space-y-4">
			{#each reviews as review (review.id)}
				<Card>
					<div class="flex items-start justify-between mb-3">
						<div class="flex items-center gap-3">
							<div class="w-10 h-10 bg-surface-100 rounded-full flex items-center justify-center text-sm font-bold text-surface-600">
								{review.reviewer.charAt(0)}
							</div>
							<div>
								<div class="flex items-center gap-2">
									<p class="text-sm font-semibold text-surface-900">{review.reviewer}</p>
									<Badge variant={platformColor(review.platform)} size="sm">{platformLabel(review.platform)}</Badge>
								</div>
								<p class="text-xs text-surface-400">{review.date}</p>
							</div>
						</div>
						<div class="flex items-center gap-1">
							<span class="text-warning-500 text-sm tracking-wider">{renderStars(review.rating)}</span>
						</div>
					</div>

					<p class="text-sm text-surface-700 leading-relaxed">{review.content}</p>

					{#if review.responded && review.response}
						<div class="mt-3 pl-4 border-l-2 border-forge-200 bg-forge-50/30 rounded-r-lg p-3">
							<p class="text-xs font-medium text-forge-700 mb-1">Your Response</p>
							<p class="text-sm text-surface-600">{review.response}</p>
						</div>
					{:else if respondingTo === review.id}
						<div class="mt-3 space-y-2">
							<Textarea bind:value={responseText} rows={2} placeholder="Write your response..." />
							<div class="flex gap-2">
								<Button variant="primary" size="sm" onclick={() => { respondingTo = null; responseText = ''; }}>
									<Send class="w-3.5 h-3.5" />
									Post Response
								</Button>
								<Button variant="ghost" size="sm" onclick={() => { respondingTo = null; responseText = ''; }}>Cancel</Button>
							</div>
						</div>
					{:else}
						<div class="mt-3">
							<Button variant="outline" size="sm" onclick={() => (respondingTo = review.id)}>
								<MessageSquare class="w-3.5 h-3.5" />
								Respond
							</Button>
						</div>
					{/if}
				</Card>
			{/each}
		</div>
	{:else if activeTab === 'referrals'}
		<Card padding={false}>
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead>
						<tr class="border-b border-surface-200 bg-surface-50">
							<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Referrer</th>
							<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Referred</th>
							<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Date</th>
							<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Status</th>
							<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Revenue</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-surface-100">
						{#each referrals as ref (ref.id)}
							<tr class="hover:bg-surface-50 transition-colors">
								<td class="px-6 py-4 text-sm font-medium text-surface-900">{ref.referrer}</td>
								<td class="px-6 py-4 text-sm text-surface-700">{ref.referred}</td>
								<td class="px-6 py-4 text-sm text-surface-500">{ref.date}</td>
								<td class="px-6 py-4">
									<Badge variant={ref.status === 'converted' ? 'success' : ref.status === 'lead' ? 'info' : 'default'} size="sm">
										{ref.status.charAt(0).toUpperCase() + ref.status.slice(1)}
									</Badge>
								</td>
								<td class="px-6 py-4 text-sm font-semibold text-surface-900 text-right">
									{ref.revenue > 0 ? formatCurrency(ref.revenue) : '—'}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</Card>

		<div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
			<Card>
				<p class="text-xs text-surface-400 uppercase tracking-wider">Total Referrals</p>
				<p class="text-2xl font-bold text-surface-900 mt-1">{referrals.length}</p>
			</Card>
			<Card>
				<p class="text-xs text-surface-400 uppercase tracking-wider">Converted</p>
				<p class="text-2xl font-bold text-success-600 mt-1">{referrals.filter((r) => r.status === 'converted').length}</p>
			</Card>
			<Card>
				<p class="text-xs text-surface-400 uppercase tracking-wider">Referral Revenue</p>
				<p class="text-2xl font-bold text-forge-600 mt-1">{formatCurrency(referrals.reduce((s, r) => s + r.revenue, 0))}</p>
			</Card>
		</div>
	{:else if activeTab === 'reputation'}
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- Platform Breakdown -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-4">Platform Breakdown</h3>
				<div class="space-y-4">
					{#each platforms as platform}
						<div class="flex items-center justify-between">
							<div class="flex items-center gap-3">
								<Badge variant={platformColor(platform)} size="sm">{platformLabel(platform)}</Badge>
								<span class="text-sm text-surface-500">{platformCount(platform)} reviews</span>
							</div>
							<div class="flex items-center gap-2">
								<span class="text-sm font-bold text-surface-900">{platformAvg(platform)}</span>
								<span class="text-warning-500 text-xs">★</span>
							</div>
						</div>
					{/each}
				</div>
			</Card>

			<!-- Rating Distribution -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-4">Rating Distribution</h3>
				<div class="space-y-3">
					{#each [5, 4, 3, 2, 1] as stars}
						<div class="flex items-center gap-3">
							<span class="text-sm text-surface-500 w-12">{stars} star</span>
							<div class="flex-1 h-2 bg-surface-100 rounded-full overflow-hidden">
								<div class="h-full bg-warning-400 rounded-full transition-all" style="width: {ratingPct(stars)}%"></div>
							</div>
							<span class="text-sm text-surface-500 w-8 text-right">{ratingCount(stars)}</span>
						</div>
					{/each}
				</div>
			</Card>

			<!-- Review Velocity -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-4">Review Velocity</h3>
				<div class="text-center py-4">
					<p class="text-4xl font-bold text-surface-900">{reviews.length}</p>
					<p class="text-sm text-surface-500 mt-1">reviews this month</p>
					<div class="flex items-center justify-center gap-1 mt-2 text-success-600">
						<TrendingUp class="w-4 h-4" />
						<span class="text-sm font-medium">+33% vs last month</span>
					</div>
				</div>
			</Card>

			<!-- Response Time -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-4">Response Metrics</h3>
				<div class="space-y-4">
					<div class="flex justify-between">
						<span class="text-sm text-surface-500">Response Rate</span>
						<span class="text-sm font-bold text-surface-900">{responseRate}%</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-surface-500">Avg Response Time</span>
						<span class="text-sm font-bold text-surface-900">4.2 hours</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-surface-500">Unresponded</span>
						<span class="text-sm font-bold text-danger-500">{reviews.filter((r) => !r.responded).length}</span>
					</div>
				</div>
			</Card>
		</div>
	{/if}
</div>

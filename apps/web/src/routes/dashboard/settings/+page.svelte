<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import {
		Building2,
		User,
		CreditCard,
		Bell,
		Shield,
		Palette,
		Users,
		Plug,
		FileText
	} from 'lucide-svelte';

	let activeTab = $state('company');

	const tabs = [
		{ id: 'company', label: 'Company', icon: Building2 },
		{ id: 'profile', label: 'Profile', icon: User },
		{ id: 'team', label: 'Team', icon: Users },
		{ id: 'billing', label: 'Billing', icon: CreditCard },
		{ id: 'notifications', label: 'Notifications', icon: Bell },
		{ id: 'integrations', label: 'Integrations', icon: Plug },
		{ id: 'templates', label: 'Templates', icon: FileText },
		{ id: 'appearance', label: 'Appearance', icon: Palette },
		{ id: 'security', label: 'Security', icon: Shield }
	];

	const notifSettings = [
		{ label: 'New job assigned', description: 'When a job is assigned to you', email: true, sms: true, push: true },
		{ label: 'Job status changes', description: 'When a job moves to a new status', email: true, sms: false, push: true },
		{ label: 'Estimate approved/declined', description: 'When a customer responds to an estimate', email: true, sms: true, push: true },
		{ label: 'Payment received', description: 'When a payment is recorded or received online', email: true, sms: false, push: true },
		{ label: 'Invoice overdue', description: 'When an invoice passes its due date', email: true, sms: false, push: false },
		{ label: 'Low inventory alerts', description: 'When stock falls below minimum quantity', email: true, sms: false, push: false },
		{ label: 'Schedule reminders', description: 'Daily schedule summary each morning', email: true, sms: false, push: true },
		{ label: 'Customer messages', description: 'When a customer sends a message', email: false, sms: true, push: true }
	];

	const integrations = [
		{ name: 'QuickBooks Online', description: 'Sync invoices, payments, and expenses', status: 'connected', icon: '📊' },
		{ name: 'Stripe', description: 'Accept card payments and ACH transfers', status: 'connected', icon: '💳' },
		{ name: 'Google Calendar', description: 'Sync your schedule with Google Calendar', status: 'not_connected', icon: '📅' },
		{ name: 'Twilio', description: 'Send SMS notifications to customers', status: 'connected', icon: '📱' },
		{ name: 'SendGrid', description: 'Send email notifications and marketing', status: 'not_connected', icon: '📧' },
		{ name: 'Zapier', description: 'Connect with 5,000+ apps via automation', status: 'not_connected', icon: '⚡' },
		{ name: 'Google Business Profile', description: 'Sync reviews and business info', status: 'not_connected', icon: '🏢' },
		{ name: 'Square', description: 'Alternative payment processing and POS', status: 'not_connected', icon: '🟦' }
	];

	const templates = [
		{ name: 'Estimate Template', description: 'Default template for customer estimates', type: 'estimate', last_modified: '2024-12-01' },
		{ name: 'Invoice Template', description: 'Default template for customer invoices', type: 'invoice', last_modified: '2024-12-01' },
		{ name: 'Appointment Reminder', description: 'SMS sent 24h before scheduled appointment', type: 'sms', last_modified: '2024-11-15' },
		{ name: 'On My Way Notification', description: 'SMS sent when technician is en route', type: 'sms', last_modified: '2024-11-15' },
		{ name: 'Job Complete Summary', description: 'Email sent after job completion', type: 'email', last_modified: '2024-11-20' },
		{ name: 'Review Request', description: 'SMS sent 24h after job completion', type: 'sms', last_modified: '2024-11-15' },
		{ name: 'Payment Receipt', description: 'Email receipt sent after payment', type: 'email', last_modified: '2024-12-01' },
		{ name: 'Overdue Invoice Reminder', description: 'Email sent when invoice is past due', type: 'email', last_modified: '2024-11-20' }
	];

	const sessions = [
		{ device: 'Chrome on macOS', location: 'Austin, TX', time: 'Current session', current: true },
		{ device: 'Safari on iPhone', location: 'Austin, TX', time: '2 hours ago', current: false },
		{ device: 'Chrome on Windows', location: 'Austin, TX', time: '1 day ago', current: false }
	];

	// Demo company data
	let companyName = $state('Smith HVAC & Plumbing');
	let companyPhone = $state('(512) 555-0100');
	let companyEmail = $state('office@smithhvac.com');
	let companyWebsite = $state('https://smithhvac.com');
	let companyAddress = $state('100 Main St, Suite 200');
	let companyCity = $state('Austin');
	let companyState = $state('TX');
	let companyZip = $state('78701');
	let defaultHourlyRate = $state('125.00');
	let taxRate = $state('8.25');
</script>

<svelte:head>
	<title>Settings — FieldForge</title>
</svelte:head>

<TopBar title="Settings" />

<div class="p-6">
	<div class="flex gap-6">
		<!-- Settings Sidebar -->
		<nav class="w-56 flex-shrink-0">
			<div class="space-y-1">
				{#each tabs as tab}
					<button
						onclick={() => (activeTab = tab.id)}
						class="flex items-center gap-3 w-full px-3 py-2.5 rounded-lg text-sm font-medium transition-all cursor-pointer
							{activeTab === tab.id
							? 'bg-forge-50 text-forge-700'
							: 'text-surface-600 hover:bg-surface-100 hover:text-surface-900'}"
					>
						<tab.icon class="w-4.5 h-4.5" />
						<span>{tab.label}</span>
					</button>
				{/each}
			</div>
		</nav>

		<!-- Settings Content -->
		<div class="flex-1 max-w-2xl">
			{#if activeTab === 'company'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Company Information</h2>
						<p class="text-sm text-surface-500 mt-1">Manage your company details and branding.</p>
					</div>

					<Card>
						<div class="space-y-4">
							<Input label="Company Name" bind:value={companyName} required />

							<div class="grid grid-cols-2 gap-4">
								<Input label="Phone" type="tel" bind:value={companyPhone} />
								<Input label="Email" type="email" bind:value={companyEmail} />
							</div>

							<Input label="Website" type="url" bind:value={companyWebsite} />

							<Input label="Address" bind:value={companyAddress} />

							<div class="grid grid-cols-3 gap-4">
								<Input label="City" bind:value={companyCity} />
								<Input label="State" bind:value={companyState} />
								<Input label="ZIP" bind:value={companyZip} />
							</div>
						</div>
					</Card>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Defaults</h3>
						<div class="space-y-4">
							<div class="grid grid-cols-2 gap-4">
								<Input label="Default Hourly Rate ($)" type="number" bind:value={defaultHourlyRate} />
								<Input label="Tax Rate (%)" type="number" bind:value={taxRate} />
							</div>
						</div>
					</Card>

					<div class="flex justify-end">
						<Button variant="primary">Save Changes</Button>
					</div>
				</div>
			{:else if activeTab === 'billing'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Billing & Subscription</h2>
						<p class="text-sm text-surface-500 mt-1">Manage your FieldForge subscription and payment methods.</p>
					</div>

					<Card>
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm font-medium text-surface-500">Current Plan</p>
								<p class="text-2xl font-bold text-surface-900 mt-1">Crew Plan</p>
								<p class="text-sm text-surface-500 mt-1">$79/month • Up to 10 team members</p>
							</div>
							<Button variant="outline">Upgrade Plan</Button>
						</div>
					</Card>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Payment Method</h3>
						<div class="flex items-center gap-4 p-4 bg-surface-50 rounded-lg">
							<div class="w-12 h-8 bg-surface-200 rounded flex items-center justify-center text-xs font-bold text-surface-600">VISA</div>
							<div>
								<p class="text-sm font-medium text-surface-900">•••• •••• •••• 4242</p>
								<p class="text-xs text-surface-500">Expires 12/2026</p>
							</div>
							<Button variant="ghost" size="sm" class="ml-auto">Update</Button>
						</div>
					</Card>
				</div>
			{:else if activeTab === 'profile'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Your Profile</h2>
						<p class="text-sm text-surface-500 mt-1">Manage your personal account details.</p>
					</div>
					<Card>
						<div class="flex items-center gap-6 mb-6">
							<div class="w-20 h-20 bg-forge-100 text-forge-600 rounded-full flex items-center justify-center text-2xl font-bold">
								BS
							</div>
							<div>
								<Button variant="outline" size="sm">Change Photo</Button>
								<p class="text-xs text-surface-400 mt-1">JPG, PNG or GIF. Max 2MB.</p>
							</div>
						</div>
						<div class="space-y-4">
							<div class="grid grid-cols-2 gap-4">
								<Input label="First Name" value="Billy" />
								<Input label="Last Name" value="Smith" />
							</div>
							<Input label="Email" type="email" value="billy@smithhvac.com" />
							<Input label="Phone" type="tel" value="(512) 555-0100" />
							<Input label="Job Title" value="Owner" />
						</div>
					</Card>
					<div class="flex justify-end">
						<Button variant="primary">Save Profile</Button>
					</div>
				</div>
			{:else if activeTab === 'team'}
				<div class="space-y-6">
					<div class="flex items-center justify-between">
						<div>
							<h2 class="text-lg font-semibold text-surface-900">Team Members</h2>
							<p class="text-sm text-surface-500 mt-1">Manage your crew and their roles.</p>
						</div>
						<Button variant="primary" size="md">
							<Users class="w-4 h-4" />
							Invite Member
						</Button>
					</div>

					<Card padding={false}>
						<div class="divide-y divide-surface-100">
							{#each teamMembers as member}
								<div class="flex items-center justify-between px-6 py-4">
									<div class="flex items-center gap-3">
										<div class="w-10 h-10 bg-forge-100 text-forge-600 rounded-full flex items-center justify-center text-sm font-bold">
											{member.name.split(' ').map((n: string) => n[0]).join('')}
										</div>
										<div>
											<p class="text-sm font-medium text-surface-900">{member.name}</p>
											<p class="text-xs text-surface-400">{member.email}</p>
										</div>
									</div>
									<div class="flex items-center gap-3">
										{#if member.trade}
											<span class="text-xs text-surface-400">{member.trade}</span>
										{/if}
										<span class="px-2 py-1 text-xs font-medium rounded-full {member.role === 'Owner' ? 'bg-forge-100 text-forge-700' : member.role === 'Office Manager' ? 'bg-purple-100 text-purple-700' : 'bg-surface-100 text-surface-600'}">
											{member.role}
										</span>
									</div>
								</div>
							{/each}
						</div>
					</Card>
				</div>
			{:else if activeTab === 'notifications'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Notifications</h2>
						<p class="text-sm text-surface-500 mt-1">Choose what notifications you receive and how.</p>
					</div>

					<Card padding={false}>
						<div class="px-6 py-3 border-b border-surface-200 bg-surface-50">
							<div class="grid grid-cols-[1fr_60px_60px_60px] gap-4 text-xs font-medium text-surface-500 uppercase tracking-wider">
								<span>Notification</span>
								<span class="text-center">Email</span>
								<span class="text-center">SMS</span>
								<span class="text-center">Push</span>
							</div>
						</div>
						<div class="divide-y divide-surface-100">
							{#each notifSettings as notif}
								<div class="grid grid-cols-[1fr_60px_60px_60px] gap-4 items-center px-6 py-4">
									<div>
										<p class="text-sm font-medium text-surface-900">{notif.label}</p>
										<p class="text-xs text-surface-400">{notif.description}</p>
									</div>
									<div class="flex justify-center">
										<input type="checkbox" checked={notif.email} class="w-4 h-4 rounded border-surface-300 text-forge-600 focus:ring-forge-500" />
									</div>
									<div class="flex justify-center">
										<input type="checkbox" checked={notif.sms} class="w-4 h-4 rounded border-surface-300 text-forge-600 focus:ring-forge-500" />
									</div>
									<div class="flex justify-center">
										<input type="checkbox" checked={notif.push} class="w-4 h-4 rounded border-surface-300 text-forge-600 focus:ring-forge-500" />
									</div>
								</div>
							{/each}
						</div>
					</Card>
					<div class="flex justify-end">
						<Button variant="primary">Save Preferences</Button>
					</div>
				</div>
			{:else if activeTab === 'integrations'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Integrations</h2>
						<p class="text-sm text-surface-500 mt-1">Connect FieldForge with your favorite tools.</p>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						{#each integrations as integration}
							<Card>
								<div class="flex items-start justify-between">
									<div class="flex items-start gap-3">
										<span class="text-2xl">{integration.icon}</span>
										<div>
											<p class="text-sm font-semibold text-surface-900">{integration.name}</p>
											<p class="text-xs text-surface-400 mt-0.5">{integration.description}</p>
										</div>
									</div>
									{#if integration.status === 'connected'}
										<Button variant="outline" size="sm">Manage</Button>
									{:else}
										<Button variant="primary" size="sm">Connect</Button>
									{/if}
								</div>
								{#if integration.status === 'connected'}
									<div class="mt-3 flex items-center gap-1.5">
										<div class="w-2 h-2 bg-green-500 rounded-full"></div>
										<span class="text-xs text-green-600 font-medium">Connected</span>
									</div>
								{/if}
							</Card>
						{/each}
					</div>
				</div>
			{:else if activeTab === 'templates'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Document Templates</h2>
						<p class="text-sm text-surface-500 mt-1">Customize your estimates, invoices, and communications.</p>
					</div>

					<Card padding={false}>
						<div class="divide-y divide-surface-100">
							{#each templates as template}
								<div class="flex items-center justify-between px-6 py-4 hover:bg-surface-50 transition-colors">
									<div class="flex items-center gap-3">
										<div class="w-10 h-10 rounded-lg flex items-center justify-center text-sm
											{template.type === 'estimate' || template.type === 'invoice' ? 'bg-forge-50 text-forge-600' : template.type === 'sms' ? 'bg-green-50 text-green-600' : 'bg-blue-50 text-blue-600'}">
											<FileText class="w-5 h-5" />
										</div>
										<div>
											<p class="text-sm font-medium text-surface-900">{template.name}</p>
											<p class="text-xs text-surface-400">{template.description}</p>
										</div>
									</div>
									<div class="flex items-center gap-3">
										<span class="text-xs text-surface-400">Modified {template.last_modified}</span>
										<Button variant="outline" size="sm">Edit</Button>
									</div>
								</div>
							{/each}
						</div>
					</Card>
				</div>
			{:else if activeTab === 'appearance'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Appearance</h2>
						<p class="text-sm text-surface-500 mt-1">Customize the look and feel of your dashboard.</p>
					</div>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Theme</h3>
						<div class="grid grid-cols-3 gap-4">
							{#each [
								{ id: 'light', label: 'Light', bg: 'bg-white', text: 'text-surface-900', border: 'border-forge-500' },
								{ id: 'dark', label: 'Dark', bg: 'bg-surface-900', text: 'text-white', border: 'border-surface-600' },
								{ id: 'system', label: 'System', bg: 'bg-gradient-to-r from-white to-surface-900', text: 'text-surface-600', border: 'border-surface-300' }
							] as theme}
								<button class="p-4 rounded-xl border-2 cursor-pointer transition-all {theme.id === 'light' ? theme.border : 'border-surface-200 hover:border-surface-400'}">
									<div class="w-full h-16 rounded-lg {theme.bg} border border-surface-200 mb-2"></div>
									<p class="text-sm font-medium text-surface-700">{theme.label}</p>
								</button>
							{/each}
						</div>
					</Card>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Brand Color</h3>
						<div class="flex items-center gap-4">
							{#each ['#2563eb', '#0891b2', '#059669', '#d97706', '#dc2626', '#7c3aed', '#db2777'] as color}
								<button
									class="w-10 h-10 rounded-full cursor-pointer transition-transform hover:scale-110 {color === '#2563eb' ? 'ring-2 ring-offset-2 ring-forge-500' : ''}"
									style="background-color: {color}"
								></button>
							{/each}
						</div>
					</Card>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Sidebar</h3>
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm font-medium text-surface-700">Collapsed by default</p>
								<p class="text-xs text-surface-400">Start with the sidebar collapsed on login</p>
							</div>
							<label class="relative inline-flex items-center cursor-pointer">
								<input type="checkbox" class="sr-only peer" />
								<div class="w-11 h-6 bg-surface-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-forge-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-surface-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-forge-600"></div>
							</label>
						</div>
					</Card>

					<div class="flex justify-end">
						<Button variant="primary">Save Appearance</Button>
					</div>
				</div>
			{:else if activeTab === 'security'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Security</h2>
						<p class="text-sm text-surface-500 mt-1">Manage your account security and access.</p>
					</div>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Change Password</h3>
						<div class="space-y-4 max-w-md">
							<Input label="Current Password" type="password" value="" placeholder="Enter current password" />
							<Input label="New Password" type="password" value="" placeholder="Enter new password" />
							<Input label="Confirm New Password" type="password" value="" placeholder="Confirm new password" />
						</div>
						<div class="mt-4">
							<Button variant="primary">Update Password</Button>
						</div>
					</Card>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Two-Factor Authentication</h3>
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm text-surface-700">Add an extra layer of security to your account</p>
								<p class="text-xs text-surface-400 mt-1">We'll send a code to your phone when you sign in</p>
							</div>
							<Button variant="outline" size="sm">Enable 2FA</Button>
						</div>
					</Card>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Active Sessions</h3>
						<div class="space-y-3">
								{#each sessions as session}
								<div class="flex items-center justify-between p-3 bg-surface-50 rounded-lg">
									<div>
										<p class="text-sm font-medium text-surface-900">{session.device}</p>
										<p class="text-xs text-surface-400">{session.location} • {session.time}</p>
									</div>
									{#if session.current}
										<span class="text-xs font-medium text-green-600">Current</span>
									{:else}
										<Button variant="ghost" size="sm">Revoke</Button>
									{/if}
								</div>
							{/each}
						</div>
					</Card>

					<Card class="border-red-200">
						<h3 class="text-base font-semibold text-danger-500 mb-2">Danger Zone</h3>
						<p class="text-sm text-surface-500 mb-4">Permanently delete your account and all associated data. This action cannot be undone.</p>
						<Button variant="danger" size="sm">Delete Account</Button>
					</Card>
				</div>
			{/if}
		</div>
	</div>
</div>

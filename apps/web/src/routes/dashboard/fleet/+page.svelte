<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Plus, Truck, MapPin, Fuel, Wrench, AlertTriangle, Calendar } from 'lucide-svelte';

	const vehicles = [
		{
			id: '1', name: 'Truck #1', make: 'Ford', model: 'F-250', year: 2022, vin: '1FTBF2B6XNEE12345',
			license_plate: 'TX-ABC-1234', assigned_to: 'Mike Thompson', status: 'active',
			mileage: 34500, next_service: '2025-01-15', fuel_type: 'Diesel',
			insurance_expiry: '2025-06-30', registration_expiry: '2025-03-15'
		},
		{
			id: '2', name: 'Van #1', make: 'Chevrolet', model: 'Express 2500', year: 2021, vin: '1GCWGAFG5M1234567',
			license_plate: 'TX-DEF-5678', assigned_to: 'Jake Rodriguez', status: 'active',
			mileage: 52300, next_service: '2024-12-20', fuel_type: 'Gas',
			insurance_expiry: '2025-06-30', registration_expiry: '2025-05-01'
		},
		{
			id: '3', name: 'Truck #2', make: 'Ram', model: '2500', year: 2023, vin: '3C6UR5DL8PG123456',
			license_plate: 'TX-GHI-9012', assigned_to: 'Sarah Lee', status: 'active',
			mileage: 18200, next_service: '2025-02-28', fuel_type: 'Diesel',
			insurance_expiry: '2025-06-30', registration_expiry: '2025-08-15'
		},
		{
			id: '4', name: 'Trailer #1', make: 'Carry-On', model: '6x12 Enclosed', year: 2020, vin: '4YMUL1216LN123456',
			license_plate: 'TX-JKL-3456', assigned_to: 'Unassigned', status: 'maintenance',
			mileage: 0, next_service: '2024-12-18', fuel_type: 'N/A',
			insurance_expiry: '2025-06-30', registration_expiry: '2025-01-10'
		}
	];

	function isServiceDueSoon(date: string): boolean {
		const d = new Date(date);
		const now = new Date();
		const diff = (d.getTime() - now.getTime()) / (1000 * 60 * 60 * 24);
		return diff <= 30;
	}
</script>

<TopBar title="Fleet">
	{#snippet actions()}
		<Button variant="primary" size="md">
			<Plus class="w-4 h-4" />
			Add Vehicle
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Fleet Overview Cards -->
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
		<Card>
			<div class="flex items-center gap-3">
				<div class="p-2.5 bg-forge-50 rounded-xl"><Truck class="w-5 h-5 text-forge-600" /></div>
				<div>
					<p class="text-xs text-surface-500">Total Vehicles</p>
					<p class="text-xl font-bold text-surface-900">{vehicles.length}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="p-2.5 bg-green-50 rounded-xl"><MapPin class="w-5 h-5 text-green-600" /></div>
				<div>
					<p class="text-xs text-surface-500">Active</p>
					<p class="text-xl font-bold text-surface-900">{vehicles.filter(v => v.status === 'active').length}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="p-2.5 bg-yellow-50 rounded-xl"><Wrench class="w-5 h-5 text-yellow-600" /></div>
				<div>
					<p class="text-xs text-surface-500">In Maintenance</p>
					<p class="text-xl font-bold text-surface-900">{vehicles.filter(v => v.status === 'maintenance').length}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="p-2.5 bg-red-50 rounded-xl"><AlertTriangle class="w-5 h-5 text-red-600" /></div>
				<div>
					<p class="text-xs text-surface-500">Service Due Soon</p>
					<p class="text-xl font-bold text-surface-900">{vehicles.filter(v => isServiceDueSoon(v.next_service)).length}</p>
				</div>
			</div>
		</Card>
	</div>

	<!-- Vehicle Cards -->
	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		{#each vehicles as vehicle (vehicle.id)}
			<Card hover>
				<div class="flex items-start justify-between mb-4">
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-surface-100 rounded-xl flex items-center justify-center">
							<Truck class="w-6 h-6 text-surface-600" />
						</div>
						<div>
							<h3 class="text-base font-semibold text-surface-900">{vehicle.name}</h3>
							<p class="text-sm text-surface-500">{vehicle.year} {vehicle.make} {vehicle.model}</p>
						</div>
					</div>
					{#if vehicle.status === 'active'}
						<Badge variant="success" size="sm">Active</Badge>
					{:else if vehicle.status === 'maintenance'}
						<Badge variant="warning" size="sm">Maintenance</Badge>
					{:else}
						<Badge variant="default" size="sm">{vehicle.status}</Badge>
					{/if}
				</div>

				<div class="grid grid-cols-2 gap-3 text-sm">
					<div>
						<span class="text-surface-400">Assigned To</span>
						<p class="font-medium text-surface-700">{vehicle.assigned_to}</p>
					</div>
					<div>
						<span class="text-surface-400">Mileage</span>
						<p class="font-medium text-surface-700">{vehicle.mileage > 0 ? vehicle.mileage.toLocaleString() + ' mi' : 'N/A'}</p>
					</div>
					<div>
						<span class="text-surface-400">License Plate</span>
						<p class="font-medium text-surface-700 font-mono">{vehicle.license_plate}</p>
					</div>
					<div>
						<span class="text-surface-400">Next Service</span>
						<p class="font-medium {isServiceDueSoon(vehicle.next_service) ? 'text-warning-500' : 'text-surface-700'}">
							{vehicle.next_service}
							{#if isServiceDueSoon(vehicle.next_service)}
								<AlertTriangle class="w-3.5 h-3.5 inline ml-1" />
							{/if}
						</p>
					</div>
				</div>
			</Card>
		{/each}
	</div>
</div>

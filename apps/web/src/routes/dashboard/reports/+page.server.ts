import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const period = url.searchParams.get('period') ?? 'month';

	try {
		const [jobsRes, invoicesRes, customersRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/jobs', { token, params: { limit: '100' } }),
			serverFetch<unknown[]>('/invoices', { token, params: { limit: '100' } }),
			serverFetch<unknown[]>('/customers', { token, params: { limit: '100' } })
		]);

		return {
			jobs: jobsRes.status === 'fulfilled' ? jobsRes.value.data : [],
			invoices: invoicesRes.status === 'fulfilled' ? invoicesRes.value.data : [],
			customers: customersRes.status === 'fulfilled' ? customersRes.value.data : [],
			filters: { period }
		};
	} catch {
		return {
			jobs: [],
			invoices: [],
			customers: [],
			filters: { period }
		};
	}
};

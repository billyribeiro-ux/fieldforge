import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const [jobsRes, customersRes, estimatesRes, invoicesRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/jobs', { token, params: { limit: '5' } }),
			serverFetch<unknown[]>('/customers', { token, params: { limit: '5' } }),
			serverFetch<unknown[]>('/estimates', { token, params: { status: 'sent', limit: '5' } }),
			serverFetch<unknown[]>('/invoices', { token, params: { status: 'sent', limit: '5' } })
		]);

		return {
			recentJobs: jobsRes.status === 'fulfilled' ? jobsRes.value.data : [],
			recentCustomers: customersRes.status === 'fulfilled' ? customersRes.value.data : [],
			pendingEstimates: estimatesRes.status === 'fulfilled' ? estimatesRes.value.data : [],
			outstandingInvoices: invoicesRes.status === 'fulfilled' ? invoicesRes.value.data : []
		};
	} catch {
		return {
			recentJobs: [],
			recentCustomers: [],
			pendingEstimates: [],
			outstandingInvoices: []
		};
	}
};

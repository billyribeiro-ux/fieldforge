import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const customerId = url.searchParams.get('customer_id') ?? '';
	const jobId = url.searchParams.get('job_id') ?? '';

	try {
		const [customersRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/customers', { token, params: { limit: '100' } })
		]);

		return {
			customers: customersRes.status === 'fulfilled' ? customersRes.value.data : [],
			prefill: { customerId, jobId }
		};
	} catch {
		return {
			customers: [],
			prefill: { customerId, jobId }
		};
	}
};

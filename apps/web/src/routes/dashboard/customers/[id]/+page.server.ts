import type { ServerLoad } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, params }) => {
	const token = locals.token;
	const customerId = params.id;

	try {
		const [customerRes, propertiesRes, notesRes] = await Promise.allSettled([
			serverFetch<unknown>(`/customers/${customerId}`, { token }),
			serverFetch<unknown[]>(`/customers/${customerId}/properties`, { token }),
			serverFetch<unknown[]>(`/customers/${customerId}/notes`, { token })
		]);

		const customer = customerRes.status === 'fulfilled' ? customerRes.value.data : null;

		if (!customer) {
			error(404, 'Customer not found');
		}

		return {
			customer,
			properties: propertiesRes.status === 'fulfilled' ? propertiesRes.value.data : [],
			notes: notesRes.status === 'fulfilled' ? notesRes.value.data : []
		};
	} catch (e) {
		if (e && typeof e === 'object' && 'status' in e) throw e;
		return {
			customer: null,
			properties: [],
			notes: [],
			error: e instanceof Error ? e.message : 'Failed to load customer'
		};
	}
};

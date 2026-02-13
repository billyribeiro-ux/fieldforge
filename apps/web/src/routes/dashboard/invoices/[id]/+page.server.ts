import type { ServerLoad } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, params }) => {
	const token = locals.token;
	const invoiceId = params.id;

	try {
		const [invoiceRes, paymentsRes] = await Promise.allSettled([
			serverFetch<unknown>(`/invoices/${invoiceId}`, { token }),
			serverFetch<unknown[]>(`/invoices/${invoiceId}/payments`, { token })
		]);

		const invoice = invoiceRes.status === 'fulfilled' ? invoiceRes.value.data : null;

		if (!invoice) {
			error(404, 'Invoice not found');
		}

		return {
			invoice,
			payments: paymentsRes.status === 'fulfilled' ? paymentsRes.value.data : []
		};
	} catch (e) {
		if (e && typeof e === 'object' && 'status' in e) throw e;
		return {
			invoice: null,
			payments: [],
			error: e instanceof Error ? e.message : 'Failed to load invoice'
		};
	}
};

import type { ServerLoad, Actions } from '@sveltejs/kit';
import { error, fail } from '@sveltejs/kit';
import { serverFetch, serverPost } from '$lib/api/server';

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

export const actions: Actions = {
	send: async ({ locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		try {
			await serverPost(`/invoices/${params.id}/send`, {}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to send invoice' });
		}
	},

	void: async ({ locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		try {
			await serverPost(`/invoices/${params.id}/void`, {}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to void invoice' });
		}
	},

	recordPayment: async ({ request, locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const amount = formData.get('amount') as string;
		const method = formData.get('method') as string;
		const reference = formData.get('reference') as string;

		if (!amount || isNaN(parseFloat(amount))) return fail(400, { error: 'Valid amount is required' });

		try {
			await serverPost(`/invoices/${params.id}/payments`, {
				amount: parseFloat(amount),
				method: method || 'card',
				reference: reference?.trim() || null
			}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to record payment' });
		}
	}
};

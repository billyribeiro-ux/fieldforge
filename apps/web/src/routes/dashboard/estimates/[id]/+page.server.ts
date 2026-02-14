import type { ServerLoad, Actions } from '@sveltejs/kit';
import { error, fail, isHttpError } from '@sveltejs/kit';
import { serverFetch, serverPost } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, params }) => {
	const token = locals.token;
	const estimateId = params.id;

	try {
		const { data } = await serverFetch<unknown>(`/estimates/${estimateId}`, { token });

		if (!data) {
			error(404, 'Estimate not found');
		}

		return { estimate: data };
	} catch (e) {
		if (isHttpError(e)) throw e;
		return {
			estimate: null,
			error: e instanceof Error ? e.message : 'Failed to load estimate'
		};
	}
};

export const actions: Actions = {
	send: async ({ locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		try {
			await serverPost(`/estimates/${params.id}/send`, {}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to send estimate' });
		}
	},

	approve: async ({ locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		try {
			await serverPost(`/estimates/${params.id}/approve`, {}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to approve estimate' });
		}
	},

	decline: async ({ request, locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const reason = formData.get('reason') as string;

		try {
			await serverPost(`/estimates/${params.id}/decline`, {
				reason: reason?.trim() || null
			}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to decline estimate' });
		}
	},

	convertToInvoice: async ({ locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		try {
			const { data } = await serverPost<{ id: string }>(`/estimates/${params.id}/convert`, {}, { token });
			return { success: true, invoiceId: (data as { id: string }).id };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to convert estimate' });
		}
	}
};

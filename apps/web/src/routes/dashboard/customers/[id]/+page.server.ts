import type { ServerLoad, Actions } from '@sveltejs/kit';
import { error, fail, isHttpError } from '@sveltejs/kit';
import { serverFetch, serverPatch, serverPost } from '$lib/api/server';

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
		if (isHttpError(e)) throw e;
		return {
			customer: null,
			properties: [],
			notes: [],
			error: e instanceof Error ? e.message : 'Failed to load customer'
		};
	}
};

export const actions: Actions = {
	update: async ({ request, locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const body: Record<string, unknown> = {};
		for (const [key, value] of formData.entries()) {
			if (typeof value === 'string' && value.trim()) {
				body[key] = value.trim();
			}
		}

		try {
			await serverPatch(`/customers/${params.id}`, body, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to update customer' });
		}
	},

	addProperty: async ({ request, locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const name = formData.get('name') as string;
		const property_type = formData.get('property_type') as string;
		const address_line1 = formData.get('address_line1') as string;
		const city = formData.get('city') as string;
		const state = formData.get('state') as string;
		const zip_code = formData.get('zip_code') as string;

		if (!name?.trim()) return fail(400, { error: 'Property name is required' });

		try {
			await serverPost(`/customers/${params.id}/properties`, {
				name: name.trim(),
				property_type: property_type || 'residential',
				address_line1: address_line1?.trim() || null,
				city: city?.trim() || null,
				state: state?.trim() || null,
				zip_code: zip_code?.trim() || null
			}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to add property' });
		}
	},

	addNote: async ({ request, locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const content = formData.get('content') as string;
		const is_internal = formData.get('is_internal') === 'true';

		if (!content?.trim()) return fail(400, { error: 'Note content is required' });

		try {
			await serverPost(`/customers/${params.id}/notes`, {
				content: content.trim(),
				is_internal
			}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to add note' });
		}
	}
};

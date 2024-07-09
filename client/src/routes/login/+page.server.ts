import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { login } from '@/forms';
import { zod } from 'sveltekit-superforms/adapters';

import { SRPClient } from '@windwalker-io/srp';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(login))
	};
};

export const actions: Actions = {
	default: async (event) => {
		const form = await superValidate(event, zod(login));
		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		const response = await fetch(
			'http://localhost:25052/auth/challenge' +
				new URLSearchParams({
					identity: form.data.username
				})
		);

		console.debug(response);

		// const client = SRPClient.create();

		return {
			form
		};
	}
};

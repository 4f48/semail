import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { register } from '@/forms';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(register))
	};
};

export const actions: Actions = {
	default: async (event) => {
		const form = await superValidate(event, zod(register));
		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		const response = await fetch('http://localhost:25052/auth/register', {
			method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
			body: JSON.stringify({
				address: form.data.address,
				password: form.data.password
			})
		});

		console.debug(response);

		return {
			form
		};
	}
};

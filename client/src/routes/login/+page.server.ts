import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { login } from '@/forms';
import { zod } from 'sveltekit-superforms/adapters';

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

		// const response = await fetch('http://localhost:25052/auth/login', {
		// 	method: 'POST',
		// 	headers: {
		// 		'Content-Type': 'application/json'
		// 	},
		// 	body: JSON.stringify({
		// 		username: form.data.username,
		// 		password: form.data.password
		// 	})
		// });

		login_user(form.data.username, form.data.password);

		// console.debug(response);

		return {
			form
		};
	}
};

import { SRPClient } from '@windwalker-io/srp';

// experimental srp client implementation for logining user
async function login_user(identity: string, password: string) {
	const client = SRPClient.create();
	const { salt, verifier } = await client.register(identity, password);
	console.debug(salt);
	console.debug(verifier);

	// const response = await fetch('http://localhost:25052/auth/login', {
	// 	method: 'POST',
	// 	headers: {
	// 		'Content-Type': 'application/json'
	// 	},
	// 	body: JSON.stringify({
	//		identity: identity,
	// 		salt: salt.toString(16),
	// 		verifier: verifier.toString(16)
	// 	})
	// });
}

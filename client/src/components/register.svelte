<script lang="ts">
	import * as Form from '@/components/ui/form';
	import { Input } from '@/components/ui/input';
	import { register, type Register } from '@/forms';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';

	export let data: SuperValidated<Infer<Register>>;
	const form = superForm(data, {
		validators: zodClient(register)
	});

	const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance>
	<Form.Field {form} name="username">
		<Form.Control let:attrs>
			<Form.Label>Username</Form.Label>
			<Input {...attrs} bind:value={$formData.username} type="text" />
		</Form.Control>
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control let:attrs>
			<Form.Label>Password</Form.Label>
			<Input {...attrs} bind:value={$formData.password} type="password" />
		</Form.Control>
	</Form.Field>
	<Form.Button>Register</Form.Button>
</form>

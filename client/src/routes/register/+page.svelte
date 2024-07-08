<script lang="ts">
	import type { PageData } from './$types.js';
	import Register from '#/register.svelte';
	export let data: PageData;
</script>

<Register data={data.form} />

<script>
	export let name;

	export let appWebsocket;
	export let cell_id;

	const postHash = await appWebsocket.callZome({
		cap: null as any,
		cell_id: cell_id,
		zome_name: 'HC_SCAFFOLD{zomeName}',
		fn_name: 'create_post',
		payload: 'my post',
		provenance: cell_id[1],
	});

</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
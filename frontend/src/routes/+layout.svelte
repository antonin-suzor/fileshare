<script lang="ts">
    import '../app.css';
    import Header from '$lib/components/Header.svelte';
    import Footer from '$lib/components/Footer.svelte';
    import { initAuth } from '$lib/api/auth.svelte';
    import { onMount } from 'svelte';

    let { children } = $props();

    onMount(initAuth);
</script>

<Header />

<main class="flex-1 bg-base-100">
    {@render children()}
</main>

<Footer />

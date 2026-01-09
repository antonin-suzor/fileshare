<script lang="ts">
    import { logout } from '$lib/api/auth.svelte';
    import { goto } from '$app/navigation';

    function handleLogout() {
        logout();
        goto('/');
    }
</script>

<svelte:head>
    <title>Log Out | FileShare</title>
</svelte:head>

<div class="flex justify-center-safe">Do you want to log out ?</div>
<div class="flex justify-center-safe">
    <button class="btn" onclick={handleLogout}>Log Out</button>
</div>

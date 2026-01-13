<script lang="ts">
    import { isLoggedIn, isVerified } from '$lib/api/auth.svelte';
</script>

<svelte:head>
    <title>Account | FileShare</title>
</svelte:head>

<div class="flex justify-center-safe">Hey ! What brings you here ?</div>
<div class="flex justify-center-safe">
    <a href="/account/signup" class="btn btn-wide"
        >{isLoggedIn() ? 'Sign Up (You are currently already logged in)' : 'Sign Up'}</a
    >
</div>
<div class="flex justify-center-safe">
    <a href="/account/login" class="btn btn-wide"
        >{isLoggedIn() ? 'Log In (You are currently already logged in)' : 'Log In'}</a
    >
</div>
{#if isLoggedIn()}
    <div class="flex justify-center-safe"><a href="/account/manage" class="btn btn-wide">Manage Account</a></div>
    <div class="flex justify-center-safe"><a href="/account/logout" class="btn btn-wide">Log Out</a></div>
    {#if !isVerified()}
        <div class="flex justify-center-safe">
            <a href="/account/logout" class="btn btn-wide">Verify you account email</a>
        </div>
    {/if}
{/if}

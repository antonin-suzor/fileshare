<script>
    import { isLoggedIn } from '$lib/api/auth.svelte';
</script>

<header class="navbar border-b bg-base-200">
    <div class="flex-1">
        <a href="/" id="nav-brand" class="btn text-xl btn-ghost">FileShare</a>
    </div>
    <nav class="flex-none">
        <ul class="menu menu-horizontal space-x-1">
            <li id="nav-home"><a href="/">Home</a></li>
            {#if isLoggedIn()}
                <li id="nav-app" class="rounded-lg outline"><a href="/uploads">Start Uploading</a></li>
                <li id="nav-app" class="rounded-lg outline"><a href="/account/manage">Manage Account</a></li>
                <li id="nav-logout"><a href="/account/logout">Logout</a></li>
            {:else}
                <li id="nav-signup" class="rounded-sm outline"><a href="/account/signup">Signup</a></li>
                <li id="nav-login" class="rounded-sm outline"><a href="/account/login">Login</a></li>
            {/if}
        </ul>
    </nav>
</header>

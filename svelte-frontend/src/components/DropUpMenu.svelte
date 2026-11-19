<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    export let buttonText: string;
    let menuShown = false;
    const dispatch = createEventDispatcher();

    function toggleMenu() {
        menuShown = !menuShown;
    }

    function closeMenu() {
        menuShown = false;
        dispatch('close');
    }

    function handleMenuKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') {
            closeMenu();
        }
    }
</script>

<div class="dropup" on:keydown={handleMenuKeydown}>
    <button type="button" aria-haspopup="menu" aria-expanded={menuShown} on:click={toggleMenu}>{buttonText}</button>
    {#if menuShown}
        <div class="drop-up-menu" role="menu" on:click={closeMenu}>
            <slot />
        </div>
    {/if}
</div>

<style>
    .dropup { position: relative; min-width: 0; }
    .dropup > button { width: 100%; height: 100%; }
    .drop-up-menu {
        position: absolute;
        z-index: 10001;
        left: 0;
        bottom: calc(100% + 5px);
        display: flex;
        flex-direction: column;
        gap: 4px;
        min-width: max(100%, 190px);
        max-width: min(90vw, 320px);
        padding: 6px;
        border: 1px solid var(--border-color);
        border-radius: 10px;
        background: var(--background);
        color: var(--text-color);
        box-shadow: 0 8px 28px #0008;
    }
    .drop-up-menu :global(button) {
        width: 100%;
        min-height: 38px;
        text-align: left;
        white-space: normal;
    }
</style>
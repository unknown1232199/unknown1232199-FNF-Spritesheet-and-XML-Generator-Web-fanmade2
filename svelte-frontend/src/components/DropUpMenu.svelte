<script lang="ts">
    export let buttonText: string;
    let menuShown = false;

    function toggleMenu() {
        menuShown = !menuShown;
    }

    function closeMenu() {
        menuShown = false;
    }
</script>

<div class="dropup">
    <button type="button" aria-expanded={menuShown} on:click={toggleMenu}>{buttonText}</button>
    {#if menuShown}
        <div class="drop-up-menu" role="menu" on:click={closeMenu}>
            <slot />
        </div>
    {/if}
</div>

<style>
    .dropup {
        position: relative;
        min-width: 0;
    }
    .dropup > button {
        width: 100%;
        height: 100%;
    }
    .drop-up-menu {
        position: absolute;
        z-index: 10001;
        left: 0;
        bottom: calc(100% + 5px);
        display: flex;
        flex-direction: column;
        gap: 3px;
        min-width: max(100%, 190px);
        max-width: min(90vw, 320px);
        padding: 5px;
        border: 1px solid var(--border-color);
        border-radius: 8px;
        background: var(--background);
        box-shadow: 0 8px 28px #0008;
        pointer-events: auto;
    }
    .drop-up-menu :global(button) {
        width: 100%;
        text-align: left;
    }
</style>
